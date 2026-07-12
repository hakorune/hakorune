//! Candidate-local control-anchor observations.
//!
//! This module relates an `If` condition to current-loop exit/backedge signals
//! in its branches. Nested-loop control flow is deliberately excluded by the
//! shared control-flow counter.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum GuardBranchV0 {
    Then,
    Else,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) struct ControlConditionSiteV0 {
    pub top_level_stmt_index: usize,
    pub condition_preorder_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum LoopControlAnchorV0 {
    CurrentLoopExitGuard {
        condition_site: ControlConditionSiteV0,
        branch: GuardBranchV0,
    },
    CurrentLoopBackedgeGuard {
        condition_site: ControlConditionSiteV0,
        branch: GuardBranchV0,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct BranchControlObservationV0 {
    pub current_loop_break: bool,
    pub function_return: bool,
    pub current_loop_continue: bool,
    pub nested_loop_seen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CandidateControlAnchorsV0 {
    pub anchors: Vec<LoopControlAnchorV0>,
}

pub(in crate::mir::builder) fn observe_branch_control_v0(
    branch: &[ASTNode],
) -> BranchControlObservationV0 {
    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(branch, detector);
    BranchControlObservationV0 {
        current_loop_break: counts.break_count > 0,
        function_return: counts.return_count > 0,
        current_loop_continue: counts.continue_count > 0,
        nested_loop_seen: counts.has_nested_loop,
    }
}

pub(in crate::mir::builder) fn observe_candidate_control_anchors_v0(
    candidate: &str,
    body: &[ASTNode],
) -> CandidateControlAnchorsV0 {
    let mut observer = AnchorObserver {
        candidate,
        next_condition_index: 0,
        anchors: Vec::new(),
    };
    for (top_level_stmt_index, stmt) in body.iter().enumerate() {
        observer.visit_stmt(stmt, top_level_stmt_index);
    }
    observer.anchors.sort();
    observer.anchors.dedup();
    CandidateControlAnchorsV0 {
        anchors: observer.anchors,
    }
}

struct AnchorObserver<'a> {
    candidate: &'a str,
    next_condition_index: usize,
    anchors: Vec<LoopControlAnchorV0>,
}

impl AnchorObserver<'_> {
    fn visit_stmt(&mut self, stmt: &ASTNode, top_level_stmt_index: usize) {
        match stmt {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let site = ControlConditionSiteV0 {
                    top_level_stmt_index,
                    condition_preorder_index: self.next_condition_index,
                };
                self.next_condition_index += 1;
                if expr_uses_candidate(condition, self.candidate) {
                    self.record_branch(site, GuardBranchV0::Then, then_body);
                    if let Some(else_body) = else_body {
                        self.record_branch(site, GuardBranchV0::Else, else_body);
                    }
                }
                for nested in then_body {
                    self.visit_stmt(nested, top_level_stmt_index);
                }
                if let Some(else_body) = else_body {
                    for nested in else_body {
                        self.visit_stmt(nested, top_level_stmt_index);
                    }
                }
            }
            ASTNode::Program { statements, .. }
            | ASTNode::ScopeBox {
                body: statements, ..
            } => {
                for nested in statements {
                    self.visit_stmt(nested, top_level_stmt_index);
                }
            }
            ASTNode::Loop { .. } | ASTNode::LoopRange { .. } => {
                // Nested-loop guards belong to that nested loop.
            }
            _ => {}
        }
    }

    fn record_branch(
        &mut self,
        condition_site: ControlConditionSiteV0,
        branch_side: GuardBranchV0,
        branch: &[ASTNode],
    ) {
        let control = observe_branch_control_v0(branch);
        if control.current_loop_break || control.function_return {
            self.anchors
                .push(LoopControlAnchorV0::CurrentLoopExitGuard {
                    condition_site,
                    branch: branch_side,
                });
        }
        if control.current_loop_continue {
            self.anchors
                .push(LoopControlAnchorV0::CurrentLoopBackedgeGuard {
                    condition_site,
                    branch: branch_side,
                });
        }
    }
}

fn expr_uses_candidate(expr: &ASTNode, candidate: &str) -> bool {
    match expr {
        ASTNode::Variable { name, .. } => name == candidate,
        ASTNode::UnaryOp { operand, .. } => expr_uses_candidate(operand, candidate),
        ASTNode::BinaryOp { left, right, .. } => {
            expr_uses_candidate(left, candidate) || expr_uses_candidate(right, candidate)
        }
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            expr_uses_candidate(object, candidate)
                || arguments
                    .iter()
                    .any(|argument| expr_uses_candidate(argument, candidate))
        }
        ASTNode::FunctionCall { arguments, .. } => arguments
            .iter()
            .any(|argument| expr_uses_candidate(argument, candidate)),
        ASTNode::FieldAccess { object, .. } => expr_uses_candidate(object, candidate),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        observe_branch_control_v0, observe_candidate_control_anchors_v0, GuardBranchV0,
        LoopControlAnchorV0,
    };
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn guard(name: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var(name)),
            right: Box::new(lit(8)),
            span: Span::unknown(),
        }
    }

    fn if_stmt(name: &str, then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
        ASTNode::If {
            condition: Box::new(guard(name)),
            then_body,
            else_body,
            span: Span::unknown(),
        }
    }

    #[test]
    fn branch_projection_separates_exit_backedge_and_nested_loop() {
        let nested = ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        };
        let observed = observe_branch_control_v0(&[
            nested,
            ASTNode::Continue {
                span: Span::unknown(),
            },
        ]);
        assert!(!observed.current_loop_break);
        assert!(observed.current_loop_continue);
        assert!(observed.nested_loop_seen);
    }

    #[test]
    fn candidate_condition_is_linked_to_exit_and_backedge_branches() {
        let body = vec![if_stmt(
            "cursor",
            vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            Some(vec![ASTNode::Continue {
                span: Span::unknown(),
            }]),
        )];
        let observed = observe_candidate_control_anchors_v0("cursor", &body);
        assert_eq!(observed.anchors.len(), 2);
        assert!(matches!(
            observed.anchors[0],
            LoopControlAnchorV0::CurrentLoopExitGuard {
                branch: GuardBranchV0::Then,
                ..
            }
        ));
        assert!(matches!(
            observed.anchors[1],
            LoopControlAnchorV0::CurrentLoopBackedgeGuard {
                branch: GuardBranchV0::Else,
                ..
            }
        ));
    }

    #[test]
    fn unrelated_condition_produces_no_anchor() {
        let body = vec![if_stmt(
            "other",
            vec![ASTNode::Return {
                value: None,
                span: Span::unknown(),
            }],
            None,
        )];
        assert!(observe_candidate_control_anchors_v0("cursor", &body)
            .anchors
            .is_empty());
    }
}
