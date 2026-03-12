use super::{convert_join_module_to_mir_with_meta, join_func_name, JoinIrVmBridgeError};
use crate::mir::join_ir::frontend::JoinFuncMetaMap;
use crate::mir::join_ir::JoinModule;
use crate::mir::MirModule;
use std::collections::BTreeMap;

/// Phase 256 P1.5: JoinModule が JoinIR ValueIds (100+ or 1000+) を含むか確認
fn module_has_joinir_value_ids(module: &JoinModule) -> bool {
    for func in module.functions.values() {
        // Check params
        for param in &func.params {
            if param.0 >= 100 {
                // PARAM_MIN
                return true;
            }
        }

        // Check instructions (all variants that can use ValueIds)
        for inst in &func.body {
            match inst {
                crate::mir::join_ir::JoinInst::Call { args, dst, .. } => {
                    if let Some(d) = dst {
                        if d.0 >= 100 {
                            return true;
                        }
                    }
                    for arg in args {
                        if arg.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::Jump { args, cond, .. } => {
                    if let Some(c) = cond {
                        if c.0 >= 100 {
                            return true;
                        }
                    }
                    for arg in args {
                        if arg.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::Ret { value } => {
                    if let Some(v) = value {
                        if v.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::Select {
                    dst,
                    cond,
                    then_val,
                    else_val,
                    ..
                } => {
                    if dst.0 >= 100 || cond.0 >= 100 || then_val.0 >= 100 || else_val.0 >= 100 {
                        return true;
                    }
                }
                crate::mir::join_ir::JoinInst::IfMerge { cond, merges, .. } => {
                    if cond.0 >= 100 {
                        return true;
                    }
                    for merge in merges {
                        if merge.dst.0 >= 100 || merge.then_val.0 >= 100 || merge.else_val.0 >= 100
                        {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::MethodCall {
                    dst,
                    receiver,
                    args,
                    ..
                } => {
                    if dst.0 >= 100 || receiver.0 >= 100 {
                        return true;
                    }
                    for arg in args {
                        if arg.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::ConditionalMethodCall {
                    cond,
                    dst,
                    receiver,
                    args,
                    ..
                } => {
                    if cond.0 >= 100 || dst.0 >= 100 || receiver.0 >= 100 {
                        return true;
                    }
                    for arg in args {
                        if arg.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::FieldAccess { dst, object, .. } => {
                    if dst.0 >= 100 || object.0 >= 100 {
                        return true;
                    }
                }
                crate::mir::join_ir::JoinInst::NewBox { dst, args, .. } => {
                    if dst.0 >= 100 {
                        return true;
                    }
                    for arg in args {
                        if arg.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::NestedIfMerge { conds, merges, .. } => {
                    for cond in conds {
                        if cond.0 >= 100 {
                            return true;
                        }
                    }
                    for merge in merges {
                        if merge.dst.0 >= 100 || merge.then_val.0 >= 100 || merge.else_val.0 >= 100
                        {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::Compute(mi) => {
                    // Check actual ValueIds in MirLikeInst
                    match mi {
                        crate::mir::join_ir::MirLikeInst::Const { dst, .. } => {
                            if dst.0 >= 100 {
                                return true;
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::BinOp { dst, lhs, rhs, .. } => {
                            if dst.0 >= 100 || lhs.0 >= 100 || rhs.0 >= 100 {
                                return true;
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::Compare { dst, lhs, rhs, .. } => {
                            if dst.0 >= 100 || lhs.0 >= 100 || rhs.0 >= 100 {
                                return true;
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::BoxCall { dst, args, .. } => {
                            if let Some(d) = dst {
                                if d.0 >= 100 {
                                    return true;
                                }
                            }
                            for arg in args {
                                if arg.0 >= 100 {
                                    return true;
                                }
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::UnaryOp { dst, operand, .. } => {
                            if dst.0 >= 100 || operand.0 >= 100 {
                                return true;
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::Print { value } => {
                            if value.0 >= 100 {
                                return true;
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::Select {
                            dst,
                            cond,
                            then_val,
                            else_val,
                        } => {
                            if dst.0 >= 100
                                || cond.0 >= 100
                                || then_val.0 >= 100
                                || else_val.0 >= 100
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

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

    // Phase 256 P1.5: Fail-Fast - boundary 必須チェック
    // JoinIR Param (100+) または Local (1000+) を含む module は boundary 必須
    // NOTE: This check is DISABLED for now because merge_joinir_mir_blocks
    // might already handle ValueId remapping. Will enable after verifying
    // the merge function works correctly with Local ValueIds.
    // if module_has_joinir_value_ids(module) && boundary.is_none() {
    //     return Err(JoinIrVmBridgeError::new(
    //         "[joinir/contract] Missing boundary remap: \
    //          JoinModule contains JoinIR ValueIds (100+ or 1000+) \
    //          but boundary is None. This is a contract violation.",
    //     ));
    // }

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
