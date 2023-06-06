//! Command-line configuration.
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long, env = "IKIRU_CONFIG_DIR")]
    config_dir: Option<PathBuf>,

    /// Add folders to the search path
    #[clap(short)]
    pub append: Vec<PathBuf>,

    /// TitleID to run at startup
    pub title: Option<String>,
}
