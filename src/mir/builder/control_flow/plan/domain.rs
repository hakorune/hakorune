use super::loop_cond::continue_with_return_recipe::ContinueWithReturnRecipe;
use crate::ast::ASTNode;

/// Phase 29bq P2.x: Extracted structure for LoopCondContinueWithReturn
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondContinueWithReturnPlan {
    /// Loop condition AST
    pub condition: ASTNode,
    /// Recipe containing body items
    pub recipe: ContinueWithReturnRecipe,
}

/// Phase 273 P1: DomainPlan - Pattern-specific plan vocabulary
///
/// DomainPlan contains pattern-specific knowledge (e.g., scan semantics).
/// Normalizer converts DomainPlan → CorePlan with ValueId generation.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum DomainPlan {
    /// Phase 29bq P2.x: LoopCondContinueWithReturn
    LoopCondContinueWithReturn(LoopCondContinueWithReturnPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum DomainPlanKind {
    LoopCondContinueWithReturn,
}

impl DomainPlanKind {
    /// Stable variant label for diagnostics and planner logs.
    pub(in crate::mir::builder) fn label(self) -> &'static str {
        match self {
            DomainPlanKind::LoopCondContinueWithReturn => "LoopCondContinueWithReturn",
        }
    }
}

impl DomainPlan {
    /// Payload-free kind for rule routing/selection.
    pub(in crate::mir::builder) fn kind(&self) -> DomainPlanKind {
        match self {
            DomainPlan::LoopCondContinueWithReturn(_) => DomainPlanKind::LoopCondContinueWithReturn,
        }
    }

    /// Stable variant label for diagnostics and planner logs.
    ///
    /// Keep this payload-free so call sites can avoid coupling to variant internals.
    pub(in crate::mir::builder) fn kind_label(&self) -> &'static str {
        self.kind().label()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::ast::{ASTNode, LiteralValue};
    use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnItem;
    use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnRecipe;
    use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;

    #[test]
    fn domain_plan_kind_and_label_match() {
        let recipe = ContinueWithReturnRecipe::new(
            vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            vec![ContinueWithReturnItem::Stmt(StmtRef::new(0))],
        );
        let plan = DomainPlan::LoopCondContinueWithReturn(LoopCondContinueWithReturnPlan {
            condition: ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            },
            recipe,
        });

        assert_eq!(plan.kind(), DomainPlanKind::LoopCondContinueWithReturn);
        assert_eq!(plan.kind_label(), "LoopCondContinueWithReturn");
    }
}

/// Phase 273 P0: Scan direction for forward/reverse scan
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ScanDirection {
    /// Forward scan: i < s.length(), i = i + 1
    Forward,
    /// Reverse scan: i >= 0, i = i - 1
    Reverse,
}

pub(in crate::mir::builder) fn scan_direction_from_step_lit(
    step_lit: i64,
) -> Option<ScanDirection> {
    match step_lit {
        1 => Some(ScanDirection::Forward),
        -1 => Some(ScanDirection::Reverse),
        _ => None,
    }
}

/// Phase 273 P0: Extracted structure for scan-with-init pattern
///
/// This structure contains all the information needed to lower a index_of-style loop.
/// Moved from legacy scan-with-init extractor for centralization.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(in crate::mir::builder) struct ScanWithInitPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Haystack variable name (e.g., "s")
    pub haystack: String,
    /// Needle variable name (e.g., "ch")
    pub needle: String,
    /// Step literal (Phase 257: can be 1 forward or -1 reverse)
    pub step_lit: i64,
    /// Early return expression (P0: must be Variable(loop_var))
    pub early_return_expr: ASTNode,
    /// Not-found return literal (P0: must be -1)
    pub not_found_return_lit: i64,
    /// Scan direction (Phase 257 P0)
    pub scan_direction: ScanDirection,
    /// Phase 258 P0: True if dynamic needle (substr.length()), false if fixed (ch)
    pub dynamic_needle: bool,
}

/// Phase 273 P2: Extracted structure for split-scan pattern
///
/// This structure contains all the information needed to lower a split-style loop.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SplitScanPlan {
    /// Haystack variable name (e.g., "s")
    pub s_var: String,
    /// Separator variable name (e.g., "separator")
    pub sep_var: String,
    /// Accumulator variable name (e.g., "result", ArrayBox)
    pub result_var: String,
    /// Loop index variable name (e.g., "i")
    pub i_var: String,
    /// Segment start position variable name (e.g., "start")
    pub start_var: String,
}

/// Phase 286 P2: Extracted structure for Pattern4 (Loop with Continue)
///
/// This structure contains all the information needed to lower a continue-style loop.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern4ContinuePlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Carrier variable names (e.g., ["sum"])
    pub carrier_vars: Vec<String>,
    /// Loop condition AST (e.g., `i < 6`)
    pub condition: ASTNode,
    /// Continue condition AST (e.g., `i == 0`)
    pub continue_condition: ASTNode,
    /// Carrier update expressions (var -> update AST)
    pub carrier_updates: std::collections::BTreeMap<String, ASTNode>,
    /// Loop increment expression (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 286 P3.1: Step placement vocabulary for Pattern2 break-style loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum Pattern2StepPlacement {
    /// Loop increment executes at the end of the iteration (default).
    Last,
    /// Loop increment executes before the break check in the body.
    BeforeBreak,
}

/// Phase 286 P3.2: Exit kind for Pattern5 infinite loop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum Pattern5ExitKind {
    /// Early return from function
    Return,
    /// Break from loop
    Break,
}

/// Phase 286 P3.2: Extracted structure for Pattern5 (Infinite Loop with Early Exit)
///
/// This structure contains all the information needed to lower a loop(true) pattern
/// with early exit (return or break).
///
/// # PoC Subset
///
/// - `loop(true)` literal only (not `loop(1)` or truthy)
/// - Return version: `if (cond) { return <expr> }` + `i = i + 1`
/// - Break version: `if (cond) { break }` + `sum = sum + 1` + `i = i + 1` (carrier_update required)
///
/// # CFG Structure (Return version)
/// ```text
/// preheader → header(PHI: i_current) → body(exit_cond)
///               ↑                           ↓
///               └───── step ←────────  else path
///                                           ↓
///                                then path: CoreExitPlan::Return
/// ```
///
/// # CFG Structure (Break version)
/// ```text
/// preheader → header(PHI: i, carrier) → body(exit_cond)
///               ↑                             ↓
///               └───── step ←──────────  else path
///                                             ↓
///                                  then path → after_bb(PHI: carrier_out)
/// ```
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern5InfiniteEarlyExitPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Exit kind (Return or Break)
    pub exit_kind: Pattern5ExitKind,
    /// Exit condition AST (e.g., `i == 3`)
    pub exit_condition: ASTNode,
    /// Return value expression (Some for Return, None for Break)
    pub exit_value: Option<ASTNode>,
    /// Carrier variable name (Some for Break with carrier, None for Return)
    pub carrier_var: Option<String>,
    /// Carrier update expression (Some for Break, None for Return)
    pub carrier_update: Option<ASTNode>,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}
