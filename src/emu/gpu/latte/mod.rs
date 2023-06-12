pub mod reg;

#[repr(C)]
pub struct LatteGpuState {
    pub registers: reg::Registers,

}
