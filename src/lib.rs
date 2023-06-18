//! # ikiru - wii u emulator
#![allow(unused)]

pub mod emu;
pub mod misc;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VER_MAJOR: u32 = pkg_version::pkg_version_major!();
pub const VER_MINOR: u32 = pkg_version::pkg_version_minor!();
pub const VER_PATCH: u32 = pkg_version::pkg_version_patch!();
