use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TitleId(u64);

impl TitleId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn id(&self) -> u64 {
        self.0
    }
}

impl std::str::FromStr for TitleId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str_radix(s, 16)?))
    }
}

impl std::fmt::Display for TitleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#016x}", self.0)
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
