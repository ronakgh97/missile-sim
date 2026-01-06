use anyhow::Result;
use clap::Parser;
use missile_sim::args::{Args, Commands, MissileArgs, TargetArgs};
use missile_sim::prelude::{
    GuidanceLawType, MissileConfig, ScenarioBuilder, TargetConfig, render_trajectory_3d,
};
use nalgebra::Vector3;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Run {
            missile,
            target,
            dt,
            total_time,
        }) => {
            println!("\nMissile Args: {:#?}", missile);
            println!("Target Args: {:#?}", target);
            println!("dt: {:?}, total_time: {:?}", dt, total_time);
            run_sim(missile, target, dt, total_time).await?;
        }
        Some(Commands::Prompt { .. }) => {}
        None => {
            println!("No subcommand provided. Use --help for usage.");
        }
    }
    Ok(())
}

async fn run_sim(
    missile_args: MissileArgs,
    target_args: TargetArgs,
    dt: Option<f64>,
    total_time: Option<f64>,
) -> Result<()> {
    let missile_position = Vector3::new(missile_args.m_x, missile_args.m_y, missile_args.m_z);
    let missile_velocity = Vector3::new(missile_args.m_vx, missile_args.m_vy, missile_args.m_vz);
    let target_position = Vector3::new(target_args.t_x, target_args.t_y, target_args.t_z);
    let target_velocity = Vector3::new(target_args.t_vx, target_args.t_vy, target_args.t_vz);

    let dt = dt.unwrap_or(0.000001);
    let total_time = total_time.unwrap_or(60.0);

    // Setup scenario from args
    let scenario = ScenarioBuilder::new("Scenario")
        .missile_config(MissileConfig {
            position: missile_position.clone(),
            velocity: missile_velocity.clone(),
            max_acceleration: missile_args.m_a_max,
            navigation_constant: missile_args.m_n.unwrap_or(5.0),
            max_closing_speed: missile_args.m_v_closing_max,
        })
        .target_config(TargetConfig {
            position: target_position.clone(),
            velocity: target_velocity.clone(),
        })
        .dt(dt)
        .total_time(total_time)
        .hit_threshold(10.0)
        .build()?;

    // TODO: Maybe set args to which guidance laws to run?, for now, lets run the base three
    let guidance_laws: Vec<GuidanceLawType> = vec![
        GuidanceLawType::PPN,
        GuidanceLawType::TPN,
        GuidanceLawType::APN(1.25),
    ];

    // Output directory
    let output_dir = "plots/trajectories/cli";

    // Run simulations

    for guidance in &guidance_laws {
        print!(" Testing {} ", guidance.name());

        // Create engine and run simulation
        let mut engine = scenario.to_engine();
        let metrics = engine.run(guidance);

        // Render trajectory plot
        let _ = render_trajectory_3d(&metrics, output_dir, &scenario.name, guidance.name());

        println!("{}", metrics.console_print());
    }

    println!();

    Ok(())
}
