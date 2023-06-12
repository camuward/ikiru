use enum_dispatch::enum_dispatch;

use cranelift::CraneliftBackend;
use interp::InterpreterBackend;
use llvm::LlvmBackend;

pub mod cranelift;
pub mod interp;
pub mod llvm;

#[enum_dispatch(BackendImpl)]
pub enum Backend {
    InterpreterBackend,
    CraneliftBackend,
    LlvmBackend,
}

#[enum_dispatch]
trait BackendImpl {
    fn compile(&self, code: &[u8]) -> Result<(), String>;
}
