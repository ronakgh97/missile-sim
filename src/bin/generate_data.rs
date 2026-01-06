use anyhow::Result;
use missile_sim::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> Result<()> {
    println!("\nGenerating data...\n");

    // Random data generator, auto creates data directory and summary file
    let path = PathBuf::from("data");
    let random_data_generator = RandomData::init(100, &path); //TODO: This should auto creates summary file, so parallel writing wont corrupt it
    let summary_csv = path.join("summary.csv");
    let summary_json = path.join("summary.json");

    // Define guidance laws to test
    let guidance_laws: Vec<GuidanceLawType> = vec![
        GuidanceLawType::PPN,
        GuidanceLawType::TPN,
        GuidanceLawType::APN(2.75),
    ];

    // Load scenarios
    let scenarios = random_data_generator.load_random_scenario();
    let total_run = scenarios.len() * guidance_laws.len();

    // Progress counter
    let completed = AtomicUsize::new(0);

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

            // Synchronized write to summary file

            {
                let _lock = file_mutex.lock().unwrap();
                if let Err(e) = metrics.export_summary_json(
                    &scenario.name,
                    guidance_name,
                    summary_json.to_str().unwrap(),
                ) {
                    eprintln!("✗ Summary export failed: {}", e);
                }
            }

            {
                let _lock = file_mutex.lock().unwrap();
                if let Err(e) = metrics.export_summary_csv(
                    &scenario.name,
                    guidance_name,
                    summary_csv.to_str().unwrap(),
                ) {
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
