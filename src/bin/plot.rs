use anyhow::Result;
use missile_sim::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn main() -> Result<()> {
    // Load preset scenarios
    let scenarios = load_preset_scenarios();

    // Define guidance laws to plots
    let guidance_laws: Vec<Box<dyn GuidanceLaw>> = vec![
        Box::new(PureProportionalNavigation),
        Box::new(TrueProportionalNavigation),
        Box::new(AugmentedProportionalNavigation::new(1.0)),
        Box::new(PurePursuit),
        Box::new(DeviatedPursuit),
        Box::new(LeadPursuit::new(1.0)),
    ];

    // Configure renderer
    let renderer = PlottersRenderer::new();
    let config = RenderConfig::default();

    // Run simulations
    let combinations: Vec<(&Scenario, &Box<dyn GuidanceLaw>)> = scenarios
        .iter()
        .flat_map(|scenario| {
            guidance_laws
                .iter()
                .map(move |guidance| (scenario, guidance))
        })
        .collect();

    combinations.par_iter().for_each(|(scenario, guidance)| {
        // Create engine and run simulation
        let mut engine = scenario.to_engine();
        let metrics = engine.run(guidance.as_ref());

        // Render plots
        #[allow(unused)]
        let trajectory_file =
            renderer.render_trajectory_3d(&metrics, &scenario.name, guidance.name(), &config);

        // Render Metrics
        #[allow(unused)]
        let metric_files =
            renderer.render_metrics(&metrics, &scenario.name, guidance.name(), &config);

        //let data_dir = config.data_dir();
        //metrics.export_csv(&scenario.name, guidance.name(), &data_dir)?;
        //metrics.export_metadata(&scenario.name, guidance.name(), &data_dir, scenario.dt)?;
        //metrics.export_summary(&scenario.name, guidance.name(), &data_dir)?;

        println!("{}", metrics.console_print());
        //println!("  Trajectory plot: {}", _traj_file);
        //println!("  Metrics: {:?} files", _metric_files.len());
    });

    Ok(())
}
