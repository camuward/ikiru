//! Master cfguration file for the application.
use std::collections::HashMap;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::util::graphic_pack::cfg::GraphicPackCfg;
use crate::util::TitleId;

// use self::profile::Profile;

// pub mod input;
// pub mod profile;

#[derive(Debug)]
pub struct Instance {
    pub cfg_dir: PathBuf,
}

/// The main cfguration file at `~/.cfg/ikiru/cfg.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Cfg {
    /// The active input cfguration.
    pub input: Option<String>,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Profile {
    pub title: TitleId,
    /// The name of the profile. If none, the title ID is used.
    pub name: Option<String>,
    /// Override the input cfguration for this game.
    pub input: Option<()>,
    /// Override the graphics pack cfguration for this game.
    #[serde(default)]
    pub graphics_packs: HashMap<String, GraphicPackCfg>,
}
