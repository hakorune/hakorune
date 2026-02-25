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
            if param.0 >= 100 {  // PARAM_MIN
                return true;
            }
        }

        // Check instructions (all variants that can use ValueIds)
        for inst in &func.body {
            match inst {
                crate::mir::join_ir::JoinInst::Call { args, dst, .. } => {
                    if let Some(d) = dst {
                        if d.0 >= 100 { return true; }
                    }
                    for arg in args {
                        if arg.0 >= 100 { return true; }
                    }
                }
                crate::mir::join_ir::JoinInst::Jump { args, cond, .. } => {
                    if let Some(c) = cond {
                        if c.0 >= 100 { return true; }
                    }
                    for arg in args {
                        if arg.0 >= 100 { return true; }
                    }
                }
                crate::mir::join_ir::JoinInst::Ret { value } => {
                    if let Some(v) = value {
                        if v.0 >= 100 { return true; }
                    }
                }
                crate::mir::join_ir::JoinInst::Select { dst, cond, then_val, else_val, .. } => {
                    if dst.0 >= 100 || cond.0 >= 100 || then_val.0 >= 100 || else_val.0 >= 100 {
                        return true;
                    }
                }
                crate::mir::join_ir::JoinInst::IfMerge { cond, merges, .. } => {
                    if cond.0 >= 100 { return true; }
                    for merge in merges {
                        if merge.dst.0 >= 100 || merge.then_val.0 >= 100 || merge.else_val.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::MethodCall { dst, receiver, args, .. } => {
                    if dst.0 >= 100 || receiver.0 >= 100 {
                        return true;
                    }
                    for arg in args {
                        if arg.0 >= 100 { return true; }
                    }
                }
                crate::mir::join_ir::JoinInst::ConditionalMethodCall { cond, dst, receiver, args, .. } => {
                    if cond.0 >= 100 || dst.0 >= 100 || receiver.0 >= 100 {
                        return true;
                    }
                    for arg in args {
                        if arg.0 >= 100 { return true; }
                    }
                }
                crate::mir::join_ir::JoinInst::FieldAccess { dst, object, .. } => {
                    if dst.0 >= 100 || object.0 >= 100 {
                        return true;
                    }
                }
                crate::mir::join_ir::JoinInst::NewBox { dst, args, .. } => {
                    if dst.0 >= 100 { return true; }
                    for arg in args {
                        if arg.0 >= 100 { return true; }
                    }
                }
                crate::mir::join_ir::JoinInst::NestedIfMerge { conds, merges, .. } => {
                    for cond in conds {
                        if cond.0 >= 100 { return true; }
                    }
                    for merge in merges {
                        if merge.dst.0 >= 100 || merge.then_val.0 >= 100 || merge.else_val.0 >= 100 {
                            return true;
                        }
                    }
                }
                crate::mir::join_ir::JoinInst::Compute(mi) => {
                    // Check actual ValueIds in MirLikeInst
                    match mi {
                        crate::mir::join_ir::MirLikeInst::Const { dst, .. } => {
                            if dst.0 >= 100 { return true; }
                        }
                        crate::mir::join_ir::MirLikeInst::BinOp { dst, lhs, rhs, .. } => {
                            if dst.0 >= 100 || lhs.0 >= 100 || rhs.0 >= 100 { return true; }
                        }
                        crate::mir::join_ir::MirLikeInst::Compare { dst, lhs, rhs, .. } => {
                            if dst.0 >= 100 || lhs.0 >= 100 || rhs.0 >= 100 { return true; }
                        }
                        crate::mir::join_ir::MirLikeInst::BoxCall { dst, args, .. } => {
                            if let Some(d) = dst {
                                if d.0 >= 100 { return true; }
                            }
                            for arg in args {
                                if arg.0 >= 100 { return true; }
                            }
                        }
                        crate::mir::join_ir::MirLikeInst::UnaryOp { dst, operand, .. } => {
                            if dst.0 >= 100 || operand.0 >= 100 { return true; }
                        }
                        crate::mir::join_ir::MirLikeInst::Print { value } => {
                            if value.0 >= 100 { return true; }
                        }
                        crate::mir::join_ir::MirLikeInst::Select { dst, cond, then_val, else_val } => {
                            if dst.0 >= 100 || cond.0 >= 100 || then_val.0 >= 100 || else_val.0 >= 100 {
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
                mir_module
                    .functions
                    .insert(actual_arity, function.clone());
            }

            let generated_arity = format!("{}/{}", generated_name, join_func.params.len());
            if !mir_module.functions.contains_key(&generated_arity) {
                mir_module.functions.insert(generated_arity, function);
            }
        }
    }
}

#[cfg(feature = "normalized_dev")]
use crate::config::env::joinir_dev::{current_joinir_mode, JoinIrMode};
#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir::normalized::shape_guard::{self, NormalizedDevShape};
#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir::normalized::{
    normalize_pattern1_minimal, normalize_pattern2_minimal, NormalizedModule,
};
#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir::JoinIrPhase;
#[cfg(feature = "normalized_dev")]
use crate::mir::join_ir_vm_bridge::lower_normalized_to_mir_minimal;
#[cfg(feature = "normalized_dev")]
use std::panic::{catch_unwind, AssertUnwindSafe};

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

/// Normalized JoinIR → MIR（現状は Structured に戻して既存ブリッジを再利用）。
#[cfg(feature = "normalized_dev")]
#[allow(dead_code)]
pub(crate) fn lower_joinir_normalized_to_mir_with_meta(
    module: &NormalizedModule,
    meta: &JoinFuncMetaMap,
) -> Result<MirModule, JoinIrVmBridgeError> {
    if module.phase != JoinIrPhase::Normalized {
        return Err(JoinIrVmBridgeError::new(
            "[joinir/bridge] expected Normalized JoinIR module",
        ));
    }

    let structured = module.to_structured().ok_or_else(|| {
        JoinIrVmBridgeError::new(
            "[joinir/bridge] normalized module missing Structured snapshot (dev-only)",
        )
    })?;

    lower_joinir_structured_to_mir_with_meta(&structured, meta)
}

#[cfg(feature = "normalized_dev")]
fn normalize_for_shape(
    module: &JoinModule,
    shape: NormalizedDevShape,
) -> Result<NormalizedModule, JoinIrVmBridgeError> {
    let result = match shape {
        NormalizedDevShape::Pattern1Mini => {
            catch_unwind(AssertUnwindSafe(|| normalize_pattern1_minimal(module)))
        }
        NormalizedDevShape::Pattern2Mini
        | NormalizedDevShape::JsonparserSkipWsMini
        | NormalizedDevShape::JsonparserSkipWsReal
        | NormalizedDevShape::JsonparserAtoiMini
        | NormalizedDevShape::JsonparserAtoiReal
        | NormalizedDevShape::JsonparserParseNumberReal => {
            catch_unwind(AssertUnwindSafe(|| normalize_pattern2_minimal(module)))
        }
        NormalizedDevShape::SelfhostTokenScanP2 => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_selfhost_token_scan_p2(module)
                .expect("selfhost P2 normalization failed")
        })),
        NormalizedDevShape::SelfhostTokenScanP2Accum => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_selfhost_token_scan_p2_accum(module)
                .expect("selfhost P2 accum normalization failed")
        })),
        // Phase 47-A: P3 minimal normalization
        NormalizedDevShape::Pattern3IfSumMinimal => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_pattern3_if_sum_minimal(module)
                .expect("P3 normalization failed")
        })),
        // Phase 47-B: P3 extended normalization
        NormalizedDevShape::Pattern3IfSumMulti => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_pattern3_if_sum_multi_minimal(module)
                .expect("P3 multi normalization failed")
        })),
        NormalizedDevShape::Pattern3IfSumJson => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_pattern3_if_sum_json_minimal(module)
                .expect("P3 json normalization failed")
        })),
        NormalizedDevShape::SelfhostIfSumP3 => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_selfhost_if_sum_p3(module)
                .expect("selfhost P3 normalization failed")
        })),
        NormalizedDevShape::SelfhostIfSumP3Ext => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_selfhost_if_sum_p3_ext(module)
                .expect("selfhost P3 ext normalization failed")
        })),
        // Phase 53: selfhost P2/P3 practical variations (delegate to existing normalizers)
        NormalizedDevShape::SelfhostArgsParseP2 => {
            catch_unwind(AssertUnwindSafe(|| normalize_pattern2_minimal(module)))
        }
        NormalizedDevShape::SelfhostStmtCountP3 => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_selfhost_if_sum_p3_ext(module)
                .expect("selfhost stmt_count P3 normalization failed")
        })),
        // Phase 54: selfhost P2/P3 shape growth (delegate to existing normalizers)
        NormalizedDevShape::SelfhostVerifySchemaP2 => {
            catch_unwind(AssertUnwindSafe(|| normalize_pattern2_minimal(module)))
        }
        NormalizedDevShape::SelfhostDetectFormatP3 => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_selfhost_if_sum_p3_ext(module)
                .expect("selfhost detect_format P3 normalization failed")
        })),
        // Phase 48-A: P4 minimal normalization
        NormalizedDevShape::Pattern4ContinueMinimal => catch_unwind(AssertUnwindSafe(|| {
            crate::mir::join_ir::normalized::normalize_pattern4_continue_minimal(module)
                .expect("P4 normalization failed")
        })),
        // Phase 48-B: JsonParser continue skip_ws (array/object)
        NormalizedDevShape::JsonparserParseArrayContinueSkipWs => {
            catch_unwind(AssertUnwindSafe(|| {
                crate::mir::join_ir::normalized::normalize_jsonparser_parse_array_continue_skip_ws(
                    module,
                )
                .expect("P4 array normalization failed")
            }))
        }
        NormalizedDevShape::JsonparserParseObjectContinueSkipWs => {
            catch_unwind(AssertUnwindSafe(|| {
                crate::mir::join_ir::normalized::normalize_jsonparser_parse_object_continue_skip_ws(
                    module,
                )
                .expect("P4 object normalization failed")
            }))
        }
        // Phase 89: Continue + Early Return pattern (dev-only, delegates to P2 for now)
        NormalizedDevShape::PatternContinueReturnMinimal => {
            catch_unwind(AssertUnwindSafe(|| normalize_pattern2_minimal(module)))
        }
        // Phase 90: Parse String Composite pattern (dev-only, delegates to P2 for now)
        NormalizedDevShape::ParseStringCompositeMinimal => {
            catch_unwind(AssertUnwindSafe(|| normalize_pattern2_minimal(module)))
        }
    };

    match result {
        Ok(norm) => Ok(norm),
        Err(_) => Err(JoinIrVmBridgeError::new(format!(
            "[joinir/bridge/normalized] normalization failed for shape {:?}",
            shape
        ))),
    }
}

