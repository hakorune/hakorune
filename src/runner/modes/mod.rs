// bench module removed with vm-legacy
pub mod macro_child;
pub mod mir;
pub use crate::runner::keep::vm;
pub use crate::runner::keep::vm_fallback;
pub use crate::runner::product::llvm;
pub use crate::runner::product::wasm;
pub use crate::runner::reference::vm_hako;

// Shared helpers extracted from common.rs (in progress)
pub mod common_util;
