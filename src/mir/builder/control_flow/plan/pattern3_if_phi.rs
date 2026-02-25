//! Pattern 3: Loop with If-Else PHI minimal lowerer (plan-side implementation)

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use crate::mir::builder::control_flow::joinir::merge::exit_line::meta_collector::ExitMetaCollector;
use crate::mir::builder::control_flow::plan::condition_env_builder::ConditionEnvBuilder;
use crate::mir::builder::control_flow::plan::conversion_pipeline::JoinIRConversionPipeline;
use crate::mir::builder::control_flow::plan::pattern_pipeline::{
    build_pattern_context, PatternPipelineContext, PatternVariant,
};
use crate::mir::builder::control_flow::joinir::trace;

/// Plan-side entry for Pattern3 if-phi lowering.
pub(crate) fn lower_if_phi_minimal(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    _func_name: &str,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    builder.cf_loop_pattern3_with_if_phi(condition, body, _func_name, debug)
}

/// Phase 194: Detection function for Pattern 3
///
/// Phase 282 P5: Updated to ExtractionBased detection with safety valve
///
/// Pattern 3 matches:
/// - Pattern kind is Pattern3IfPhi (safety valve, O(1) early rejection)
/// - Extraction validates: if-else PHI + NO break/continue/nested-if (return → Ok(None))
pub(crate) fn can_lower(
    _builder: &MirBuilder,
    ctx: &crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext,
) -> bool {
    use crate::mir::loop_pattern_detection::LoopPatternKind;

    // Step 1: Early rejection guard (safety valve, O(1))
    if ctx.pattern_kind != LoopPatternKind::Pattern3IfPhi {
        if ctx.debug {
            trace::trace().debug(
                "pattern3/can_lower",
                &format!("reject: pattern_kind={:?}", ctx.pattern_kind),
            );
        }
        return false;
    }

    // Step 2: ExtractionBased validation (SSOT, deep check)
    use crate::mir::builder::control_flow::plan::extractors::pattern3::extract_loop_with_if_phi_parts;

    match extract_loop_with_if_phi_parts(ctx.condition, ctx.body) {
        Ok(Some(_)) => {
            if ctx.debug {
                trace::trace().debug("pattern3/can_lower", "accept: extractable (Phase 282 P5)");
            }
            true
        }
        Ok(None) => {
            if ctx.debug {
                trace::trace().debug(
                    "pattern3/can_lower",
                    "reject: not Pattern3 (no if-else PHI or has control flow)",
                );
            }
            false
        }
        Err(e) => {
            if ctx.debug {
                trace::trace().debug("pattern3/can_lower", &format!("error: {}", e));
            }
            false
        }
    }
}

/// Phase 194: Lowering function for Pattern 3
///
/// Phase 282 P5: Re-extracts for SSOT (no caching from can_lower)
pub(crate) fn lower(
    builder: &mut MirBuilder,
    ctx: &crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext,
) -> Result<Option<ValueId>, String> {
    use crate::mir::builder::control_flow::plan::extractors::pattern3::extract_loop_with_if_phi_parts;

    // Re-extract (SSOT principle - no caching from can_lower)
    let parts = extract_loop_with_if_phi_parts(ctx.condition, ctx.body)?
        .ok_or_else(|| "[pattern3] Not a loop with if-phi pattern in lower()".to_string())?;

    if ctx.debug {
        trace::trace().debug(
            "pattern3/lower",
            &format!(
                "loop_var={}, merged_var={}, carriers={} (Phase 282 P5)",
                parts.loop_var, parts.merged_var, parts.carrier_count
            ),
        );
    }

    lower_if_phi_minimal(builder, ctx.condition, ctx.body, ctx.func_name, ctx.debug)
}

