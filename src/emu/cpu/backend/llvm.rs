use super::BackendImpl;

#[derive(Debug)]
pub struct LlvmBackend {}

impl BackendImpl for LlvmBackend {
    fn compile(&self, code: &[u8]) -> Result<(), String> {
        todo!()
    }
}
