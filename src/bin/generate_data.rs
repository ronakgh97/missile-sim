use anyhow::Result;
use missile_sim::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> Result<()> {
    println!("\nGenerating data...\n");

    // Define guidance laws to test
    let guidance_laws: Vec<GuidanceLawType> = vec![
        GuidanceLawType::PPN,
        GuidanceLawType::TPN,
        GuidanceLawType::APN(2.75),
    ];

    // Load scenarios
    let scenarios = load_random_scenario(500);
    let total_run = scenarios.len() * guidance_laws.len();

    // Progress counter
    let completed = AtomicUsize::new(0);

    // Output paths
    let summary_file = "data/summary.csv";

    // Initialize summary file with header
    std::fs::create_dir_all("data")?;
    let mut file = File::create(summary_file)?;
    writeln!(
        file,
        "scenario,guidance_law,duration,miss_distance,hit,timesteps"
    )?;
    drop(file);

    // Mutex for synchronized file writes
    let file_mutex = Mutex::new(());

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

            // Synchronized write to summary file
            {
                let _lock = file_mutex.lock().unwrap();
                if let Err(e) = metrics.export_summary(&scenario.name, guidance_name, summary_file)
                {
                    eprintln!("✗ Summary export failed: {}", e);
                }
            }

            // Progress tracking
            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            println!(
                "[{}/{}] {} - {} {}",
                done,
                total_run,
                scenario.name,
                guidance_name,
                metrics.console_print()
            );
        }
    });

    Ok(())
}
