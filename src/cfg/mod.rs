//! Master config file for the application.
use std::collections::HashMap;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use self::game::GameCfgs;
// use self::profile::Profile;
use crate::util::graphic_pack::cfg::GraphicPackCfg;
use crate::util::TitleId;

pub mod game;
// pub mod input;
// pub mod profile;

#[derive(Debug)]
pub struct Instance {
    pub cfg_dir: PathBuf,
    pub game_cfgs: GameCfgs,
}

/// The main config file at `~/.config/ikiru/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Cfg {
    /// The active input config.
    pub input: Option<String>,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Profile {
    pub title: TitleId,
    /// The name of the profile. If none, the title ID is used.
    pub name: Option<String>,
    /// Override the input config for this game.
    pub input: Option<()>,
    /// Override the graphics pack config for this game.
    #[serde(default)]
    pub graphics_packs: HashMap<String, GraphicPackCfg>,
}
