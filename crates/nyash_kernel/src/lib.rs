// Minimal NyRT static shim library (libnyrt.a)
// Exposes C ABI entry points used by AOT/JIT-emitted objects.

#[cfg(all(feature = "legacy-entry", feature = "lifecycle-core"))]
compile_error!("nyash_kernel: legacy-entry and lifecycle-core require separate build invocations");

#[cfg(feature = "lifecycle-core")]
pub use entry::run_normalized_entry;

mod backend_env;
mod c_string;
mod encode;
mod entry;
mod env_flags;
mod exports;
mod ffi;
mod hako_forward;
mod hako_forward_bridge;
mod observe;
mod plugin;
mod rss_observe;
mod user_box_registry;

pub use exports::*;
pub use ffi::dynamic_v2_lease::*;
pub use ffi::lifecycle::*;
pub use ffi::weak::*;
pub use plugin::*;

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod tests;
