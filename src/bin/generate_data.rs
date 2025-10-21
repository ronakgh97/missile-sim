use missile_sim::prelude::*;
use rayon::prelude::*;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Generating training data with parallel processing...\n");

    // Load scenarios
    let scenarios = load_train_data();
    let total = scenarios.len() * 2;

    // Progress counter
    let completed = AtomicUsize::new(0);

    let guidance_laws: Vec<Box<dyn GuidanceLaw>> = vec![
        Box::new(PureProportionalNavigation),
        Box::new(TrueProportionalNavigation),
    ];

    let config = RenderConfig::default();
    let data_dir = config.data_dir();

    // PARALLEL
    scenarios.par_iter().for_each(|scenario| {
        for guidance in &guidance_laws {
            // Run simulation
            let mut engine = scenario.to_engine();
            let metrics = engine.run(guidance.as_ref());

            // Export data
            let guidance_name = guidance.name();

            if let Err(e) = metrics.export_csv(&scenario.name, guidance_name, &data_dir) {
                eprintln!("✗ CSV export failed: {}", e);
            }

            if let Err(e) =
                metrics.export_metadata(&scenario.name, guidance_name, &data_dir, scenario.dt)
            {
                eprintln!("✗ Metadata export failed: {}", e);
            }

            if let Err(e) = metrics.export_summary(&scenario.name, guidance_name, &data_dir) {
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

    println!("   Data saved to: {}/csv/", data_dir);

    Ok(())
}
