// Minimal NyRT static shim library (libnyrt.a)
// Exposes C ABI entry points used by AOT/JIT-emitted objects.

mod encode;
mod entry;
mod exports;
mod ffi;
mod hako_forward_bridge;
mod hako_forward;
mod plugin;
mod user_box_registry;

pub use exports::*;
pub use ffi::lifecycle::*;
pub use ffi::weak::*;
pub use plugin::*;

#[cfg(test)]
mod tests;
