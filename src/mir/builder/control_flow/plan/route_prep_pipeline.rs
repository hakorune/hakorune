//! Phase 179-B: Generic Route Prep Pipeline Context
//!
//! Unified preprocessing pipeline for loop route variants.
//!
//! ## Design Philosophy
//!
//! **Pure Analysis Container**: RoutePrepContext is a "解析済みコンテキスト箱" that:
//! - Only holds preprocessing results (no JoinIR emission)
//! - Only depends on analyzer boxes (never lowering logic)
//! - Uses Option<T> for variant-specific data
//!
//! ## Responsibility Separation
//!
//! ```
//! RoutePrepContext (this file)
//! ├─ Loop variable extraction        → CommonPatternInitializer
//! ├─ Carrier analysis               → CarrierInfo
//! ├─ Loop scope construction        → LoopScopeShapeBuilder
//! └─ Variant-specific preprocessing → ConditionEnvBuilder, etc.
//!
//! Variant Lowerers (dedicated route modules)
//! ├─ JoinIR generation              → lower_*_minimal()
//! ├─ Boundary construction          → JoinInlineBoundaryBuilder
//! └─ MIR merge                      → dedicated route lowerers + joinir::merge
//! ```
//!
//! ## Benefits
//!
//! - **Code reduction**: 1012 lines → ~195 lines (81% reduction)
//! - **Single source of truth**: All route variants use same preprocessing logic
//! - **Testability**: Can test preprocessing independently
//! - **Consistency**: Uniform error messages and trace output

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::condition_env::ConditionBinding;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::lowering::loop_update_analyzer::UpdateExpr;
use crate::mir::loop_route_detection::support::trim::TrimLoopHelper;
use crate::mir::BasicBlockId;
use crate::mir::ValueId;
use std::collections::{BTreeMap, BTreeSet}; // Phase 222.5-D: HashMap → BTreeMap for determinism

use crate::mir::builder::control_flow::plan::common_init::CommonPatternInitializer;
use crate::mir::builder::control_flow::plan::loop_scope_shape_builder::LoopScopeShapeBuilder;
use crate::mir::builder::control_flow::plan::loop_true_counter_extractor::LoopTrueCounterExtractorBox;

/// Phase 179-B: Unified route preprocessing context
///
/// Pure "解析済みコンテキスト箱" - holds only preprocessing results.
/// JoinIR emission and PHI assembly remain in existing lowerers.
///
/// # Design Constraints
///
/// - **Analyzer-only dependencies**: Never depends on lowering logic
/// - **No emission**: No JoinIR/MIR generation in this context
/// - **Variant data**: Route-specific data stored in Option<T>
///
/// # Usage
///
/// `RouteVariant` is the semantic route selector for shared preprocessing.
/// Historical numbered labels remain traceability-only in comments/docs.
///
/// ```rust
/// let ctx = build_route_prep_context(
///     builder,
///     condition,
///     body,
///     RouteVariant::LoopSimpleWhile,
/// )?;
///
/// // Use preprocessed data for lowering
/// let join_module = lower_simple_while_minimal(ctx.loop_scope)?;
/// ```
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct RoutePrepContext {
    // === Common Data (All Route Variants) ===
    /// Loop variable name (e.g., "i")
    pub loop_var_name: String,

    /// Loop variable HOST ValueId
    pub loop_var_id: ValueId,

    /// Carrier information (loop variable + carriers)
    pub carrier_info: CarrierInfo,

    /// Loop scope shape (header/body/latch/exit structure)
    pub loop_scope: LoopScopeShape,

    // === Break/Continue Variant: Condition Data ===
    /// Condition environment (variable → JoinIR ValueId mapping)
    /// Used by break and continue variants
    pub condition_env: Option<ConditionEnv>,

    /// Condition bindings (HOST↔JoinIR value mappings)
    /// Used by break and continue variants
    pub condition_bindings: Option<Vec<ConditionBinding>>,

    /// Carrier update expressions (variable → UpdateExpr)
    /// Used by break (multi-carrier) and continue (Select-based updates) variants
    pub carrier_updates: Option<BTreeMap<String, UpdateExpr>>, // Phase 222.5-D: HashMap → BTreeMap for determinism

    // === Break/Continue Variant: Trim Support ===
    /// Trim loop helper (if Trim pattern detected during promotion)
    /// Used by break-route string trim path; continue-route support TBD
    pub trim_helper: Option<TrimLoopHelper>,

    // === Break Variant: Exit Condition ===
    /// Effective break condition (may be modified for Trim pattern)
    /// Used only by the break variant
    pub break_condition: Option<ASTNode>,

    // === If-Sum Variant: Generalization (Phase 213) ===
    /// Loop condition AST node
    /// Used by if-sum variant for dynamic loop condition lowering
    pub loop_condition: Option<ASTNode>,

    /// Loop body AST nodes
    /// Used by if-sum variant to extract if statement for if-sum lowering
    pub loop_body: Option<Vec<ASTNode>>,
}

