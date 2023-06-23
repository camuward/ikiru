use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use serde::Deserialize;

use super::{de_opt, de_str, de_u32};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename = "menu")]
pub struct MetaXml {
    #[serde(rename = "type")]
    pub menu_type: String,
    pub access: String,

    #[serde(deserialize_with = "de_u32")]
    pub version: u32,

    #[serde(deserialize_with = "de_str")]
    pub product_code: String,

    #[serde(deserialize_with = "de_str")]
    pub content_platform: String,

    #[serde(deserialize_with = "de_str")]
    pub company_code: String,

    #[serde(deserialize_with = "de_str")]
    pub mastering_date: String,

    #[serde(deserialize_with = "de_u32")]
    pub logo_type: u32,

    #[serde(deserialize_with = "de_str")]
    pub app_launch_type: String,

    #[serde(deserialize_with = "de_str")]
    pub invisible_flag: String,

    #[serde(deserialize_with = "de_str")]
    pub no_managed_flag: String,

    #[serde(deserialize_with = "de_str")]
    pub no_event_log: String,

    #[serde(deserialize_with = "de_str")]
    pub no_icon_database: String,

    #[serde(deserialize_with = "de_str")]
    pub launching_flag: String,

    #[serde(deserialize_with = "de_str")]
    pub install_flag: String,

    #[serde(deserialize_with = "de_u32")]
    pub closing_msg: u32,

    #[serde(deserialize_with = "de_u32")]
    pub title_version: u32,

    #[serde(deserialize_with = "de_str")]
    pub title_id: String,

    #[serde(deserialize_with = "de_str")]
    pub group_id: String,

    #[serde(deserialize_with = "de_str")]
    pub boss_id: String,

    #[serde(deserialize_with = "de_str")]
    pub os_version: String,

    #[serde(deserialize_with = "de_str")]
    pub app_size: String,

    #[serde(deserialize_with = "de_str")]
    pub common_save_size: String,

    #[serde(deserialize_with = "de_str")]
    pub account_save_size: String,

    #[serde(deserialize_with = "de_str")]
    pub common_boss_size: String,

    #[serde(deserialize_with = "de_str")]
    pub account_boss_size: String,

    #[serde(deserialize_with = "de_u32")]
    pub save_no_rollback: u32,

    #[serde(deserialize_with = "de_str")]
    pub join_game_id: String,

    #[serde(deserialize_with = "de_str")]
    pub join_game_mode_mask: String,

    #[serde(deserialize_with = "de_u32")]
    pub bg_daemon_enable: u32,

    #[serde(deserialize_with = "de_u32")]
    pub olv_accesskey: u32,

    #[serde(deserialize_with = "de_u32")]
    pub wood_tin: u32,

    #[serde(deserialize_with = "de_u32")]
    pub e_manual: u32,

    #[serde(deserialize_with = "de_u32")]
    pub e_manual_version: u32,

    #[serde(deserialize_with = "de_str")]
    pub region: String,

    #[serde(deserialize_with = "de_u32")]
    pub pc_cero: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_esrb: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_bbfc: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_usk: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_pegi_gen: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_pegi_fin: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_pegi_prt: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_pegi_bbfc: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_cob: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_grb: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_cgsrr: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_oflc: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_reserved0: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_reserved1: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_reserved2: u32,
    #[serde(deserialize_with = "de_u32")]
    pub pc_reserved3: u32,

    #[serde(deserialize_with = "de_u32")]
    pub ext_dev_nunchaku: u32,
    #[serde(deserialize_with = "de_u32")]
    pub ext_dev_classic: u32,
    #[serde(deserialize_with = "de_u32")]
    pub ext_dev_urcc: u32,
    #[serde(deserialize_with = "de_u32")]
    pub ext_dev_board: u32,
    #[serde(deserialize_with = "de_u32")]
    pub ext_dev_usb_keyboard: u32,
    #[serde(deserialize_with = "de_u32")]
    pub ext_dev_etc: u32,
    #[serde(deserialize_with = "de_opt")]
    pub ext_dev_etc_name: Option<String>,

    #[serde(deserialize_with = "de_u32")]
    pub eula_version: u32,

    #[serde(deserialize_with = "de_u32")]
    pub drc_use: u32,

    #[serde(deserialize_with = "de_u32")]
    pub network_use: u32,

    #[serde(deserialize_with = "de_u32")]
    pub online_account_use: u32,

    #[serde(deserialize_with = "de_u32")]
    pub direct_boot: u32,

    #[serde(deserialize_with = "de_opt")]
    pub longname_ja: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_en: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_fr: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_de: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_it: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_es: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_zhs: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_ko: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_nl: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_pt: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_ru: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub longname_zht: Option<String>,

    #[serde(deserialize_with = "de_opt")]
    pub shortname_ja: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_en: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_fr: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_de: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_it: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_es: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_zhs: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_ko: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_nl: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_pt: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_ru: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub shortname_zht: Option<String>,

    #[serde(deserialize_with = "de_opt")]
    pub publisher_ja: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_en: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_fr: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_de: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_it: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_es: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_zhs: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_ko: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_nl: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_pt: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_ru: Option<String>,
    #[serde(deserialize_with = "de_opt")]
    pub publisher_zht: Option<String>,
}
