use crate::simulation::SimulationMetrics;

pub struct RenderConfig {
    pub output_dir: String,
    pub width: u32,
    pub height: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            output_dir: "plots".to_string(),
            width: 800,
            height: 600,
        }
    }
}

pub trait Renderer {
    fn render_trajectory_3d(
        &self,
        metrics: &SimulationMetrics,
        scenario_name: &str,
        guidance_name: &str,
        config: &RenderConfig,
    ) -> Result<String, Box<dyn std::error::Error>>;

    fn render_metrics(
        &self,
        metrics: &SimulationMetrics,
        scenario_name: &str,
        guidance_name: &str,
        config: &RenderConfig,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}
