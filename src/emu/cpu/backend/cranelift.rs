use super::BackendImpl;

#[derive(Debug)]
pub struct CraneliftBackend {}

impl BackendImpl for CraneliftBackend {
    fn compile(&self, code: &[u8]) -> Result<(), String> {
        todo!()
    }
}
