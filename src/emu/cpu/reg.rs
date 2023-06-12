use ikiru_macro::Reg;

pub trait Reg {
    fn set(&mut self, value: u32);

    fn get(&self) -> u32;
    fn get_be(&self) -> u32 {
        self.get().to_be()
    }
}

// 0x0D0002W1C - official name unknown
#[derive(Reg)]
pub struct AcrViData {
    data: u32,
}

// 0x0D000224 - official name unknown
#[derive(Reg)]
pub struct AcrViAddr {
    addr: u32,
}

// 0x0D000228 - official name unknown
#[derive(Reg)]
pub struct AcrViCtrl {
    has_ownership: bool, // exact purpose not understood
                         // other fields unknown
}

// 0x6400/0x640C/0x6418/0x6424
#[derive(Reg)]
pub struct SiCoutBuf {
    output1: u8,
    output0: u8,
    cmd: u8,
}

// 0x6404/0x6410/0x641C/0x6428
#[derive(Reg)]
pub struct SiCinBufH {
    input3: u8,
    input2: u8,
    input1: u8,
    #[reg(width = "6")]
    input0: u8,

    err_latch: bool,
    err_stat: bool,
}

// 0x6408/0x6414/0x6420/0x642C
#[derive(Reg)]
pub struct SiCinBufL {
    input4: u8,
    input5: u8,
    input6: u8,
    input7: u8,
}

// 0x6430
#[derive(Reg)]
pub struct SiPoll {
    #[reg(index = "16..26")]
    x: u16,
    #[reg(index = "8")]
    y: u8,
    #[reg(index = "4..8")]
    en: u8,
    #[reg(index = "0..4")]
    vbcpy: u8,
}

// 0x6434
#[derive(Reg)]
pub struct SiComCSR {
    transfer_start: bool,
    channel: u8,
    callback_enable: bool,
    command_enable: bool,

    #[reg(width = "7")]
    inlnth: u8,
    #[reg(width = "7")]
    outlnth: u8,

    channelenable: bool,
    #[reg(width = "2")]
    ukn_channel_num_maybe: u8,
    rdstintmsk: bool,
    rdstint: bool,
    comerr: bool,
    tcintmask: bool,
    tcint: bool,
}

// 0x6438
#[derive(Reg)]
#[reg(repr)]
pub struct SiSR {
    unrun3: bool,
    ovrun3: bool,
    coll3: bool,
    norep3: bool,
    wrst3: bool,
    rdst3: bool,

    #[reg(index = "8")]
    unrun2: bool,
    ovrun2: bool,
    coll2: bool,
    norep2: bool,
    wrst2: bool,
    rdst2: bool,

    #[reg(index = "16")]
    unrun1: bool,
    ovrun1: bool,
    coll1: bool,
    norep1: bool,
    wrst1: bool,
    rdst1: bool,

    #[reg(index = "24")]
    unrun0: bool,
    ovrun0: bool,
    coll0: bool,
    norep0: bool,
    wrst0: bool,
    rdst0: bool,

    #[reg(index = "31")]
    wr: bool,
}
