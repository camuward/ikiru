//! Master configuration file for the application.
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Serialize, Deserialize};

mod input;

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Config {
    games_dirs: Vec<PathBuf>,
}
