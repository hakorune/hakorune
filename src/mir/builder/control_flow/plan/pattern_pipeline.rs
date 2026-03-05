//! Phase 179-B: Generic Pattern Pipeline Context
//!
//! Unified preprocessing pipeline for Patterns 1-4.
//!
//! ## Design Philosophy
//!
//! **Pure Analysis Container**: RoutePrepContext is a "解析済みコンテキスト箱" that:
//! - Only holds preprocessing results (no JoinIR emission)
//! - Only depends on analyzer boxes (never lowering logic)
//! - Uses Option<T> for pattern-specific data
//!
//! ## Responsibility Separation
//!
//! ```
//! RoutePrepContext (this file)
//! ├─ Loop variable extraction        → CommonPatternInitializer
//! ├─ Carrier analysis               → CarrierInfo
//! ├─ Loop scope construction        → LoopScopeShapeBuilder
//! └─ Pattern-specific preprocessing → ConditionEnvBuilder, etc.
//!
//! Pattern Lowerers (pattern1-4.rs)
//! ├─ JoinIR generation              → lower_*_minimal()
//! ├─ Boundary construction          → JoinInlineBoundaryBuilder
//! └─ MIR merge                      → JoinIRConversionPipeline
//! ```
//!
//! ## Benefits
//!
//! - **Code reduction**: 1012 lines → ~195 lines (81% reduction)
//! - **Single source of truth**: All patterns use same preprocessing logic
//! - **Testability**: Can test preprocessing independently
//! - **Consistency**: Uniform error messages and trace output

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::join_ir::lowering::loop_update_summary::LoopUpdateSummary; // Phase 213
use crate::mir::loop_pattern_detection::trim_loop_helper::TrimLoopHelper;
use crate::mir::BasicBlockId;
use crate::mir::ValueId;
use std::collections::{BTreeMap, BTreeSet}; // Phase 222.5-D: HashMap → BTreeMap for determinism

use crate::mir::builder::control_flow::plan::common_init::CommonPatternInitializer;
use crate::mir::builder::control_flow::plan::loop_true_counter_extractor::LoopTrueCounterExtractorBox;
use crate::mir::builder::control_flow::plan::loop_scope_shape_builder::LoopScopeShapeBuilder;

/// Phase 179-B: Unified Pattern Pipeline Context
///
/// Pure "解析済みコンテキスト箱" - holds only preprocessing results.
/// JoinIR emission and PHI assembly remain in existing lowerers.
///
/// # Design Constraints
///
/// - **Analyzer-only dependencies**: Never depends on lowering logic
/// - **No emission**: No JoinIR/MIR generation in this context
/// - **Pattern variants**: Pattern-specific data stored in Option<T>
///
/// # Usage
///
/// ```rust
/// let ctx = build_route_prep_context(
///     builder,
///     condition,
///     body,
///     PatternVariant::Pattern1,
/// )?;
///
/// // Use preprocessed data for lowering
/// let join_module = lower_simple_while_minimal(ctx.loop_scope)?;
/// ```
#[derive(Debug, Clone)]
pub(crate) struct RoutePrepContext {
    // === Common Data (All Patterns) ===
    /// Loop variable name (e.g., "i")
    pub loop_var_name: String,

    /// Loop variable HOST ValueId
    pub loop_var_id: ValueId,

    /// Carrier information (loop variable + carriers)
    pub carrier_info: CarrierInfo,

    /// Loop scope shape (header/body/latch/exit structure)
    pub loop_scope: LoopScopeShape,

    // === Pattern 2/4: Break/Continue Condition ===
    /// Condition environment (variable → JoinIR ValueId mapping)
    /// Used by Pattern 2 (break condition) and Pattern 4 (continue condition)
    pub condition_env: Option<ConditionEnv>,

    /// Condition bindings (HOST↔JoinIR value mappings)
    /// Used by Pattern 2 and Pattern 4
    pub condition_bindings: Option<Vec<ConditionBinding>>,

