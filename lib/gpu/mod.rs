pub mod latte;
pub mod shader;

pub struct Gpu {
    regs: Box<latte::reg::Registers>,
}
