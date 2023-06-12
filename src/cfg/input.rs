use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

mod joycon;
mod profile;
mod sdl;
mod xinput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMap {
    pub controllers: HashMap<u8, Controller>,
}

pub fn list_devices() -> Vec<InputType> {
    sdl::list_devices()
        .into_iter()
        .chain(xinput::list_devices())
        .collect()
}

#[enum_dispatch]
pub trait Input {
    /// Polls the input device for the current state.
    fn poll(&mut self, last: InputState) -> InputState;
}

#[enum_dispatch(Input)]
#[derive(Debug, Serialize, Deserialize)]
pub enum InputApi {
    Sdl(sdl::Input),
    XInput(xinput::Input),
}

#[derive(Debug)]
pub struct InputState {
    
}

pub struct InputCfg {
    controllers: Vec<EmuController>,
}

/// An emulated Wii U controller.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmuController {
    /// 
    profile: Option<String>,
    /// The input device to use for this controller.
    input: InputType,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmuInput {
    A,
    B,
    X,
    Y,
    L,
    R,
    Zl,
    Zr,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Start,
    Select,
    LStickPress,
    LStickX(f64),
    LStickY(f64),
    RStickPress,
    RStickX(f64),
    RStickY(f64),
}

impl std::fmt::Display for EmuInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EmuInput::Zl => "ZL",
                EmuInput::Zr => "ZR",
                EmuInput::DPadUp => "DPAD-Up",
                EmuInput::DPadDown => "DPAD-Down",
                EmuInput::DPadLeft => "DPAD-Left",
                EmuInput::DPadRight => "DPAD-Right",
                EmuInput::XAxisP => "X-Axis+",
                EmuInput::YAxisP => "Y-Axis+",
                EmuInput::XAxisN => "X-Axis-",
                EmuInput::YAxisN => "Y-Axis-",
                EmuInput::XRotationP => "X-Rotation+",
                EmuInput::YRotationP => "Y-Rotation+",
                EmuInput::XRotationN => "X-Rotation-",
                EmuInput::YRotationN => "Y-Rotation-",
                EmuInput::XTriggerP => "X-Trigger+",
                EmuInput::YTriggerP => "Y-Trigger+",
                EmuInput::XTriggerN => "X-Trigger-",
                EmuInput::YTriggerN => "Y-Trigger-",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
