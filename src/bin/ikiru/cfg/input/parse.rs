use serde::{Deserializer, Serializer};

use crate::cfg::input::EmuController;

pub fn ser<S: Serializer>(
    controllers: &[Option<EmuController>; 8],
    ser: S,
) -> Result<S::Ok, S::Error> {
    // let mut map = HashMap::new();
    todo!()
}

pub fn de<'de, D: Deserializer<'de>>(de: D) -> Result<[Option<EmuController>; 8], D::Error> {
    // let map: HashMap<u8, Option<EmuController>>
    todo!()
}
