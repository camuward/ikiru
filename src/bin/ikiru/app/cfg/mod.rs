use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::cfg::{Profile, Cfg};
use ikiru::misc::TitleId;

pub mod input;

#[derive(Debug)]
pub struct Instance {
    pub cfg_dir: PathBuf,
    pub cfg_file: File,
    pub cfg: Cfg,
    
    pub game_cfgs: GameCfg,
}

pub struct SettingsWindow {

}

#[derive(Debug, Default)]
pub struct GameCfg {
    active: BTreeMap<TitleId, usize>,
    all: Vec<Profile>,
}

impl GameCfg {
    pub fn add_profile(&mut self, cfg: Profile) {
        let index = self.all.len();
        self.all.push(cfg);
        if self.active.get(&cfg.title).is_none() {
            self.active.insert(cfg.title, index);
        }
    }

    /// Set the active profile for a title. If `profile` is `None`, the default
    /// profile is used.
    pub fn set_active(&mut self, title: TitleId, profile: Option<String>) {
        let needle = profile.as_deref().unwrap_or("default");
        // self.active.insert(title, );
    }

    pub fn get_cfg(&self, title: TitleId) -> &Profile {
        self.active
            .get(&title)
            .map(|i| &self.all[*i])
            .unwrap_or_else(|| {
                self.all.push(Profile::builder().build().unwrap());
                self.all.last().unwrap()
            })
    }
}
