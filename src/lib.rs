#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::doc_overindented_list_items)]

/// Raw and unsafe FFmpeg functions, structs and constants,
pub use rusty_ffmpeg::ffi;

#[macro_use]
mod macros;

mod shared;

pub mod avcodec;
// The macOS bundle generates bindings for the libraries in the source archive.
#[cfg(not(all(feature = "bundled_h264", target_os = "macos")))]
pub mod avdevice;
#[cfg(not(all(feature = "bundled_h264", target_os = "macos")))]
pub mod avfilter;
pub mod avformat;
pub mod avutil;
#[cfg(not(all(feature = "bundled_h264", target_os = "macos")))]
pub mod swresample;
pub mod swscale;

pub mod error;

pub use shared::{build_array, UnsafeDerefMut};