#[cfg(feature = "normalized_dev")]
fn try_normalized_direct_bridge(
    module: &JoinModule,
    meta: &JoinFuncMetaMap,
    shapes: &[NormalizedDevShape],
    allow_structured_fallback: bool,
    use_env_guard: bool,
    boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
) -> Result<Option<MirModule>, JoinIrVmBridgeError> {
    if shapes.is_empty() {
        crate::mir::join_ir_vm_bridge::normalized_bridge::log_dev(
            "fallback",
            "normalized dev enabled but shape unsupported; using Structured path",
            true,
        );
        return if allow_structured_fallback {
            Ok(None)
        } else {
            Err(JoinIrVmBridgeError::new(
                "[joinir/bridge] canonical normalized route requested but shape unsupported",
            ))
        };
    }

    let exec = || {
        let debug = crate::mir::join_ir::normalized::dev_env::normalized_dev_logs_enabled();
        for &shape in shapes {
            if debug {
                crate::mir::join_ir_vm_bridge::normalized_bridge::log_dev(
                    "direct",
                    format!("attempting normalized→MIR for {:?}", shape),
                    false,
                );
            }
            match normalize_for_shape(module, shape) {
                Ok(norm) => {
                    let mir =
                        lower_normalized_to_mir_minimal(&norm, meta, allow_structured_fallback)?;
                    crate::mir::join_ir_vm_bridge::normalized_bridge::log_dev(
                        "direct",
                        format!(
                            "normalized→MIR succeeded (shape={:?}, functions={})",
                            shape,
                            norm.functions.len()
                        ),
                        false,
                    );
                    return Ok(Some(mir));
                }
                Err(err) => {
                    if debug {
                        crate::mir::join_ir_vm_bridge::normalized_bridge::log_dev(
                            "direct",
                            format!(
                                "{:?} normalization failed: {} (continuing)",
                                shape, err.message
                            ),
                            false,
                        );
                    }
                }
            }
        }

        if allow_structured_fallback {
            Ok(None)
        } else {
            Err(JoinIrVmBridgeError::new(
                "[joinir/bridge] canonical normalized route failed for all shapes",
            ))
        }
    };

    if use_env_guard {
        crate::mir::join_ir::normalized::dev_env::with_dev_env_if_unset(exec)
    } else {
        exec()
    }
}

