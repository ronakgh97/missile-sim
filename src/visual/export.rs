use crate::simulation::SimulationMetrics;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct SimulationMetadata {
    pub scenario_name: String,
    pub guidance_law: String,
    pub duration: f64,
    pub miss_distance: f64,
    pub hit: bool,
    pub timesteps: usize,
}

impl SimulationMetrics {
    /// Export run summary data to JSON file (appends to array)
    #[allow(unused)]
    pub fn export_summary_json(
        &self,
        scenario_name: &str,
        guidance_name: &str,
        path_to_file: &str,
    ) -> Result<()> {
        let path = Path::new(path_to_file);

        // Build metadata
        let metadata = SimulationMetadata {
            scenario_name: scenario_name.to_string(),
            guidance_law: guidance_name.to_string(),
            duration: *self.time_history.last().unwrap_or(&0.0),
            miss_distance: self.miss_distance,
            hit: self.hit,
            timesteps: self.time_history.len(),
        };

        // Read existing JSON array if present
        let mut summaries: Vec<SimulationMetadata> = if path.exists() {
            let data = fs::read_to_string(path)?;
            if data.trim().is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
            }
        } else {
            Vec::new()
        };

        // Append new summary
        summaries.push(metadata);

        // Write JSON array
        let mut file = fs::OpenOptions::new().write(true).append(true).open(path)?;
        let json = serde_json::to_string_pretty(&summaries)?;
        writeln!(file, "{}", json)?;

        Ok(())
    }

    /// Export run summary data to CSV file (appends to existing file)
    pub fn export_summary_csv(
        &self,
        scenario_name: &str,
        guidance_name: &str,
        path_to_file: &str,
    ) -> Result<()> {
        let mut writer = csv::WriterBuilder::new()
            .has_headers(false) // Disable headers since they should be initialized before parallel execution
            .from_writer(
                fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path_to_file)?,
            );

        writer.write_record(&[
            scenario_name,
            guidance_name,
            &format!("{:.2}", self.time_history.last().unwrap_or(&0.0)),
            &format!("{:.2}", self.miss_distance),
            &format!("{}", if self.hit { 1 } else { 0 }),
            &format!("{}", self.time_history.len()),
        ])?;

        writer.flush()?;
        Ok(())
    }
}