/// Route variant selector
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteVariant {
    /// loop_simple_while route
    LoopSimpleWhile,
    /// loop_break route
    LoopBreak,
    /// if_phi_join route
    IfPhiJoin,
    /// loop_continue_only route
    LoopContinueOnly,
}

impl RoutePrepContext {
    /// Get the number of carriers (excluding loop variable)
    #[cfg(test)]
    pub fn carrier_count(&self) -> usize {
        self.carrier_info.carrier_count()
    }

    /// Check if this is a Trim pattern
    #[cfg(test)]
    pub fn is_trim_pattern(&self) -> bool {
        self.trim_helper.is_some()
    }

    /// Check if this has condition environment (break/continue variants)
    #[cfg(test)]
    pub fn has_condition_env(&self) -> bool {
        self.condition_env.is_some()
    }

    /// Check if this has carrier updates (break/continue variants)
    #[cfg(test)]
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
    /// This determines whether to use AST-based if_phi_join lowering or the general lowering path.
    #[allow(dead_code)]
    pub fn is_if_phi_join_pattern(&self) -> bool {
        // Check if loop_body has if statement
        let if_stmt = self.extract_if_statement();
        if if_stmt.is_none() {
            return false;
        }

        // Phase 222: Check if if condition is a simple comparison and normalizable
        // Complex conditions (e.g., i % 2 == 1) use the conservative non-if-sum path.
        if let Some(ASTNode::If { condition, .. }) = if_stmt {
            use crate::mir::join_ir::lowering::condition_pattern::{
                analyze_condition_capability, ConditionCapability,
            };

            // Capability check: if_phi_join lowerer が扱える比較か
            if analyze_condition_capability(condition) != ConditionCapability::IfPhiJoinComparable {
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
    #[allow(dead_code)]
    pub fn extract_if_statement(&self) -> Option<&ASTNode> {
        self.loop_body
            .as_ref()
            .and_then(|body| body.iter().find(|stmt| matches!(stmt, ASTNode::If { .. })))
    }
}

/// Build route preprocessing context
///
/// This consolidates all preprocessing steps shared by route variants:
/// 1. Loop variable extraction (CommonPatternInitializer)
/// 2. LoopScopeShape construction (LoopScopeShapeBuilder)
/// 3. Variant-specific analysis (ConditionEnv, carrier updates, etc.)
/// 4. Trim pattern promotion (if applicable)
///
/// # Arguments
///
/// * `builder` - MirBuilder instance
/// * `condition` - Loop condition AST node
/// * `body` - Loop body AST nodes
/// * `variant` - Route variant selector (`RouteVariant`)
///
/// # Returns
///
/// RoutePrepContext with all preprocessing results
///
/// # Errors
///
/// Returns error if:
/// - Loop variable not found in variable_map
/// - Condition variable not found (break/continue variants)
/// - Trim pattern promotion fails (break/continue variants)
#[allow(dead_code)]
pub(crate) fn build_route_prep_context(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    variant: RouteVariant,
) -> Result<RoutePrepContext, String> {
    // Step 1: Common initialization (all route variants)
    //
    // Phase 104: Break variant supports `loop(true)` by extracting the counter from the body.
    // This path must be strict and conservative to avoid "accidental" routing.
    let (loop_var_name, loop_var_id, carrier_info) = if variant == RouteVariant::LoopBreak
        && LoopTrueCounterExtractorBox::is_loop_true(condition)
    {
        let (name, host_id) = LoopTrueCounterExtractorBox::extract_loop_counter_from_body(
            body,
            &builder.variable_ctx.variable_map,
        )?;
        let carrier_info =
            CarrierInfo::from_variable_map(name.clone(), &builder.variable_ctx.variable_map)?;
        (name, host_id, carrier_info)
    } else {
        CommonPatternInitializer::initialize_pattern(
            builder,
            condition,
            &builder.variable_ctx.variable_map,
            None, // No exclusions for now (break/continue variants filter carriers later)
        )?
    };

    // Step 2: Build LoopScopeShape
    let loop_scope = match variant {
        RouteVariant::LoopSimpleWhile | RouteVariant::IfPhiJoin => {
            // Simple/if-sum variants: no body_locals needed (condition-only analysis)
            LoopScopeShapeBuilder::empty_body_locals(
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BasicBlockId(0),
                BTreeSet::new(),
            )
        }
        RouteVariant::LoopBreak | RouteVariant::LoopContinueOnly => {
            // Break/continue variants: extract body_locals for trim support and promotion
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

    // Step 3: Variant-specific preprocessing
    let (
        condition_env,
        condition_bindings,
        carrier_updates,
        trim_helper,
        break_condition,
        loop_condition,
        loop_body,
    ) = match variant {
        RouteVariant::LoopSimpleWhile => {
            // Simple while variant: no additional preprocessing needed
            (None, None, None, None, None, None, None)
        }
        RouteVariant::IfPhiJoin => {
            // If-sum variant: Phase 213 stores loop condition/body for AST-based lowering.
            (
                None,                    // No condition_env
                None,                    // No condition_bindings
                None,                    // No carrier_updates (old style)
                None,                    // No trim_helper
                None,                    // No break_condition
                Some(condition.clone()), // loop_condition (Phase 213)
                Some(body.to_vec()),     // loop_body (Phase 213)
            )
        }
        RouteVariant::LoopBreak | RouteVariant::LoopContinueOnly => {
            // Break/continue variants: full preprocessing is handled in dedicated lowerers.
            // For now, return empty values (populated by route-specific logic).
            //
            // Note: these variants have complex preprocessing that includes:
            // - Break/continue condition analysis
            // - Carrier update analysis
            // - Trim pattern promotion
            // This remains in dedicated break/continue paths for now and will be
            // gradually migrated into this pipeline in future phases.
            (None, None, None, None, None, None, None)
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
        loop_condition, // Phase 213
        loop_body,      // Phase 213
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
    fn test_route_variant_equality() {
        assert_eq!(RouteVariant::LoopSimpleWhile, RouteVariant::LoopSimpleWhile);
        assert_ne!(RouteVariant::LoopSimpleWhile, RouteVariant::LoopBreak);
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
                    },
                    CarrierVar {
                        name: "count".to_string(),
                        host_id: ValueId(11),
                        join_id: None,
                        role: crate::mir::join_ir::lowering::carrier_info::CarrierRole::LoopState,
                        init: crate::mir::join_ir::lowering::carrier_info::CarrierInit::FromHost, // Phase 228
                    },
                ],
                trim_helper: None,
                promoted_body_locals: Vec::new(), // Phase 224
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
            loop_condition: None, // Phase 213
            loop_body: None,      // Phase 213
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
                promoted_body_locals: Vec::new(), // Phase 224
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
            loop_condition: None, // Phase 213
            loop_body: None,      // Phase 213
        };

        assert!(ctx.is_trim_pattern());
        assert_eq!(ctx.trim_helper.as_ref().unwrap().original_var, "ch");
    }
}
