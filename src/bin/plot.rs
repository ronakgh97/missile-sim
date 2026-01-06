use anyhow::Result;
use colored::Colorize;
use missile_sim::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[tokio::main]
async fn main() -> Result<()> {
    // Load preset scenarios
    let scenarios = load_preset_scenarios().await;

    // Define guidance laws to plot
    let guidance_laws: Vec<GuidanceLawType> = vec![
        GuidanceLawType::PPN,
        GuidanceLawType::TPN,
        GuidanceLawType::APN(2.25),
        GuidanceLawType::PP,
        GuidanceLawType::DP,
        GuidanceLawType::LP(1.75),
    ];

    // Output directories
    let trajectories_dir = "plots/trajectories";
    let metrics_dir = "plots/metrics";

    // Run simulations
    let combinations: Vec<(&Scenario, &GuidanceLawType)> = scenarios
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
        let metrics = engine.run(guidance);

        // Render trajectory plot
        let trajectory_dir = format!("{}/{}", trajectories_dir, scenario.name);
        let _ = render_trajectory_3d(&metrics, &trajectory_dir, &scenario.name, guidance.name());

        // Render metric plots
        let metric_dir = format!("{}/{}", metrics_dir, scenario.name);
        let _ = render_metrics(&metrics, &metric_dir, guidance.name());

        println!(
            "Scenario: {} | Guidance: {}\n{}\n",
            scenario.name,
            guidance.name().to_string().bright_cyan(),
            metrics.console_print()
        );
    });

    Ok(())
}
