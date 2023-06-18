use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::cfg::{Cfg, Profile};
use ikiru::misc::TitleId;

pub mod input;

#[derive(Debug)]
pub struct Instance {
    pub cfg_dir: PathBuf,
    pub cfg_file: File,
    pub cfg: Cfg,

    pub game_cfgs: GameCfg,
}

pub struct SettingsWindow {}

#[derive(Debug, Default)]
pub struct GameCfg {
    active: BTreeMap<TitleId, usize>,
    all: Vec<Profile>,
}

impl GameCfg {
    pub fn add_profile(&mut self, cfg: Profile) {
        let title_id = cfg.title;
        let index = self.all.len();
        self.all.push(cfg);
        if self.active.get(&title_id).is_none() {
            self.active.insert(title_id, index);
        }
    }

    /// Set the active profile for a title. If `profile` is `None`, the default
    /// profile for the title is used.
    pub fn set_active(&mut self, title: TitleId, profile: Option<String>) {
        let needle = profile.as_deref().unwrap_or("default");
        // self.active.insert(title, );
    }

    pub fn get_cfg(&mut self, title: TitleId) -> &Profile {
        match self.active.get(&title) {
            Some(i) => &self.all[*i],
            None => {
                let default = crate::cfg::ProfileBuilder::default()
                    .title(title)
                    .build()
                    .unwrap();

                self.all.push(default);
                self.all.last().unwrap()
            }
        }
    }
}
