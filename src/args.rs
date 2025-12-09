use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "missile-sim",
    version = "1.0.0-beta",
    about = "",
    long_about = ""
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {},
}
