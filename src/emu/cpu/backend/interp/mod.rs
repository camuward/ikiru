use super::BackendImpl;

#[derive(Debug)]
pub struct InterpreterBackend {}

impl BackendImpl for InterpreterBackend {
    fn compile(&self, code: &[u8]) -> Result<(), String> {
        todo!()
    }
}
