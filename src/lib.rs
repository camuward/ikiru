#![allow(dead_code)]
pub use self::util::title_id::TitleId;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod app;
pub mod cfg;
pub mod cli;
pub mod emu;
pub mod gfx;
pub mod util;
