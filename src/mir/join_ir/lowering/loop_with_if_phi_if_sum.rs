//! Phase 213: Pattern 3 if-sum AST-based lowerer
//!
//! This module implements AST-based JoinIR lowering for "simple if-sum" patterns.
//!
//! # Target Pattern
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
//! - **80/20 rule**: Only handles simple patterns, rejects complex ones (Fail-Fast)
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
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox; // Phase 252 P1
use crate::mir::join_ir::lowering::exit_meta_builder::IfSumExitMetaBuilderBox;
#[cfg(debug_assertions)]
use crate::mir::join_ir::lowering::condition_pattern::{
    analyze_condition_capability, ConditionCapability,
};
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::{
    BinOpKind, CompareOp, ConstValue, JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst,
    UnaryOp,
};
use crate::mir::ValueId;

/// Phase 213: Lower if-sum pattern to JoinIR using AST
///
/// # Arguments
///
/// * `loop_condition` - Loop condition AST (e.g., `i < len`)
/// * `if_stmt` - If statement AST from loop body
/// * `body` - Full loop body AST (for finding counter update)
/// * `cond_env` - ConditionEnv for variable resolution (Phase 220-D)
/// * `join_value_space` - Unified ValueId allocator
///
/// # Returns
///
/// * `Ok((JoinModule, JoinFragmentMeta))` - JoinIR module with exit metadata
/// * `Err(String)` - Pattern not supported or extraction failed
/// Phase 256.7: Condition binding for if-sum lowerer
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
    let trace = DebugOutputBox::new_dev("joinir/pattern3/if-sum");
    trace.log("start", "Starting AST-based if-sum lowering");

    // Allocator for extracting condition values
    let mut alloc_value = || join_value_space.alloc_local();

    #[cfg(debug_assertions)]
    if let ASTNode::If { condition, .. } = if_stmt {
        let capability = analyze_condition_capability(condition);
        debug_assert!(
            matches!(capability, ConditionCapability::IfSumComparable),
            "[if-sum] Unsupported condition passed to AST-based lowerer: {:?}",
            capability
        );
    }

    // Step 1: Extract loop condition info (e.g., i < len → var="i", op=Lt, limit=ValueId)
    // Phase 220-D: Now returns ValueId and instructions for limit
    // Phase 242-EX-A: Now supports complex LHS (e.g., `i % 2 == 1`)
    // Uses cond_env for variable resolution (e.g., `len` in `i < len`)
    let (loop_var, loop_op, loop_lhs_val, loop_limit_val, loop_limit_insts) =
        extract_loop_condition(loop_condition, &mut alloc_value, cond_env)?;
    trace.log(
        "loop-cond",
        &format!("{} {:?} ValueId({})", loop_var, loop_op, loop_limit_val.0)
    );

    // Step 3: Extract then-branch update (e.g., sum = sum + 1 → var="sum", addend=<expr>)
    // Phase 256.7: update_addend is now ASTNode (supports variables like separator)
    let (update_var, update_addend_ast) = extract_then_update(if_stmt)?;
    trace.log(
        "then-update",
        &format!("{} += {:?}", update_var, update_addend_ast)
    );

    // Step 4: Extract counter update (e.g., i = i + 1 → var="i", step=1)
    let (counter_var, counter_step) = extract_counter_update(body, &loop_var)?;
    trace.log(
        "counter-update",
        &format!("{} += {}", counter_var, counter_step)
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
        extract_if_condition(if_stmt, &mut alloc_value, cond_env)?;
    trace.log(
        "if-cond",
        &format!("{} {:?} ValueId({})", if_var, if_op, if_value_val.0)
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
    let sum_for_tail = if let Some(uncond_addend_ast) = extract_unconditional_update(body, &update_var) {
        trace.log(
            "uncond-update",
            &format!("{} += {:?}", update_var, uncond_addend_ast)
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
        &format!("{} {:?} ValueId({})", loop_var, loop_op, loop_limit_val.0)
    );
    trace.log(
        "summary-if",
        &format!("{} {:?} ValueId({})", if_var, if_op, if_value_val.0)
    );
    trace.log(
        "summary-result",
        &format!("expr_result={:?}", sum_final)
    );

    Ok((join_module, fragment_meta))
}

/// Extract loop condition: variable, operator, and limit
///
/// Phase 220-D: Now supports both literals and variables via ConditionEnv.
/// Phase 242-EX-A: Now supports complex LHS expressions (e.g., `i % 2 == 1`).
///
/// Supports: `var < lit`, `var <= lit`, `var > lit`, `var >= lit`
/// Supports: `var < var2`, `var <= var2`, etc.
/// Supports: `expr CmpOp lit` (e.g., `i % 2 == 1`)
///
/// # Returns
///
/// * `Ok((var_name, op, lhs_value, rhs_value, instructions))` where:
///   - `var_name`: Loop variable name (e.g., "i"), empty string for complex LHS
///   - `op`: Comparison operator
///   - `lhs_value`: Optional LHS ValueId (None for simple variable, Some for complex expr)
///   - `rhs_value`: ValueId for the RHS operand
///   - `instructions`: JoinIR instructions to generate both LHS and RHS values
fn extract_loop_condition<F>(
    cond: &ASTNode,
    alloc_value: &mut F,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, Option<ValueId>, ValueId, Vec<JoinInst>), String>
where
    F: FnMut() -> ValueId,
{
    use crate::mir::join_ir::lowering::condition_pattern::{normalize_comparison, ConditionValue};

    // Phase 242-EX-A: Try normalization first for simple cases (fast path)
    if let Some(norm) = normalize_comparison(cond) {
        // Extract normalized variable name and operator
        let var_name = norm.left_var;

        // Convert mir::CompareOp to join_ir::CompareOp
        let op = match norm.op {
            crate::mir::CompareOp::Lt => CompareOp::Lt,
            crate::mir::CompareOp::Gt => CompareOp::Gt,
            crate::mir::CompareOp::Le => CompareOp::Le,
            crate::mir::CompareOp::Ge => CompareOp::Ge,
            crate::mir::CompareOp::Eq => CompareOp::Eq,
            crate::mir::CompareOp::Ne => CompareOp::Ne,
        };

        // Lower the right-hand side using condition_lowerer
        let mut limit_instructions = Vec::new();
        let limit_value = match norm.right {
            ConditionValue::Literal(lit) => {
                let val_id = alloc_value();
                limit_instructions.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: val_id,
                    value: ConstValue::Integer(lit),
                }));
                val_id
            }
            ConditionValue::Variable(var_name) => {
                let var_node = ASTNode::Variable {
                    name: var_name,
                    span: crate::ast::Span {
                        start: 0,
                        end: 0,
                        line: 1,
                        column: 1,
                    },
                };
                lower_value_expression(&var_node, alloc_value, cond_env, None, None, &mut limit_instructions)? // Phase 92 P2-2 + Phase 252
            }
        };

        return Ok((var_name, op, None, limit_value, limit_instructions));
    }

    // Phase 242-EX-A: Normalization failed → handle complex conditions dynamically
    // Support: `expr CmpOp expr` (e.g., `i % 2 == 1`, `a + b > c`)
    match cond {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            use crate::ast::BinaryOperator;

            // Convert operator to CompareOp
            let op = match operator {
                BinaryOperator::Less => CompareOp::Lt,
                BinaryOperator::Greater => CompareOp::Gt,
                BinaryOperator::LessEqual => CompareOp::Le,
                BinaryOperator::GreaterEqual => CompareOp::Ge,
                BinaryOperator::Equal => CompareOp::Eq,
                BinaryOperator::NotEqual => CompareOp::Ne,
                _ => {
                    return Err(format!(
                        "[if-sum] Unsupported operator in condition: {:?}",
                        operator
                    ))
                }
            };

            // Extract base variable name from LHS first
            let var_name = extract_base_variable(left);

            // Phase 256.7-fix: Check if LHS is a simple variable (the loop variable)
            // If so, return None for lhs_val so that the caller uses i_param instead.
            // This avoids using the wrong ValueId from cond_env (which has the loop_var
            // mapped to a ValueId allocated before local_cond_env remapping).
            let (lhs_val_opt, mut instructions) = match left.as_ref() {
                ASTNode::Variable { name, .. } if name == &var_name => {
                    // Simple variable - let caller use i_param
                    (None, Vec::new())
                }
                _ => {
                    // Complex expression - lower it
                    let mut insts = Vec::new();
                    let lhs = lower_value_expression(left, alloc_value, cond_env, None, None, &mut insts)?;
                    (Some(lhs), insts)
                }
            };

            // Lower right-hand side
            let rhs_val = lower_value_expression(right, alloc_value, cond_env, None, None, &mut instructions)?; // Phase 92 P2-2 + Phase 252

            Ok((var_name, op, lhs_val_opt, rhs_val, instructions))
        }
        _ => Err("[if-sum] Expected comparison in condition".to_string()),
    }
}

