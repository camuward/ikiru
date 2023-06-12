use std::collections::BTreeMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::util::TitleId;

#[derive(Debug)]
pub struct GameCfgs {
    active: BTreeMap<TitleId, usize>,
    all: Vec<GameCfg>,
}

impl GameCfgs {
    pub fn get_cfg(&self, title: TitleId) -> &GameCfg {
        self.active
            .get(&title)
            .map(|i| &self.all[*i])
            .unwrap_or_else(|| {
                self.all.push(GameCfg::builder().build().unwrap());
                self.all.last().unwrap()
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct GameCfg {
    /// Override the input configuration for this game.
    input: Option<String>,
}
