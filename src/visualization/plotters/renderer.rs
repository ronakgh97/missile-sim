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
        let dir = config.trajectory_output_path(scenario_name);
        fs::create_dir_all(&dir)?;

        let filename = format!("{dir}/{guidance_name}_trajectory.png");
        let title = format!("{scenario_name} - {guidance_name}");

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
        let dir = config.metrics_dir(scenario_name);
        fs::create_dir_all(&dir)?;

        let base_name = format!("{dir}/{guidance_name}");

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
