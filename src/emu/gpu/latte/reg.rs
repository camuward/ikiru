use serde::{Deserialize, Serialize, Serializer, de::Deserializer};

const NUM_SAMPLERS_PER_STAGE: usize = 18;

autopad::autopad! {
    #[repr(C)]
    pub struct Registers {
        0x8958 => vgt_primitive_type: u32,
        0xA400 => td_ps_sampler_border_color: [u32; NUM_SAMPLERS_PER_STAGE],
    }
}

impl Registers {
    /// The size of the register block in bytes.
    const RAW_SIZE: usize = 0x10000 * 4 + 9 * 4;
    /// The size of the compacted register block in bytes.
    const SER_SIZE: usize = todo!();

    fn serialize(&self) -> impl IntoIterator<Item = u32> + '_ {
        let base = self as *const Registers as *const u32;
        GPU_REG_SER_MAP_V1
            .into_iter()
            .map(|&(addr, count)| (addr..addr + count))
            .flat_map(move |regs| regs.map(move |reg| unsafe { *base.add(reg) }))
    }
}

const _: () = {
    // assert_eq!(std::mem::size_of::<Registers>(), 0x10000 * 4 + 9 * 4);
    // assert_eq!(Registers::default().serialize().count() * 4, todo!());
};

impl Serialize for Registers {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let mut seq = ser.serialize_seq(None)?;
        for val in self.serialize() {
            seq.serialize_element(&val)?;
        }
        seq.end()
    }
}

impl<'a> Deserialize<'a> for Registers {
    fn deserialize<D: Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
        todo!()
    }
}

// tied to GPUCompactedRegisterState - DO NOT MODIFY
// this list is based on the same list used internally by GX2ContextState, excluding ALU constants
const GPU_REG_SER_MAP_V1: &[(usize, usize)] = &[
    // Cfg register
    (0x2232, 0x2),
    (0x2235, 0x1),
    (0x223A, 0x1),
    (0x2256, 0x1),
    (0x22C8, 0x1),
    (0x2300, 0x6),
    (0x2310, 0xC),
    (0x2363, 0x1),
    (0x2404, 0x2),
    (0x2542, 0x1),
    (0x25C5, 0x1),
    (0x260C, 0x1),
    (0x2900, 0x48), // sampler border color VS
    (0x2980, 0x48), // sampler border color PS
    (0x2A00, 0x48), // sampler border color GS
    // Context register
    (0xA000, 0x2),
    (0xA003, 0x3),
    (0xA00A, 0x4),
    (0xA010, 0x38), // color buffer registers + others
    (0xA050, 0x34), // SQ_ALU_CONST_BUFFER_SIZE_PS_0
    (0xA08C, 0x1),
    (0xA08E, 0x4),
    (0xA094, 0x40),
    (0xA0D5, 0x1),
    (0xA0E0, 0x20),
    (0xA100, 0x9),
    (0xA10C, 0x3),
    (0xA10F, 0x60), // mostly PA_CL_VPORT_* registers
    (0xA185, 0xA),
    (0xA191, 0x27),
    (0xA1E0, 0x9),
    (0xA200, 0x1),
    (0xA202, 0x7),
    (0xA210, 0x29),
    (0xA250, 0x34),
    (0xA284, 0xC),
    (0xA290, 0x1),
    (0xA292, 0x2),
    (0xA29B, 0x1),
    (0xA2A1, 0x1),
    (0xA2A5, 0x1),
    (0xA2A8, 0x2),
    (0xA2AC, 0x3),
    (0xA2B4, 0x3),
    (0xA2B8, 0x3),
    (0xA2BC, 0x3),
    (0xA2C0, 0x3),
    (0xA2C8, 0x1),
    (0xA2CA, 0x1),
    (0xA2CC, 0x1),
    (0xA2CE, 0x1),
    (0xA300, 0x9),
    (0xA30C, 0x1),
    (0xA312, 0x1),
    (0xA316, 0x2),
    (0xA343, 0x2),
    (0xA349, 0x3),
    (0xA34C, 0x2),
    (0xA351, 0x1),
    (0xA37E, 0x6),
    // Resource registers
    (0xE000, 0x70),
    (0xE380, 0x70),
    (0xE460, 0x70),
    (0xE7E0, 0x70),
    (0xE8B9, 0x7),
    (0xE8C0, 0x70),
    (0xE930, 0x70),
    (0xECB0, 0x70),
    (0xED89, 0x7),
    // Sampler registers
    (0xF000, 0x36),
    (0xF036, 0x36),
    (0xF06C, 0x36),
    // Loop const registers
    (0xF880, 0x60),
];
