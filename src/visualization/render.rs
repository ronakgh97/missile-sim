use crate::simulation::SimulationMetrics;

pub struct RenderConfig {
    pub base_output_dir: String,
    pub width: u32,
    pub height: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            base_output_dir: "plots".to_string(),
            width: 1280,
            height: 900,
        }
    }
}

impl RenderConfig {
    pub fn trajectory_output_path(&self, scenario_name: &str) -> String {
        format!("{}/trajectories/{}", self.base_output_dir, scenario_name,)
    }

    pub fn metrics_dir(&self, scenario_name: &str) -> String {
        format!("{}/metrics/{}", self.base_output_dir, scenario_name,)
    }

    pub fn data_dir(&self) -> String {
        format!("{}/data", self.base_output_dir,)
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