impl MirBuilder {
    /// Phase 179-B: Pattern 3 (Loop with If-Else PHI) minimal lowerer
    ///
    /// **Refactored**: Now uses PatternPipelineContext for unified preprocessing
    ///
    /// # Phase 213: Dual-Mode Architecture
    ///
    /// - **if-sum mode**: When `ctx.is_if_sum_pattern()` is true, uses AST-based lowering
    /// - **legacy mode**: Otherwise, uses hardcoded PoC lowering for backward compatibility
    ///
    /// # Pipeline (Phase 179-B)
    /// 1. Build preprocessing context → PatternPipelineContext
    /// 2. Check if-sum pattern → branch to appropriate lowerer
    /// 3. Call JoinIR lowerer → JoinModule
    /// 4. Create boundary from context → JoinInlineBoundary
    /// 5. Merge MIR blocks → JoinIRConversionPipeline
    pub(in crate::mir::builder) fn cf_loop_pattern3_with_if_phi(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        _func_name: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        let ctx = build_pattern_context(self, condition, body, PatternVariant::Pattern3)?;

        // Phase 213: AST-based if-sum pattern detection
        // Phase 242-EX-A: Legacy mode removed - all if-sum patterns now handled dynamically
        if !ctx.is_if_sum_pattern() {
            // Not an if-sum pattern → let router try other patterns or fall back
            trace::trace().debug(
                "pattern3",
                "Not an if-sum pattern, returning None to try other patterns",
            );
            return Ok(None);
        }

        trace::trace().debug(
            "pattern3",
            "Detected if-sum pattern, using AST-based lowerer",
        );
        self.lower_pattern3_if_sum(&ctx, condition, body, debug)
    }

