use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::{fs, io};

use egui::TextureHandle;

use crate::app::{cfg::GameCfg, win::hub};
use crate::cfg::game::{GameEntry, GameLibrary};
use crate::cfg::{Cfg, CfgError};
use crate::cli::{self, Cli, CliError};
use ikiru::game::TitleId;

#[derive(Clone)]
pub struct Instance {
    pub cfg_dir: PathBuf,
    pub cfg_file: PathBuf,
    pub cfg: Cfg,

    pub init_layout: hub::LayoutType,
    pub game_cfgs: GameCfg,
    pub game_library: GameLibrary,
    pub game_img_tex: BTreeMap<TitleId, TextureHandle>,
}

impl Instance {
    /// Search the game directories for games.
    pub fn reload(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl TryFrom<&Cli> for Instance {
    type Error = eyre::Error;

    fn try_from(cli: &Cli) -> Result<Self, Self::Error> {
        let cfg_dir: PathBuf = find_cfg_dir(cli)?;
        let cfg_file: PathBuf = find_cfg_file(&cfg_dir)?;

        let cfg: Cfg = toml_edit::de::from_slice(&std::fs::read(&cfg_file)?)
            .map_err(|e| CfgError::Parse(e, cfg_file.clone()))?;

        let game_library = GameLibrary::new(cfg.game_dirs.iter().chain(&cli.append))?;

        Ok(Instance {
            init_layout: cfg.layout.unwrap_or_default(),
            game_library,

            cfg_dir,
            cfg_file,
            cfg,

            game_cfgs: Default::default(),
            game_img_tex: Default::default(),
        })
    }
}

#[instrument(ret, level = "debug")]
fn find_cfg_dir(cli: &Cli) -> eyre::Result<PathBuf> {
    let get_default_dir = || dirs::config_dir().map(|path| path.join("ikiru"));
    let path = cli.cfg_dir.clone().or_else(get_default_dir);

    Ok(match path {
        None => bail!(CliError::CfgDirNotFound),
        Some(dir) if !dir.try_exists()? => {
            let _e = debug_span!("create_cfg_dir", path = ?dir).entered();
            std::fs::create_dir_all(&dir)?;

            dir
        }
        Some(dir) if !dir.is_dir() => bail!(CliError::CfgDirNotDir(dir)),
        Some(dir) => dir,
    })
}

#[instrument(ret, level = "debug")]
fn find_cfg_file(cfg_dir: &Path) -> eyre::Result<PathBuf> {
    let cfg_file = cfg_dir.join("config.toml");

    if !cfg_file.try_exists()? {
        let _e = debug_span!("create_cfg_file", path = ?cfg_file).entered();

        let cfg = toml_edit::ser::to_string(&Cfg::default())?;
        std::fs::write(&cfg_file, cfg)?;
    }

    Ok(cfg_file)
}
