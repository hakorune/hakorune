use super::super::{MirBuilder, ValueId};

/// Phase 287 P5: Guard against rewriting primitive type toString/stringify/str
///
/// Returns true if should block rewrite (primitive type detected)
/// SSOT: toString/stringify/str use universal slot #0, not user-defined methods
fn should_block_primitive_str_rewrite(
    builder: &MirBuilder,
    object_value: ValueId,
    method: &str,
) -> bool {
    // Only guard toString/stringify/str methods
    if !(method == "toString" || method == "stringify" || method == "str") {
        return false;
    }

    let Some(recv_type) = builder.type_ctx.value_types.get(&object_value) else {
        return false;
    };

    use crate::mir::MirType;
    let is_primitive = match recv_type {
        MirType::Integer | MirType::Float | MirType::Bool | MirType::String => true,
        MirType::Box(name) if name == "IntegerBox" || name == "FloatBox"
            || name == "BoolBox" || name == "StringBox" => true,
        _ => false,
    };

    if is_primitive {
        if crate::config::env::builder_static_call_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[P287-GUARD] Primitive type {:?} uses universal slot #0 for {}, not user-defined method",
                recv_type, method));
        }
        true
    } else {
        false
    }
}

/// Gate: whether instance→function rewrite is enabled.
fn rewrite_enabled() -> bool {
    // New primary flag (P4): NYASH_REWRITE_KNOWN_DEFAULT (default ON; allow explicit OFF)
    if let Some(v) = crate::config::env::builder_rewrite_known_default() {
        let s = v.to_ascii_lowercase();
        if s == "0" || s == "false" || s == "off" {
            return false;
        }
        if s == "1" || s == "true" || s == "on" {
            return true;
        }
        // fallthrough to legacy if malformed
    }
    // Legacy flag (kept for compatibility): NYASH_BUILDER_REWRITE_INSTANCE (default ON)
    match crate::config::env::builder_rewrite_instance_mode()
        .as_deref()
        .map(|v| v.to_ascii_lowercase())
    {
        Some(ref s) if s == "0" || s == "false" || s == "off" => false,
        Some(ref s) if s == "1" || s == "true" || s == "on" => true,
        _ => true, // default ON (spec unchanged; can opt out by setting ...=0)
    }
}

fn rewrite_call_args_for_signature(
    builder: &MirBuilder,
    fname: &str,
    object_value: ValueId,
    mut arg_values: Vec<ValueId>,
) -> Vec<ValueId> {
    let expected_params = builder
        .current_module
        .as_ref()
        .and_then(|module| module.functions.get(fname))
        .map(|func| func.signature.params.len());

    // Static-lowered methods have params == arg_values.len().
    // Instance-lowered methods have params == arg_values.len() + 1 (receiver first).
    let prepend_receiver = !matches!(expected_params, Some(n) if n == arg_values.len());
    if prepend_receiver {
        let mut call_args = Vec::with_capacity(arg_values.len() + 1);
        call_args.push(object_value);
        call_args.append(&mut arg_values);
        call_args
    } else {
        arg_values
    }
}

/// Try Known‑route instance→function rewrite.
/// 既存の安全ガード（user_defined/存在確認/ENV）を尊重して関数化する。
#[allow(dead_code)]
pub(crate) fn try_known_rewrite(
    builder: &mut MirBuilder,
    object_value: ValueId,
    cls: &str,
    method: &str,
    arg_values: Vec<ValueId>,
) -> Option<Result<ValueId, String>> {
    // Global gate
    if !rewrite_enabled() {
        return None;
    }
    // Receiver must be Known (origin 由来)
    if builder
        .type_ctx
        .value_origin_newbox
        .get(&object_value)
        .is_none()
    {
        return None;
    }
    // Only user-defined boxes (plugin/core boxesは対象外)
    if !builder.comp_ctx.user_defined_boxes.contains_key(cls) { // Phase 285LLVM-1.1: HashMap
        return None;
    }

    // Phase 287 P5: Unified primitive guard (toString/stringify/str use universal slot #0)
    if should_block_primitive_str_rewrite(builder, object_value, method) {
        return None;
    }

    // Policy gates（従来互換）
    let allow_userbox_rewrite = crate::config::env::builder_dev_rewrite_userbox();
    let allow_new_origin = crate::config::env::builder_dev_rewrite_new_origin();
    let from_new_origin = builder
        .type_ctx
        .value_origin_newbox
        .get(&object_value)
        .is_some();
    let arity = arg_values.len();
    let fname = crate::mir::builder::calls::function_lowering::generate_method_function_name(
        cls, method, arity,
    );
    let module_has = if let Some(ref module) = builder.current_module {
        module.functions.contains_key(&fname)
    } else {
        false
    };
    if !((module_has || allow_userbox_rewrite) || (from_new_origin && allow_new_origin)) {
        return None;
    }
    // Materialize function call according to lowered function signature:
    // - instance: receiver + args
    // - static:   args only
    let mut call_args = rewrite_call_args_for_signature(builder, &fname, object_value, arg_values);
    if let Err(e) = crate::mir::builder::ssa::local::finalize_args(builder, &mut call_args) {
        return Some(Err(e));
    }
    let dst = builder.next_value_id();
    if let Err(e) = builder.emit_unified_call(
        Some(dst),
        crate::mir::builder::builder_calls::CallTarget::Global(fname.clone()),
        call_args,
    ) {
        return Some(Err(e));
    }
    // Annotate and emit choose
    let chosen = fname.clone();
    builder.annotate_call_result_from_func_name(dst, &chosen);
    let meta = serde_json::json!({
        "recv_cls": cls,
        "method": method,
        "arity": arity,
        "chosen": chosen,
        "reason": "userbox-rewrite",
        "certainty": "Known",
    });
    super::super::observe::resolve::emit_choose(builder, meta);
    Some(Ok(dst))
}

