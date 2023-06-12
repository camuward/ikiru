pub mod disassembler;

pub struct Assembler {
    // in
    virtual_addr: u32,
    /// If set, alignment will always be set to 1 (even for `.align` directive!)
    force_unaligned: bool,

    // out
    data: smallvec::SmallVec<[u8; 16]>,
}

pub enum PpcAsmOp {}
