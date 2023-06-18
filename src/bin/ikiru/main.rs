#![allow(unused)]

#[macro_use]
extern crate eyre;

#[macro_use]
extern crate tracing;

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::PathBuf;
use std::sync::Arc;

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

    // read the config
    let mut inst = Instance::try_from(&cli)?;

    // grab a handle to the config file
    let cfg_file = File::options()
        .read(true)
        .write(true)
        .open(&inst.cfg_file)?;

    match &cli.subcmd {
        Some(subcmd) => subcmd.exec(&cli, inst)?,
        None => {
            let (tx, rx) = oneshot::channel();

            start(|cc| Box::new(App::new(cc, inst, tx).unwrap())).unwrap();

            let inst = rx.recv().unwrap();

            // write the updated config
            // std::fs::write(
            //     inst.cfg_file,
            //     toml_edit::ser::to_string_pretty(&inst.borrow().cfg)?,
            // )?;
        }
    }

    Ok(())
}

/// Run the [`App`](app::App).
fn start<F>(app_creator: F) -> Result<(), eframe::Error>
where
    F: FnOnce(&eframe::CreationContext) -> Box<dyn eframe::App> + 'static,
{
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(480.0, 360.0)),
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native("ikiru", options, Box::new(|cc| app_creator(cc)))
}
