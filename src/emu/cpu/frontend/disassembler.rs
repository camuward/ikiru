pub struct DisInstr {
    asm_code: u32,
    operand_mask: u8,
    operand: [DisOperand; 5],
    branch_hint: bool,
}

pub struct DisOperand {
    kind: u8,
    reg_index: u16,
    imm: OperandValue,
}

pub enum OperandValue {
    Signed(i32, u8),
    Unsigned(u32, u8),
}
