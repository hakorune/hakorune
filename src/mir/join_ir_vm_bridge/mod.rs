//! Phase 27-shortterm S-4: JoinIR → Rust VM Bridge
//!
//! 目的: Structured JoinIR を Rust VM / MIR 側へ渡すブリッジ層
//!
//! ## Architecture
//! ```text
//! Structured JoinIR → MirModule → Rust VM → Result
//!         ↑                ↑          ↑
//!     recipe/lower      VM input    Execution
//!       output           format    (GC, plugins)
//! ```
//!
//! ## Design Principles
//! - Structured JoinIR の mainline lower/bridge を単一入口に保つ
//! - Recipe-first runtime lane と喧嘩する別入口を増やさない
//! - VM の機能（GC、プラグイン、エラーハンドリング）を活用
//!
//! ## Minimal Instruction Set (S-4.3)
//! - **Compute**: Const, BinOp, Compare
//! - **BoxCall**: StringBox メソッド呼び出し
//! - **Call/Jump/Ret**: 制御フロー
//!
//! Phase 27-shortterm scope: skip_ws で green 化できれば成功

#![allow(dead_code)]

use crate::backend::VMError;
use crate::mir::join_ir::JoinFuncId;

#[macro_use]
mod logging {
    macro_rules! debug_log {
        ($($arg:tt)*) => {
            if crate::config::env::joinir_vm_bridge_debug() {
                crate::runtime::get_global_ring0().log.debug(&format!($($arg)*));
            }
        };
    }
}

mod convert;
// Phase 190: Modular converters
mod block_allocator; // Phase 260 P0.2: Block ID allocation utility
mod block_finalizer; // Phase 260 P0.3: PHI-preserving block finalization
mod bridge;
mod call_generator; // Phase 260 P0.2: Call instruction generation utility
mod handlers; // Phase 260 P0.2: Modularized JoinIR instruction handlers
mod joinir_block_converter;
mod joinir_function_converter;
mod merge_variable_handler; // Phase 260 P0.2: Merge copy emission utility
mod meta;
mod runner;
mod terminator_builder; // Phase 260 P0.3: Terminator creation utility

#[cfg(test)]
mod tests;

// Phase 190: Use modularized converters
pub(crate) use bridge::{bridge_joinir_to_mir, bridge_joinir_to_mir_with_meta};
#[allow(unused_imports)]
pub(crate) use convert::convert_joinir_to_mir;
pub(crate) use convert::convert_mir_like_inst; // helper for sub-modules
pub(crate) use joinir_function_converter::JoinIrFunctionConverter;
pub use meta::convert_join_module_to_mir_with_meta;
pub use runner::run_joinir_via_vm;

/// Phase 27-shortterm S-4 エラー型
#[derive(Debug, Clone)]
pub struct JoinIrVmBridgeError {
    pub message: String,
}

impl JoinIrVmBridgeError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl From<VMError> for JoinIrVmBridgeError {
    fn from(err: VMError) -> Self {
        JoinIrVmBridgeError::new(format!("VM error: {:?}", err))
    }
}

/// JoinFuncId から MIR 用の関数名を生成
pub(crate) fn join_func_name(id: JoinFuncId) -> String {
    format!("join_func_{}", id.0)
}
