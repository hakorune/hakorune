//! Pattern2 lowering orchestration (wiring + emission)
//!
//! Phase 106: the orchestrator is intentionally thin.
//! Most "do the work" logic lives in explicit step boxes under `pattern2_steps/`.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::naming::StaticMethodId;
use crate::mir::ValueId;

/// Phase 256.5: Get current static box name from builder context or function name
///
/// First tries `comp_ctx.current_static_box`, then falls back to extracting
/// from the current function name using `StaticMethodId::parse()`.
///
/// This fallback is needed for Pattern2 break condition lowering when
/// `this.method(...)` calls require the box context (e.g., `this.is_whitespace(...)`).
fn current_box_name_for_lowering(builder: &MirBuilder) -> Option<String> {
    // First: from compilation context
    builder.comp_ctx.current_static_box.clone().or_else(|| {
        // Fallback: extract from current function name
        builder
            .scope_ctx
            .current_function
            .as_ref()
            .and_then(|f| StaticMethodId::parse(&f.signature.name))
            .map(|id| id.box_name)
    })
}

use crate::mir::builder::control_flow::plan::pattern2_steps::apply_policy_step_box::ApplyPolicyStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::body_local_derived_step_box::BodyLocalDerivedStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::carrier_updates_step_box::CarrierUpdatesStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::emit_joinir_step_box::EmitJoinIRStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::gather_facts_step_box::GatherFactsStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::merge_step_box::MergeStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::normalize_body_step_box::NormalizeBodyStepBox;
use crate::mir::builder::control_flow::plan::pattern2_steps::post_loop_early_return_step_box::PostLoopEarlyReturnStepBox;
use crate::mir::builder::control_flow::joinir::trace::trace;

pub(crate) struct Pattern2LoweringOrchestrator;

impl Pattern2LoweringOrchestrator {
    pub(in crate::mir::builder) fn run(
        builder: &mut MirBuilder,
        condition: &ASTNode,
        body: &[ASTNode],
        _func_name: &str,
        debug: bool,
        fn_body: Option<&[ASTNode]>,
        skeleton: Option<&crate::mir::loop_canonicalizer::LoopSkeleton>,
    ) -> Result<Option<ValueId>, String> {
        let verbose = debug || crate::config::env::joinir_dev_enabled();

        trace().debug("pattern2", "Calling Pattern 2 minimal lowerer");

        use crate::mir::builder::control_flow::plan::pattern_pipeline::{
            build_pattern_context, PatternVariant,
        };
        let ctx = build_pattern_context(builder, condition, body, PatternVariant::Pattern2)?;

        trace().varmap("pattern2_start", &builder.variable_ctx.variable_map);

        let facts = GatherFactsStepBox::gather(builder, condition, body, fn_body, &ctx, verbose)?;
        let inputs = ApplyPolicyStepBox::apply(condition, body, facts)?;

        // Phase 263 P0.2: pattern2::api 経由で try_promote を呼び出し（入口SSOT）
        use crate::mir::builder::control_flow::plan::pattern2::api::{
            try_promote, PromoteDecision,
        };
        let mut inputs = match try_promote(builder, condition, body, inputs, debug, verbose)? {
            PromoteDecision::Promoted(result) => {
                result.inputs
            }
            PromoteDecision::NotApplicable(result) => {
                trace().debug(
                    "pattern2",
                    "Pattern2 promotion not applicable, continuing without promotion",
                );
                result.inputs
            }
            PromoteDecision::Freeze(reason) => {
                // Pattern2 should handle this but implementation is missing → Fail-Fast
                return Err(reason);
            }
        };

        // Phase 256.5: Wire current_static_box_name from builder context or function name
        inputs.current_static_box_name = current_box_name_for_lowering(builder);

        let normalized = NormalizeBodyStepBox::run(builder, condition, body, &mut inputs, verbose)?;
        let normalized_body = normalized.normalized_body;
        let analysis_body = normalized_body.as_deref().unwrap_or(body);
        let effective_break_condition = normalized.effective_break_condition;

        BodyLocalDerivedStepBox::apply(analysis_body, &mut inputs, verbose)?;
        let carrier_updates =
            CarrierUpdatesStepBox::analyze_and_filter(analysis_body, &mut inputs, verbose);
        let emitted = EmitJoinIRStepBox::emit(
            builder,
            condition,
            analysis_body,
            &effective_break_condition,
            &carrier_updates,
            &mut inputs,
            debug,
            verbose,
            skeleton,
        )?;

        let out = MergeStepBox::merge(builder, emitted.join_module, emitted.boundary, debug)?;
        PostLoopEarlyReturnStepBox::maybe_emit(builder, inputs.post_loop_early_return.as_ref())?;
        Ok(out)
    }
}
