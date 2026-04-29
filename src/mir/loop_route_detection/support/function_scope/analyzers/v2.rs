use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::ValueId;
use std::collections::BTreeSet;

use super::super::helpers::*;
use super::super::types::{CapturedEnv, CapturedKind, CapturedVar};

/// Phase 200-C: Analyze captured vars with condition/body instead of loop_ast
///
/// This variant solves the pointer comparison problem when the loop AST is constructed
/// dynamically (e.g., in break-route lowering). Instead of passing a loop_ast reference,
/// we pass the condition and body directly and perform structural matching.
///
/// # Arguments
///
/// * `fn_body` - AST nodes of the function body (for analysis)
/// * `loop_condition` - Condition expression of the loop
/// * `loop_body` - Body statements of the loop
/// * `scope` - LoopScopeShape (for excluding loop params and body-locals)
///
/// # Returns
///
/// `CapturedEnv` containing all captured variables
#[allow(dead_code)]
pub(crate) fn analyze_captured_vars_v2(
    fn_body: &[ASTNode],
    loop_condition: &ASTNode,
    loop_body: &[ASTNode],
    scope: &LoopScopeShape,
) -> CapturedEnv {
    use std::env;

    let debug = env::var("NYASH_CAPTURE_DEBUG").is_ok();

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[capture/debug] Starting capture analysis v2 (structural matching)"
        ));
    }

    // Step 1: Find loop position in fn_body by structural matching
    let loop_index = find_loop_index_by_structure(fn_body, loop_condition, loop_body);

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        match loop_index {
            Some(idx) => ring0.log.debug(&format!("[capture/debug] Loop found at index {} by structure", idx)),
            None => ring0.log.debug(&format!("[capture/debug] Loop not found in function body by structure (may be unit test or synthetic case)")),
        }
    }

    // Step 2: Collect local declarations BEFORE the loop
    let pre_loop_locals = if let Some(idx) = loop_index {
        collect_local_declarations(&fn_body[..idx])
    } else {
        // No loop found in fn_body - might be a unit test or synthetic case
        // Still collect all locals from fn_body
        collect_local_declarations(fn_body)
    };

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[capture/debug] Found {} pre-loop local declarations",
            pre_loop_locals.len()
        ));
    }

    let mut env = CapturedEnv::new();

    // Step 3: For each pre-loop local, check capture criteria
    for (name, init_expr) in &pre_loop_locals {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[capture/check] Checking variable '{}'", name));
        }

        // 3a: Is init expression a safe constant?
        if !is_safe_const_init(init_expr) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': init is not a safe constant",
                    name
                ));
            }
            continue;
        }

        // 3b: Is this variable reassigned anywhere in fn_body?
        if is_reassigned_in_fn(fn_body, name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': reassigned in function",
                    name
                ));
            }
            continue;
        }

        // 3c: Is this variable used in loop (condition or body)?
        if !is_used_in_loop_parts(loop_condition, loop_body, name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("[capture/reject] '{}': not used in loop", name));
            }
            continue;
        }

        // 3d: Skip if already in pinned, carriers, or body_locals
        if scope.pinned.contains(name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': is a pinned variable",
                    name
                ));
            }
            continue;
        }

        if scope.carriers.contains(name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': is a carrier variable",
                    name
                ));
            }
            continue;
        }

        if scope.body_locals.contains(name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': is a body-local variable",
                    name
                ));
            }
            continue;
        }

        // All checks passed: add to CapturedEnv
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[capture/accept] '{}': ALL CHECKS PASSED, adding to CapturedEnv",
                name
            ));
        }

        env.add_var(CapturedVar {
            name: name.clone(),
            host_id: ValueId(0), // Placeholder, resolved by active binding path.
            is_immutable: true,
            kind: CapturedKind::Explicit,
        });
    }

    // Phase 245C: Capture function parameters used in loop
    let names_in_loop = collect_names_in_loop_parts(loop_condition, loop_body);

    // pre-loop local names (already processed above)
    let pre_loop_local_names: BTreeSet<String> = pre_loop_locals
        .iter()
        .map(|(name, _)| name.clone())
        .collect();

    // Check each variable used in loop
    for name in names_in_loop {
        // Skip if already processed as pre-loop local
        if pre_loop_local_names.contains(&name) {
            continue;
        }

        // Skip if already in pinned, carriers, or body_locals
        if scope.pinned.contains(&name)
            || scope.carriers.contains(&name)
            || scope.body_locals.contains(&name)
        {
            continue;
        }

        // Skip if reassigned in function (function parameters should not be reassigned)
        if is_reassigned_in_fn(fn_body, &name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/param/reject] '{}': reassigned in function",
                    name
                ));
            }
            continue;
        }

        // This is a function parameter-like variable - add to CapturedEnv
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[capture/param/accept] '{}': function parameter used in loop",
                name
            ));
        }

        env.add_var(CapturedVar {
            name: name.clone(),
            host_id: ValueId(0), // Placeholder, resolved by active binding path.
            is_immutable: true,
            kind: CapturedKind::Explicit,
        });
    }

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[capture/result] Captured {} variables: {:?}",
            env.vars.len(),
            env.vars.iter().map(|v| &v.name).collect::<Vec<_>>()
        ));
    }

    env
}
