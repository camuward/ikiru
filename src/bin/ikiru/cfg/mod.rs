//! Master config file for the application.
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use ikiru::misc::graphic_pack::cfg::GraphicPackCfg;
use ikiru::misc::TitleId;

pub mod input;

/// The main config file at `~/.config/ikiru/config.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cfg {
    /// The directories to search for games.
    #[serde(default)]
    pub game_dirs: Vec<PathBuf>,

    /// Game configurations.
    #[serde(default)]
    pub active: BTreeMap<TitleId, String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<Profile>,

    /// Set a default view mode for the hub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<HubLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Profile {
    pub title: TitleId,
    /// The name of the profile. If none, the title ID is used.
    #[serde(default)]
    pub name: Option<String>,
    /// Override the input config for this game.
    #[serde(default)]
    pub input: Option<input::Controllers>,
    /// Override the graphics pack config for this game.
    #[serde(default)]
    pub graphics_packs: HashMap<String, GraphicPackCfg>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HubLayout {
    #[default]
    Grid,
    List,
    Pro,
}

#[derive(Debug, Error)]
pub enum CfgError {
    #[error("failed to parse config file ({1})\n{0}")]
    Parse(toml_edit::de::Error, PathBuf),
}
