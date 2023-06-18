use serde::{Deserialize, Serialize};

use crate::cfg::input::{ControllerKind, Map};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub kind: ControllerKind,
    pub map: Map,
}
