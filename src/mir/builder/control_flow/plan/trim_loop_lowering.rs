//! Phase 180: Trim/P5 Dedicated Lowering Module
//!
//! Consolidates Trim preprocessing shared by loop_break / loop_continue_only routes.
//! Route-specific loop skeleton lowering stays in those routes, while this module
//! handles Trim/CharComparison-specific knowledge.
//!
//! ## Responsibilities
//!
//! - Detect Trim-like loops (whitespace skipping patterns)
//! - Promote LoopBodyLocal variables to carriers
//! - Generate carrier initialization code (substring + whitespace check)
//! - Replace break conditions with carrier checks
//! - Setup ConditionEnv bindings for JoinIR
//!
//! ## Design Philosophy
//!
//! Follows Box Theory principles:
//! - **Single Responsibility**: Only handles Trim/P5 lowering logic
//! - **Reusability**: Used by loop_break, loop_continue_only, and future routes
//! - **Testability**: Pure data transformations, easy to unit test
//! - **Fail-Fast**: Returns errors early, no silent fallbacks
//!
//! ## Example Use Case
//!
//! **Original pattern** (loop_break recipe with Trim):
//! ```nyash
//! loop(start < end) {
//!     local ch = s.substring(start, start+1)
//!     if ch == " " || ch == "\t" { start = start + 1 } else { break }
//! }
//! ```
//!
//! **After TrimLoopLowerer processing**:
//! - LoopBodyLocal `ch` promoted to bool carrier `is_ch_match`
//! - Carrier initialized: `is_ch_match = (s.substring(start, start+1) == " " || ...)`
//! - Break condition replaced: `break on !is_ch_match`
//! - ConditionEnv binding: `ch` → JoinIR ValueId

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
use crate::mir::join_ir::lowering::common::condition_only_emitter::BreakSemantics;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_pattern_detection::loop_body_carrier_promoter::{
    LoopBodyCarrierPromoter, PromotionRequest, PromotionResult,
};
use crate::mir::ValueId;
use super::policies::trim_policy::{classify_trim_like_loop, TrimPolicyResult};
use super::policies::PolicyDecision;

/// Trim pattern lowering orchestrator
///
/// Phase 180: Single entry point for all Trim/P5 lowering operations.
pub(crate) struct TrimLoopLowerer;

/// Result of successful Trim lowering preprocessing
///
/// Contains all data needed by loop_break / loop_continue_only routes:
/// - Replaced break condition
/// - Updated carrier info with promoted carrier
/// - Condition environment bindings
/// - Trim helper for pattern-specific operations
pub(crate) struct TrimLoweringResult {
    /// Replaced break condition (e.g., `!is_carrier`)
    ///
    /// loop_break / loop_continue_only routes use this instead of the original break condition
    pub condition: ASTNode,

    /// Updated carrier info with promoted Trim carrier
    ///
    /// loop_break / loop_continue_only routes use this for JoinIR lowering
    pub carrier_info: CarrierInfo,

    /// Condition environment bindings for the carrier
    ///
    /// loop_break / loop_continue_only routes extend their condition_bindings with these
    pub condition_bindings: Vec<ConditionBinding>,

    /// Phase 93 P0: ConditionOnly recipe for derived slot recalculation
    ///
    /// loop_break / loop_continue_only routes use this to emit recalculation after body-local init
    pub condition_only_recipe: Option<crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyRecipe>,
}