/// Variant: try Known rewrite but honor a requested destination.
pub(crate) fn try_known_rewrite_to_dst(
    builder: &mut MirBuilder,
    want_dst: Option<ValueId>,
    object_value: ValueId,
    cls: &str,
    method: &str,
    arg_values: Vec<ValueId>,
) -> Option<Result<ValueId, String>> {
    if !rewrite_enabled() {
        return None;
    }
    if builder
        .type_ctx
        .value_origin_newbox
        .get(&object_value)
        .is_none()
    {
        return None;
    }
    if !builder.comp_ctx.user_defined_boxes.contains_key(cls) { // Phase 285LLVM-1.1: HashMap
        return None;
    }

    // Phase 287 P5: Unified primitive guard (toString/stringify/str use universal slot #0)
    if should_block_primitive_str_rewrite(builder, object_value, method) {
        return None;
    }

    let allow_userbox_rewrite = crate::config::env::builder_dev_rewrite_userbox();
    let allow_new_origin = crate::config::env::builder_dev_rewrite_new_origin();
    let from_new_origin = builder
        .type_ctx
        .value_origin_newbox
        .get(&object_value)
        .is_some();
    let arity = arg_values.len();
    let fname = crate::mir::builder::calls::function_lowering::generate_method_function_name(
        cls, method, arity,
    );
    let module_has = if let Some(ref module) = builder.current_module {
        module.functions.contains_key(&fname)
    } else {
        false
    };
    if !((module_has || allow_userbox_rewrite) || (from_new_origin && allow_new_origin)) {
        return None;
    }
    // unified global function call (module-local)
    let mut call_args = rewrite_call_args_for_signature(builder, &fname, object_value, arg_values);
    if let Err(e) = crate::mir::builder::ssa::local::finalize_args(builder, &mut call_args) {
        return Some(Err(e));
    }
    let actual_dst = want_dst.unwrap_or_else(|| builder.next_value_id());
    if let Err(e) = builder.emit_unified_call(
        Some(actual_dst),
        crate::mir::builder::builder_calls::CallTarget::Global(fname.clone()),
        call_args,
    ) {
        return Some(Err(e));
    }
    builder.annotate_call_result_from_func_name(actual_dst, &fname);
    let meta = serde_json::json!({
        "recv_cls": cls,
        "method": method,
        "arity": arity,
        "chosen": fname,
        "reason": "userbox-rewrite",
        "certainty": "Known",
    });
    super::super::observe::resolve::emit_choose(builder, meta);
    Some(Ok(actual_dst))
}

/// Fallback: when exactly one user-defined method matches by name/arity across the module,
/// resolve to that even if class inference failed. Deterministic via uniqueness and user-box prefix.
#[allow(dead_code)]
pub(crate) fn try_unique_suffix_rewrite(
    builder: &mut MirBuilder,
    object_value: ValueId,
    method: &str,
    arg_values: Vec<ValueId>,
) -> Option<Result<ValueId, String>> {
    if !rewrite_enabled() {
        return None;
    }
    // Only attempt if receiver is Known (keeps behavior stable and avoids surprises)
    if builder
        .type_ctx
        .value_origin_newbox
        .get(&object_value)
        .is_none()
    {
        return None;
    }
    let mut cands: Vec<String> = builder.method_candidates(method, arg_values.len());
    if cands.len() != 1 {
        return None;
    }
    let fname = cands.remove(0);
    // 🎯 Phase 21.7++ Phase 3: StaticMethodId SSOT 実装
    let id = crate::mir::naming::StaticMethodId::parse(&fname)?;
    if !builder.comp_ctx.user_defined_boxes.contains_key(&id.box_name) { // Phase 285LLVM-1.1: HashMap
        return None;
    }

    // Phase 287 P5: Unified primitive guard (toString/stringify/str use universal slot #0)
    if should_block_primitive_str_rewrite(builder, object_value, method) {
        return None;
    }

    // unified
    let arity_us = arg_values.len();
    let mut call_args = rewrite_call_args_for_signature(builder, &fname, object_value, arg_values);
    if let Err(e) = crate::mir::builder::ssa::local::finalize_args(builder, &mut call_args) {
        return Some(Err(e));
    }
    let dst = builder.next_value_id();
    if let Err(e) = builder.emit_unified_call(
        Some(dst),
        crate::mir::builder::builder_calls::CallTarget::Global(fname.clone()),
        call_args,
    ) {
        return Some(Err(e));
    }
    builder.annotate_call_result_from_func_name(dst, &fname);
    let meta = serde_json::json!({
        "recv_cls": builder.type_ctx.value_origin_newbox.get(&object_value).cloned().unwrap_or_default(),
        "method": method,
        "arity": arity_us,
        "chosen": fname,
        "reason": "unique-suffix",
        "certainty": "Heuristic",
    });
    super::super::observe::resolve::emit_choose(builder, meta);
    Some(Ok(dst))
}

