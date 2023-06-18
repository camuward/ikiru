use std::fmt;
use std::str::FromStr;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TitleId(#[serde(serialize_with = "ser", deserialize_with = "de")] u64);

fn ser<S: Serializer>(title: &u64, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_str(&format!("{:016x}", title))
}

fn de<'de, D: Deserializer<'de>>(de: D) -> Result<u64, D::Error> {
    de.deserialize_str(Vis)
}

impl TitleId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn id(&self) -> u64 {
        self.0
    }
}

struct Vis;

impl Visitor<'_> for Vis {
    type Value = u64;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a titleid hex string or raw value")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TitleId::from_str(v).map_err(E::custom)?.0)
    }
}

impl FromStr for TitleId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str_radix(s, 16)?))
    }
}

impl fmt::Display for TitleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn title_id_from_str() {
        assert_eq!(
            TitleId::new(0x0005000E101C9400),
            "0005000E101C9400".parse().unwrap(),
        );
    }

    #[test]
    fn title_id_to_str() {
        assert_eq!(
            TitleId::new(0x0005000E101C9400).to_string(),
            "0005000E101C9400",
        );
    }
}
