//! Command-line configuration.
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::app::cfg::Instance;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long, env = "IKIRU_CFG_DIR")]
    pub cfg_dir: Option<PathBuf>,

    /// Add folders to the search path
    #[clap(short, long)]
    pub append: Vec<PathBuf>,

    #[clap(subcommand)]
    pub subcmd: Option<SubCmd>,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    /// Run a Wii U title
    Run {
        /// TitleId or path to game folder/archive
        title: String,
    },
    /// Link to existing Cemu installation
    Link {
        /// Path to Cemu. This folder should contain `Cemu.exe` and
        /// `settings.xml`.
        path: PathBuf,
    },
    /// Update ikiru to the latest version
    Update,
    /// Convert a Wii U disc image to a WUD or WUX
    Convert {
        /// Input path. May be a file or directory.
        /// 
        /// # Extensions
        /// 
        /// - `.wud`: Wii U disc image
        /// - `.wux`: Wii U disc image (compressed)
        input: PathBuf,
        /// Output path. If no file extension is provided, the output will be
        /// a directory.
        output: PathBuf,
    }
}

impl SubCmd {
    pub fn exec(&self, cli: &Cli, instance: Box<Instance>) -> eyre::Result<()> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("cfg directory {0} is not a directory")]
    CfgDirNotDir(PathBuf),
    #[error(
        "could not find cfg directory. please manually specify it with --cfg-dir or set the \
        IKIRU_CFG_DIR environment variable"
    )]
    CfgDirNotFound,
}
