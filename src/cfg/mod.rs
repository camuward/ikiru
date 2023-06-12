//! Master config file for the application.
use std::collections::HashMap;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// use self::profile::Profile;
use ikiru::util::graphic_pack::cfg::GraphicPackCfg;
use ikiru::util::TitleId;

// pub mod input;
// pub mod profile;

/// The main config file at `~/.config/ikiru/config.toml`.
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Cfg {
    /// The active input config.
    pub input: InputMap,
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Profile {
    pub title: TitleId,
    /// The name of the profile. If none, the title ID is used.
    pub name: Option<String>,
    /// Override the input config for this game.
    pub input: Option<InputMap>,
    /// Override the graphics pack config for this game.
    #[serde(default)]
    pub graphics_packs: HashMap<String, GraphicPackCfg>,
}
