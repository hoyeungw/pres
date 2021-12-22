#[macro_use]
mod macros;

#[cfg(all(unix, not(target_os = "redox")))]
#[path = "sys/unix/mod.rs"]
mod sys;

mod types;
pub mod input;
pub mod event;
pub mod raw;
pub mod clear;
pub mod cursor;
mod r#async;

pub use r#async::{AsyncReader, async_stdin};

