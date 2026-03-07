//! Phase 188-Impl-2: LoopBreak (loop with conditional break) Minimal Lowerer
//!
//! Target: apps/tests/joinir_min_loop.hako
//!
//! Code:
//! ```nyash
//! static box JoinIrMin {
//!   main() {
//!     local i = 0
//!     loop(i < 3) {
//!       if i >= 2 { break }
//!       i = i + 1
//!     }
//!     return i
//!   }
//! }
//! ```
//!
//! Expected JoinIR:
//! ```text
//! fn main(i_init):
//!   result = loop_step(i_init)
//!   return result
//!
//! fn loop_step(i):
//!   // Natural exit condition check
//!   const_3 = 3
//!   cmp_lt = (i < 3)
//!   exit_cond = !cmp_lt
//!   Jump(k_exit, [i], cond=exit_cond)  // natural exit
//!
//!   // Break condition check
//!   const_2 = 2
//!   break_cond = (i >= 2)
//!   Jump(k_exit, [i], cond=break_cond) // early break exit
//!
//!   // Body (increment)
//!   const_1 = 1
//!   i_next = i + 1
//!   Call(loop_step, [i_next])          // tail recursion
//!
//! fn k_exit(i_exit):
//!   return i_exit
//! ```
//!
//! ## Design Notes
//!
//! This is a MINIMAL implementation targeting joinir_min_loop.hako specifically.
//! It establishes the infrastructure for loop_break lowering, building on simple-while lowering.
//!
//! Key differences from LoopSimpleWhile:
//! - **Multiple Exit Paths**: Natural exit + break exit
//! - **Exit PHI**: k_exit receives exit value (i) from both paths
//! - **Sequential Jumps**: Natural exit check → break check → body
//!
//! Following the "80/20 rule" from CLAUDE.md - get it working first, generalize later.

use crate::ast::ASTNode;
mod body_local_init;
mod boundary_builder;
mod carrier_update;
mod header_break_lowering;
mod tail_builder;
#[cfg(test)]
mod tests;

use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, JoinFragmentMeta};
use crate::mir::join_ir::lowering::common::body_local_derived_emitter::BodyLocalDerivedRecipe;
use crate::mir::join_ir::lowering::common::balanced_depth_scan_emitter::BalancedDepthScanRecipe;
use crate::mir::join_ir::lowering::condition_to_joinir::ConditionEnv;
use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::join_ir::lowering::step_schedule::{
    build_loop_break_schedule_from_decision, decide_loop_break_schedule, LoopBreakScheduleFactsBox,
    LoopBreakStepKind,
};
use crate::mir::loop_canonicalizer::LoopSkeleton;
use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinInst, JoinModule, MirLikeInst, UnaryOp};
use crate::mir::loop_route_detection::loop_condition_scope::{
    extract_loop_body_local_names, LoopConditionScopeBox,
};
use crate::mir::ValueId;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use body_local_init::emit_body_local_inits;
use boundary_builder::build_fragment_meta;
use carrier_update::{emit_carrier_updates, CarrierUpdateResult};
use header_break_lowering::{lower_break_condition, lower_header_condition};
use tail_builder::emit_tail_call;
use std::collections::BTreeMap; // Phase 222.5-D: HashMap → BTreeMap for determinism

pub(crate) struct LoopWithBreakLoweringInputs<'a> {
    pub scope: LoopScopeShape,
    pub condition: &'a ASTNode,
    pub break_condition: &'a ASTNode,
    pub env: &'a ConditionEnv,
    pub carrier_info: &'a CarrierInfo,
    pub carrier_updates: &'a BTreeMap<String, UpdateExpr>,
    pub body_ast: &'a [ASTNode],
    pub body_local_env: Option<&'a mut LoopBodyLocalEnv>,
    pub allowed_body_locals_for_conditions: Option<&'a [String]>,
    pub join_value_space: &'a mut JoinValueSpace,
    pub skeleton: Option<&'a LoopSkeleton>,
    /// Phase 93 P0: ConditionOnly recipe for derived slot recalculation
    pub condition_only_recipe: Option<&'a crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyRecipe>,
    /// Phase 94: BodyLocalDerived recipe (P5b escape `ch` reassignment + conditional counter).
    pub body_local_derived_recipe: Option<&'a BodyLocalDerivedRecipe>,
    /// Phase 29ab P4: Derived slot recipe for seg-like conditional assignments.
    pub body_local_derived_slot_recipe: Option<
        &'a crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::BodyLocalDerivedSlotRecipe,
    >,
    /// Phase 107: Balanced depth-scan recipe (find_balanced_* family).
    pub balanced_depth_scan_recipe: Option<&'a BalancedDepthScanRecipe>,
    /// Phase 252: Name of the static box being lowered (for this.method(...) in break conditions)
    pub current_static_box_name: Option<String>,
}