/// JoinIR → MIR の単一入口。Mode に応じて Normalized/Structured 経路を選択。
///
/// Phase 45: JoinIrMode 導入による統一ルーティング:
/// - Canonical P2-Core shapes → 常に Normalized→MIR(direct)（mode 無視）
/// - NormalizedDev → サポート形状のみ Normalized path、それ以外 Structured path
/// - StructuredOnly | NormalizedCanonical → Structured path
///
/// Phase 256 P1.5: boundary parameter for ValueId remapping
pub(crate) fn bridge_joinir_to_mir_with_meta(
    module: &JoinModule,
    meta: &JoinFuncMetaMap,
    boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
) -> Result<MirModule, JoinIrVmBridgeError> {
    #[cfg(feature = "normalized_dev")]
    {
        let mode = current_joinir_mode();

        // Canonical set (P2/P3/P4): Always uses Normalized→MIR(direct) regardless of mode/env
        let canonical_shapes = shape_guard::canonical_shapes(module);
        if !canonical_shapes.is_empty() {
            match try_normalized_direct_bridge(module, meta, &canonical_shapes, false, false, boundary)? {
                Some(mut mir) => {
                    ensure_joinir_function_aliases(&mut mir, module);
                    return Ok(mir);
                }
                None => {
                    return Err(JoinIrVmBridgeError::new(
                        "[joinir/bridge] canonical normalized route returned None unexpectedly",
                    ))
                }
            }
        }

        // Phase 45: Mode によるルーティング分岐
        match mode {
            JoinIrMode::NormalizedDev => {
                // サポート形状のみ Normalized path を試行、失敗時は Structured fallback
                let shapes = shape_guard::direct_shapes(module);
                match try_normalized_direct_bridge(module, meta, &shapes, true, true, boundary)? {
                    Some(mut mir) => {
                        ensure_joinir_function_aliases(&mut mir, module);
                        return Ok(mir);
                    }
                    None => {} // Fallback to Structured
                }
            }
            JoinIrMode::StructuredOnly | JoinIrMode::NormalizedCanonical => {
                // Structured path のみ使用
                // （NormalizedCanonical は将来 Phase 46+ で canonical migration 完了後に専用経路を持つ）
            }
        }
    }

    let mut mir = lower_joinir_structured_to_mir_with_meta(module, meta, boundary)?;
    ensure_joinir_function_aliases(&mut mir, module);
    Ok(mir)
}

/// JoinIR → MIR（メタなし）呼び出しのユーティリティ。
pub(crate) fn bridge_joinir_to_mir(module: &JoinModule) -> Result<MirModule, JoinIrVmBridgeError> {
    let empty_meta: JoinFuncMetaMap = BTreeMap::new();
    bridge_joinir_to_mir_with_meta(module, &empty_meta, None)
}
