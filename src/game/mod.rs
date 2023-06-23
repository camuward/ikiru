use std::marker::PhantomData;

use serde::Deserialize;

pub use self::app::*;
pub use self::meta::*;
pub use self::title_id::*;

mod app;
mod meta;
mod title_id;

pub fn de_str<'de, D>(de: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct InnerValue {
        #[serde(default, rename = "$value")]
        value: String,
    }

    let helper = InnerValue::deserialize(de)?;
    Ok(helper.value)
}

pub fn de_opt<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct InnerValue {
        #[serde(default, rename = "$value")]
        value: String,
    }

    let helper = InnerValue::deserialize(de)?;
    Ok(Some(helper.value).filter(|s| !s.is_empty()))
}

pub fn de_u32<'de, D>(de: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct InnerValue {
        #[serde(rename = "$value")]
        value: u32,
    }

    let helper = InnerValue::deserialize(de)?;
    Ok(helper.value)
}