/// Lower LoopBreak (loop with conditional break) to JoinIR
///
/// # Phase 188-Impl-2: Pure JoinIR Fragment Generation
///
/// This version generates JoinIR using **local ValueIds only** (0, 1, 2, ...).
/// It has NO knowledge of the host function's ValueId space. The boundary mapping
/// is handled separately via JoinInlineBoundary.
///
/// ## Design Philosophy
///
/// - **Box A**: JoinIR Frontend (doesn't know about host ValueIds)
/// - **Box B**: This function - converts to JoinIR with local IDs
/// - **Box C**: JoinInlineBoundary - stores boundary info
/// - **Box D**: merge_joinir_mir_blocks - injects Copy instructions
///
/// This clean separation ensures JoinIR lowerers are:
/// - Pure transformers (no side effects)
/// - Reusable (same lowerer works in any context)
/// - Testable (can test JoinIR independently)
///
/// # Arguments
///
/// * `_scope` - LoopScopeShape (reserved for future generic implementation)
/// * `condition` - Loop condition AST node (e.g., `i < end`)
/// * `env` - ConditionEnv for variable resolution (JoinIR-local ValueIds) - Phase 171-fix
///
/// # Returns
///
/// * `Ok(JoinModule)` - Successfully lowered to JoinIR
/// * `Err(String)` - Route shape not matched or lowering error
///
/// # Boundary Contract
///
/// This function returns a JoinModule with:
/// - **Input slot**: ValueId(0) in main function represents the loop variable init
/// - **Output slot**: k_exit returns the final loop variable value
/// - **Caller responsibility**: Create JoinInlineBoundary to map ValueIds
///
/// # Phase 171-fix: ConditionEnv Usage
///
/// The caller must build a ConditionEnv that maps variable names to JoinIR-local ValueIds.
/// This ensures JoinIR never accesses HOST ValueIds directly.
///
/// # Phase 172-3: ExitMeta Return
/// # Phase 33-14: JoinFragmentMeta Return
///
/// Returns `(JoinModule, JoinFragmentMeta)` where:
/// - `expr_result`: k_exit's return value (i_exit) - this is what `return i` uses
/// - `exit_meta`: carrier bindings for variable_map updates
///
/// # Arguments
///
/// * `break_condition` - AST node for the break condition (e.g., `i >= 2`) - Phase 170-B
/// * `carrier_info` - Phase 176-3: Carrier metadata for dynamic multi-carrier support
/// * `carrier_updates` - Phase 176-3: Update expressions for each carrier variable
/// * `body_ast` - Phase 191: Loop body AST for body-local init lowering
/// * `body_local_env` - Phase 185-2: Optional mutable body-local variable environment for init expressions
/// * `join_value_space` - Phase 201: Unified JoinIR ValueId allocator (Local region: 1000+)
/// * `skeleton` - Phase 92 P0-3: Optional LoopSkeleton for ConditionalStep support
pub(crate) fn lower_loop_with_break_minimal(
    inputs: LoopWithBreakLoweringInputs<'_>,
) -> Result<(JoinModule, JoinFragmentMeta), String> {
    let LoopWithBreakLoweringInputs {
        scope: _scope,
        condition,
        break_condition,
        env,
        carrier_info,
        carrier_updates,
        body_ast,
        body_local_env,
        allowed_body_locals_for_conditions,
        join_value_space,
        skeleton,
        condition_only_recipe,
        body_local_derived_recipe,
        body_local_derived_slot_recipe,
        balanced_depth_scan_recipe,
        current_static_box_name, // Phase 252
    } = inputs;

    let mut body_local_env = body_local_env;
    let dev_log = DebugOutputBox::new_dev("joinir/loop_break");
    // Phase 170-D-impl-3: Validate that conditions only use supported variable scopes
    // LoopConditionScopeBox checks that loop conditions don't reference loop-body-local variables
    let loop_var_name = &carrier_info.loop_var_name; // Phase 176-3: Extract from CarrierInfo
    let loop_cond_scope =
        LoopConditionScopeBox::analyze(loop_var_name, &[condition, break_condition], Some(&_scope));

    if loop_cond_scope.has_loop_body_local() {
        // Phase 224: Filter out promoted variables from body-local check
        // Variables that were promoted to carriers should not trigger the error
        let body_local_names = extract_loop_body_local_names(&loop_cond_scope.vars);
        let unpromoted_locals: Vec<&String> = body_local_names
            .iter()
            .filter(|name| !carrier_info.promoted_body_locals.contains(*name))
            .filter(|name| {
                allowed_body_locals_for_conditions
                    .map(|allow| !allow.iter().any(|s| s.as_str() == (*name).as_str()))
                    .unwrap_or(true)
            })
            .copied()
            .collect();

        if !unpromoted_locals.is_empty() {
            return Err(error_tags::freeze(&format!(
                "[loop_break/body_local_slot/contract/unhandled_vars] Unsupported LoopBodyLocal variables in condition: {:?} (promoted={:?}, allowed={:?})",
                unpromoted_locals,
                carrier_info.promoted_body_locals,
                allowed_body_locals_for_conditions.unwrap_or(&[])
            )));
        }

        dev_log.log_if_enabled(|| {
            format!(
                "Phase 224: All {} body-local variables were handled (promoted or allowed): {:?}",
                body_local_names.len(),
                body_local_names
            )
        });
    }

    dev_log.log_if_enabled(|| {
        format!(
            "Phase 170-D: Condition variables verified: {:?}",
            loop_cond_scope.var_names()
        )
    });

    // Phase 286 P1: Use JoinValueSpace for unified ValueId allocation
    // - Param region (100-999): For function parameters (main, loop_step, k_exit)
    // - Local region (1000+): For local variables within functions
    // - ConditionEnv, CarrierInfo use Param region

    let mut join_module = JoinModule::new();

    // ==================================================================
    // Function IDs allocation
    // ==================================================================
    let main_id = JoinFuncId::new(0);
    let loop_step_id = JoinFuncId::new(1);
    let k_exit_id = JoinFuncId::new(2);

    // ==================================================================
    // ValueId allocation (Phase 286 P1: Param region for function params)
    // ==================================================================
    // Phase 176-3: Multi-carrier support - allocate parameters for all carriers
    let carrier_count = carrier_info.carriers.len();

    dev_log.log_if_enabled(|| {
        format!(
            "Phase 176-3: Generating JoinIR for {} carriers: {:?}",
            carrier_count,
            carrier_info
                .carriers
                .iter()
                .map(|c| &c.name)
                .collect::<Vec<_>>()
        )
    });

    // Phase 286 P1: main() parameters use Param region (100-999)
    // These are function parameters and must be in Param region
    let i_init = join_value_space.alloc_param();
    let mut carrier_init_ids: Vec<ValueId> = Vec::new();
    for _ in 0..carrier_count {
        carrier_init_ids.push(join_value_space.alloc_param());
    }
    let loop_result = join_value_space.alloc_local(); // result from loop_step

    // Allocate local values that don't need the closure
    let exit_cond = join_value_space.alloc_local(); // Exit condition (negated loop condition)
    let _const_1 = join_value_space.alloc_local(); // Increment constant
    let _i_next = join_value_space.alloc_local(); // i + 1
    let i_exit = join_value_space.alloc_param(); // Exit parameter (PHI) - Phase 286 P1: function parameter

    // Phase 286 P1: Allocate k_exit parameters (must be in Param region)
    let mut carrier_exit_ids: Vec<ValueId> = Vec::new();
    for _ in 0..carrier_count {
        carrier_exit_ids.push(join_value_space.alloc_param());
    }

    // Phase 286 P1: Allocator closure for functions that expect FnMut() -> ValueId
    // Must be created AFTER all direct join_value_space calls to avoid borrow checker issues
    let mut alloc_local_fn = || join_value_space.alloc_local();

    // Phase 201: loop_step() parameters MUST match ConditionEnv's ValueIds!
    // This is critical because condition lowering uses ConditionEnv to resolve variables.
    // If loop_step.params[0] != env.get(loop_var_name), the condition will reference wrong ValueIds.
    let i_param = env.get(loop_var_name).ok_or_else(|| {
        format!(
            "Phase 201: ConditionEnv missing loop variable '{}' - required for loop_step param",
            loop_var_name
        )
    })?;

    // Phase 201: Carrier params for loop_step - use CarrierInfo's join_id
    let mut carrier_param_ids: Vec<ValueId> = Vec::new();
    for carrier in &carrier_info.carriers {
        let carrier_join_id = carrier.join_id.ok_or_else(|| {
            format!(
                "Phase 201: CarrierInfo missing join_id for carrier '{}'",
                carrier.name
            )
        })?;
        carrier_param_ids.push(carrier_join_id);
    }

    dev_log.log_if_enabled(|| {
        format!(
            "Phase 201: loop_step params - i_param={:?}, carrier_params={:?}",
            i_param, carrier_param_ids
        )
    });
    dev_log.log_if_enabled(|| {
        format!(
            "loop_var='{}' env.get(loop_var)={:?}, carriers={:?}",
            loop_var_name,
            env.get(loop_var_name),
            carrier_info
                .carriers
                .iter()
                .map(|c| (c.name.clone(), c.join_id))
                .collect::<Vec<_>>()
        )
    });

    // Phase 169 / Phase 171-fix / Phase 240-EX / Phase 244 / Phase 252: Lower condition
    let (cond_value, mut cond_instructions) = lower_header_condition(
        condition,
        env,
        carrier_info,
        loop_var_name,
        i_param,
        &mut alloc_local_fn,
        current_static_box_name.as_deref(), // Phase 252
    )?;

    // Note: exit_cond, _const_1, _i_next, i_exit were already allocated above (before closure)

    // ==================================================================
    // main() function
    // ==================================================================
    // Phase 176-3: Multi-carrier support - main() includes all carrier parameters
    // main() takes (i, carrier1, carrier2, ...) as parameters (boundary inputs)
    // The host will inject Copy instructions for each
    let mut main_params = vec![i_init];
    main_params.extend(carrier_init_ids.iter().copied());
    let mut main_func = JoinFunction::new(main_id, "main".to_string(), main_params);

    // Phase 176-3: Multi-carrier support - Call includes all carrier inits
    // result = loop_step(i_init, carrier1_init, carrier2_init, ...)
    let mut call_args = vec![i_init];
    call_args.extend(carrier_init_ids.iter().copied());
    main_func.body.push(JoinInst::Call {
        func: loop_step_id,
        args: call_args,
        k_next: None,
        dst: Some(loop_result),
    });

    // return result (LoopBreak returns the final loop variable value)
    main_func.body.push(JoinInst::Ret {
        value: Some(loop_result),
    });

    join_module.add_function(main_func);

    // ==================================================================
    // loop_step(i, carrier1, carrier2, ...) function
    // ==================================================================
    // Phase 176-3: Multi-carrier support - loop_step includes all carrier parameters
    let mut loop_params = vec![i_param];
    loop_params.extend(carrier_param_ids.iter().copied());
    let mut loop_step_func = JoinFunction::new(loop_step_id, "loop_step".to_string(), loop_params);

    // Decide evaluation order (header/body-init/break/updates/tail) up-front.
    // Phase 93 P0: Pass condition_only_recipe existence to schedule context.
    // When recipe exists, body-init must happen before break check.
    let has_allowed_body_locals_in_conditions = loop_cond_scope.has_loop_body_local()
        && allowed_body_locals_for_conditions
            .map(|allow| !allow.is_empty())
            .unwrap_or(false);
    let schedule_facts = LoopBreakScheduleFactsBox::gather(
        body_local_env.as_ref().map(|env| &**env),
        carrier_info,
        condition_only_recipe.is_some(),
        body_local_derived_recipe.is_some(),
        has_allowed_body_locals_in_conditions,
    );
    let schedule_decision = decide_loop_break_schedule(&schedule_facts);
    let schedule = build_loop_break_schedule_from_decision(&schedule_decision);

    // Collect fragments per step; append them according to the schedule below.
    let mut header_block: Vec<JoinInst> = Vec::new();
    let mut body_init_block: Vec<JoinInst> = Vec::new();
    let mut break_block: Vec<JoinInst> = Vec::new();
    let mut carrier_update_block: Vec<JoinInst> = Vec::new();
    let mut tail_block: Vec<JoinInst> = Vec::new();

    // ------------------------------------------------------------------
    // Natural Exit Condition Check (Phase 169: from AST)
    // ------------------------------------------------------------------
    // Insert all condition evaluation instructions
    header_block.append(&mut cond_instructions);

    // Negate the condition for exit check: exit_cond = !cond_value
    header_block.push(JoinInst::Compute(MirLikeInst::UnaryOp {
        dst: exit_cond,
        op: UnaryOp::Not,
        operand: cond_value,
    }));

    // Phase 176-3: Multi-carrier support - Jump includes all carrier values
    // Jump(k_exit, [i, carrier1, carrier2, ...], cond=exit_cond)  // Natural exit path
    let mut natural_exit_args = vec![i_param];
    natural_exit_args.extend(carrier_param_ids.iter().copied());
    header_block.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: natural_exit_args,
        cond: Some(exit_cond),
    });

    // ------------------------------------------------------------------
    // Phase 191: Body-Local Variable Initialization
    // ------------------------------------------------------------------
    // Phase 246-EX: CRITICAL FIX - Move body-local init BEFORE break condition check
    //
    // Why: Break conditions may depend on body-local variables (e.g., digit_pos < 0).
    // If we check the break condition before computing digit_pos, we're checking against
    // the previous iteration's value (or initial value), causing incorrect early exits.
    //
    // Evaluation order:
    // 1. Natural exit condition (i < len) - uses loop params only
    // 2. Body-local init (digit_pos = digits.indexOf(ch)) - compute fresh values
    // 3. Break condition (digit_pos < 0) - uses fresh body-local values
    // 4. Carrier updates (result = result * 10 + digit_pos) - uses body-local values
    //
    // Lower body-local variable initialization expressions to JoinIR
    // This must happen BEFORE break condition AND carrier updates since both may reference body-locals
    emit_body_local_inits(
        env,
        body_ast,
        body_local_env.as_deref_mut(),
        condition_only_recipe,
        body_local_derived_slot_recipe,
        balanced_depth_scan_recipe,
        current_static_box_name.clone(),
        &mut alloc_local_fn,
        &mut body_init_block,
        &dev_log,
    )?;

    // ------------------------------------------------------------------
    // Phase 170-B / Phase 244 / Phase 92 P2-2 / Phase 252: Lower break condition
    // ------------------------------------------------------------------
    // Phase 92 P2-2: Moved after body-local init to support body-local variable references
    let (break_cond_value, break_cond_instructions) = lower_break_condition(
        break_condition,
        env,
        carrier_info,
        loop_var_name,
        i_param,
        &mut alloc_local_fn,
        body_local_env.as_ref().map(|e| &**e), // Phase 92 P2-2: Pass body_local_env
        current_static_box_name.as_deref(), // Phase 252
    )?;

    // ------------------------------------------------------------------
    // Phase 170-B: Break Condition Check (delegated to condition_to_joinir)
    // ------------------------------------------------------------------
    // Phase 246-EX: Rewrite break condition instructions to use fresh body-local values
    //
    // Problem: Break condition was normalized (e.g., "digit_pos < 0" → "!is_digit_pos")
    // and lowered before body-local init. It references the carrier param which has stale values.
    //
    // Solution: Replace references to promoted carriers with fresh body-local computations.
    // (See common::dual_value_rewriter for the name-based rules.)
    use crate::mir::join_ir::lowering::common::dual_value_rewriter::rewrite_break_condition_insts;
    break_block.extend(rewrite_break_condition_insts(
        break_cond_instructions,
        carrier_info,
        body_local_env.as_ref().map(|e| &**e),
        &mut alloc_local_fn,
    ));

    // Phase 176-3: Multi-carrier support - Jump includes all carrier values
    // Jump(k_exit, [i, carrier1, carrier2, ...], cond=break_cond)  // Break exit path
    let mut break_exit_args = vec![i_param];
    break_exit_args.extend(carrier_param_ids.iter().copied());
    break_block.push(JoinInst::Jump {
        cont: k_exit_id.as_cont(),
        args: break_exit_args,
        cond: Some(break_cond_value), // Phase 170-B: Use lowered condition
    });

    let CarrierUpdateResult {
        updated_carrier_values,
        loop_var_next_override,
    } = emit_carrier_updates(
        env,
        carrier_info,
        carrier_updates,
        body_local_env.as_deref_mut(),
        body_local_derived_recipe,
        skeleton,
        &carrier_param_ids,
        &mut alloc_local_fn,
        &mut carrier_update_block,
        &dev_log,
    )?;

    let loop_var_next_override = if loop_var_next_override.is_none() {
        if let Some(update_expr) = extract_loop_var_update_expr(body_ast, loop_var_name) {
            let next_id = crate::mir::join_ir::lowering::condition_lowerer::lower_value_expression(
                &update_expr,
                &mut alloc_local_fn,
                env,
                body_local_env.as_ref().map(|e| &**e),
                current_static_box_name.as_deref(),
                &mut tail_block,
            )?;
            Some(next_id)
        } else {
            None
        }
    } else {
        loop_var_next_override
    };

    emit_tail_call(
        loop_step_id,
        i_param,
        &updated_carrier_values,
        loop_var_next_override,
        &mut alloc_local_fn,
        &mut tail_block,
        &dev_log,
    );

    // Apply scheduled order to assemble the loop_step body.
    for step in schedule.iter() {
        match step {
            LoopBreakStepKind::HeaderCond => loop_step_func.body.append(&mut header_block),
            LoopBreakStepKind::BodyInit => loop_step_func.body.append(&mut body_init_block),
            LoopBreakStepKind::BreakCheck => loop_step_func.body.append(&mut break_block),
            LoopBreakStepKind::Updates => loop_step_func.body.append(&mut carrier_update_block),
            LoopBreakStepKind::Tail => loop_step_func.body.append(&mut tail_block),
            // Phase 47-A: IfPhiJoin steps are not used in loop_break lowering.
            LoopBreakStepKind::IfCond
            | LoopBreakStepKind::ThenUpdates
            | LoopBreakStepKind::ElseUpdates => {
                panic!("IfPhiJoin step kinds should not appear in loop_break lowering");
            }
            // Phase 48-A: LoopContinueOnly steps are not used in loop_break lowering.
            LoopBreakStepKind::ContinueCheck => {
                panic!("LoopContinueOnly step kinds should not appear in loop_break lowering");
            }
        }
    }

    join_module.add_function(loop_step_func);

    // ==================================================================
    // k_exit(i_exit, carrier1_exit, carrier2_exit, ...) function - Exit PHI
    // ==================================================================
    // Phase 176-3: Multi-carrier support - k_exit receives all carrier exit values
    // LoopBreak key difference: k_exit receives exit values from both paths (natural + break)
    // Note: carrier_exit_ids were already allocated above (before closure)

    let debug_dump = crate::config::env::joinir_debug_level() > 0;
    if debug_dump {
        let strict_debug = DebugOutputBox::new("joinir/loop_break");
        strict_debug.log_if_enabled(|| {
            format!(
                "k_exit param layout: i_exit={:?}, carrier_exit_ids={:?}",
                i_exit, carrier_exit_ids
            )
        });
        for (idx, carrier) in carrier_info.carriers.iter().enumerate() {
            let exit_id = carrier_exit_ids.get(idx).copied().unwrap_or(ValueId(0));
            strict_debug.log("k_exit", &format!("carrier '{}' exit -> {:?}", carrier.name, exit_id));
        }
    }

    let mut exit_params = vec![i_exit];
    exit_params.extend(carrier_exit_ids.iter().copied());
    let mut k_exit_func = JoinFunction::new(
        k_exit_id,
        "k_exit".to_string(),
        exit_params, // Exit PHI: receives (i, carrier1, carrier2, ...) from both exit paths
    );

    // return i_exit (return final loop variable value)
    k_exit_func.body.push(JoinInst::Ret {
        value: Some(i_exit),
    });

    join_module.add_function(k_exit_func);

    // Set entry point
    join_module.entry = Some(main_id);

    dev_log.log_simple("Generated JoinIR for loop_break route shape (Phase 170-B)");
    dev_log.log_simple("Functions: main, loop_step, k_exit");
    dev_log.log_simple("Loop condition from AST (ConditionLoweringBox)");
    dev_log.log_simple("Break condition from AST (ConditionLoweringBox)");
    dev_log.log_simple("Exit PHI: k_exit receives i from both natural exit and break");

    let fragment_meta = build_fragment_meta(carrier_info, loop_var_name, i_exit, &carrier_exit_ids);

    dev_log.log_if_enabled(|| {
        format!(
            "Phase 33-14/176-3: JoinFragmentMeta {{ expr_result: {:?}, carriers: {} }}",
            i_exit,
            carrier_info.carriers.len()
        )
    });

    Ok((join_module, fragment_meta))
}

fn extract_loop_var_update_expr(body_ast: &[ASTNode], loop_var_name: &str) -> Option<ASTNode> {
    let mut matches = Vec::new();

    for node in body_ast {
        if let ASTNode::Assignment { target, value, .. } = node {
            if matches!(
                target.as_ref(),
                ASTNode::Variable { name, .. } if name == loop_var_name
            ) {
                matches.push((**value).clone());
            }
        }
    }

    if matches.len() == 1 {
        Some(matches.remove(0))
    } else {
        None
    }
}
