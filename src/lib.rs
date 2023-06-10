#![allow(dead_code)]

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VER_MAJOR: u32 = pkg_version::pkg_version_major!();
pub const VER_MINOR: u32 = pkg_version::pkg_version_minor!();
pub const VER_PATCH: u32 = pkg_version::pkg_version_patch!();

pub mod app;
pub mod cfg;
pub mod cli;
pub mod emu;
pub mod gfx;
pub mod util;
