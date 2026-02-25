/*!
 * Macro child mode (split modules)
 */

mod entry;
mod transforms;

pub use entry::run_macro_child;
pub use transforms::normalize_core_pass;
