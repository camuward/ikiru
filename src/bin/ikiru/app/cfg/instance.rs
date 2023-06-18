use std::collections::{BTreeMap, HashSet};
use std::io;
use std::path::PathBuf;

use crate::app::cfg::GameCfg;
use crate::cfg::{self, Cfg, CfgError, HubLayout};
use crate::cli::{self, Cli, CliError};
use ikiru::misc::TitleId;

#[derive(Clone)]
pub struct Instance {
    pub cfg_dir: PathBuf,
    pub cfg_file: PathBuf,
    pub cfg: Cfg,

    pub layout: HubLayout,
    pub game_cfgs: GameCfg,
    pub game_dirs: Vec<PathBuf>,
    pub game_library: BTreeMap<TitleId, PathBuf>,
    pub game_img_tex: BTreeMap<TitleId, egui::TextureHandle>,
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
        let cfg_dir: PathBuf = {
            let _e = debug_span!("find_cfg_dir").entered();

            // get the default config dir, or use the one specified by the user
            let get_default_dir = || dirs::config_dir().map(|path| path.join("ikiru"));
            let path = cli.cfg_dir.clone().or_else(get_default_dir);

            match path {
                None => bail!(CliError::CfgDirNotFound),
                Some(dir) if !dir.try_exists()? => {
                    let _e = debug_span!("create_cfg_dir", path = ?dir).entered();
                    std::fs::create_dir_all(&dir)?;

                    dir
                }
                Some(dir) if !dir.is_dir() => bail!(CliError::CfgDirNotDir(dir)),
                Some(dir) => dir,
            }
        };

        let cfg_file = cfg_dir.join("config.toml");
        if !cfg_file.try_exists()? {
            let _e = debug_span!("create_cfg_file", path = ?cfg_file).entered();

            let cfg = toml_edit::ser::to_string(&cfg::Cfg::default())?;
            std::fs::write(&cfg_file, cfg)?;
        }

        let cfg: cfg::Cfg = toml_edit::de::from_slice(&std::fs::read(&cfg_file)?)
            .map_err(|e| CfgError::Parse(e, cfg_file.clone()))?;

        let mut game_dirs: HashSet<PathBuf> = cfg
            .game_dirs
            .iter()
            .chain(&cli.append)
            .map(|p| p.canonicalize())
            .collect::<Result<_, _>>()?;

        // let game_library = game_dirs.;
        let game_library = Default::default();

        Ok(Instance {
            layout: cfg.layout.unwrap_or_default(),
            game_dirs: game_dirs.into_iter().collect(),
            game_library,

            cfg_dir,
            cfg_file,
            cfg,

            game_cfgs: Default::default(),
            game_img_tex: Default::default(),
        })
    }
}
