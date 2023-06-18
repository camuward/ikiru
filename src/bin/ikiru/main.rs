#![allow(unused)]

#[macro_use]
extern crate eyre;

#[macro_use]
extern crate tracing;

use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use eyre::Context;

use self::app::{cfg::Instance, App};
use self::cfg::CfgError;
use self::cli::{Cli, CliError};

pub mod app;
pub mod cfg;
pub mod cli;
pub mod gfx;

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let inst = read_cfg(&cli)?;

    match &cli.subcmd {
        Some(subcmd) => subcmd.exec(&cli, inst)?,
        None => start(|cc| App::new(cc, inst).unwrap()).unwrap(),
    }

    Ok(())
}

#[instrument(ret(level = tracing::Level::DEBUG))]
fn read_cfg(cli: &Cli) -> eyre::Result<Instance> {
    let cfg_dir: PathBuf = {
        let _e = debug_span!("find_cfg_dir").entered();

        // get the default config dir, or use the one specified by the user
        let get_default_dir = || dirs::config_dir().map(|path| path.join("ikiru"));
        let path = cli.cfg_dir.clone().or_else(get_default_dir);

        match path {
            None => bail!(CliError::CfgDirNotFound),
            Some(dir) if !dir.try_exists()? => {
                let _e = debug_span!("create_cfg_dir", dir = ?dir).entered();
                std::fs::create_dir_all(&dir)?;

                dir
            }
            Some(dir) if !dir.is_dir() => bail!(CliError::CfgDirNotDir(dir)),
            Some(dir) => dir,
        }
    };

    let cfg_file: PathBuf = {
        let path = cfg_dir.join("config.toml");
        if !path.try_exists()? {
            let _e = debug_span!("create_cfg_file", file = ?path).entered();
            let doc = toml_edit::Document::default();
            std::fs::write(&path, doc.to_string())?;
        }
        path
    };

    let buf = std::fs::read_to_string(&cfg_file)?;

    Ok(Instance {
        cfg_dir,
        cfg_file: File::options().read(true).write(true).open(&cfg_file)?,
        cfg: toml_edit::de::from_str(&buf).map_err(|e| CfgError::Parse(e, cfg_file))?,

        game_cfgs: Default::default(),
    })
}

/// Run the [`App`](app::App).
fn start<F, A>(app_creator: F) -> Result<(), eframe::Error>
where
    F: FnOnce(&eframe::CreationContext) -> A + 'static,
    A: eframe::App + 'static,
{
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(480.0, 360.0)),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native("ikiru", options, Box::new(|cc| Box::new(app_creator(cc))))
}
