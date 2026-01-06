use crate::simulation::SimulationMetrics;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
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

//TODO: I DONT KNOW WHAT THE HELL IS HAPPENING, AND WHAT TO DO 😑 😑

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

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

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
        let json = serde_json::to_string_pretty(&summaries)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Export run summary data to CSV file (appends to existing file)
    pub fn export_summary_csv(
        &self,
        scenario_name: &str,
        guidance_name: &str,
        path_to_file: &str,
    ) -> Result<()> {
        let path = Path::new(path_to_file);

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