    /// Carrier update expressions (variable → UpdateExpr)
    /// Used by Pattern 2 (multi-carrier) and Pattern 4 (Select-based updates)
    pub carrier_updates: Option<BTreeMap<String, UpdateExpr>>, // Phase 222.5-D: HashMap → BTreeMap for determinism

    // === Pattern 2/4: Trim Pattern Support ===
    /// Trim loop helper (if Trim pattern detected during promotion)
    /// Used by Pattern 2 (string trim) - Pattern 4 support TBD
    pub trim_helper: Option<TrimLoopHelper>,

    // === Pattern 2: Break Condition ===
    /// Effective break condition (may be modified for Trim pattern)
    /// Used only by Pattern 2
    pub break_condition: Option<ASTNode>,

    // === Pattern 3: If-Sum Generalization (Phase 213) ===
    /// Loop condition AST node
    /// Used by Pattern 3 for dynamic loop condition lowering
    pub loop_condition: Option<ASTNode>,

    /// Loop body AST nodes
    /// Used by Pattern 3 to extract if statement for if-sum lowering
    pub loop_body: Option<Vec<ASTNode>>,

    /// Loop update summary with then/else expressions
    /// Used by Pattern 3 for dynamic carrier update lowering
    pub loop_update_summary: Option<LoopUpdateSummary>,
}

/// Pattern variant selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PatternVariant {
    /// Pattern 1: Simple while loop (no break, no continue, no if-else PHI)
    Pattern1,
    /// Pattern 2: Loop with break statement
    Pattern2,
    /// Pattern 3: Loop with if-else PHI (no break/continue)
    Pattern3,
    /// Pattern 4: Loop with continue statement
    Pattern4,
}

impl RoutePrepContext {
    /// Get the number of carriers (excluding loop variable)
    pub fn carrier_count(&self) -> usize {
        self.carrier_info.carrier_count()
    }

    /// Check if this is a Trim pattern
    pub fn is_trim_pattern(&self) -> bool {
        self.trim_helper.is_some()
    }

    /// Check if this has condition environment (Pattern 2/4)
    pub fn has_condition_env(&self) -> bool {
        self.condition_env.is_some()
    }

    /// Check if this has carrier updates (Pattern 2/4)
    pub fn has_carrier_updates(&self) -> bool {
        self.carrier_updates.is_some()
    }

    /// Phase 213: Check if this is a simple if-sum pattern for AST-based lowering
    ///
    /// Returns true if:
    /// 1. loop_body contains an if statement
    /// 2. if condition is a simple comparison (var CmpOp literal) - Phase 219-fix
    /// 3. carrier composition matches if-sum pattern (1 counter + 1-2 accumulators)
    ///
    /// This determines whether to use AST-based lowering or legacy PoC lowering.
    pub fn is_if_sum_pattern(&self) -> bool {
        // Check if loop_body has if statement
        let if_stmt = self.extract_if_statement();
        if if_stmt.is_none() {
            return false;
        }

        // Phase 222: Check if if condition is a simple comparison and normalizable
        // Complex conditions (e.g., i % 2 == 1) → fallback to legacy mode
        if let Some(ASTNode::If { condition, .. }) = if_stmt {
            use crate::mir::join_ir::lowering::condition_pattern::{
                analyze_condition_capability, ConditionCapability,
            };

            // Capability check: if-sum lowerer が扱える比較か
            if analyze_condition_capability(condition) != ConditionCapability::IfSumComparable {
                return false;
            }
        }

        // Phase 219: Use assignment-based carrier detection
        // (1 counter like "i" + 1-2 accumulators like "sum", "count")
        use crate::mir::join_ir::lowering::loop_update_summary::analyze_loop_updates_from_ast;
        let carrier_names: Vec<String> = self
            .carrier_info
            .carriers
            .iter()
            .map(|c| c.name.clone())
            .collect();

        // Add loop variable to carrier list (it's also part of the pattern)
        let mut all_names = vec![self.loop_var_name.clone()];
        all_names.extend(carrier_names);

        // Phase 219: Pass loop body AST for assignment-based detection
        let empty_body = vec![];
        let loop_body = self.loop_body.as_ref().unwrap_or(&empty_body);
        let summary = analyze_loop_updates_from_ast(&all_names, loop_body);
        summary.is_simple_if_sum_pattern()
    }

