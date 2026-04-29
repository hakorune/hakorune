use super::{convert_join_module_to_mir_with_meta, join_func_name, JoinIrVmBridgeError};
use crate::mir::join_ir::frontend::JoinFuncMetaMap;
use crate::mir::join_ir::JoinModule;
use crate::mir::MirModule;
use std::collections::BTreeMap;

fn ensure_joinir_function_aliases(mir_module: &mut MirModule, join_module: &JoinModule) {
    for (func_id, join_func) in &join_module.functions {
        let generated_name = join_func_name(*func_id);
        let function = mir_module
            .functions
            .get(&join_func.name)
            .or_else(|| mir_module.functions.get(&generated_name))
            .cloned();

        if let Some(function) = function {
            if !mir_module.functions.contains_key(&join_func.name) {
                mir_module
                    .functions
                    .insert(join_func.name.clone(), function.clone());
            }

            if !mir_module.functions.contains_key(&generated_name) {
                mir_module
                    .functions
                    .insert(generated_name.clone(), function.clone());
            }

            let actual_arity = format!("{}/{}", join_func.name, join_func.params.len());
            if !mir_module.functions.contains_key(&actual_arity) {
                mir_module.functions.insert(actual_arity, function.clone());
            }

            let generated_arity = format!("{}/{}", generated_name, join_func.params.len());
            if !mir_module.functions.contains_key(&generated_arity) {
                mir_module.functions.insert(generated_arity, function);
            }
        }
    }
}

/// Structured JoinIR → MIR（既存経路）の明示エントリ。
pub(crate) fn lower_joinir_structured_to_mir_with_meta(
    module: &JoinModule,
    meta: &JoinFuncMetaMap,
    boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
) -> Result<MirModule, JoinIrVmBridgeError> {
    if !module.is_structured() {
        return Err(JoinIrVmBridgeError::new(
            "[joinir/bridge] expected Structured JoinIR module",
        ));
    }

    convert_join_module_to_mir_with_meta(module, meta, boundary)
}

/// JoinIR → MIR の単一入口。
///
/// Phase R1/R4: runtime bridge is Structured-only; the removed dev-only
/// normalized helper route no longer exists in this module.
///
/// Phase 256 P1.5: boundary parameter for ValueId remapping
pub(crate) fn bridge_joinir_to_mir_with_meta(
    module: &JoinModule,
    meta: &JoinFuncMetaMap,
    boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
) -> Result<MirModule, JoinIrVmBridgeError> {
    let mut mir = lower_joinir_structured_to_mir_with_meta(module, meta, boundary)?;
    ensure_joinir_function_aliases(&mut mir, module);
    Ok(mir)
}

/// JoinIR → MIR（メタなし）呼び出しのユーティリティ。
pub(crate) fn bridge_joinir_to_mir(module: &JoinModule) -> Result<MirModule, JoinIrVmBridgeError> {
    let empty_meta: JoinFuncMetaMap = BTreeMap::new();
    bridge_joinir_to_mir_with_meta(module, &empty_meta, None)
}
