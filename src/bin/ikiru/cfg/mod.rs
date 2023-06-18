//! Master config file for the application.
use std::collections::HashMap;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use ikiru::misc::graphic_pack::cfg::GraphicPackCfg;
use ikiru::misc::TitleId;

pub mod input;

/// The main config file at `~/.config/ikiru/config.toml`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    /// The active input config.
    #[serde(default)]
    pub input: input::Controllers,
    #[serde(default)]
    pub profiles: Vec<Profile>,

    #[serde(default)]
    pub view: HubViewMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Profile {
    pub title: TitleId,
    /// The name of the profile. If none, the title ID is used.
    pub name: Option<String>,
    /// Override the input config for this game.
    pub input: Option<input::Controllers>,
    /// Override the graphics pack config for this game.
    #[serde(default)]
    pub graphics_packs: HashMap<String, GraphicPackCfg>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HubViewMode {
    #[default]
    Grid,
    List,
    Pro,
}

#[derive(Debug, Error)]
pub enum CfgError {
    #[error("failed to parse config file ({1})\n{0:?}")]
    Parse(toml_edit::de::Error, PathBuf),
}
