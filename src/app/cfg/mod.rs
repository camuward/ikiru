use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::cfg::{Profile, Cfg};
use crate::util::TitleId;

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
        self.active.insert(title, index);
    }

    pub fn set_active(&mut self, cfg: Profile) {
        self.active.insert(title, index);
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
