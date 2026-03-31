//! Phase 213: if_phi_join if-sum AST-based lowerer
//!
//! This module implements AST-based JoinIR lowering for a simple if-sum
//! branch inside the `if_phi_join` route.
//!
//! # Target Route Shape
//!
//! ```nyash
//! loop(i < len) {
//!     if i > 0 {
//!         sum = sum + 1
//!     }
//!     i = i + 1
//! }
//! ```
//!
//! # Design Philosophy
//!
//! - **AST-driven**: Loop condition, if condition, and updates extracted from AST
//! - **80/20 rule**: Only handles simple route shapes, rejects complex ones (Fail-Fast)
//! - **Reuses existing infrastructure**: JoinValueSpace, ExitMeta, CarrierInfo
//!
//! # Comparison with Legacy PoC
//!
//! | Aspect           | Legacy (loop_with_if_phi_minimal.rs) | AST-based (this file) |
//! |------------------|--------------------------------------|----------------------|
//! | Loop condition   | Hardcoded (i <= 5)                   | From `condition` AST |
//! | If condition     | Hardcoded (i % 2 == 1)               | From `if_stmt` AST   |
//! | Carrier updates  | Hardcoded (sum + i)                  | From AST assignments |
//! | Flexibility      | Test-only                            | Any if-sum pattern   |

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::carrier_info::JoinFragmentMeta;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowerer::lower_value_expression;
#[cfg(debug_assertions)]
use crate::mir::join_ir::lowering::condition_pattern::{
    analyze_condition_capability, ConditionCapability,
};
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox; // Phase 252 P1
use crate::mir::join_ir::lowering::exit_meta_builder::IfSumExitMetaBuilderBox;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{
    BinOpKind, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst, UnaryOp,
};
use crate::mir::ValueId;

mod extract;

#[cfg(test)]
mod tests;

/// Phase 213: Condition binding for if-sum lowerer
/// Maps a variable name to its JoinIR ValueId (used for main() params)
pub struct IfSumConditionBinding {
    pub name: String,
    pub join_value: ValueId,
}