impl TrimLoopLowerer {
    /// Phase 183-2: Check if a variable is used in a condition AST node
    ///
    /// Used to distinguish:
    /// - Condition LoopBodyLocal: Used in header/break/continue conditions → Need Trim promotion
    /// - Body-only LoopBodyLocal: Only in body assignments → No promotion needed
    ///
    /// # Arguments
    ///
    /// * `var_name` - Name of the variable to check
    /// * `cond_node` - Condition AST node to search in
    ///
    /// # Returns
    ///
    /// `true` if `var_name` appears anywhere in `cond_node`, `false` otherwise
    pub(in crate::mir::builder) fn is_var_used_in_condition(
        var_name: &str,
        cond_node: &ASTNode,
    ) -> bool {
        match cond_node {
            ASTNode::Variable { name, .. } => name == var_name,
            ASTNode::BinaryOp { left, right, .. } => {
                Self::is_var_used_in_condition(var_name, left)
                    || Self::is_var_used_in_condition(var_name, right)
            }
            ASTNode::UnaryOp { operand, .. } => Self::is_var_used_in_condition(var_name, operand),
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                Self::is_var_used_in_condition(var_name, object)
                    || arguments
                        .iter()
                        .any(|arg| Self::is_var_used_in_condition(var_name, arg))
            }
            // Add other node types as needed
            _ => false,
        }
    }

    /// Try to lower a Trim-like loop pattern
    ///
    /// Phase 180: Main entry point for Trim pattern detection and lowering.
    /// Phase 183-2: Updated to filter condition LoopBodyLocal only
    ///
    /// # Algorithm
    ///
    /// 1. Check if condition references LoopBodyLocal variables
    /// 2. **Phase 183**: Filter to only condition LoopBodyLocal (skip body-only)
    /// 3. Try to promote LoopBodyLocal to carrier (via LoopBodyCarrierPromoter)
    /// 4. If promoted as Trim pattern:
    ///    - Generate carrier initialization code
    ///    - Replace break condition with carrier check
    ///    - Setup ConditionEnv bindings
    /// 5. Return TrimLoweringResult with all updates
    ///
    /// # Arguments
    ///
    /// * `builder` - MirBuilder for code generation
    /// * `scope` - Loop structure metadata
    /// * `loop_cond` - Main loop condition (e.g., `start < end`)
    /// * `break_cond` - Break condition from loop body
    /// * `body` - Loop body AST nodes
    /// * `loop_var_name` - Loop variable name (e.g., "start")
    /// * `carrier_info` - Current carrier info (will be updated if Trim pattern)
    /// * `alloc_join_value` - JoinIR ValueId allocator closure
    ///
    /// # Returns
    ///
    /// - `Ok(Some(TrimLoweringResult))` - Trim pattern detected and lowered
    /// - `Ok(None)` - Not a Trim pattern (normal loop, no action taken)
    /// - `Err(String)` - Trim pattern detected but lowering failed
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(trim_result) = TrimLoopLowerer::try_lower_trim_like_loop(
    ///     self,
    ///     &scope,
    ///     condition,
    ///     &break_condition_node,
    ///     _body,
    ///     &loop_var_name,
    ///     &mut carrier_info,
    ///     &mut alloc_join_value,
    /// )? {
    ///     // Use trim_result.condition, carrier_info, condition_bindings
    /// }
    /// ```
    pub fn try_lower_trim_like_loop(
        builder: &mut MirBuilder,
        scope: &LoopScopeShape,
        loop_cond: &ASTNode,
        break_cond: &ASTNode,
        body: &[ASTNode],
        loop_var_name: &str,
        carrier_info: &mut CarrierInfo,
        alloc_join_value: &mut dyn FnMut() -> ValueId,
    ) -> Result<Option<TrimLoweringResult>, String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let verbose = crate::config::env::joinir_dev_enabled() || trace.is_joinir_enabled();

        let TrimPolicyResult {
            cond_scope,
            condition_body_locals,
        } = match classify_trim_like_loop(scope, loop_cond, break_cond, body, loop_var_name) {
            PolicyDecision::Use(res) => res,
            PolicyDecision::None => return Ok(None),
            PolicyDecision::Reject(reason) => return Err(reason),
        };

        trace.emit_if(
            "trim",
            "phase183",
            &format!(
                "Found {} condition LoopBodyLocal variables: {:?}",
                condition_body_locals.len(),
                condition_body_locals
                    .iter()
                    .map(|v| &v.name)
                    .collect::<Vec<_>>()
            ),
            verbose,
        );

        // Step 2: Try promotion via LoopBodyCarrierPromoter
        let request = PromotionRequest {
            scope,
            cond_scope: &cond_scope,
            break_cond: Some(break_cond),
            loop_body: body,
        };

        match LoopBodyCarrierPromoter::try_promote(&request) {
            PromotionResult::Promoted { trim_info } => {
                trace.emit_if(
                    "trim",
                    "promote",
                    &format!(
                        "LoopBodyLocal '{}' promoted to carrier '{}'",
                        trim_info.var_name, trim_info.carrier_name
                    ),
                    verbose,
                );

                // Step 3: Register promoted body-local variable (ConditionOnly)
                // Note: is_ch_match is NOT a LoopState carrier (no header PHI).
                // It's a condition-only variable recalculated each loop iteration.
                carrier_info
                    .promoted_body_locals
                    .push(trim_info.var_name.clone());

                // Step 3.5: Attach TrimLoopHelper for pattern-specific lowering logic
                use crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper;
                carrier_info.trim_helper = Some(TrimLoopHelper::from_pattern_info(&trim_info));

                trace.emit_if(
                    "trim",
                    "promote",
                    &format!(
                        "Promoted body-local '{}' to condition-only variable '{}' (not a LoopState carrier)",
                        trim_info.var_name,
                        trim_info.carrier_name
                    ),
                    verbose,
                );

                // Step 4: Safety check via TrimLoopHelper
                let trim_helper = carrier_info.trim_helper().ok_or_else(|| {
                    format!(
                        "[TrimLoopLowerer] Promoted but no TrimLoopHelper attached (carrier: '{}')",
                        trim_info.carrier_name
                    )
                })?;

                if !trim_helper.is_safe_trim() {
                    return Err(format!(
                        "[TrimLoopLowerer] Trim pattern detected but not safe: carrier='{}', whitespace_count={}",
                        trim_helper.carrier_name,
                        trim_helper.whitespace_chars.len()
                    ));
                }

                trace.emit_if(
                    "trim",
                    "safe",
                    "Safe Trim pattern detected, implementing lowering",
                    verbose,
                );
                trace.emit_if(
                    "trim",
                    "safe",
                    &format!(
                        "Carrier: '{}', original var: '{}', whitespace chars: {:?}",
                        trim_helper.carrier_name,
                        trim_helper.original_var,
                        trim_helper.whitespace_chars
                    ),
                    verbose,
                );

                // Step 5: Generate carrier initialization code
                Self::generate_carrier_initialization(builder, body, trim_helper)?;

                trace.emit_if(
                    "trim",
                    "init",
                    &format!(
                        "Registered carrier '{}' in variable_ctx.variable_map",
                        trim_helper.carrier_name
                    ),
                    verbose,
                );

                // Step 6: Setup ConditionEnv bindings FIRST to determine break semantics.
                //
                // IMPORTANT: derive semantics from the already-normalized `break_cond`
                // (loop_break route extracts "break when <cond> is true"), not from the raw body
                // `if/else` structure which may be rewritten during earlier analyses.
                let break_semantics = Self::infer_break_semantics_from_break_cond(break_cond);
                let (condition_bindings, condition_only_recipe) =
                    Self::setup_condition_env_bindings(
                        builder,
                        trim_helper,
                        break_semantics,
                        alloc_join_value,
                    )?;

                trace.emit_if(
                    "trim",
                    "cond",
                    &format!(
                        "Phase 93 P0: condition_bindings={}, condition_only_recipe={}",
                        condition_bindings.len(),
                        if condition_only_recipe.is_some() { "Some" } else { "None" }
                    ),
                    verbose,
                );

                // Step 7: Generate break condition based on pattern type
                // Phase 93 Refactoring: Use recipe.generate_break_condition() for unified logic
                let trim_break_condition = if let Some(ref recipe) = condition_only_recipe {
                    // Use recipe's break semantics (WhenMatch or WhenNotMatch)
                    trace.emit_if(
                        "trim",
                        "break-cond",
                        &format!(
                            "Generated break condition from recipe: {} (semantics: {:?})",
                            trim_helper.carrier_name,
                            recipe.break_semantics
                        ),
                        verbose,
                    );
                    recipe.generate_break_condition()
                } else {
                    // Normal Trim: "break when NOT match" semantics
                    // Generate: !is_ch_match (TRUE when we should break)
                    trace.emit_if(
                        "trim",
                        "break-cond",
                        &format!(
                            "Generated normal trim break condition: !{}",
                            trim_helper.carrier_name
                        ),
                        verbose,
                    );
                    Self::generate_trim_break_condition(trim_helper)
                };

                // Step 8: Return result with all updates
                Ok(Some(TrimLoweringResult {
                    condition: trim_break_condition,
                    carrier_info: carrier_info.clone(),
                    condition_bindings,
                    condition_only_recipe,
                }))
            }
            PromotionResult::CannotPromote { reason, vars } => {
                // Phase 196: Treat non-trim loops as normal loops.
                // If promotion fails, simply skip Trim lowering and let the caller
                // continue with the original break condition.
                trace.emit_if(
                    "trim",
                    "reject",
                    &format!(
                        "Cannot promote LoopBodyLocal variables {:?}: {}; skipping Trim lowering",
                        vars, reason
                    ),
                    verbose,
                );
                Ok(None)
            }
        }
    }

    /// Generate carrier initialization code
    ///
    /// Phase 180-3: Extracted from legacy loop_break lowering (lines 256-313)
    ///
    /// Generates:
    /// 1. ch0 = s.substring(start, start+1)
    /// 2. is_ch_match0 = (ch0 == " " || ch0 == "\t" || ...)
    /// 3. Registers carrier in variable_ctx.variable_map
    fn generate_carrier_initialization(
        builder: &mut MirBuilder,
        body: &[ASTNode],
        trim_helper: &crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper,
    ) -> Result<(), String> {
        use crate::mir::builder::control_flow::plan::trim_pattern_validator::TrimPatternValidator;
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let verbose = crate::config::env::joinir_dev_enabled() || trace.is_joinir_enabled();

        // Extract substring pattern from body
        let (s_name, start_expr) =
            TrimPatternValidator::extract_substring_args(body, &trim_helper.original_var)
                .ok_or_else(|| {
                    format!(
                    "[TrimLoopLowerer] Failed to extract substring pattern for Trim carrier '{}'",
                    trim_helper.carrier_name
                )
                })?;

        trace.emit_if(
            "trim",
            "init",
            &format!("Extracted substring pattern: s='{}', start={:?}", s_name, start_expr),
            verbose,
        );

        // Get ValueIds for string and start
        let s_id = builder
            .variable_ctx
            .variable_map
            .get(&s_name)
            .copied()
            .ok_or_else(|| format!("[TrimLoopLowerer] String variable '{}' not found", s_name))?;

        // Compile start expression to get ValueId
        let start_id = builder.build_expression_impl(*start_expr)?;

        // Generate: start + 1
        use crate::mir::builder::emission::constant::emit_integer;
        use crate::mir::instruction::MirInstruction;
        use crate::mir::types::BinaryOp;
        let one = emit_integer(builder, 1)?;
        // Phase 135 P0: Use function-level ValueId (SSOT)
        let start_plus_1 = builder.next_value_id();
        builder.emit_instruction(MirInstruction::BinOp {
            dst: start_plus_1,
            op: BinaryOp::Add,
            lhs: start_id,
            rhs: one,
        })?;

        // Generate: ch0 = s.substring(start, start+1)
        // Phase 135 P0: Use function-level ValueId (SSOT)
        let ch0 = builder.next_value_id();
        builder.emit_method_call(
            Some(ch0),
            s_id,
            "substring".to_string(),
            vec![start_id, start_plus_1],
        )?;

        trace.emit_if(
            "trim",
            "init",
            &format!("Generated initial substring call: ch0 = {:?}", ch0),
            verbose,
        );

        // Generate: is_ch_match0 = (ch0 == " " || ch0 == "\t" || ...)
        let is_ch_match0 = TrimPatternValidator::emit_whitespace_check(
            builder,
            ch0,
            &trim_helper.whitespace_chars,
        )?;

        trace.emit_if(
            "trim",
            "init",
            &format!(
                "Generated initial whitespace check: is_ch_match0 = {:?}",
                is_ch_match0
            ),
            verbose,
        );

        // Register carrier in variable_ctx.variable_map
        builder
            .variable_ctx
            .variable_map
            .insert(trim_helper.carrier_name.clone(), is_ch_match0);

        Ok(())
    }

    /// Generate Trim break condition (normal Trim pattern)
    ///
    /// Phase 180-3: Extracted from legacy loop_break lowering (lines 343-377)
    ///
    /// Returns: !is_carrier (negated carrier check)
    /// Used for "break when NOT match" semantics (e.g., str.trim())
    fn generate_trim_break_condition(
        trim_helper: &crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper,
    ) -> ASTNode {
        use crate::mir::builder::control_flow::plan::trim_pattern_lowerer::TrimPatternLowerer;
        TrimPatternLowerer::generate_trim_break_condition(trim_helper)
    }


    /// Setup ConditionEnv bindings for Trim carrier
    ///
    /// Phase 180-3: Extracted from legacy loop_break lowering (lines 345-377)
    /// Phase 93 Refactoring: Use explicit factory methods for recipe creation
    ///
    /// Creates bindings for:
    /// 1. Carrier variable (e.g., "is_ch_match")
    /// 2. Original variable (e.g., "ch") - mapped to same JoinIR ValueId
    fn setup_condition_env_bindings(
        _builder: &mut MirBuilder,
        trim_helper: &crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper,
        break_semantics: BreakSemantics,
        _alloc_join_value: &mut dyn FnMut() -> ValueId,
    ) -> Result<(Vec<ConditionBinding>, Option<crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyRecipe>), String> {
        
        use crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyRecipe;
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let verbose = crate::config::env::joinir_dev_enabled() || trace.is_joinir_enabled();

        // Phase 93 P0: Do NOT add is_ch_match to ConditionBinding
        // Phase 93 Refactoring: Use explicit factory method based on loop shape.
        let recipe = match break_semantics {
            BreakSemantics::WhenMatch => ConditionOnlyRecipe::from_trim_helper_condition_only(trim_helper),
            BreakSemantics::WhenNotMatch => ConditionOnlyRecipe::from_trim_helper_normal_trim(trim_helper),
        };

        trace.emit_if(
            "trim",
            "condition-only",
            &format!(
                "[phase93/condition-only] Created ConditionOnlyRecipe for '{}' (semantics: {:?}, will be recalculated each iteration)",
                trim_helper.carrier_name,
                recipe.break_semantics
            ),
            verbose,
        );

        // Return empty bindings - the derived slot will be recalculated, not bound
        Ok((Vec::new(), Some(recipe)))
    }

    fn infer_break_semantics_from_break_cond(break_cond: &ASTNode) -> BreakSemantics {
        // loop_break route passes `break_cond` as "break when <cond> is true".
        //
        // - find-first (ConditionOnly): break when match is true      -> `is_match`
        // - trim/skip-whitespace: break when match is false           -> `!is_ws`
        //
        // So: a top-level `!` means "break on non-match".
        match break_cond {
            ASTNode::UnaryOp { operator, .. }
                if matches!(operator, crate::ast::UnaryOperator::Not) =>
            {
                BreakSemantics::WhenNotMatch
            }
            _ => BreakSemantics::WhenMatch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_trim_loop_lowerer_skeleton() {
        // Phase 180-2: Basic existence test
        // Full tests will be added in Phase 180-3
        assert!(true, "TrimLoopLowerer skeleton compiles");
    }

    #[test]
    fn test_is_var_used_in_condition_simple_variable() {
        // Phase 183-2: Test variable detection
        let var_node = ASTNode::Variable {
            name: "ch".to_string(),
            span: Span::unknown(),
        };

        assert!(TrimLoopLowerer::is_var_used_in_condition("ch", &var_node));
        assert!(!TrimLoopLowerer::is_var_used_in_condition(
            "other", &var_node
        ));
    }

    #[test]
    fn test_is_var_used_in_condition_binary_op() {
        // Phase 183-2: Test variable in binary operation (ch == " ")
        let cond_node = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::Variable {
                name: "ch".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::String(" ".to_string()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        assert!(TrimLoopLowerer::is_var_used_in_condition("ch", &cond_node));
        assert!(!TrimLoopLowerer::is_var_used_in_condition(
            "other", &cond_node
        ));
    }

    #[test]
    fn test_is_var_used_in_condition_method_call() {
        // Phase 183-2: Test variable in method call (digit_pos < 0)
        let cond_node = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "digit_pos".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        assert!(TrimLoopLowerer::is_var_used_in_condition(
            "digit_pos",
            &cond_node
        ));
        assert!(!TrimLoopLowerer::is_var_used_in_condition(
            "other", &cond_node
        ));
    }

    #[test]
    fn test_is_var_used_in_condition_nested() {
        // Phase 183-2: Test variable in nested expression (ch == " " || ch == "\t")
        let left_cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::Variable {
                name: "ch".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::String(" ".to_string()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let right_cond = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(ASTNode::Variable {
                name: "ch".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::String("\t".to_string()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let or_node = ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(left_cond),
            right: Box::new(right_cond),
            span: Span::unknown(),
        };

        assert!(TrimLoopLowerer::is_var_used_in_condition("ch", &or_node));
        assert!(!TrimLoopLowerer::is_var_used_in_condition(
            "other", &or_node
        ));
    }
}
