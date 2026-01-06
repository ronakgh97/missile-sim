use crate::simulation::SimulationMetrics;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct SimulationDataPoint {
    pub time: f64,
    pub missile_x: f64,
    pub missile_y: f64,
    pub missile_z: f64,
    pub missile_vx: f64,
    pub missile_vy: f64,
    pub missile_vz: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub target_vx: f64,
    pub target_vy: f64,
    pub target_vz: f64,
    pub distance: f64,
    pub acceleration: f64,
    pub los_rate: f64,

    /// Closing speed (rate of range decrease) - recorded for all guidance laws
    /// for analysis purposes. Used directly by TPN, recorded for PPN comparison.
    pub closing_speed: f64,
    pub hit: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SimulationMetadata {
    pub scenario_name: String,
    pub guidance_law: String,
    pub missie_loc: Vec<[f64; 3]>,
    pub target_loc: Vec<[f64; 3]>,
    pub duration: f64,
    pub miss_distance: f64,
    pub hit: bool,
    pub hit_timesteps: usize,
    pub dt: f64,
}

impl SimulationMetrics {
    /// Export to CSV for ML training and JSON metadata
    fn build_data_points(&self) -> Vec<SimulationDataPoint> {
        (0..self.time_history.len())
            .map(|i| SimulationDataPoint {
                time: self.time_history[i],
                missile_x: self.missile_trajectory[i].x,
                missile_y: self.missile_trajectory[i].y,
                missile_z: self.missile_trajectory[i].z,
                missile_vx: self.missile_velocity[i].x,
                missile_vy: self.missile_velocity[i].y,
                missile_vz: self.missile_velocity[i].z,
                target_x: self.target_trajectory[i].x,
                target_y: self.target_trajectory[i].y,
                target_z: self.target_trajectory[i].z,
                target_vx: self.target_velocity[i].x,
                target_vy: self.target_velocity[i].y,
                target_vz: self.target_velocity[i].z,
                distance: self.distance_history[i],
                acceleration: self.acceleration_history[i],
                los_rate: self.los_rate_history[i],
                closing_speed: self.closing_speed_history[i],
                hit: self.hit,
            })
            .collect()
    }

    /// Export to CSV using SimulationDataPoint
    pub fn export_csv(
        &self,
        scenario_name: &str,
        guidance_name: &str,
        output_dir: &str,
    ) -> Result<String> {
        fs::create_dir_all(output_dir)?;

        let filename = format!("{output_dir}/{scenario_name}_{guidance_name}.csv");
        let mut file = File::create(&filename)?;

        // Write CSV header
        writeln!(
            file,
            "time,missile_x,missile_y,missile_z,missile_vx,missile_vy,missile_vz,target_x,target_y,target_z,target_vx,target_vy,target_vz,distance,acceleration,los_rate,closing_speed,hit"
        )?;

        // Build data points
        let data_points = self.build_data_points();

        // Write rows
        for dp in &data_points {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                dp.time,
                dp.missile_x,
                dp.missile_y,
                dp.missile_z,
                dp.missile_vx,
                dp.missile_vy,
                dp.missile_vz,
                dp.target_x,
                dp.target_y,
                dp.target_z,
                dp.target_vx,
                dp.target_vy,
                dp.target_vz,
                dp.distance,
                dp.acceleration,
                dp.los_rate,
                dp.closing_speed,
                if dp.hit { 1 } else { 0 },
            )?;
        }

        Ok(filename)
    }

    #[allow(unused)] // Who the hell uses JSEN these days...
    /// Export metadata JSON using SimulationDataPoint
    pub fn export_json(
        &self,
        scenario_name: &str,
        guidance_name: &str,
        output_dir: &str,
        dt: f64,
    ) -> Result<String> {
        fs::create_dir_all(output_dir)?;

        let data_points = self.build_data_points();

        // Extract missile and target locations for metadata arrays
        let missile_locs: Vec<[f64; 3]> = data_points
            .iter()
            .map(|dp| [dp.missile_x, dp.missile_y, dp.missile_z])
            .collect();

        let target_locs: Vec<[f64; 3]> = data_points
            .iter()
            .map(|dp| [dp.target_x, dp.target_y, dp.target_z])
            .collect();

        let metadata = SimulationMetadata {
            scenario_name: scenario_name.to_string(),
            guidance_law: guidance_name.to_string(),
            missie_loc: missile_locs,
            target_loc: target_locs,
            duration: *self.time_history.last().unwrap_or(&0.0),
            miss_distance: self.miss_distance,
            hit: self.hit,
            hit_timesteps: self.time_history.len(),
            dt,
        };

        let filename = format!("{output_dir}/{scenario_name}_{guidance_name}.json");
        let json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&filename, json)?;

        Ok(filename)
    }

    /// Append summary to CSV (for comparing all runs)
    pub fn export_summary(
        &self,
        scenario_name: &str,
        guidance_name: &str,
        summary_file: &str,
    ) -> Result<()> {
        let path = Path::new(summary_file);

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Append data (header should be initialized before parallel execution)
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        writeln!(
            file,
            "{},{},{:.2},{:.2},{},{}",
            scenario_name,
            guidance_name,
            self.time_history.last().unwrap_or(&0.0),
            self.miss_distance,
            if self.hit { 1 } else { 0 },
            self.time_history.len(),
        )?;

        Ok(())
    }
}
