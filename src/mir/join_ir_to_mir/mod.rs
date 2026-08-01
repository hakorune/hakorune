//! Structured JoinIR to MIR conversion.
//!
//! This is a conversion authority, never a VM execution lane.  The ordinary
//! VM runner remains the sole owner of MIR execution.
use crate::mir::join_ir::JoinFuncId;

#[macro_use]
mod logging {
    macro_rules! debug_log {
        ($($arg:tt)*) => {
            if crate::config::env::joinir_dev::debug_enabled() {
                crate::runtime::get_global_ring0().log.debug(&format!($($arg)*));
            }
        };
    }
}

mod convert;
// Phase 190: Modular converters
mod block_allocator; // Phase 260 P0.2: Block ID allocation utility
mod bridge;
mod call_generator; // Phase 260 P0.2: Call instruction generation utility
mod joinir_block_converter;
mod joinir_function_converter;
mod module_converter;

#[cfg(test)]
mod tests;
pub(crate) use bridge::lower_structured_joinir_to_mir;
pub(crate) use convert::convert_mir_like_inst; // helper for sub-modules
pub(crate) use joinir_function_converter::JoinIrFunctionConverter;

/// JoinIR-to-MIR conversion error.
#[derive(Debug, Clone)]
pub struct JoinIrToMirError {
    pub message: String,
}

impl JoinIrToMirError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

/// JoinFuncId から MIR 用の関数名を生成
pub(crate) fn join_func_name(id: JoinFuncId) -> String {
    format!("join_func_{}", id.0)
}
