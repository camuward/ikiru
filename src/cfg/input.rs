pub struct InputSettings {
    input_devices: Vec<EmuController>,
}

pub struct EmuController {
    name: String,
    bindings: HashMap<EmuInput, DeviceInput>,
}

pub struct DeviceInput {
    /// The device this input is bound to.
    id: (),
    /// The action on the device that controls this input.
    input: (),
}
