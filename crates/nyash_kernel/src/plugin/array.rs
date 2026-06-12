pub use super::array_compat::*;
pub use super::array_runtime_aliases::*;

#[cfg(test)]
use super::array_handle_cache::with_array_box;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
