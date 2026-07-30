//! Phase 190: JoinIR Function Converter
//!
//! 責務: JoinIR の関数を MIR 関数に変換
//! - パラメータ・ローカル変数の設定
//! - ブロック変換の統合
//! - 関数署名の管理

use crate::mir::join_ir::{JoinFuncId, JoinFunction};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};
use std::collections::BTreeMap;

use super::joinir_block_converter::JoinIrBlockConverter;
use super::JoinIrVmBridgeError;

pub(crate) struct JoinIrFunctionConverter;

impl JoinIrFunctionConverter {
    /// Phase 256 P1.8: Convert function with actual function name map
    ///
    /// This variant ensures Call instructions use actual function names ("main", "loop_step", "k_exit")
    /// instead of generated names ("join_func_0", "join_func_1", etc.)
    pub(crate) fn convert_function_with_func_names(
        join_func: &JoinFunction,
        func_name_map: BTreeMap<JoinFuncId, String>,
    ) -> Result<MirFunction, JoinIrVmBridgeError> {
        let entry_block = BasicBlockId(0);

        let param_types = join_func
            .params
            .iter()
            .map(|_| MirType::Unknown)
            .collect::<Vec<_>>();

        let signature = FunctionSignature {
            name: join_func.name.clone(),
            params: param_types,
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };

        let mut mir_func = MirFunction::new(signature, entry_block);
        mir_func.params = join_func.params.clone();

        // Phase 256 P1.8: Use BlockConverter with function name map
        let mut block_converter = JoinIrBlockConverter::new_with_func_names(func_name_map);
        block_converter.convert_function_body(&mut mir_func, &join_func.body)?;

        // Phase 279 P0: Type propagation for JoinIR → MIR conversion (SSOT)
        Self::propagate_types(&mut mir_func)
            .map_err(|e| JoinIrVmBridgeError::new(format!("Type propagation failed: {}", e)))?;

        debug_log!(
            "[joinir_vm_bridge] Function '{}' has {} blocks:",
            mir_func.signature.name,
            mir_func.blocks.len()
        );
        for (block_id, block) in &mir_func.blocks {
            debug_log!(
                "  Block {:?}: {} instructions, terminator={:?}",
                block_id,
                block.instructions.len(),
                block.terminator
            );
        }

        Ok(mir_func)
    }

    /// Phase 279 P0: Type propagation for JoinIR-converted MIR
    ///
    /// SSOT 型伝播パイプラインを呼び出す。
    /// lifecycle.rs と同じ入口を使用することで、順序ドリフトを防止。
    fn propagate_types(mir_func: &mut MirFunction) -> Result<(), String> {
        use crate::mir::type_propagation::TypePropagationPipeline;

        // Extract value_types to avoid borrow conflicts
        let mut value_types = std::mem::take(&mut mir_func.metadata.value_types);

        // Phase 279 P0: Use SSOT type propagation pipeline
        // 順序固定: Copy → BinOp → Copy → PHI
        TypePropagationPipeline::run(mir_func, &mut value_types)?;

        // Put value_types back
        mir_func.metadata.value_types = value_types;
        Ok(())
    }

    // Phase 279 P0: repropagate_binop_types() static method removed
    // Moved to TypePropagationPipeline (SSOT)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_function_converter_exists() {
        // Basic module structure test
        assert!(true);
    }
}
