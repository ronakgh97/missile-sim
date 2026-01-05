use clap::Parser;
use missile_sim::args::{Args, Commands, MissileArgs, TargetArgs};

#[tokio::main]
async fn main() {
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
            println!("dt: {:?}, total_time: {}", dt, total_time);
        }

        Some(Commands::Prompt { .. }) => {}
        None => {
            println!("No subcommand provided. Use --help for usage.");
        }
    }
}

#[allow(unused)]
async fn run_sim(
    missile_args: MissileArgs,
    target_args: TargetArgs,
    dt: Option<f64>,
    total_time: f64,
) {
}
