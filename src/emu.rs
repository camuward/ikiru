//! Hardware and software emulation of the Wii U.
pub mod cafe;
pub mod cpu;
pub mod gpu;

pub struct Instance {
    os_func_table: Vec<cafe::os::common::FunctionEntry>,
    os_data_table: Vec<cafe::os::common::PointerEntry>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            os_func_table: Vec::new(),
            os_data_table: Vec::new(),
        }
    }
}