/// Extract base variable name from an expression
///
/// For `i % 2`, returns "i". For `a + b`, returns "a". For literals, returns empty string.
fn extract_base_variable(expr: &ASTNode) -> String {
    match expr {
        ASTNode::Variable { name, .. } => name.clone(),
        ASTNode::BinaryOp { left, .. } => extract_base_variable(left),
        _ => String::new(),
    }
}

/// Extract if condition: variable, operator, and value
///
/// Phase 220-D: Now supports variables via ConditionEnv
/// Phase 242-EX-A: Now supports complex LHS via extract_loop_condition
fn extract_if_condition<F>(
    if_stmt: &ASTNode,
    alloc_value: &mut F,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, Option<ValueId>, ValueId, Vec<JoinInst>), String>
where
    F: FnMut() -> ValueId,
{
    match if_stmt {
        ASTNode::If { condition, .. } => {
            extract_loop_condition(condition, alloc_value, cond_env) // Same format
        }
        _ => Err("[if-sum] Expected If statement".to_string()),
    }
}

/// Extract then-branch update: variable and addend AST
///
/// Phase 256.7: Returns ASTNode instead of i64 to support variables (e.g., separator)
///
/// Supports: `var = var + <expr>` (expr can be literal or variable)
pub fn extract_then_update(if_stmt: &ASTNode) -> Result<(String, ASTNode), String> {
    match if_stmt {
        ASTNode::If { then_body, .. } => {
            // Find assignment in then block
            for stmt in then_body {
                if let ASTNode::Assignment { target, value, .. } = stmt {
                    let target_name = extract_variable_name(&**target)?;
                    // Check if value is var + <expr>
                    if let ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        let lhs_name = extract_variable_name(left)?;
                        if lhs_name == target_name {
                            // Phase 256.7: Return AST node (supports Variable, Literal, etc.)
                            let addend = right.as_ref().clone();
                            return Ok((target_name, addend));
                        }
                    }
                }
            }
            Err("[if-sum] No valid accumulator update found in then block".to_string())
        }
        _ => Err("[if-sum] Expected If statement".to_string()),
    }
}

