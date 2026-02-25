//! Pattern 1: Simple While Loop minimal lowerer (plan-side implementation)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::pattern_pipeline::{
    build_pattern_context, PatternVariant,
};
use crate::mir::builder::control_flow::joinir::trace;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

/// Plan-side implementation for Pattern1 minimal lowering.
/// patterns/ 側から thin wrapper 経由で呼ばれる。
pub(crate) fn lower_simple_while_minimal(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    _func_name: &str,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    use crate::mir::join_ir::lowering::simple_while_minimal::lower_simple_while_minimal;

    // Phase 195: Use unified trace
    trace::trace().debug("pattern1", "Calling Pattern 1 minimal lowerer");

    // Phase 179-B: Use PatternPipelineContext for unified preprocessing
    let ctx = build_pattern_context(builder, condition, body, PatternVariant::Pattern1)?;

    // Phase 195: Use unified trace
    trace::trace().varmap("pattern1_start", &builder.variable_ctx.variable_map);

    // Phase 202-A: Create JoinValueSpace for unified ValueId allocation
    // Pattern 1 uses Param region for boundary input slots (loop var) and Local region for temps.
    use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
    let mut join_value_space = JoinValueSpace::new();

    // Call Pattern 1 lowerer with preprocessed scope
    let join_module = match lower_simple_while_minimal(ctx.loop_scope, &mut join_value_space) {
        Some(module) => module,
        None => {
            // Phase 195: Use unified trace
            trace::trace().debug("pattern1", "Pattern 1 lowerer returned None");
            return Ok(None);
        }
    };

    // Phase 179-B: Create boundary from context
    // Phase 201: Use JoinInlineBoundaryBuilder for clean construction
    // Canonical Builder pattern - see docs/development/current/main/joinir-boundary-builder-pattern.md
    //
    // Phase 132: Add exit_bindings to enable ExitLineReconnector
    // This ensures `return i` after loop returns the final value (3) instead of initial (0)
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
    use crate::mir::join_ir::lowering::inline_boundary::LoopExitBinding;
    use crate::mir::join_ir::lowering::join_value_space::PARAM_MIN;
    use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;

    // Phase 132-Post: Extract k_exit's parameter ValueId from join_module (Box-First)
    let k_exit_func = join_module.require_function("k_exit", "Pattern 1");
    let join_exit_value = k_exit_func
        .params
        .first()
        .copied()
        .expect("k_exit must have parameter for exit value");

    // Phase 132: Create exit binding for loop variable
    let exit_binding = LoopExitBinding {
        carrier_name: ctx.loop_var_name.clone(),
        join_exit_value,
        host_slot: ctx.loop_var_id,
        role: CarrierRole::LoopState,
    };

    let boundary = JoinInlineBoundaryBuilder::new()
        .with_inputs(
            vec![ValueId(PARAM_MIN)], // JoinIR's main() parameter (loop variable, Param region)
            vec![ctx.loop_var_id],    // Host's loop variable
        )
        .with_exit_bindings(vec![exit_binding]) // Phase 132: Enable exit PHI & variable_map update
        .with_loop_var_name(Some(ctx.loop_var_name.clone())) // Phase 33-16: Enable header PHI generation for SSA correctness
        .build();

    // Phase 33-22: Use JoinIRConversionPipeline for unified conversion flow
    use crate::mir::builder::control_flow::plan::conversion_pipeline::JoinIRConversionPipeline;
    let _ = JoinIRConversionPipeline::execute(
        builder,
        join_module,
        Some(&boundary),
        "pattern1",
        debug,
    )?;

    // Phase 188-Impl-4-FIX: Return Void instead of trying to emit Const
    //
    // PROBLEM: Emitting instructions after merge_joinir_mir_blocks is fragile because:
    // 1. merge creates exit block and switches to it
    // 2. We try to add Const to exit block
    // 3. But subsequent code (return statement) might overwrite the block
    //
    // SOLUTION: Loops don't produce values - they return Void.
    // The subsequent "return 0" statement will emit its own Const + Return.
    //
    // This is cleaner because:
    // - Loop lowering doesn't need to know about the return value
    // - The return statement handles its own code generation
    // - No risk of instructions being lost due to block management issues

    let void_val = crate::mir::builder::emission::constant::emit_void(builder)?;

    // Phase 195: Use unified trace
    trace::trace().debug(
        "pattern1",
        &format!("Loop complete, returning Void {:?}", void_val),
    );

    Ok(Some(void_val))
}

