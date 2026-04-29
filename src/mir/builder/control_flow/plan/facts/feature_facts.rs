//! Phase 29an P1: FeatureFacts SSOT (ExitUsage placeholders)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(in crate::mir::builder) struct ExitUsageFacts {
    pub has_break: bool,
    pub has_continue: bool,
    pub has_return: bool,
    pub has_unwind: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ExitKindFacts {
    Return,
    Break,
    Continue,
    Unwind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct ExitMapFacts {
    pub kinds_present: BTreeSet<ExitKindFacts>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum CleanupKindFacts {
    /// Cleanup vocabulary is kept as a structural slot for CorePlan adoption.
    /// Release extraction currently projects exit usage, not cleanup facts.
    #[allow(dead_code)]
    Return,
    #[allow(dead_code)]
    Break,
    #[allow(dead_code)]
    Continue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CleanupFacts {
    pub kinds_present: BTreeSet<CleanupKindFacts>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopFeatureFacts {
    pub nested_loop: bool,
    pub exit_usage: ExitUsageFacts,
    pub exit_map: Option<ExitMapFacts>,
    pub value_join: Option<ValueJoinFacts>,
    pub cleanup: Option<CleanupFacts>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct ValueJoinFacts {
    pub needed: bool,
}

pub(in crate::mir::builder) fn try_extract_loop_feature_facts(
    body: &[ASTNode],
) -> Result<LoopFeatureFacts, Freeze> {
    let mut exit_usage = ExitUsageFacts::default();
    for stmt in body {
        update_exit_usage_from_stmt(stmt, &mut exit_usage);
    }
    let nested_loop = detect_nested_loop(body);
    let mut kinds_present = BTreeSet::new();
    if exit_usage.has_return {
        kinds_present.insert(ExitKindFacts::Return);
    }
    if exit_usage.has_break {
        kinds_present.insert(ExitKindFacts::Break);
    }
    if exit_usage.has_continue {
        kinds_present.insert(ExitKindFacts::Continue);
    }
    if exit_usage.has_unwind {
        kinds_present.insert(ExitKindFacts::Unwind);
    }
    let exit_map = if kinds_present.is_empty() {
        None
    } else {
        Some(ExitMapFacts { kinds_present })
    };
    Ok(LoopFeatureFacts {
        nested_loop,
        exit_usage,
        exit_map,
        value_join: None,
        cleanup: None,
    })
}

pub(in crate::mir::builder) fn detect_nested_loop(body: &[ASTNode]) -> bool {
    body.iter().any(stmt_has_nested_loop)
}

fn stmt_has_nested_loop(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            detect_nested_loop(then_body)
                || else_body
                    .as_ref()
                    .is_some_and(|nested| detect_nested_loop(nested))
        }
        ASTNode::Program { statements, .. } => detect_nested_loop(statements),
        ASTNode::ScopeBox { body, .. } => detect_nested_loop(body),
        _ => false,
    }
}

fn update_exit_usage_from_stmt(stmt: &ASTNode, usage: &mut ExitUsageFacts) {
    match stmt {
        ASTNode::Break { .. } => usage.has_break = true,
        ASTNode::Continue { .. } => usage.has_continue = true,
        ASTNode::Return { .. } => usage.has_return = true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for nested in then_body {
                update_exit_usage_from_stmt(nested, usage);
            }
            if let Some(else_body) = else_body {
                for nested in else_body {
                    update_exit_usage_from_stmt(nested, usage);
                }
            }
        }
        ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {}
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{try_extract_loop_feature_facts, ExitKindFacts, ExitUsageFacts};
    use crate::ast::{ASTNode, LiteralValue, Span};

    fn lit_bool(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn exit_usage_tracks_break_continue_return_in_if() {
        let body = vec![
            ASTNode::If {
                condition: Box::new(lit_bool(true)),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: Some(vec![ASTNode::Continue {
                    span: Span::unknown(),
                }]),
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: None,
                span: Span::unknown(),
            },
        ];
        let facts = try_extract_loop_feature_facts(&body).expect("Ok");
        assert_eq!(
            facts.exit_usage,
            ExitUsageFacts {
                has_break: true,
                has_continue: true,
                has_return: true,
                has_unwind: false,
            }
        );
        assert!(!facts.nested_loop);
        let exit_map = facts.exit_map.expect("exit_map");
        assert_eq!(exit_map.kinds_present.len(), 3);
        assert!(exit_map.kinds_present.contains(&ExitKindFacts::Return));
        assert!(exit_map.kinds_present.contains(&ExitKindFacts::Break));
        assert!(exit_map.kinds_present.contains(&ExitKindFacts::Continue));
    }

    #[test]
    fn exit_usage_ignores_nested_loops() {
        let nested = ASTNode::Loop {
            condition: Box::new(lit_bool(true)),
            body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        };
        let facts = try_extract_loop_feature_facts(&[nested]).expect("Ok");
        assert_eq!(facts.exit_usage, ExitUsageFacts::default());
        assert!(facts.nested_loop);
        assert!(facts.exit_map.is_none());
    }

    #[test]
    fn nested_loop_detects_if_branch_loop() {
        let nested = ASTNode::If {
            condition: Box::new(lit_bool(true)),
            then_body: vec![ASTNode::Loop {
                condition: Box::new(lit_bool(true)),
                body: vec![],
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };
        let facts = try_extract_loop_feature_facts(&[nested]).expect("Ok");
        assert!(facts.nested_loop);
    }
}
