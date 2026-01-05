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
    /// Missile final position x-coordinate
    #[arg(long = "m-xf")]
    pub m_xf: f64,

    /// Missile final position y-coordinate
    #[arg(long = "m-yf")]
    pub m_yf: f64,

    /// Missile final position z-coordinate
    #[arg(long = "m-zf")]
    pub m_zf: f64,

    /// Missile initial velocity x-component
    #[arg(long = "m-vx")]
    pub m_vx: f64,

    /// Missile initial velocity y-component
    #[arg(long = "m-vy")]
    pub m_vy: f64,

    /// Missile initial velocity z-component
    #[arg(long = "m-vz")]
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
    /// Target final position x-coordinate
    #[arg(long = "t-xf")]
    pub t_xf: f64,

    /// Target final position y-coordinate
    #[arg(long = "t-yf")]
    pub t_yf: f64,

    /// Target final position z-coordinate
    #[arg(long = "t-zf")]
    pub t_zf: f64,

    /// Target initial velocity x-component
    #[arg(long = "t-vx")]
    pub t_vx: f64,

    /// Target initial velocity y-component
    #[arg(long = "t-vy")]
    pub t_vy: f64,

    /// Target initial velocity z-component
    #[arg(long = "t-vz")]
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
        #[arg(long)]
        dt: Option<f64>,

        /// Total simulation time
        #[arg(long)]
        total_time: f64,
    },

    /// Prompt input for scenario generation
    Prompt {},
}
