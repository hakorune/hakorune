// Minimal NyRT static shim library (libnyrt.a)
// Exposes C ABI entry points used by AOT/JIT-emitted objects.

mod backend_env;
mod encode;
mod entry;
mod exports;
mod ffi;
mod hako_forward;
mod hako_forward_bridge;
mod observe;
mod plugin;
mod rss_observe;
mod user_box_registry;

pub use exports::*;
pub use ffi::lifecycle::*;
pub use ffi::weak::*;
pub use plugin::*;

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod tests;
