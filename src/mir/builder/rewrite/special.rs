use super::super::MirBuilder;

/// Phase 287 P5: Trace helper for toString normalization debugging
fn trace_tostring(msg: impl std::fmt::Display) {
    if crate::config::env::builder_static_call_trace() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[P287-BOXCALL] {}", msg));
    }
}

/// Phase 287 P4: toString/stringify/str → Call(Method) normalization (SSOT)
///
/// Root cause fix: toString is a universal display method that should always use
/// canonical Method callee (not Global fallback). This ensures:
/// - VM/LLVM/JoinIR consistent behavior
/// - Primitive types (Integer/Float/Bool/String) work correctly
/// - No receiver type inference errors
///
/// Strategy: Intercept toString/stringify/str EARLY and emit Call(Method) directly.
/// Do NOT pass to Known/Unique rewrite (those are for user-defined methods only).
pub(crate) fn try_early_str_like_to_dst(
    builder: &mut MirBuilder,
    want_dst: Option<super::super::ValueId>,
    object_value: super::super::ValueId,
    _class_name_opt: &Option<String>,
    method: &str,
    arity: usize,
) -> Option<Result<super::super::ValueId, String>> {
    // Only handle toString/stringify/str with arity=0
    if !(method == "toString" || method == "stringify" || method == "str") || arity != 0 {
        return None;
    }

    trace_tostring(format!(
        "Normalizing {method} to Call(Method) for object_value={:?}",
        object_value
    ));

    // Phase 287 P4: ALWAYS emit Call(Method) for toString/stringify/str.
    // Do NOT rewrite to Global.
    let actual_dst = want_dst.unwrap_or_else(|| builder.next_value_id());

    if let Err(e) = builder.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
        Some(actual_dst),
        object_value,
        "RuntimeDataBox",
        method,
        vec![],
        super::super::EffectMask::PURE,
        crate::mir::definitions::call_unified::TypeCertainty::Union,
    )) {
        return Some(Err(e.to_string()));
    }

    // Annotate result type as value-world String (toString always returns text).
    builder
        .type_ctx
        .value_types
        .insert(actual_dst, super::super::MirType::String);

    trace_tostring(format!(
        "Emitted Call(Method) for {method} -> dst={:?}",
        actual_dst
    ));

    Some(Ok(actual_dst))
}

/// To-dst variant: equals/1 consolidation with requested destination
pub(crate) fn try_special_equals_to_dst(
    builder: &mut MirBuilder,
    want_dst: Option<super::super::ValueId>,
    object_value: super::super::ValueId,
    class_name_opt: &Option<String>,
    method: &str,
    args: Vec<super::super::ValueId>,
) -> Option<Result<super::super::ValueId, String>> {
    if method != "equals" || args.len() != 1 {
        return None;
    }
    if let Some(cls) = class_name_opt.as_ref() {
        if let Some(res) = super::known::try_known_rewrite_to_dst(
            builder,
            want_dst,
            object_value,
            cls,
            method,
            args.clone(),
        ) {
            return Some(res);
        }
    }
    super::known::try_unique_suffix_rewrite_to_dst(builder, want_dst, object_value, method, args)
}
