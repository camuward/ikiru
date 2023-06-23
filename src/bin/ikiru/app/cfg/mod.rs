use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::cfg::{Cfg, Profile, ProfileBuilder};
use ikiru::game::TitleId;

pub use self::instance::Instance;

pub mod input;
mod instance;

pub struct SettingsWindow {}

#[derive(Debug, Clone, Default)]
pub struct GameCfg {
    active: BTreeMap<TitleId, usize>,
    all: Vec<Profile>,
}

impl GameCfg {
    /// Add a profile to the config.
    pub fn add_profile(&mut self, cfg: Profile) {
        // let title_id = cfg.title;
        // let index = self.all.len();
        self.all.push(cfg);
        // if self.active.get(&title_id).is_none() {
        //     self.active.insert(title_id, index);
        // }
    }

    /// Set the active profile for a title. If `profile` is `None`, the default
    /// profile for the title is used.
    pub fn set_active(&mut self, title: TitleId, profile: Option<String>) {
        let _e = tracing::trace_span!("set_active", title = ?title, profile = ?profile, profiles = ?self.all).entered();

        let index = self
            .all
            .iter()
            .position(|p| p.title == title && p.name == profile);

        match index {
            Some(i) => _ = self.active.insert(title, i),
            None if profile.is_none() => _ = self.insert_default(title),
            _ => tracing::error!("failed to set profile for {title} to {}", profile.unwrap()),
        }
    }

    /// Get the active profile for a title.
    pub fn get_cfg(&mut self, title: TitleId) -> &Profile {
        match self.active.get(&title) {
            Some(i) => &self.all[*i],
            None => self.insert_default(title),
        }
    }

    fn insert_default(&mut self, title: TitleId) -> &Profile {
        let default = ProfileBuilder::default().title(title).build().unwrap();

        self.all.push(default);
        self.all.last().unwrap()
    }
}
