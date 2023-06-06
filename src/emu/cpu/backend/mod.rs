pub mod cranelift;
pub mod interp;
pub mod llvm;

pub struct Backend {
    pub name: String,
    pub backend: Box<dyn BackendImpl>,
}

trait BackendImpl {
    fn compile(&self, code: &[u8]) -> Result<(), String>;
}
