## Missile-Sim

A missile guidance system simulator supporting multiple guidance laws in 3D combat scenarios.
This lib enables exploration of proportional navigation dynamics through realistic simulations with trajectory
plots, performance metrics, and machine learning dataset generation.

### Guidance Laws

These are some vanilla standard guidance laws:

- **Pure Proportional Navigation (PPN)**: Classical guidance using line-of-sight rate [PPN](src/guidance/ppn.rs)
    - `a_c = N × V_m × λ̇`
    - N = Navigation constant (typically 3-5)

- **True Proportional Navigation (TPN)**: Accounts for closing speed in addition to LOS rate [TPN](src/guidance/tpn.rs)
    - `a_c = N × (V_c) × λ̇`
    - V_c = Closing velocity

- **Augmented Proportional Navigation (APN)**: Adds target acceleration compensation & ZEM
  factor [APN](src/guidance/apn.rs)
    - `a_APN = a_PN + N * ZEM / T_const^2`
    - ZEM = Predicted miss distance at intercept

These below are made up, not standard, and are included for comparison:

- **Pure Pursuit (PP)**: Aims directly at target's current position
    - Simple pursuit strategy, often inefficient against maneuvering targets

- **Lead Pursuit (LP)**: Aims ahead of target with configurable lead factor
    - `a_c = K × (target_predicted_pos - missile_pos)`
    - K = Lead factor

### Usage

```rust
use missile_sim::prelude::*;

fn main() {
    let bvr_intercept = Scenario::builder("BVR Fighter Intercept")
        .missile(Missile {
            state: State3D {
                position: Vector3::new(0.0, 0.0, 10000.0),
                velocity: Vector3::new(900.0, 0.0, 0.0), // Mach ~2.6
            },
            max_acceleration: 350.0, // ~35g
            navigation_constant: 4.0,
            max_closing_speed: 1800.0,
        })
        .target(Target {
            state: State3D {
                position: Vector3::new(25000.0, 3000.0, 10500.0),
                velocity: Vector3::new(-320.0, 20.0, 0.0), // fighter cruise
            },
            acceleration: Vector3::new(0.0, 25.0, 0.0),
        })
        .dt(0.01)
        .total_time(35.0)
        .hit_threshold(10.0)
        .build()
        .except("Failed to build scenario");

    let metrics = scenario.simulate(&PureProportionalNavigation); // using ppn
    println!("Scenario: {}", scenario.name);
    println!("Result:   {}", metrics.console_summary());
    println!("Steps:    {}", metrics.time_history.len());
    println!();
}
```

Checkout [examples](./examples) for more detailed example scenarios and performance comparisons across guidance laws.

### Scenarios plot

These plot showcase the trajectories between `'dumb'` homing missiles and `'smart'` guided missile
![TPN_plot](assets/Plot_TPN.png)
![PP_plot](assets/Plot_PP.png)

### Performance Metrics

Heuristic hit performance metrics for each guidance law,
run it using `cargo bench --bench bencher -- <run_count>`

![Stats](assets/Summary_10000.png)

- LP: 83.8% hit rate (8375 hits)
- PP: 12.3% hit rate (1233 hits)
- PPN: 84.7% hit rate (8467 hits)
- APN: 61.2% hit rate (6118 hits)
- TPN: 56.6% hit rate (5661 hits)

- PPN: 573.27 average miss distance
- TPN: 1151.05 average miss distance
- APN: 881.99 average miss distance
- PP: 962.64 average miss distance
- LP: 522.06 average miss distance

- PP: 56.67 impact time
- LP: 20.46 impact time
- PPN: 19.59 impact time
- APN: 27.76 impact time
- TPN: 29.81 impact time