/// Phase 256.7: Extract unconditional update from loop body
///
/// Looks for `var = var + <expr>` where var is the accumulator (update_var)
/// and the statement is NOT inside the if statement.
///
/// Pattern: After `if i > 0 { result = result + separator }` there might be
/// `result = result + arr.get(i)` which is the unconditional append.
///
/// # Arguments
///
/// * `body` - Full loop body AST
/// * `update_var` - Accumulator variable name (e.g., "result")
///
/// # Returns
///
/// * `Some(ASTNode)` - Addend expression if unconditional update found
/// * `None` - No unconditional update in loop body
pub fn extract_unconditional_update(body: &[ASTNode], update_var: &str) -> Option<ASTNode> {
    // Skip the if statement (already handled) and loop counter update
    // Look for direct assignment to update_var at the loop body level
    for stmt in body {
        // Skip If statements (already processed)
        if matches!(stmt, ASTNode::If { .. }) {
            continue;
        }

        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let Ok(target_name) = extract_variable_name(&**target) {
                // Check if this is an update to our accumulator (e.g., result = result + ...)
                if target_name == update_var {
                    if let ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if let Ok(lhs_name) = extract_variable_name(left) {
                            if lhs_name == target_name {
                                // This is an unconditional append: var = var + <expr>
                                return Some(right.as_ref().clone());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract counter update: variable and step
///
/// Looks for `var = var + lit` where var is the loop variable
fn extract_counter_update(body: &[ASTNode], loop_var: &str) -> Result<(String, i64), String> {
    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let Ok(target_name) = extract_variable_name(&**target) {
                if target_name == loop_var {
                    if let ASTNode::BinaryOp {
                        operator: crate::ast::BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        let lhs_name = extract_variable_name(left)?;
                        if lhs_name == target_name {
                            let step = extract_integer_literal(right)?;
                            return Ok((target_name, step));
                        }
                    }
                }
            }
        }
    }
    Err(format!(
        "[if-sum] No counter update found for '{}'",
        loop_var
    ))
}

/// Extract variable name from AST node
fn extract_variable_name(node: &ASTNode) -> Result<String, String> {
    match node {
        ASTNode::Variable { name, .. } => Ok(name.clone()),
        _ => Err(format!("[if-sum] Expected variable, got {:?}", node)),
    }
}

/// Extract integer literal from AST node
///
/// Phase 220-D: This function is now only used for simple updates (e.g., `sum + 1`).
/// For condition values, use `lower_value_expression()` which supports variables.
fn extract_integer_literal(node: &ASTNode) -> Result<i64, String> {
    match node {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(n),
            ..
        } => Ok(*n),
        _ => Err(format!("[if-sum] Expected integer literal, got {:?}", node)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};
    use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
    use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
    use crate::mir::join_ir::{BinOpKind, JoinInst, MirLikeInst};

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assignment(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn if_sum_lowering_supports_i_mod_2_eq_1_filter() {
        // Pattern3/4 で使う複雑条件 (i % 2 == 1) が JoinIR に落ちることを確認
        let mut join_value_space = JoinValueSpace::new();
        let mut cond_env = ConditionEnv::new();
        let i_id = join_value_space.alloc_param();
        let len_id = join_value_space.alloc_param();
        cond_env.insert("i".to_string(), i_id);
        cond_env.insert("len".to_string(), len_id);

        let loop_condition = bin(BinaryOperator::Less, var("i"), var("len"));
        let if_condition = bin(
            BinaryOperator::Equal,
            bin(BinaryOperator::Modulo, var("i"), int_lit(2)),
            int_lit(1),
        );

        let sum_update = assignment(var("sum"), bin(BinaryOperator::Add, var("sum"), int_lit(1)));
        let counter_update = assignment(var("i"), bin(BinaryOperator::Add, var("i"), int_lit(1)));

        let if_stmt = ASTNode::If {
            condition: Box::new(if_condition),
            then_body: vec![sum_update],
            else_body: None,
            span: Span::unknown(),
        };
        let body = vec![if_stmt.clone(), counter_update];

        let (module, _meta) = lower_if_sum_pattern(
            &loop_condition,
            &if_stmt,
            &body,
            &cond_env,
            &mut join_value_space,
            &[],
        )
        .expect("if-sum lowering should succeed");

        let mut has_mod = false;
        let mut has_compare = false;

        for func in module.functions.values() {
            for inst in &func.body {
                match inst {
                    JoinInst::Compute(MirLikeInst::BinOp {
                        op: BinOpKind::Mod, ..
                    }) => {
                        has_mod = true;
                    }
                    JoinInst::Compute(MirLikeInst::Compare { .. }) => {
                        has_compare = true;
                    }
                    _ => {}
                }
            }
        }

        assert!(has_mod, "expected modulo lowering in JoinIR output");
        assert!(has_compare, "expected compare lowering in JoinIR output");
    }
}
