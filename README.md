> NOTE: Ongoing holy refactor!!!

# Missile-Sim

A high-performance missile guidance system simulator supporting multiple guidance laws in 3D combat scenarios.
This lib enables exploration of proportional navigation dynamics through realistic simulations with 3D trajectory
plots, performance metrics, and machine learning dataset generation.

## Guidance Laws

This simulator implements six different guidance laws:

- **Pure Proportional Navigation (PPN)**: Classical guidance using line-of-sight rate
    - `a_c = N × V_m × λ̇`
    - N = Navigation constant (typically 3-5)

- **True Proportional Navigation (TPN)**: Accounts for closing speed in addition to LOS rate
    - `a_c = N × V_c × λ̇`
    - V_c = Closing velocity

- **Augmented Proportional Navigation (APN)**: Adds target acceleration compensation
    - `a_c = N × V_c × λ̇ + (N/2) × a_t`
    - a_t = Target acceleration (estimated)

- **Pure Pursuit (PP)**: Aims directly at target's current position
    - Simple pursuit strategy, often inefficient against maneuvering targets

- **Deviated Pursuit (DP)**: Pursuit with deviation angle correction
    - Improved pursuit with angular correction

- **Lead Pursuit (LP)**: Aims ahead of target with configurable lead factor
    - `a_c = K × (target_predicted_pos - missile_pos)`
    - K = Lead factor

## Example Usage

```rust
use missile_sim::prelude::*;
fn main() {
    let scenario = Scenario::builder("head-on-intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(100.0, 0.0, 0.0),
            max_acceleration: 30.0,
            navigation_constant: 3.0,
            max_closing_speed: 1000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(1000.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
        })
        .dt(0.01)
        .total_time(20.0)
        .hit_threshold(5.0)
        .build()
        .unwrap();

    let metrics = scenario.simulate(&GuidanceLawType::ppn());
    println!("Scenario: {}", scenario.name);
    println!("Result:   {}", metrics.console_summary());
    println!("Steps:    {}", metrics.time_history.len());
    println!();
}
```

### Example Scenarios

```rust
async fn scene_0() -> Result<Scenario> {
    ScenarioBuilder::new("Perpendicular-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(500.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 1250.0, 0.0),
            max_acceleration: 1500.0,
            navigation_constant: 5.0,
            max_closing_speed: 8000.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(-5000.0, 2000.0, 0.0),
            velocity: Vector3::new(1200.0, 0.0, 0.0),
        })
        .dt(0.00001)
        .total_time(60.0)
        .hit_threshold(10.0)
        .build()
}
```

### Scenarios plot

![Demo_1](assets/PPN_trajectory.png)
![Demo_2](assets/TPN_trajectory.png)

### Summary Metrics

![Hit Statistics](assets/Summary_1000.png)

- Out of 1000 random scenarios each, PPN achieved 389 hits, TPN 310 hits, and APN 519 hits, PP achieved 738 hits, DP 670
  hits, and LP 856 hits.

![Hit_Statistics](assets/Summary_5000.png)

- Out of 5000 random scenarios each, PPN achieved 2000 hits, TPN 1606 hits, and APN 2648 hits, PP achieved 3741 hits,
  DP 3411 hits, and LP 4279 hits.

![Hit_Statistics](assets/Summary_10000.png)

- Out of 10000 random scenarios each, PPN achieved 4031 hits, TPN 3217 hits, and APN 5412 hits, PP achieved 7473 hits,
  DP 6835 hits, and LP 8552 hits.