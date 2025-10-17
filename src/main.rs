use missile_sim::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Missile Guidance Simulation\n");

    // Load preset scenarios
    let scenarios = load_preset_scenarios();

    // Define guidance laws to plots
    let guidance_laws: Vec<Box<dyn GuidanceLaw>> = vec![
        Box::new(PureProportionalNavigation),
        Box::new(TrueProportionalNavigation),
    ];

    // Configure renderer
    let renderer = PlottersRenderer::new();
    let config = RenderConfig::default();

    // Run simulations
    for scenario in &scenarios {
        println!(" Scenario: {}", scenario.name);

        for guidance in &guidance_laws {
            print!("    Testing {}... ", guidance.name());

            // Create engine and run simulation
            let mut engine = scenario.to_engine();
            let metrics = engine.run(guidance.as_ref());

            // Render plots
            let traj_file = renderer.render_trajectory_3d(
                &metrics,
                &scenario.name,
                guidance.name(),
                &config,
            )?;

            let metric_files =
                renderer.render_metrics(&metrics, &scenario.name, guidance.name(), &config)?;

            println!("{}", metrics.console_print());
            println!("  Trajectory plot: {}", traj_file);
            println!("  Metrics: {:?} files", metric_files);
        }

        println!();
    }
    Ok(())
}