    /// Phase 213: Extract if statement from loop body
    ///
    /// Returns the first if statement found in loop_body, if any.
    pub fn extract_if_statement(&self) -> Option<&ASTNode> {
        self.loop_body
            .as_ref()
            .and_then(|body| body.iter().find(|stmt| matches!(stmt, ASTNode::If { .. })))
    }
}

/// Build pattern preprocessing context
///
/// This consolidates all preprocessing steps shared by Patterns 1-4:
/// 1. Loop variable extraction (CommonPatternInitializer)
/// 2. LoopScopeShape construction (LoopScopeShapeBuilder)
/// 3. Pattern-specific analysis (ConditionEnv, carrier updates, etc.)
/// 4. Trim pattern promotion (if applicable)
///
/// # Arguments
///
/// * `builder` - MirBuilder instance
/// * `condition` - Loop condition AST node
/// * `body` - Loop body AST nodes
/// * `variant` - Pattern variant selector
///
/// # Returns
///
/// RoutePrepContext with all preprocessing results
///
/// # Errors
///
/// Returns error if:
/// - Loop variable not found in variable_map
/// - Condition variable not found (Pattern 2/4)
/// - Trim pattern promotion fails (Pattern 2/4)
pub(crate) fn build_route_prep_context(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    variant: PatternVariant,
) -> Result<RoutePrepContext, String> {
    // Step 1: Common initialization (all patterns)
    //
    // Phase 104: Pattern2 now supports `loop(true)` by extracting the counter from the body.
    // This path must be strict and conservative to avoid "accidental" routing.
    let (loop_var_name, loop_var_id, carrier_info) = if variant == PatternVariant::Pattern2
        && LoopTrueCounterExtractorBox::is_loop_true(condition)
    {
        let (name, host_id) = LoopTrueCounterExtractorBox::extract_loop_counter_from_body(
            body,
            &builder.variable_ctx.variable_map,
        )?;
        let carrier_info = CarrierInfo::from_variable_map(name.clone(), &builder.variable_ctx.variable_map)?;
        (name, host_id, carrier_info)
    } else {
        CommonPatternInitializer::initialize_pattern(
            builder,
            condition,
            &builder.variable_ctx.variable_map,
            None, // No exclusions for now (Pattern 2/4 will filter carriers later)
        )?
    };

    // Step 2: Build LoopScopeShape
    let loop_scope = match variant {
        PatternVariant::Pattern1 | PatternVariant::Pattern3 => {
            // Pattern 1, 3: No body_locals needed (condition-only analysis)
            LoopScopeShapeBuilder::empty_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
            )
        }
        PatternVariant::Pattern2 | PatternVariant::Pattern4 => {
            // Pattern 2, 4: Extract body_locals for Trim support and carrier promotion
            LoopScopeShapeBuilder::with_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
                body,
            )
        }
    };

    // Step 3: Pattern-specific preprocessing
    let (
        condition_env,
        condition_bindings,
        carrier_updates,
        trim_helper,
        break_condition,
        loop_condition,
        loop_body,
        loop_update_summary,
    ) = match variant {
        PatternVariant::Pattern1 => {
            // Pattern 1: No additional preprocessing needed
            (None, None, None, None, None, None, None, None)
        }
        PatternVariant::Pattern3 => {
            // Pattern 3: Phase 213 - Store loop condition and body for AST-based lowering
            (
                None,                    // No condition_env
                None,                    // No condition_bindings
                None,                    // No carrier_updates (old style)
                None,                    // No trim_helper
                None,                    // No break_condition
                Some(condition.clone()), // loop_condition (Phase 213)
                Some(body.to_vec()),     // loop_body (Phase 213)
                None,                    // loop_update_summary (reserved for future use)
            )
        }
        PatternVariant::Pattern2 | PatternVariant::Pattern4 => {
            // Pattern 2/4: Full preprocessing will be handled by existing code
            // For now, return empty values (will be populated by pattern-specific logic)
            //
            // Note: Pattern 2/4 have complex preprocessing that includes:
            // - Break/continue condition analysis
            // - Carrier update analysis
            // - Trim pattern promotion
            // These will remain in pattern2/pattern4.rs for now and will be
            // gradually migrated into this pipeline in future phases.
            (None, None, None, None, None, None, None, None)
        }
    };

    Ok(RoutePrepContext {
        loop_var_name,
        loop_var_id,
        carrier_info,
        loop_scope,
        condition_env,
        condition_bindings,
        carrier_updates,
        trim_helper,
        break_condition,
        loop_condition,      // Phase 213
        loop_body,           // Phase 213
        loop_update_summary, // Phase 213
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    // Helper: Create a simple condition (i < 10)
    fn test_condition(var_name: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: var_name.to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(10),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_pattern_variant_equality() {
        assert_eq!(PatternVariant::Pattern1, PatternVariant::Pattern1);
        assert_ne!(PatternVariant::Pattern1, PatternVariant::Pattern2);
    }

    #[test]
    fn test_context_carrier_count() {
        use crate::mir::join_ir::lowering::carrier_info::CarrierVar;

        let ctx = RoutePrepContext {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(5),
            carrier_info: CarrierInfo {
                loop_var_name: "i".to_string(),
                loop_var_id: ValueId(5),
                carriers: vec![
                    CarrierVar {
                        name: "sum".to_string(),
                        host_id: ValueId(10),
                        join_id: None,
                        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
                        #[cfg(feature = "normalized_dev")]
                        binding_id: None,
                    },
                    CarrierVar {
                        name: "count".to_string(),
                        host_id: ValueId(11),
                        join_id: None,
                        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
                        #[cfg(feature = "normalized_dev")]
                        binding_id: None,
                    },
                ],
                trim_helper: None,
                promoted_loopbodylocals: Vec::new(), // Phase 224
                #[cfg(feature = "normalized_dev")]
                promoted_bindings: std::collections::BTreeMap::new(), // Phase 76
            },
            loop_scope: LoopScopeShapeBuilder::empty_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
            ),
            condition_env: None,
            condition_bindings: None,
            carrier_updates: None,
            trim_helper: None,
            break_condition: None,
            loop_condition: None,      // Phase 213
            loop_body: None,           // Phase 213
            loop_update_summary: None, // Phase 213
        };

        assert_eq!(ctx.carrier_count(), 2);
        assert!(!ctx.is_trim_pattern());
        assert!(!ctx.has_condition_env());
        assert!(!ctx.has_carrier_updates());
    }

    #[test]
    fn test_context_with_trim() {
        let ctx = RoutePrepContext {
            loop_var_name: "pos".to_string(),
            loop_var_id: ValueId(5),
            carrier_info: CarrierInfo {
                loop_var_name: "pos".to_string(),
                loop_var_id: ValueId(5),
                carriers: vec![],
                trim_helper: Some(TrimLoopHelper {
                    original_var: "ch".to_string(),
                    carrier_name: "is_whitespace".to_string(),
                    whitespace_chars: vec![" ".to_string(), "\t".to_string()],
                }),
                promoted_loopbodylocals: Vec::new(), // Phase 224
                #[cfg(feature = "normalized_dev")]
                promoted_bindings: std::collections::BTreeMap::new(), // Phase 76
            },
            loop_scope: LoopScopeShapeBuilder::empty_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
            ),
            condition_env: None,
            condition_bindings: None,
            carrier_updates: None,
            trim_helper: Some(TrimLoopHelper {
                original_var: "ch".to_string(),
                carrier_name: "is_whitespace".to_string(),
                whitespace_chars: vec![" ".to_string(), "\t".to_string()],
            }),
            break_condition: None,
            loop_condition: None,      // Phase 213
            loop_body: None,           // Phase 213
            loop_update_summary: None, // Phase 213
        };

        assert!(ctx.is_trim_pattern());
        assert_eq!(ctx.trim_helper.as_ref().unwrap().original_var, "ch");
    }
}
