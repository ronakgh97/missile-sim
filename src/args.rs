#[allow(unused)]
use clap::builder::ValueParser;
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

/// Missile simulation command line arguments
#[derive(Parser, Debug, Clone)]
pub struct MissileArgs {
    /// Missile initial position x-coordinate
    #[arg(long = "m-x", allow_negative_numbers = true)]
    pub m_x: f64,

    /// Missile initial position y-coordinate
    #[arg(long = "m-y", allow_negative_numbers = true)]
    pub m_y: f64,

    /// Missile initial position z-coordinate
    #[arg(long = "m-z", allow_negative_numbers = true)]
    pub m_z: f64,

    /// Missile initial velocity x-component
    #[arg(long = "m-vx", allow_negative_numbers = true)]
    pub m_vx: f64,

    /// Missile initial velocity y-component
    #[arg(long = "m-vy", allow_negative_numbers = true)]
    pub m_vy: f64,

    /// Missile initial velocity z-component
    #[arg(long = "m-vz", allow_negative_numbers = true)]
    pub m_vz: f64,

    /// Missile maximum acceleration
    #[arg(long = "m-a-max")]
    pub m_a_max: f64,

    /// Missile navigation constant
    #[arg(long = "m-n")]
    pub m_n: Option<f64>,

    /// Missile maximum closing velocity
    #[arg(long = "m-v-closing-max")]
    pub m_v_closing_max: f64,
}

/// Target simulation command line arguments
#[derive(Parser, Debug, Clone)]
pub struct TargetArgs {
    /// Target initial position x-coordinate
    #[arg(long = "t-x", allow_negative_numbers = true)]
    pub t_x: f64,

    /// Target initial position y-coordinate
    #[arg(long = "t-y", allow_negative_numbers = true)]
    pub t_y: f64,

    /// Target initial position z-coordinate
    #[arg(long = "t-z", allow_negative_numbers = true)]
    pub t_z: f64,

    /// Target initial velocity x-component
    #[arg(long = "t-vx", allow_negative_numbers = true)]
    pub t_vx: f64,

    /// Target initial velocity y-component
    #[arg(long = "t-vy", allow_negative_numbers = true)]
    pub t_vy: f64,

    /// Target initial velocity z-component
    #[arg(long = "t-vz", allow_negative_numbers = true)]
    pub t_vz: f64,
}

/// Command line subcommands
#[derive(Subcommand)]
pub enum Commands {
    Run {
        /// Missile parameters
        #[command(flatten)]
        missile: MissileArgs,

        /// Target parameters
        #[command(flatten)]
        target: TargetArgs,

        /// Time step (optional)
        //#[arg(long, value_parser = ValueParser::new(|v: &str| v.parse::<f64>().map_err(|_| "Invalid number")))]
        #[arg(long)]
        dt: Option<f64>,

        /// Total simulation time
        //#[arg(long, value_parser = ValueParser::new(|v: &str| v.parse::<f64>().map_err(|_| "Invalid number")))]
        #[arg(long)]
        total_time: Option<f64>,
    },

    /// Prompt input for scenario generation
    Prompt {},
}