    /// Phase 213: AST-based if-sum lowerer
    ///
    /// Dynamically lowers loop condition, if condition, and carrier updates from AST.
    /// Target: `phase212_if_sum_min.hako` (RC=2)
    fn lower_pattern3_if_sum(
        &mut self,
        ctx: &PatternPipelineContext,
        condition: &ASTNode,
        body: &[ASTNode],
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        use crate::mir::join_ir::lowering::loop_with_if_phi_if_sum::lower_if_sum_pattern;

        // Phase 202-B: Create JoinValueSpace for unified ValueId allocation
        use crate::mir::join_ir::lowering::join_value_space::JoinValueSpace;
        let mut join_value_space = JoinValueSpace::new();

        // Extract if statement from loop body
        let if_stmt = ctx.extract_if_statement().ok_or_else(|| {
            "[cf_loop/pattern3] if-sum pattern detected but no if statement found".to_string()
        })?;

        // Phase 220-D: Build ConditionEnv for variable resolution
        use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
        let loop_var_name = ctx.loop_var_name.clone();
        let loop_var_id = ctx.loop_var_id;

        #[allow(unused_mut)]
        let (mut cond_env, mut condition_bindings, _loop_var_join_id) =
            ConditionEnvBuilder::build_for_break_condition_v2(
                condition,
                &loop_var_name,
                &self.variable_ctx.variable_map,
                loop_var_id,
                &mut join_value_space,
            )?;

        // Phase 256.7: Add then-update variable to cond_env (e.g., separator in join/2)
        use crate::mir::join_ir::lowering::loop_with_if_phi_if_sum::extract_then_update;
        if let Ok((_update_var, update_addend_ast)) = extract_then_update(if_stmt) {
            if let crate::ast::ASTNode::Variable { name, .. } = &update_addend_ast {
                if !cond_env.contains(name) {
                    if let Some(&host_id) = self.variable_ctx.variable_map.get(name) {
                        let join_id = join_value_space.alloc_param();
                        cond_env.insert(name.clone(), join_id);
                        condition_bindings.push(ConditionBinding {
                            name: name.clone(),
                            host_value: host_id,
                            join_value: join_id,
                        });
                        trace::trace().debug(
                            "pattern3/if-sum",
                            &format!("Added then-update variable '{}' to cond_env", name),
                        );
                    }
                }
            }
        }

        // Phase 80-B (P1): Register BindingIds for condition variables (dev-only)
        #[cfg(feature = "normalized_dev")]
        {
            // Register loop variable BindingId
            // Phase 136 Step 4/7: Use binding_ctx for lookup
            if let Some(bid) = self.binding_ctx.lookup(&loop_var_name) {
                cond_env.register_loop_var_binding(bid, _loop_var_join_id);
                if debug {
                    trace::trace().emit_if(
                        "phase80",
                        "p3",
                        &format!(
                            "Registered loop var '{}' BindingId({}) -> ValueId({})",
                            loop_var_name, bid.0, _loop_var_join_id.0
                        ),
                        true,
                    );
                }
            }

            // Register condition binding BindingIds
            // These are variables from the condition expression (e.g., "len" in "i < len")
            // May include ConditionOnly carriers if they appear in the condition
            for binding in &condition_bindings {
                // Phase 136 Step 4/7: Use binding_ctx for lookup
                if let Some(bid) = self.binding_ctx.lookup(&binding.name) {
                    cond_env.register_condition_binding(bid, binding.join_value);
                    if debug {
                        trace::trace().emit_if(
                            "phase80",
                            "p3",
                            &format!(
                                "Registered condition binding '{}' BindingId({}) -> ValueId({})",
                                binding.name, bid.0, binding.join_value.0
                            ),
                            true,
                        );
                    }
                }
            }
        }

        trace::trace().debug(
            "pattern3/if-sum",
            &format!("ConditionEnv bindings = {}", condition_bindings.len()),
        );
        for binding in &condition_bindings {
            trace::trace().debug(
                "pattern3/if-sum",
                &format!(
                    "  '{}': HOST {:?} → JoinIR {:?}",
                    binding.name, binding.host_value, binding.join_value
                ),
            );
        }

        // Phase 64: Ownership analysis (dev-only, analysis-only)
        #[cfg(feature = "normalized_dev")]
        {
            use crate::mir::join_ir::ownership::analyze_loop;

            // Collect parent-defined variables from function scope
            // For now, use all variables in variable_map except loop_var
            let parent_defined: Vec<String> = self
                .variable_ctx
                .variable_map
                .keys()
                .filter(|name| *name != &loop_var_name)
                .cloned()
                .collect();

            match analyze_loop(condition, body, &parent_defined) {
                Ok(plan) => {
                    // Convert ConditionBinding Vec to BTreeSet<String> for consistency check
                    let condition_binding_names: std::collections::BTreeSet<String> =
                        condition_bindings.iter().map(|b| b.name.clone()).collect();

                    // Run consistency checks
                    if let Err(e) = check_ownership_plan_consistency(
                        &plan,
                        &ctx.carrier_info,
                        &condition_binding_names,
                    ) {
                        trace::trace()
                            .dev("phase64/ownership", &format!("Consistency check failed: {}", e));
                        return Err(e);
                    }

                    trace::trace().debug(
                        "pattern3/if-sum",
                        &format!(
                            "OwnershipPlan analysis succeeded: {} owned vars, {} relay writes, {} captures",
                            plan.owned_vars.len(),
                            plan.relay_writes.len(),
                            plan.captures.len()
                        ),
                    );
                }
                Err(e) => {
                    trace::trace().dev(
                        "phase64/ownership",
                        &format!("Analysis failed (continuing with legacy): {}", e),
                    );
                    // Don't fail - analysis is optional in Phase 64
                }
            }
        }

        // Call AST-based if-sum lowerer with ConditionEnv
        // Phase 256.7: Convert condition_bindings to IfSumConditionBinding format
        use crate::mir::join_ir::lowering::loop_with_if_phi_if_sum::IfSumConditionBinding;
        let if_sum_bindings: Vec<IfSumConditionBinding> = condition_bindings
            .iter()
            .map(|b| IfSumConditionBinding {
                name: b.name.clone(),
                join_value: b.join_value,
            })
            .collect();
        let (join_module, fragment_meta) = lower_if_sum_pattern(
            condition,
            if_stmt,
            body,
            &cond_env,
            &mut join_value_space,
            &if_sum_bindings, // Phase 256.7
        )?;

        let exit_meta = &fragment_meta.exit_meta;

        trace::trace().debug(
            "pattern3/if-sum",
            &format!("ExitMeta: {} exit values", exit_meta.exit_values.len()),
        );
        for (carrier_name, join_value) in &exit_meta.exit_values {
            trace::trace().debug(
                "pattern3/if-sum",
                &format!("  {} → ValueId({})", carrier_name, join_value.0),
            );
        }

        // Build exit bindings using ExitMetaCollector
        // Phase 228-8: Pass carrier_info to include ConditionOnly carriers
        let exit_bindings =
            ExitMetaCollector::collect(self, exit_meta, Some(&ctx.carrier_info), debug);

        // Build boundary with carrier inputs
        use crate::mir::join_ir::lowering::JoinInlineBoundaryBuilder;

        // Phase 256.7: main() params = condition_bindings (no loop_var in main for if-sum)
        // if-sum lowerer's main() takes condition_bindings as params, not loop_var
        let join_inputs: Vec<ValueId> = if_sum_bindings.iter().map(|b| b.join_value).collect();
        let host_inputs: Vec<ValueId> = condition_bindings.iter().map(|b| b.host_value).collect();

        // Phase 214: Verify length consistency (fail-fast assertion)
        debug_assert_eq!(
            join_inputs.len(),
            host_inputs.len(),
            "[pattern3/if-sum] join_inputs.len({}) != host_inputs.len({})",
            join_inputs.len(),
            host_inputs.len()
        );

        trace::trace().debug(
            "pattern3/if-sum",
            &format!(
                "Boundary inputs: {} total (condition_bindings)",
                join_inputs.len()
            ),
        );

        // Phase 215-2: Pass expr_result to boundary
        // Phase 220-D: Pass condition_bindings for variable remapping
        // Phase 256.7-fix: Pass carrier_info for loop_var_id and carrier host_ids
        let mut boundary_builder = JoinInlineBoundaryBuilder::new()
            .with_inputs(join_inputs, host_inputs)
            .with_condition_bindings(condition_bindings) // Phase 220-D: Map condition-only vars
            .with_exit_bindings(exit_bindings)
            .with_loop_var_name(Some(ctx.loop_var_name.clone()))
            .with_carrier_info(ctx.carrier_info.clone());

        // Add expr_result if present
        if let Some(expr_id) = fragment_meta.expr_result {
            trace::trace().debug(
                "pattern3/if-sum",
                &format!("Passing expr_result={:?} to boundary", expr_id),
            );
            boundary_builder = boundary_builder.with_expr_result(Some(expr_id));
        }

        let boundary = boundary_builder.build();

        // Execute JoinIR conversion pipeline
        let merge_result = JoinIRConversionPipeline::execute(
            self,
            join_module,
            Some(&boundary),
            "pattern3/if-sum",
            debug,
        )?;

        // Phase 215-2: Return expr_result if present (expr-position loop)
        if let Some(expr_val) = merge_result {
            trace::trace().debug(
                "pattern3/if-sum",
                &format!("Loop complete, returning expr_result {:?}", expr_val),
            );
            Ok(Some(expr_val))
        } else {
            // Statement-position loop (carrier-only)
            use crate::mir::builder::emission::constant;
            let void_val = constant::emit_void(self)?;
            trace::trace().debug(
                "pattern3/if-sum",
                &format!("Loop complete, returning Void {:?}", void_val),
            );
            Ok(Some(void_val))
        }
    }

