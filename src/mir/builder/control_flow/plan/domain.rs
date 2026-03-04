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
/// DomainPlan is currently a single-variant payload.
/// Keep this as a dedicated alias so planner/normalizer signatures stay stable
/// while route vocabulary has only one active plan label.
pub(in crate::mir::builder) type DomainPlan = LoopCondContinueWithReturnPlan;

pub(in crate::mir::builder) const DOMAIN_PLAN_LABEL_LOOP_COND_CONTINUE_WITH_RETURN: &str =
    "LoopCondContinueWithReturn";

/// Stable plan label for diagnostics and planner logs.
pub(in crate::mir::builder) fn domain_plan_label(_plan: &DomainPlan) -> &'static str {
    DOMAIN_PLAN_LABEL_LOOP_COND_CONTINUE_WITH_RETURN
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
    fn domain_plan_kind_label_is_stable() {
        let recipe = ContinueWithReturnRecipe::new(
            vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            vec![ContinueWithReturnItem::Stmt(StmtRef::new(0))],
        );
        let plan = LoopCondContinueWithReturnPlan {
            condition: ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            },
            recipe,
        };

        assert_eq!(domain_plan_label(&plan), "LoopCondContinueWithReturn");
    }
}

/// Phase 273 P0: Scan direction for forward/reverse scan
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(test)]
pub(in crate::mir::builder) enum ScanDirection {
    /// Forward scan: i < s.length(), i = i + 1
    Forward,
    /// Reverse scan: i >= 0, i = i - 1
    Reverse,
}

#[cfg(test)]
pub(in crate::mir::builder) fn scan_direction_from_step_lit(
    step_lit: i64,
) -> Option<ScanDirection> {
    match step_lit {
        1 => Some(ScanDirection::Forward),
        -1 => Some(ScanDirection::Reverse),
        _ => None,
    }
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
