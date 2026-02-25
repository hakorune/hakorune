//! Structural validation for Normalized shadow JoinModule (dev/strict)
//!
//! - Fail-Fast in strict mode with `freeze_with_hint`
//! - No value parity; structure only

use crate::mir::join_ir::{JoinFuncId, JoinInst, JoinModule};
use crate::mir::join_ir::lowering::error_tags;
use super::env_layout::EnvLayout;
use std::collections::BTreeMap;

/// Verify Normalized JoinModule structure emitted by the shadow lowerer.
///
/// ## Contract
/// - Module phase must be Normalized
/// - Entry exists and all functions share the same env arity
/// - No IfMerge/NestedIfMerge (PHI is forbidden in Phase 129-B scope)
/// - join_k tailcalls must target one function that ends with `Ret(Some)`
pub fn verify_normalized_structure(
    module: &JoinModule,
    expected_env_fields: usize,
) -> Result<(), String> {
    // Check phase
    if !module.is_normalized() {
        return Err(error_tags::freeze_with_hint(
            "phase129/join_k/not_normalized",
            &format!("module phase is not Normalized: {:?}", module.phase),
            "ensure the shadow lowering marks module as Normalized",
        ));
    }

    if module.functions.is_empty() {
        return Err(error_tags::freeze_with_hint(
            "phase129/join_k/no_functions",
            "no functions in module",
            "ensure the shadow lowering emits at least the entry function",
        ));
    }

    // Check entry point
    let entry_id = module.entry.ok_or_else(|| {
        error_tags::freeze_with_hint(
            "phase129/join_k/no_entry",
            "no entry point in module",
            "ensure the shadow lowering sets JoinModule.entry",
        )
    })?;

    // Check entry function exists
    let entry_func = module.functions.get(&entry_id).ok_or_else(|| {
        error_tags::freeze_with_hint(
            "phase129/join_k/entry_missing",
            &format!("entry function {:?} not found", entry_id),
            "ensure the emitted module includes the entry function id",
        )
    })?;

    // Env layout: writes + inputs (SSOT)
    if entry_func.params.len() != expected_env_fields {
        return Err(error_tags::freeze_with_hint(
            "phase129/join_k/env_arity_mismatch",
            &format!(
                "env args mismatch: expected {}, got {}",
                expected_env_fields,
                entry_func.params.len()
            ),
            "ensure env params are built from (writes + inputs) SSOT",
        ));
    }

    // All functions in this shadow module must share the same env param arity.
    for (fid, func) in &module.functions {
        if func.params.len() != expected_env_fields {
            return Err(error_tags::freeze_with_hint(
                "phase129/join_k/env_arity_mismatch",
                &format!(
                    "env args mismatch in {:?}: expected {}, got {}",
                    fid,
                    expected_env_fields,
                    func.params.len()
                ),
                "ensure all continuations share the same env layout (writes + inputs)",
            ));
        }
    }

    // PHI prohibition (Phase 129-B scope): no IfMerge/NestedIfMerge in shadow output.
    for (fid, func) in &module.functions {
        for inst in &func.body {
            if matches!(inst, JoinInst::IfMerge { .. } | JoinInst::NestedIfMerge { .. }) {
                return Err(error_tags::freeze_with_hint(
                    "phase129/join_k/phi_forbidden",
                    &format!("PHI-like merge instruction found in {:?}", fid),
                    "Phase 129-B join_k path forbids IfMerge/NestedIfMerge; use join_k tailcall merge instead",
                ));
            }
        }
    }

    // Detect join_k tailcall form (if present) and validate it.
    fn tailcall_target(func: &crate::mir::join_ir::JoinFunction) -> Option<(JoinFuncId, usize)> {
        match func.body.last()? {
            JoinInst::Call {
                func,
                args,
                k_next: None,
                dst: None,
            } => Some((*func, args.len())),
            _ => None,
        }
    }

    let mut tailcall_targets: Vec<(JoinFuncId, usize)> = Vec::new();
    for func in module.functions.values() {
        if let Some((target, argc)) = tailcall_target(func) {
            tailcall_targets.push((target, argc));
        }
    }

    if tailcall_targets.is_empty() {
        return Ok(());
    }

    // join_k merge should have at least two branch continuations tailcalling the same target.
    if tailcall_targets.len() < 2 {
        return Err(error_tags::freeze_with_hint(
            "phase129/join_k/tailcall_count",
            &format!(
                "join_k tailcall form requires >=2 tailcalls, got {}",
                tailcall_targets.len()
            ),
            "ensure both then/else branches tailcall join_k as the last instruction",
        ));
    }

    let first_target = tailcall_targets[0].0;
    for (target, argc) in &tailcall_targets {
        if *target != first_target {
            return Err(error_tags::freeze_with_hint(
                "phase129/join_k/tailcall_target_mismatch",
                "tailcalls do not target a single join_k function",
                "ensure then/else both tailcall the same join_k function id",
            ));
        }
        if *argc != expected_env_fields {
            return Err(error_tags::freeze_with_hint(
                "phase129/join_k/tailcall_arg_arity_mismatch",
                &format!(
                    "tailcall env arg count mismatch: expected {}, got {}",
                    expected_env_fields, argc
                ),
                "ensure join_k is called with the full env fields list (writes + inputs)",
            ));
        }
    }

    let join_k_func = module.functions.get(&first_target).ok_or_else(|| {
        error_tags::freeze_with_hint(
            "phase129/join_k/join_k_missing",
            "tailcall target join_k function not found in module",
            "ensure join_k is registered in JoinModule.functions",
        )
    })?;

    // Phase 129-C: Check if join_k tailcalls post_k (post-if continuation)
    match join_k_func.body.last() {
        Some(JoinInst::Ret { value: Some(_) }) => {
            // Phase 129-B: if-as-last pattern (join_k returns directly)
            Ok(())
        }
        Some(JoinInst::Call {
            func: post_k_id,
            args,
            k_next: None,
            dst: None,
        }) => {
            // Phase 129-C: post-if pattern (join_k tailcalls post_k)
            // Verify post_k exists
            let post_k_func = module.functions.get(post_k_id).ok_or_else(|| {
                error_tags::freeze_with_hint(
                    "phase129/post_k/post_k_missing",
                    "join_k tailcalls post_k but post_k function not found in module",
                    "ensure post_k is registered in JoinModule.functions",
                )
            })?;

            // Verify post_k has same env arity
            if post_k_func.params.len() != expected_env_fields {
                return Err(error_tags::freeze_with_hint(
                    "phase129/post_k/env_arity_mismatch",
                    &format!(
                        "post_k env args mismatch: expected {}, got {}",
                        expected_env_fields,
                        post_k_func.params.len()
                    ),
                    "ensure post_k shares the same env layout (writes + inputs)",
                ));
            }

            // Verify join_k passes correct number of args to post_k
            if args.len() != expected_env_fields {
                return Err(error_tags::freeze_with_hint(
                    "phase129/post_k/tailcall_arg_arity_mismatch",
                    &format!(
                        "join_k→post_k arg count mismatch: expected {}, got {}",
                        expected_env_fields,
                        args.len()
                    ),
                    "ensure join_k passes full env to post_k",
                ));
            }

            // Verify post_k ends with Ret
            match post_k_func.body.last() {
                Some(JoinInst::Ret { .. }) => Ok(()),
                _ => Err(error_tags::freeze_with_hint(
                    "phase129/post_k/not_ret",
                    "post_k must end with Ret",
                    "ensure post_k executes post-if statements and returns",
                )),
            }
        }
        _ => Err(error_tags::freeze_with_hint(
            "phase129/join_k/invalid_terminator",
            "join_k must end with Ret(Some) or TailCall(post_k)",
            "Phase 129-B: join_k→Ret(Some), Phase 129-C: join_k→TailCall(post_k)",
        )),
    }
}

/// Phase 130 P3: Verify env map keyset stays within env layout
///
/// ## Contract
/// - The env map must not introduce variables outside the env layout (`writes + inputs`)
///
/// ## Implementation Note
/// This is a structural check only. It does not (yet) prove that `inputs` are never
/// reassigned; it only ensures the env map does not grow beyond the declared layout.
pub fn verify_env_writes_discipline(
    env_map: &BTreeMap<String, crate::mir::ValueId>,
    env_layout: &EnvLayout,
    context: &str,
) -> Result<(), String> {
    let expected_fields: std::collections::BTreeSet<&String> = env_layout
        .writes
        .iter()
        .chain(env_layout.inputs.iter())
        .collect();

    for var_name in env_map.keys() {
        if !expected_fields.contains(var_name) {
            return Err(error_tags::freeze_with_hint(
                "phase130/verifier/env_unexpected_var",
                &format!(
                    "{}: unexpected variable '{}' in env map (not in writes or inputs)",
                    context, var_name
                ),
                "ensure env updates only reference variables from the env layout (writes + inputs)",
            ));
        }
    }

    Ok(())
}