    // AST-based if-sum lowering (legacy mode removed Phase 242-EX-A)
}

/// Phase 64: Ownership plan consistency checks (dev-only)
///
/// Validates OwnershipPlan against existing CarrierInfo and ConditionBindings.
/// This is analysis-only - no behavior change.
///
/// # Checks
///
/// 1. **Multi-hop relay rejection**: `relay_path.len() > 1` → Err with `[ownership/relay:runtime_unsupported]` tag
/// 2. **Carrier set consistency**: plan carriers vs existing carriers (warn-only)
/// 3. **Condition captures consistency**: plan captures vs condition bindings (warn-only)
///
/// Phase 70-A: Standardized error tag for runtime unsupported patterns.
/// Phase 71-Pre: Delegated to OwnershipPlanValidator box.
#[cfg(feature = "normalized_dev")]
fn check_ownership_plan_consistency(
    plan: &crate::mir::join_ir::ownership::OwnershipPlan,
    carrier_info: &crate::mir::join_ir::lowering::carrier_info::CarrierInfo,
    condition_bindings: &std::collections::BTreeSet<String>,
) -> Result<(), String> {
    use crate::mir::join_ir::ownership::OwnershipPlanValidator;
    OwnershipPlanValidator::validate_all(plan, carrier_info, condition_bindings)
}
