# Missile Simulation

A simple project that simulates missile guidance systems in various combat scenarios.
This tool helps you explore the dynamics of proportional navigation laws by running realistic simulations and
visualizing the results through 3D plots, performance metrics and generate data model training.

## Features

- **Multiple Guidance Laws**: Supports Pure Proportional Navigation (PPN) and True Proportional Navigation (TPN).
- **Diverse Scenarios**: Includes scenarios like head-on intercepts, tail chases, air strikes, and more.
- **Physics without Assumption**: Uses basic kinematics with configurable missile and target parameters.
- **Visualization**: Generates trajectory plots and metric charts using Plotters.
- **Train Data Generation**: Outputs data suitable for training machine learning models.
- **Extensible**: Easy to add new guidance laws, scenarios, or renderers.

## Prerequisites

- Rust (edition 2024 or later)
- Cargo (Rust's package manager)

## Testing the Simulation

   ```bash
   git clone https://github.com/ronakgh97/missile-sim.git
   cd missile-sim
   cargo build --release
   cargo run --release
   ```

- You add more preset scenarios or modify existing ones in the `src/scenarios/`
  directory, [Presets](src/scenarios/presets.rs), Consider setting the dt to `1000 hz ~ 0.0001s` from high speed
  missile,

- The program will simulate each scenario with both PPN and TPN, printing progress to the console and saving plots to
  the
  `plots/` directory.

## Demo Scenarios

The simulation includes several preset scenarios, each with unique missile and target configurations:

![Demo_1](PPN_trajectory.png)
![Demo_2](TPN_trajectory.png)

### Predefined Scenarios

```rust
pub fn load_preset_scenarios() -> Vec<Scenario> {
    vec![
        test_0(),
        test_1(),
        test_2(),
        test_3(),
        test_4(),
        test_5(),
        test_6(),
        test_7(),
    ]
}

fn test_0() -> Scenario {
    ScenarioBuilder::new("Perpendicular-Intercept")
        .missile_config(MissileConfig {
            position: Vector3::new(500.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 700.0, 0.0),
            max_acceleration: 1200.0,
            navigation_constant: 4.0,
            max_closing_speed: 800.0,
        })
        .target_config(TargetConfig {
            position: Vector3::new(-5000.0, 2000.0, 0.0),
            velocity: Vector3::new(500.0, 0.0, 0.0),
        })
        .dt(0.0001) // 1000 Hz update rate
        .total_time(30.0)
        .hit_threshold(5.0)
        .build()
}
```


## Guidance Laws

- **Pure Proportional Navigation (PPN)**: Uses line-of-sight rate for guidance.
- **True Proportional Navigation (TPN)**: Accounts for closing speed in addition to LOS rate.

## Plots and Outputs

For each scenario and guidance law combination, the simulation generates:

- **3D Trajectory Plot**: Shows the paths of the missile and target in 3D space.
- **Metrics Plots**: Charts for acceleration, closing speed, distance, and line-of-sight rate over time.
- **Console Output**: Summary of simulation results, including hit/miss status and key metrics.

### For Example

```bash
 Scenario: Perpendicular-Intercept
    Testing PPN... Travel Duration: 5.54 | Miss Distance: 4.98 | Hit: 1
    Testing TPN... Travel Duration: 5.48 | Miss Distance: 4.96 | Hit: 1

 Scenario: VTOL-Urban-Strike
    Testing PPN... Travel Duration: 2.64 | Miss Distance: 4.92 | Hit: 1
    Testing TPN... Travel Duration: 2.63 | Miss Distance: 4.95 | Hit: 1

 Scenario: Jet-Head-On-Intercept
    Testing PPN... Travel Duration: 2.68 | Miss Distance: 4.90 | Hit: 1
    Testing TPN... Travel Duration: 2.68 | Miss Distance: 4.92 | Hit: 1

 Scenario: Ground-Attack-Intercept
    Testing PPN... Travel Duration: 4.99 | Miss Distance: 4.99 | Hit: 1
    Testing TPN... Travel Duration: 4.99 | Miss Distance: 4.94 | Hit: 1

 Scenario: Spiral-Evasion
    Testing PPN... Travel Duration: 6.21 | Miss Distance: 4.99 | Hit: 1
    Testing TPN... Travel Duration: 6.25 | Miss Distance: 4.99 | Hit: 1

 Scenario: Terrain-Hugging-Chase
    Testing PPN... Travel Duration: 5.59 | Miss Distance: 4.99 | Hit: 1
    Testing TPN... Travel Duration: 5.62 | Miss Distance: 4.97 | Hit: 1

 Scenario: Hypersonic-Intercept
    Testing PPN... Travel Duration: 2.11 | Miss Distance: 4.81 | Hit: 1
    Testing TPN... Travel Duration: 2.11 | Miss Distance: 4.93 | Hit: 1

 Scenario: Cinematic-Perpendicular
    Testing PPN... Travel Duration: 5.00 | Miss Distance: 5.00 | Hit: 1
    Testing TPN... Travel Duration: 5.00 | Miss Distance: 4.97 | Hit: 1
```

All plots,data,metrics are saved in the `plots/` directory.

## Data Points

## Extracted Data Point

```rust
#[derive(Serialize, Deserialize)]
pub struct SimulationDataPoint {
    pub time: f64,
    pub missile_x: f64,
    pub missile_y: f64,
    pub missile_z: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub distance: f64,
    pub acceleration: f64,
    pub los_rate: f64,
    pub closing_speed: f64,
    pub hit: bool,
}
```

Sample CSV

```csv
time,missile_x,missile_y,missile_z,target_x,target_y,target_z,distance,acceleration,los_rate,closing_speed,hit
0,500,0,0,-5000,0,2000,5852.349955359812,0,0.0832116788321168,709.1168559049115,1
0.0001,499.99999763972784,0,0.07,-4999.95,0,2000,5852.279041658663,236.02721627199978,0.08321231714816434,709.1349853735624,1
0.0002,499.99999291910507,0,0.1399999999204133,-4999.9,0,2000,5852.208126144515,236.03506121505495,0.08321295539689638,709.153115361552,1
0.00030000000000000003,499.99998583805325,0,0.20999999968164523,-4999.849999999999,0,2000,5852.137208817314,236.04290623260368,0.08321359357830214,709.1712458688597,1
0.0004,499.99997639649393,0,0.27999999920409324,-4999.799999999999,0,2000,5852.066289677011,236.0507513245979,0.08321423169237087,709.189376895465,1
0.0005,499.9999645943487,0,0.3499999984081468,-4999.749999999999,0,2000,5851.995368723552,236.0585964909894,0.08321486973909183,709.2075084413473,1
0.0006000000000000001,499.99995043153905,0,0.41999999721418735,-4999.699999999999,0,2000,5851.924445956885,236.06644173173027,0.08321550771845428,709.2256405064858,1
0.0007000000000000001,499.9999339079866,0,0.48999999554258866,-4999.649999999999,0,2000,5851.85352137696,236.0742870467726,0.08321614563044744,709.2437730908599,1
0.0008000000000000001,499.99991502361286,0,0.5599999933137163,-4999.5999999999985,0,2000,5851.782594983724,236.08213243606806,0.08321678347506059,709.2619061944491,1
0.0009000000000000002,499.9998937783394,0,0.6299999904479281,-4999.549999999998,0,2000,5851.711666777125,236.08997789956882,0.08321742125228293,709.2800398172327,1
0.0010000000000000002,499.99987017208787,0,0.6999999868655737,-4999.499999999998,0,2000,5851.640736757111,236.09782343722668,0.08321805896210376,709.29817395919,1
0.0011000000000000003,499.9998442047798,0,0.7699999824869951,-4999.449999999998,0,2000,5851.569804923631,236.10566904899355,0.08321869660451228,709.3163086203002,1
0.0012000000000000003,499.9998158763367,0,0.8399999772325262,-4999.399999999998,0,2000,5851.498871276632,236.11351473482145,0.08321933417949776,709.3344438005427,1
0.0013000000000000004,499.9997851866802,0,0.9099999710224929,-4999.349999999998,0,2000,5851.427935816062,236.12136049466213,0.08321997168704942,709.352579499897,1
0.0014000000000000004,499.99975213573185,0,0.9799999637772132,-4999.299999999997,0,2000,5851.356998541869,236.1292063284676,0.08322060912715652,709.3707157183424,1
0.0015000000000000005,499.99971672341326,0,1.0499999554169974,-4999.249999999997,0,2000,5851.2860594540025,236.13705223618973,0.08322124649980829,709.3888524558579,1
0.0016000000000000005,499.99967894964595,0,1.1199999458621475,-4999.199999999997,0,2000,5851.21511855241,236.1448982177803,0.083221883804994,709.4069897124227,1
0.0017000000000000006,499.99963881435156,0,1.1899999350329578,-4999.149999999997,0,2000,5851.144175837038,236.15274427319125,0.08322252104270285,709.4251274880166,1
0.0018000000000000006,499.9995963174517,0,1.2599999228497145,-4999.099999999997,0,2000,5851.073231307836,236.16059040237445,0.0832231582129241,709.4432657826187,1
```

- Data points are saved as CSV and JSON in the `plots/data/` directory for each scenario and guidance law.

# Hire me EA 🥲🙏