pub fn lower_if_sum_pattern(
    loop_condition: &ASTNode,
    if_stmt: &ASTNode,
    body: &[ASTNode],
    cond_env: &ConditionEnv,
    join_value_space: &mut JoinValueSpace,
    condition_bindings: &[IfSumConditionBinding], // Phase 256.7: External variable bindings
) -> Result<(JoinModule, JoinFragmentMeta), String> {
    // Phase 252 P1: Use DebugOutputBox for unified trace output
    let trace = DebugOutputBox::new_dev("joinir/if_phi_join/if_sum");
    trace.log("start", "Starting AST-based if-sum lowering");

    // Allocator for extracting condition values
    let mut alloc_value = || join_value_space.alloc_local();

    #[cfg(debug_assertions)]
    if let ASTNode::If { condition, .. } = if_stmt {
        let capability = analyze_condition_capability(condition);
        debug_assert!(
            matches!(capability, ConditionCapability::IfPhiJoinComparable),
            "[if-sum] Unsupported condition passed to AST-based lowerer: {:?}",
            capability
        );
    }

    // Step 1: Extract loop condition info (e.g., i < len → var="i", op=Lt, limit=ValueId)
    // Phase 220-D: Now returns ValueId and instructions for limit
    // Phase 242-EX-A: Now supports complex LHS (e.g., `i % 2 == 1`)
    // Uses cond_env for variable resolution (e.g., `len` in `i < len`)
    let (loop_var, loop_op, loop_lhs_val, loop_limit_val, loop_limit_insts) =
        extract::extract_loop_condition(loop_condition, &mut alloc_value, cond_env)?;
    trace.log(
        "loop-cond",
        &format!("{} {:?} ValueId({})", loop_var, loop_op, loop_limit_val.0),
    );

    // Step 3: Extract then-branch update (e.g., sum = sum + 1 → var="sum", addend=<expr>)
    // Phase 256.7: update_addend is now ASTNode (supports variables like separator)
    let (update_var, update_addend_ast) = extract::extract_then_update(if_stmt)?;
    trace.log(
        "then-update",
        &format!("{} += {:?}", update_var, update_addend_ast),
    );

    // Step 4: Extract counter update (e.g., i = i + 1 → var="i", step=1)
    let (counter_var, counter_step) = extract::extract_counter_update(body, &loop_var)?;
    trace.log(
        "counter-update",
        &format!("{} += {}", counter_var, counter_step),
    );

    // Step 5: Generate JoinIR
    let mut alloc_value = || join_value_space.alloc_local();
    let mut join_module = JoinModule::new();

    // Function IDs
    let main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);
    let k_exit_id = JoinFuncId::new(2);

    // === ValueId allocation ===
    // main() locals
    let i_init_val = alloc_value(); // i = 0
    let sum_init_val = alloc_value(); // sum = 0
    let loop_result = alloc_value(); // result from loop_step

    // loop_step params
    let i_param = alloc_value();
    let sum_param = alloc_value();

    // Phase 256.7: Create a local cond_env with correct loop variable mapping
    // The caller's cond_env has loop_var → ValueId(100) (Param region),
    // but we need loop_var → i_param (Local region) for correct expression lowering
    // IMPORTANT: Also remap condition bindings to loop_step's own params!
    let mut local_cond_env = cond_env.clone();
    local_cond_env.insert(loop_var.clone(), i_param);
    local_cond_env.insert(update_var.clone(), sum_param);
    // Note: Condition binding remapping happens AFTER loop_step_cond_params allocation (see below)

    // loop_step locals
    // Phase 220-D: loop_limit_val and if_value_val are already allocated by extract_*_condition()
    // and will be used directly from their return values
    let cmp_loop = alloc_value(); // loop condition comparison
    let exit_cond = alloc_value(); // negated loop condition
    let if_cmp = alloc_value(); // if condition comparison
    let sum_then = alloc_value(); // sum + addend (Phase 256.7: addend via lower_value_expression)
                                  // Phase 256.7: const_0, sum_else, step_const removed (+0 else branch eliminated)
    let sum_new = alloc_value(); // Select result for sum
    let i_next = alloc_value(); // i + step

    // k_exit params
    let sum_final = alloc_value();

    // === main() params setup ===
    // Phase 256.7: main() params = condition_bindings (external variables like arr, separator)
    // These are the ValueIds that the boundary knows how to map to HOST
    let main_params: Vec<ValueId> = condition_bindings.iter().map(|b| b.join_value).collect();

    // Phase 256.7: Remap local_cond_env to use main_params (not separate loop_step params)
    // The boundary maps main_params → HOST, so we must use main_params in JoinIR instructions
    // This ensures lower_value_expression uses ValueIds the merger can remap
    for (binding, &main_param) in condition_bindings.iter().zip(main_params.iter()) {
        local_cond_env.insert(binding.name.clone(), main_param);
    }
    let cond_env = &local_cond_env; // Shadow the original cond_env

    // Step 2: Extract if condition info (e.g., i > 0 → var="i", op=Gt, value=ValueId)
    //
    // IMPORTANT (Phase 283 P0): Extract using the local_cond_env.
    // The pre-loop extraction phase runs before `i_param`/`sum_param` exist, so
    // `i % 2 == 1` would resolve `i` via the caller's ConditionEnv and produce
    // JoinIR ValueIds that the boundary cannot remap → undefined ValueId in MIR.
    //
    // By extracting here, Variable("i") resolves to `i_param` and remains remappable.
    let (if_var, if_op, if_lhs_val, if_value_val, if_value_insts) =
        extract::extract_if_condition(if_stmt, &mut alloc_value, cond_env)?;
    trace.log(
        "if-cond",
        &format!("{} {:?} ValueId({})", if_var, if_op, if_value_val.0),
    );

    // === main() function ===
    let mut main_func = JoinFunction::new(main_id, "main".to_string(), main_params.clone());

    // i_init = 0 (initial value from ctx)
    main_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: i_init_val,
        value: ConstValue::Integer(0), // TODO: Get from AST
    }));

    // sum_init = "" (empty string for string accumulator) or 0 (for integer)
    // Phase 256.7: Use empty string for join/2 pattern
    main_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: sum_init_val,
        value: ConstValue::String(String::new()), // Empty string for string accumulator
    }));

    // result = loop_step(i_init, sum_init, ...condition_bindings)
    // Phase 256.7: Pass condition bindings to loop_step
    let mut loop_step_call_args = vec![i_init_val, sum_init_val];
    loop_step_call_args.extend(main_params.iter().copied());
    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: loop_step_call_args,
        k_next: None,
        dst: Some(loop_result),
    });

    main_func.body.push(JoinInst::Ret {
        value: Some(loop_result),
    });

    join_module.add_function(main_func);

    // === loop_step(i, sum, ...condition_bindings) function ===
    // Phase 256.7: loop_step params include condition_bindings for external variables
    // Use main_params for condition bindings - the boundary maps these to HOST values
    let mut loop_step_params = vec![i_param, sum_param];
    loop_step_params.extend(main_params.iter().copied());
    let mut loop_step_func = JoinFunction::new(
        loop_step_id,
        "loop_step".to_string(),
        loop_step_params.clone(),
    );

    // --- Exit Condition Check ---
    // Phase 220-D: Prepend loop limit instructions (generated from AST)
    // Phase 242-EX-A: Now handles complex LHS expressions
    // This handles both literals (Const) and variables (from ConditionEnv)
    for inst in loop_limit_insts {
        loop_step_func.body.push(inst);
    }

    // Compare: i < limit (or other op from AST)
    // Phase 242-EX-A: Use computed LHS if available, otherwise use loop parameter
    let loop_lhs = loop_lhs_val.unwrap_or(i_param);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: cmp_loop,
            op: loop_op,
            lhs: loop_lhs,
            rhs: loop_limit_val,
        }));

    // exit_cond = !cmp_loop
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::UnaryOp {
            dst: exit_cond,
            op: UnaryOp::Not,
            operand: cmp_loop,
        }));

    // Jump to exit if condition is false
    loop_step_func.body.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: vec![sum_param],
        cond: Some(exit_cond),
    });

    // --- If Condition (AST-based) ---
    // Phase 220-D: Prepend if value instructions (generated from AST)
    // Phase 242-EX-A: Now handles complex LHS expressions
    for inst in if_value_insts {
        loop_step_func.body.push(inst);
    }

    // Compare: if_var <op> if_value
    // Phase 242-EX-A: Use computed LHS if available, otherwise use loop parameter
    let if_lhs = if_lhs_val.unwrap_or(i_param);
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Compare {
            dst: if_cmp,
            op: if_op,
            lhs: if_lhs,
            rhs: if_value_val,
        }));

    // --- Then Branch ---
    // Phase 256.7: Lower addend expression (supports variables like separator)
    let mut then_update_insts = Vec::new();
    let addend_val = lower_value_expression(
        &update_addend_ast,
        &mut alloc_value,
        cond_env,
        None,
        None,
        &mut then_update_insts,
    )?;
    for inst in then_update_insts {
        loop_step_func.body.push(inst);
    }
    // sum_then = sum + addend
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: sum_then,
            op: BinOpKind::Add,
            lhs: sum_param,
            rhs: addend_val,
        }));

    // --- Select ---
    // Phase 256.7: else は保持（+0 撤去）
    // Select: sum_new = cond ? sum_then : sum_param
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Select {
            dst: sum_new,
            cond: if_cmp,
            then_val: sum_then,
            else_val: sum_param, // 保持
        }));

    // --- Phase 256.7: Unconditional Append ---
    // Handle patterns like: result = result + arr.get(i)
    // This comes AFTER the conditional update (if any)
    let sum_for_tail =
        if let Some(uncond_addend_ast) = extract::extract_unconditional_update(body, &update_var) {
            trace.log(
                "uncond-update",
                &format!("{} += {:?}", update_var, uncond_addend_ast),
            );

            // Allocate ValueId for the unconditional append result
            let sum_after = alloc_value();

            // Lower the unconditional addend expression (e.g., arr.get(i))
            let mut uncond_insts = Vec::new();
            let uncond_val = lower_value_expression(
                &uncond_addend_ast,
                &mut alloc_value,
                cond_env,
                None,
                None,
                &mut uncond_insts,
            )?;

            // Emit instructions for lowering the addend
            for inst in uncond_insts {
                loop_step_func.body.push(inst);
            }

            // sum_after = sum_new + uncond_val
            loop_step_func
                .body
                .push(JoinInst::Compute(MirLikeInst::BinOp {
                    dst: sum_after,
                    op: BinOpKind::Add,
                    lhs: sum_new,
                    rhs: uncond_val,
                }));

            sum_after // Use this in tail call
        } else {
            sum_new // No unconditional update, use conditional result
        };

    // --- Counter Update ---
    let step_const2 = alloc_value();
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::Const {
            dst: step_const2,
            value: ConstValue::Integer(counter_step),
        }));
    loop_step_func
        .body
        .push(JoinInst::Compute(MirLikeInst::BinOp {
            dst: i_next,
            op: BinOpKind::Add,
            lhs: i_param,
            rhs: step_const2,
        }));

    // --- Tail Recursion ---
    // Phase 256.7: Pass condition bindings to recursive call
    // Use sum_for_tail which accounts for unconditional append (if any)
    // Use main_params for condition bindings (same values passed through)
    let mut tail_call_args = vec![i_next, sum_for_tail];
    tail_call_args.extend(main_params.iter().copied());
    loop_step_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: tail_call_args,
        k_next: None,
        dst: None,
    });

    join_module.add_function(loop_step_func);

    // === k_exit(sum_final) function ===
    let mut k_exit_func = JoinFunction::new(k_exit_id, "k_exit".to_string(), vec![sum_final]);

    k_exit_func.body.push(JoinInst::Ret {
        value: Some(sum_final),
    });

    join_module.add_function(k_exit_func);
    join_module.entry = Some(main_id);

    // Phase 118: Build ExitMeta using IfSumExitMetaBuilderBox
    // SSOT: use the accumulator name extracted from AST (no hardcoded carrier names).
    let exit_meta_builder = IfSumExitMetaBuilderBox::new();
    let exit_meta = exit_meta_builder.build_single(update_var.clone(), sum_final)?;

    // Phase 215-2: Use with_expr_result to propagate sum as loop result
    // sum_final is the k_exit return value - this is what `return sum` should use
    let fragment_meta = JoinFragmentMeta::with_expr_result(sum_final, exit_meta);

    trace.log("complete", "Generated AST-based JoinIR");
    trace.log(
        "summary-loop",
        &format!("{} {:?} ValueId({})", loop_var, loop_op, loop_limit_val.0),
    );
    trace.log(
        "summary-if",
        &format!("{} {:?} ValueId({})", if_var, if_op, if_value_val.0),
    );
    trace.log("summary-result", &format!("expr_result={:?}", sum_final));

    Ok((join_module, fragment_meta))
}
