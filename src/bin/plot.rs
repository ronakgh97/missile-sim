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
        Box::new(AugmentedProportionalNavigation::new(2.25)),
        Box::new(PurePursuit),
        Box::new(DeviatedPursuit),
        Box::new(LeadPursuit::new(1.75)),
    ];

    // Output directories
    let trajectories_dir = "plots/trajectories";
    let metrics_dir = "plots/metrics";

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

        // Render trajectory plot
        let trajectory_dir = format!("{}/{}", trajectories_dir, scenario.name);
        let _ = render_trajectory_3d(&metrics, &trajectory_dir, &scenario.name, guidance.name());

        // Render metric plots
        let metric_dir = format!("{}/{}", metrics_dir, scenario.name);
        let _ = render_metrics(&metrics, &metric_dir, guidance.name());

        println!("{}", metrics.console_print());
    });

    Ok(())
}
