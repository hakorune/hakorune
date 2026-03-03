use crate::ast::ASTNode;
use super::loop_cond::continue_with_return_recipe::ContinueWithReturnRecipe;

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
        match self.kind() {
            DomainPlanKind::LoopCondContinueWithReturn => "LoopCondContinueWithReturn",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue};
    use crate::ast::Span;
    use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnRecipe;
    use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnItem;
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

/// Phase 286 P2.1: Extracted structure for Pattern1 (Simple While Loop)
///
/// This structure contains all the information needed to lower a simple while loop.
/// Pattern1 is the simplest loop: no break, no continue, no if-else-phi.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1SimpleWhilePlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub condition: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 29ap P2: Extracted structure for Pattern1 char-map loop
///
/// This structure captures the stdlib-style `to_lower`/`to_upper` loop shape.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1CharMapPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Loop condition AST (e.g., `i < s.length()`)
    pub condition: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
    /// Haystack variable name (e.g., "s")
    pub haystack_var: String,
    /// Result accumulator variable name (e.g., "result")
    pub result_var: String,
    /// Receiver variable name for the transform method (e.g., "me")
    pub receiver_var: String,
    /// Transform method name (e.g., "char_to_lower")
    pub transform_method: String,
}

/// Phase 29ap P3: Extracted structure for Pattern1 array join loop
///
/// This structure captures the stdlib-style `StringUtils.join` loop shape.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern1ArrayJoinPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Loop condition AST (e.g., `i < arr.length()`)
    pub condition: ASTNode,
    /// Guard condition AST (e.g., `i > 0`)
    pub if_condition: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
    /// Array variable name (e.g., "arr")
    pub array_var: String,
    /// Result accumulator variable name (e.g., "result")
    pub result_var: String,
    /// Separator variable name (e.g., "separator")
    pub separator_var: String,
}

/// Phase 286 P2.3: Extracted structure for Pattern9 (Accumulator Const Loop)
///
/// This structure contains all the information needed to lower an accumulator loop.
/// Pattern9 extends Pattern1 with an accumulator variable (e.g., sum = sum + 1).
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern9AccumConstLoopPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Accumulator variable name (e.g., "sum")
    pub acc_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub condition: ASTNode,
    /// Accumulator update expression AST (e.g., `sum + 1` or `sum + i`)
    pub acc_update: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
    pub loop_increment: ASTNode,
}

/// Phase 286 P2.4: Extracted structure for Pattern8 (BoolPredicateScan)
///
/// This structure contains all the information needed to lower a boolean predicate scan loop.
/// Pattern8 scans a string with a predicate check (e.g., is_digit) and returns false on first failure.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern8BoolPredicateScanPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Haystack variable name (e.g., "s")
    pub haystack: String,
    /// Predicate receiver name (e.g., "me")
    pub predicate_receiver: String,
    /// Predicate method name (e.g., "is_digit")
    pub predicate_method: String,
    /// Loop condition AST (e.g., `i < s.length()`)
    pub condition: ASTNode,
    /// Loop increment literal (P0: must be 1)
    pub step_lit: i64,
}

/// Phase 286 P2.6: Extracted structure for Pattern3 (Loop with If-Phi)
///
/// This structure contains all the information needed to lower an if-phi merge loop.
/// Pattern3 is a loop with conditional carrier update via if-else branching.
///
/// # Structure
/// ```text
/// loop(i < N) {
///     if (condition) {
///         carrier = then_update
///     } else {
///         carrier = else_update
///     }
///     i = i + step
/// }
/// ```
///
/// # CFG Layout
/// ```text
/// preheader → header(PHI: i, carrier) → body(if_condition)
///              ↓                            ↓
///            after                     then | else
///                                        ↓     ↓
///                                       merge(PHI: carrier)
///                                          ↓
///                                        step(i_next)
///                                          ↓
///                                      back-edge to header
/// ```
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct Pattern3IfPhiPlan {
    /// Loop variable name (e.g., "i")
    pub loop_var: String,
    /// Carrier variable name (e.g., "sum")
    pub carrier_var: String,
    /// Loop condition AST (e.g., `i < 3`)
    pub condition: ASTNode,
    /// If condition AST (e.g., `i > 0`)
    pub if_condition: ASTNode,
    /// Then branch update AST (e.g., `sum + 1`)
    pub then_update: ASTNode,
    /// Else branch update AST (e.g., `sum + 0`)
    pub else_update: ASTNode,
    /// Loop increment expression AST (e.g., `i + 1`)
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
