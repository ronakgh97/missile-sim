use anyhow::Result;
use missile_sim::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> Result<()> {
    println!("Generating training data with parallel processing...\n");

    // Load scenarios
    let scenarios = load_train_data();
    let total = scenarios.len() * 1;

    // Progress counter
    let completed = AtomicUsize::new(0);

    let guidance_laws: Vec<GuidanceLawType> = vec![GuidanceLawType::PPN, GuidanceLawType::TPN];

    // Output paths
    let summary_file = "data/summary.csv";

    // PARALLEL
    scenarios.par_iter().for_each(|scenario| {
        for guidance in &guidance_laws {
            // Run simulation
            let mut engine = scenario.to_engine();
            let metrics = engine.run(guidance);

            // Export data
            let guidance_name = guidance.name();

            // if let Err(e) = metrics.export_csv(&scenario.name, guidance_name, "data/csv") {
            //     eprintln!("✗ CSV export failed: {}", e);
            // }

            /*if let Err(e) =
                metrics.export_json(&scenario.name, guidance_name, "data/json", scenario.dt)
            {
                eprintln!("✗ Metadata export failed: {}", e);
            }*/

            if let Err(e) = metrics.export_summary(&scenario.name, guidance_name, summary_file) {
                eprintln!("✗ Summary export failed: {}", e);
            }

            // Progress tracking
            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            println!(
                "[{}/{}] {} - {} {}",
                done,
                total,
                scenario.name,
                guidance_name,
                metrics.console_print()
            );
        }
    });

    Ok(())
}