/// Phase 194: Detection function for Pattern 1
///
/// Phase 282 P3: Updated to ExtractionBased detection with safety valve
///
/// Pattern 1 matches:
/// - Pattern kind is Pattern1SimpleWhile (safety valve, O(1) early rejection)
/// - Extraction validates: 比較条件 + no control flow + 単純step
pub(crate) fn can_lower(
    _builder: &MirBuilder,
    ctx: &crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext,
) -> bool {
    use crate::mir::loop_pattern_detection::LoopPatternKind;

    // Step 1: Early rejection guard (safety valve, O(1))
    if ctx.pattern_kind != LoopPatternKind::Pattern1SimpleWhile {
        if ctx.debug {
            trace::trace().debug(
                "pattern1/can_lower",
                &format!("reject: pattern_kind={:?}", ctx.pattern_kind),
            );
        }
        return false;
    }

    // Step 2: ExtractionBased validation (SSOT, deep check)
    use crate::mir::builder::control_flow::plan::extractors::pattern1::extract_simple_while_parts;

    match extract_simple_while_parts(ctx.condition, ctx.body) {
        Ok(Some(_)) => {
            if ctx.debug {
                trace::trace().debug("pattern1/can_lower", "accept: extractable (Phase 282 P3)");
            }
            true
        }
        Ok(None) => {
            if ctx.debug {
                trace::trace().debug(
                    "pattern1/can_lower",
                    "reject: not simple while (has control flow or complex step)",
                );
            }
            false
        }
        Err(e) => {
            if ctx.debug {
                trace::trace().debug("pattern1/can_lower", &format!("error: {}", e));
            }
            false
        }
    }
}

/// Phase 194: Lowering function for Pattern 1
///
/// Phase 282 P3: Re-extracts for SSOT (no caching from can_lower)
pub(crate) fn lower(
    builder: &mut MirBuilder,
    ctx: &crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext,
) -> Result<Option<ValueId>, String> {
    use crate::mir::builder::control_flow::plan::extractors::pattern1::extract_simple_while_parts;

    // Re-extract (SSOT principle - no caching from can_lower)
    let parts = extract_simple_while_parts(ctx.condition, ctx.body)?
        .ok_or_else(|| "[pattern1] Not a simple while pattern in lower()".to_string())?;

    if ctx.debug {
        trace::trace().debug(
            "pattern1/lower",
            &format!("loop_var={} (Phase 282 P3)", parts.loop_var),
        );
    }

    // Call existing internal lowerer (ctx.condition/ctx.body を直接使用)
    // Note: parts は検証結果のみで、AST は ctx から再利用
    builder.cf_loop_pattern1_minimal(ctx.condition, ctx.body, ctx.func_name, ctx.debug)
}

impl MirBuilder {
    /// Phase 179-B: Pattern 1 (Simple While Loop) minimal lowerer
    /// Phase 202-A: JoinValueSpace Integration
    ///
    /// **Refactored**: Now uses PatternPipelineContext for unified preprocessing
    ///
    /// # Pipeline (Phase 179-B)
    /// 1. Build preprocessing context → PatternPipelineContext
    /// 2. Call JoinIR lowerer → JoinModule
    /// 3. Create boundary from context → JoinInlineBoundary
    /// 4. Merge MIR blocks → JoinIRConversionPipeline
    pub(in crate::mir::builder) fn cf_loop_pattern1_minimal(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        _func_name: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        lower_simple_while_minimal(self, condition, body, _func_name, debug)
    }
}
