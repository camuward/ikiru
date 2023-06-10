use serde::{Deserialize, Serialize};

use super::{InputState, InputApi};

pub fn list_devices() -> Vec<InputApi> {
    todo!()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {}

impl super::Input for Input {
    fn poll(&mut self) -> InputState {
        todo!()
    }
}
