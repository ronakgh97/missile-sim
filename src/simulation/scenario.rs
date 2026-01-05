use crate::entity::{Missile, MissileConfig, Target, TargetConfig};
use crate::simulation::engine::SimulationEngine;
use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Scenario {
    pub name: String,
    pub missile_config: MissileConfig,
    pub target_config: TargetConfig,
    pub dt: f64,
    pub total_time: f64,
    pub hit_threshold: f64,
}

impl Scenario {
    pub fn to_engine(&self) -> SimulationEngine {
        let missile = Missile::new(self.missile_config.clone());
        let target = Target::new(self.target_config.clone());

        SimulationEngine {
            missile,
            target,
            time: 0.0,
            dt: self.dt,
            max_time: self.total_time,
            hit_threshold: self.hit_threshold,
        }
    }
}

pub struct ScenarioBuilder {
    name: String,
    missile_config: Option<MissileConfig>,
    target_config: Option<TargetConfig>,
    dt: f64,
    total_time: f64,
    hit_threshold: f64,
}

impl ScenarioBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            missile_config: None,
            target_config: None,
            dt: 0.01,
            total_time: 60.0,
            hit_threshold: 5.0,
        }
    }

    pub fn missile_config(mut self, config: MissileConfig) -> Self {
        self.missile_config = Some(config);
        self
    }

    pub fn target_config(mut self, config: TargetConfig) -> Self {
        self.target_config = Some(config);
        self
    }

    pub fn dt(mut self, dt: f64) -> Self {
        self.dt = dt;
        self
    }

    pub fn total_time(mut self, total_time: f64) -> Self {
        self.total_time = total_time;
        self
    }

    pub fn hit_threshold(mut self, threshold: f64) -> Self {
        self.hit_threshold = threshold;
        self
    }

    pub fn build(self) -> Result<Scenario> {
        Ok(Scenario {
            name: self.name,
            missile_config: self.missile_config.context("missile_config is required")?,
            target_config: self.target_config.context("target_config is required")?,
            dt: self.dt,
            total_time: self.total_time,
            hit_threshold: self.hit_threshold,
        })
    }
}
