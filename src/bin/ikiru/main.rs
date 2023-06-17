use clap::Parser;
use cli::Cli;

mod app;
mod cfg;
mod cli;
mod gfx;

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let inst = read_cfg(&cli)?;

    match cli.subcmd {
        Some(subcmd) => subcmd.exec(&cli, inst)?,
        None => start_eframe(|cc| Box::new(App::new(cc, inst).unwrap())).unwrap(),
    }

    Ok(())
}

#[tracing::instrument(ret(level = Level::DEBUG))]
pub fn read_cfg(cli: &Cli) -> eyre::Result<Instance> {
    let cfg_dir = tracing::debug_span!("find_cfg_dir").in_scope(|| {
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
    });

    let cfg_file = File::open(cfg_dir.join("config.toml"))?;
    let buf = std::io::read_to_string(cfg_file)?;

    Ok(Instance {
        cfg_dir,
        cfg_file,
        cfg: toml_edit::de::from_str(&buf)?,

        game_cfgs: Default::default(),
    })
}

/// Run the [`App`](app::App).
fn start_eframe<F>(app_creator: F) -> Result<(), eframe::Error>
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

    eframe::run_native("ikiru", options, Box::new(app_creator))
}
