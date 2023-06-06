use serde::{Deserialize, Serialize};

/// Console Language
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsoleLanguage {
    /// Japanese
    JA,
    /// English
    EN,
    /// French
    FR,
    /// German
    DE,
    /// Italian
    IT,
    /// Spanish
    ES,
    /// Korean
    KO,
    /// Dutch
    NL,
    /// Portuguese
    PT,
    /// Russian
    RU,
    /// Chinese
    #[serde(alias = "zhs", alias = "zht")]
    ZH,
}
