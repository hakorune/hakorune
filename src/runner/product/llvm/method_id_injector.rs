//! Method ID injection for LLVM mode
//!
//! Handles method_id injection for BoxCall instructions.

use nyash_rust::mir::MirModule;

/// Method ID injection Box
///
/// **Responsibility**: Inject method_id for BoxCall where resolvable
/// **Input**: &mut MirModule
/// **Output**: usize (number of injection sites)
pub struct MethodIdInjectorBox;

impl MethodIdInjectorBox {
    /// Inject method_id for BoxCall instructions
    ///
    /// This function resolves plugin calls and injects method_id where possible.
    /// Returns the number of injection sites.
    pub fn inject(module: &mut MirModule) -> usize {
        let injected = crate::mir::passes::method_id_inject::inject_method_ids(module);
        if injected > 0 {
            crate::cli_v!("[LLVM] method_id injected: {} places", injected);
        }
        injected
    }
}
