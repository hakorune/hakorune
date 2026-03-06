use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::condition_lowering_box::ConditionContext;
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::lowering::error_tags;
use crate::mir::join_ir::lowering::expr_lowerer::{ExprContext, ExprLowerer};
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::scope_manager::LoopBreakScopeManager;
use crate::mir::join_ir::JoinInst;
use crate::mir::loop_pattern_detection::function_scope_capture::CapturedEnv;
use crate::mir::ValueId;

/// Build a LoopBreakScopeManager for ExprLowerer paths.
fn make_scope_manager<'a>(
    condition_env: &'a ConditionEnv,
    body_local_env: Option<&'a LoopBodyLocalEnv>,
    captured_env: Option<&'a CapturedEnv>,
    carrier_info: &'a CarrierInfo,
) -> LoopBreakScopeManager<'a> {
    LoopBreakScopeManager {
        condition_env,
        loop_body_local_env: body_local_env,
        captured_env,
        carrier_info,
    }
}

/// Lower the header condition.
///
/// # Phase 252: current_static_box_name Parameter
///
/// Added to support `this.method(...)` in header conditions for static boxes.
pub(crate) fn lower_header_condition(
    condition: &ASTNode,
    env: &ConditionEnv,
    carrier_info: &CarrierInfo,
    loop_var_name: &str,
    loop_var_id: ValueId,
    alloc_value: &mut dyn FnMut() -> ValueId,
    current_static_box_name: Option<&str>, // Phase 252
) -> Result<(ValueId, Vec<JoinInst>), String> {
    use crate::mir::join_ir::lowering::condition_lowering_box::ConditionLoweringBox;

    let debug = DebugOutputBox::new_dev("joinir/loop_break");

    let empty_body_env = LoopBodyLocalEnv::new();
    let empty_captured_env = CapturedEnv::new();
    let scope_manager = make_scope_manager(
        env,
        Some(&empty_body_env),
        Some(&empty_captured_env),
        carrier_info,
    );

    if !ExprLowerer::<LoopBreakScopeManager>::is_supported_condition(condition) {
        return Err(error_tags::lowering_error(
            "loop_break/header_condition",
            "ConditionLoweringBox does not support this condition (legacy path removed)",
        ));
    }

    let mut dummy_builder = MirBuilder::new();
    let mut expr_lowerer =
        ExprLowerer::new(&scope_manager, ExprContext::Condition, &mut dummy_builder);

    let mut context = ConditionContext {
        loop_var_name: loop_var_name.to_string(),
        loop_var_id,
        scope: &scope_manager,
        alloc_value,
        current_static_box_name: current_static_box_name.map(|s| s.to_string()), // Phase 252
    };

    let value_id = expr_lowerer.lower_condition(condition, &mut context).map_err(|e| {
        format!(
            "[joinir/loop_break/phase244] ConditionLoweringBox failed on supported condition: {:?}",
            e
        )
    })?;

    let instructions = expr_lowerer.take_last_instructions();
    debug.log(
        "phase244",
        &format!(
            "Header condition via ConditionLoweringBox: {} instructions",
            instructions.len()
        ),
    );
    Ok((value_id, instructions))
}

/// Lower the break condition via ExprLowerer (no legacy fallback).
///
/// # Phase 92 P2-2: Body-Local Variable Support
///
/// Added `body_local_env` parameter to support break conditions that reference
/// body-local variables (e.g., `ch == '"'` in escape patterns).
///
/// # Phase 252: current_static_box_name Parameter
///
/// Added to support `this.method(...)` in break conditions for static boxes.
pub(crate) fn lower_break_condition(
    break_condition: &ASTNode,
    env: &ConditionEnv,
    carrier_info: &CarrierInfo,
    loop_var_name: &str,
    loop_var_id: ValueId,
    alloc_value: &mut dyn FnMut() -> ValueId,
    body_local_env: Option<&LoopBodyLocalEnv>, // Phase 92 P2-2
    current_static_box_name: Option<&str>, // Phase 252
) -> Result<(ValueId, Vec<JoinInst>), String> {
    use crate::mir::join_ir::lowering::condition_lowering_box::ConditionLoweringBox;

    // Phase 92 P2-2: Use provided body_local_env or empty if None
    let empty_body_env = LoopBodyLocalEnv::new();
    let body_env_ref = body_local_env.unwrap_or(&empty_body_env);

    let empty_captured_env = CapturedEnv::new();
    let scope_manager = make_scope_manager(
        env,
        Some(body_env_ref), // Phase 92 P2-2: Pass body_local_env to scope
        Some(&empty_captured_env),
        carrier_info,
    );

    let mut dummy_builder = MirBuilder::new();
    let mut expr_lowerer =
        ExprLowerer::new(&scope_manager, ExprContext::Condition, &mut dummy_builder);

    let mut context = ConditionContext {
        loop_var_name: loop_var_name.to_string(),
        loop_var_id,
        scope: &scope_manager,
        alloc_value,
        current_static_box_name: current_static_box_name.map(|s| s.to_string()), // Phase 252
    };

    let value_id = expr_lowerer
        .lower_condition(break_condition, &mut context)
        .map_err(|e| {
            format!(
                "[joinir/loop_break/phase244] ConditionLoweringBox failed to lower break condition: {}",
                e
            )
        })?;

    Ok((value_id, expr_lowerer.take_last_instructions()))
}
