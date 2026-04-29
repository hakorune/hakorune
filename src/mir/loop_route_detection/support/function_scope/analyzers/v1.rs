use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::ValueId;

use super::super::helpers::*;
use super::super::types::{CapturedEnv, CapturedKind, CapturedVar};

/// Analyzes function-scoped variables that can be safely captured for loop conditions/body.
///
/// # Phase 200-B Implementation
///
/// Detects function-scoped variables that are effectively immutable constants
/// within a loop context (e.g., `digits` in JsonParser._atoi()).
///
/// # Detection Criteria
///
/// A variable is captured if ALL of the following conditions are met:
///
/// 1. **Declared before the loop**: Variable must be declared in function scope before the loop
/// 2. **Safe constant init**: Initialized with string/integer literal only
/// 3. **Never reassigned**: Variable is never reassigned within the function (is_immutable = true)
/// 4. **Referenced in loop**: Variable is referenced in loop condition or body
/// 5. **Not a loop parameter**: Variable is not in scope.loop_params
/// 6. **Not a body-local**: Variable is not in scope.body_locals
///
/// # Example
///
/// ```nyash
/// method _atoi(s, pos, len) {
///     local digits = "0123456789"  // ✅ Captured (declared before loop, never reassigned)
///     local value = 0               // ❌ Not captured (reassigned in loop body)
///     loop(pos < len) {
///         local ch = s.charAt(pos)  // ❌ Not captured (body-local)
///         local digit = digits.indexOf(ch)
///         value = value * 10 + digit
///         pos = pos + 1
///     }
/// }
/// ```
///
/// # Arguments
///
/// * `fn_body` - AST nodes of the function body (for analysis)
/// * `loop_ast` - AST node of the loop statement
/// * `scope` - LoopScopeShape (for excluding loop params and body-locals)
///
/// # Returns
///
/// `CapturedEnv` containing all captured variables
#[allow(dead_code)]
pub(crate) fn analyze_captured_vars(
    fn_body: &[ASTNode],
    loop_ast: &ASTNode,
    scope: &LoopScopeShape,
) -> CapturedEnv {
    use std::env;

    let debug = env::var("NYASH_CAPTURE_DEBUG").is_ok();

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[capture/debug] Starting capture analysis"));
    }

    // Step 1: Find loop position in fn_body
    let loop_index = match find_stmt_index(fn_body, loop_ast) {
        Some(idx) => idx,
        None => {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/debug] Loop not found in function body, returning empty CapturedEnv"
                ));
            }
            return CapturedEnv::new();
        }
    };

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[capture/debug] Loop found at index {}",
            loop_index
        ));
    }

    // Step 2: Collect local declarations BEFORE the loop
    let pre_loop_locals = collect_local_declarations(&fn_body[..loop_index]);

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[capture/debug] Found {} pre-loop local declarations",
            pre_loop_locals.len()
        ));
    }

    let mut env = CapturedEnv::new();

    // Step 3: For each pre-loop local, check capture criteria
    for (name, init_expr) in pre_loop_locals {
        if debug {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[capture/check] Checking variable '{}'", name));
        }

        // 3a: Is init expression a safe constant?
        if !is_safe_const_init(&init_expr) {
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
        if is_reassigned_in_fn(fn_body, &name) {
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
        if !is_used_in_loop(loop_ast, &name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("[capture/reject] '{}': not used in loop", name));
            }
            continue;
        }

        // 3d: Skip if already in pinned, carriers, or body_locals
        if scope.pinned.contains(&name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': is a pinned variable",
                    name
                ));
            }
            continue;
        }

        if scope.carriers.contains(&name) {
            if debug {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[capture/reject] '{}': is a carrier variable",
                    name
                ));
            }
            continue;
        }

        if scope.body_locals.contains(&name) {
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
        // Note: We don't have access to variable_map here, so we use a placeholder ValueId
        // The actual host_id is resolved by the active condition/boundary binding path.
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