/// Variant: unique-suffix rewrite honoring requested destination.
pub(crate) fn try_unique_suffix_rewrite_to_dst(
    builder: &mut MirBuilder,
    want_dst: Option<ValueId>,
    object_value: ValueId,
    method: &str,
    arg_values: Vec<ValueId>,
) -> Option<Result<ValueId, String>> {
    if !rewrite_enabled() {
        return None;
    }
    if builder
        .type_ctx
        .value_origin_newbox
        .get(&object_value)
        .is_none()
    {
        return None;
    }
    let mut cands: Vec<String> = builder.method_candidates(method, arg_values.len());
    if cands.len() != 1 {
        return None;
    }
    let fname = cands.remove(0);
    // 🎯 Phase 21.7++ Phase 3: StaticMethodId SSOT 実装
    let id = crate::mir::naming::StaticMethodId::parse(&fname)?;
    if !builder.comp_ctx.user_defined_boxes.contains_key(&id.box_name) { // Phase 285LLVM-1.1: HashMap
        return None;
    }

    // Phase 287 P5: Unified primitive guard (toString/stringify/str use universal slot #0)
    if should_block_primitive_str_rewrite(builder, object_value, method) {
        return None;
    }

    let _name_const =
        match crate::mir::builder::name_const::make_name_const_result(builder, &fname) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
    let arity_us = arg_values.len();
    let mut call_args = rewrite_call_args_for_signature(builder, &fname, object_value, arg_values);
    if let Err(e) = crate::mir::builder::ssa::local::finalize_args(builder, &mut call_args) {
        return Some(Err(e));
    }
    let actual_dst = want_dst.unwrap_or_else(|| builder.next_value_id());
    if let Err(e) = builder.emit_unified_call(
        Some(actual_dst),
        crate::mir::builder::builder_calls::CallTarget::Global(fname.clone()),
        call_args,
    ) {
        return Some(Err(e));
    }
    builder.annotate_call_result_from_func_name(actual_dst, &fname);
    let meta = serde_json::json!({
        "recv_cls": builder.type_ctx.value_origin_newbox.get(&object_value).cloned().unwrap_or_default(),
        "method": method,
        "arity": arity_us,
        "chosen": fname,
        "reason": "unique-suffix",
        "certainty": "Heuristic",
    });
    super::super::observe::resolve::emit_choose(builder, meta);
    Some(Ok(actual_dst))
}

/// Unified entry: try Known rewrite first, then unique-suffix fallback.
#[allow(dead_code)]
pub(crate) fn try_known_or_unique(
    builder: &mut MirBuilder,
    object_value: ValueId,
    class_name_opt: &Option<String>,
    method: &str,
    arg_values: Vec<ValueId>,
) -> Option<Result<ValueId, String>> {
    if let Some(cls) = class_name_opt.as_ref() {
        if let Some(res) = try_known_rewrite(builder, object_value, cls, method, arg_values.clone())
        {
            return Some(res);
        }
    }
    try_unique_suffix_rewrite(builder, object_value, method, arg_values)
}

/// Variant: honor requested destination
pub(crate) fn try_known_or_unique_to_dst(
    builder: &mut MirBuilder,
    want_dst: Option<ValueId>,
    object_value: ValueId,
    class_name_opt: &Option<String>,
    method: &str,
    arg_values: Vec<ValueId>,
) -> Option<Result<ValueId, String>> {
    if let Some(cls) = class_name_opt.as_ref() {
        if let Some(res) = try_known_rewrite_to_dst(
            builder,
            want_dst,
            object_value,
            cls,
            method,
            arg_values.clone(),
        ) {
            return Some(res);
        }
    }
    try_unique_suffix_rewrite_to_dst(builder, want_dst, object_value, method, arg_values)
}
