use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;

use super::{de_str, de_u32};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename = "app")]
pub struct AppXml {
    #[serde(deserialize_with = "de_u32")]
    pub version: u32,

    #[serde(deserialize_with = "de_str")]
    pub os_version: String,

    #[serde(deserialize_with = "de_str")]
    pub title_id: String,

    #[serde(deserialize_with = "de_str")]
    pub title_version: String,

    #[serde(deserialize_with = "de_u32")]
    pub sdk_version: u32,

    #[serde(deserialize_with = "de_str")]
    pub app_type: String,

    #[serde(deserialize_with = "de_str")]
    pub group_id: String,

    #[serde(deserialize_with = "de_str")]
    pub os_mask: String,

    #[serde(deserialize_with = "de_str")]
    pub common_id: String,
}

impl AppXml {
    pub fn from_game_dir(path: &Path) -> eyre::Result<Self> {
        let path = path.join("code/app.xml");
        let reader = BufReader::new(File::open(&path)?);

        Ok(quick_xml::de::from_reader(reader)?)
    }
}
