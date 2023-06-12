//! Command-line configuration.
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use thiserror::Error;
use tracing::Level;

use crate::app::cfg::Instance;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long, env = "IKIRU_CFG_DIR")]
    cfg_dir: Option<PathBuf>,

    /// Add folders to the search path
    #[clap(short, long)]
    pub append: Vec<PathBuf>,

    /// TitleID to run at startup
    pub title: Option<String>,
}

impl Cli {
    #[tracing::instrument(ret(level = Level::DEBUG))]
    pub fn read_cfg(&self) -> eyre::Result<Instance> {
        let cfg_dir = self.find_cfg_dir()?;
        let cfg_file = File::open(cfg_dir.join("config.toml"))?;
        let buf = std::io::read_to_string(cfg_file)?;

        Ok(Instance {
            cfg_dir,
            cfg_file,
            cfg: toml_edit::de::from_str(&buf)?,

            game_cfgs: Default::default(),
        })
    }

    #[tracing::instrument(ret(level = Level::DEBUG))]
    fn find_cfg_dir(&self) -> eyre::Result<PathBuf> {
        let default_dir = || dirs::config_dir().map(|path| path.join("ikiru"));

        match self.cfg_dir.clone().or_else(default_dir) {
            None => eyre::bail!(CliError::CfgDirNotFound),
            Some(dir) if !dir.try_exists()? => {
                tracing::debug_span!("create_cfg_dir", dir = ?dir)
                    .in_scope(|| std::fs::create_dir_all(&dir))?;

                Ok(dir)
            }
            Some(dir) if !dir.is_dir() => eyre::bail!(CliError::CfgDirNotDir(dir)),
            Some(dir) => Ok(dir),
        }
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
