pub struct FunctionEntry {
    lib_hash: Hash,
    func_hash: Hash,
    name: String,
    hle_func: i32,
}

pub struct PointerEntry {
    lib_hash: Hash,
    func_hash: Hash,
    v_ptr: u32,
}

pub struct Hash(u32, u32);

impl Hash {
    pub fn from_name(name: &str) -> Self {
        let mut h1 = 0x688BA2BA;
        let mut h2 = 0xF64A71D5;

        for c in name.bytes() {
            h1 += c as u32;
            h1 = (h1 << 3) | (h1 >> 29);
            h2 ^= c as u32;
            h2 = (h2 << 7) | (h2 >> 25);
            h1 += h2;
            h2 += c as u32;
            h2 = (h2 << 3) | (h2 >> 29);
        }

        Self(h1, h2)
    }
}
