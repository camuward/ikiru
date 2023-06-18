use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

mod joycon;
mod parse;
mod profile;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Controllers {
    #[serde(serialize_with = "parse::ser")]
    #[serde(deserialize_with = "parse::de")]
    controllers: [Option<EmuController>; 8],
}

/// An emulated Wii U controller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmuController {
    Profile(String),
    Custom {
        kind: ControllerKind,
        #[serde(default)]
        map: Map,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControllerKind {
    Gamepad,
    Pro,
    Classic,
    Wiimote,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Map(HashMap<EventType, ()>);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Hash, Serialize, Deserialize), name(EventType))]
pub enum Event {
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

// EmuInput::Zl => "ZL",
// EmuInput::Zr => "ZR",
// EmuInput::DPadUp => "DPAD-Up",
// EmuInput::DPadDown => "DPAD-Down",
// EmuInput::DPadLeft => "DPAD-Left",
// EmuInput::DPadRight => "DPAD-Right",
// EmuInput::XAxisP => "X-Axis+",
// EmuInput::YAxisP => "Y-Axis+",
// EmuInput::XAxisN => "X-Axis-",
// EmuInput::YAxisN => "Y-Axis-",
// EmuInput::XRotationP => "X-Rotation+",
// EmuInput::YRotationP => "Y-Rotation+",
// EmuInput::XRotationN => "X-Rotation-",
// EmuInput::YRotationN => "Y-Rotation-",
// EmuInput::XTriggerP => "X-Trigger+",
// EmuInput::YTriggerP => "Y-Trigger+",
// EmuInput::XTriggerN => "X-Trigger-",
// EmuInput::YTriggerN => "Y-Trigger-",

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let expected = Controllers {
            controllers: [
                Some(EmuController::Profile("test".to_owned())),
                Some(EmuController::Custom {
                    kind: ControllerKind::Gamepad,
                    map: Map(HashMap::new()),
                }),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        };

        let s = "
            controllers.0 = \"test\"
            [controllers.1]
            kind = \"Gamepad\"
        ";

        assert_eq!(expected, toml_edit::de::from_str(s).unwrap());
    }
}
