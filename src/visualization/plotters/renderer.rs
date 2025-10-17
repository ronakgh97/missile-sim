use crate::simulation::SimulationMetrics;
use crate::visualization::render::{RenderConfig, Renderer};
use std::fs;

pub struct PlottersRenderer;

impl PlottersRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for PlottersRenderer {
    fn render_trajectory_3d(
        &self,
        metrics: &SimulationMetrics,
        scenario_name: &str,
        guidance_name: &str,
        config: &RenderConfig,
    ) -> Result<String, Box<dyn std::error::Error>> {
        fs::create_dir_all(&config.output_dir)?;

        let filename = format!(
            "{}/{}_{}_3d_trajectory.png",
            config.output_dir, scenario_name, guidance_name
        );

        let title = format!("{} - {}", scenario_name, guidance_name);

        super::trajectory_3d::plot_3d_trajectory(
            metrics,
            &filename,
            &title,
            config.width,
            config.height,
        )?;

        Ok(filename)
    }

    fn render_metrics(
        &self,
        metrics: &SimulationMetrics,
        scenario_name: &str,
        guidance_name: &str,
        config: &RenderConfig,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        fs::create_dir_all(&config.output_dir)?;

        let base_name = format!("{}/{}_{}", config.output_dir, scenario_name, guidance_name);

        super::charts::plot_all_metrics(
            metrics,
            &base_name,
            config.width * 2 / 3,
            config.height * 2 / 3,
        )
    }
}

impl Default for PlottersRenderer {
    fn default() -> Self {
        Self::new()
    }
}
