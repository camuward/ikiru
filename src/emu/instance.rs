use crate::cafe::os::common::{FunctionEntry, PointerEntry};

/// Emulated Wii U.
pub struct Instance {
    os_func_table: Vec<FunctionEntry>,
    os_data_table: Vec<PointerEntry>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            os_func_table: Vec::new(),
            os_data_table: Vec::new(),
        }
    }
}
