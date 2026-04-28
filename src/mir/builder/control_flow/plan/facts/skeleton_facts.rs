//! Phase 29an P0: SkeletonFacts SSOT (Loop/If/BranchN/StraightLine)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum SkeletonKind {
    Loop,
    If2,
    BranchN,
    StraightLine,
}

/// Feature slot for Recipe-first migration (Phase A).
///
/// NOTE: Not used yet. Phase B will populate these slots.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(in crate::mir::builder) struct FeatureSlot {
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct SkeletonFacts {
    pub kind: SkeletonKind,
    /// Feature slots for Recipe-first (Phase A). Default empty.
    pub feature_slots: Vec<FeatureSlot>,
}

impl Default for SkeletonFacts {
    fn default() -> Self {
        Self {
            kind: SkeletonKind::StraightLine,
            feature_slots: Vec::new(),
        }
    }
}

#[cfg(test)]
pub(in crate::mir::builder) fn try_extract_skeleton_facts_from_stmt(
    stmt: &ASTNode,
) -> Result<Option<SkeletonFacts>, Freeze> {
    let kind = match stmt {
        ASTNode::Loop { .. } => SkeletonKind::Loop,
        ASTNode::If { .. } => SkeletonKind::If2,
        ASTNode::MatchExpr { .. } => SkeletonKind::BranchN,
        _ => return Ok(None),
    };

    Ok(Some(SkeletonFacts {
        kind,
        feature_slots: vec![],
    }))
}

pub(in crate::mir::builder) fn try_extract_loop_skeleton_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<SkeletonFacts>, Freeze> {
    let _ = (condition, body);
    Ok(Some(SkeletonFacts {
        kind: SkeletonKind::Loop,
        feature_slots: vec![],
    }))
}

#[cfg(test)]
pub(in crate::mir::builder) fn infer_region_skeleton_facts(
    stmts: &[ASTNode],
) -> Result<Option<SkeletonFacts>, Freeze> {
    let mut found = None;
    let mut count = 0usize;

    for stmt in stmts {
        let Some(facts) = try_extract_skeleton_facts_from_stmt(stmt)? else {
            continue;
        };
        count += 1;
        if count == 1 {
            found = Some(facts);
        } else {
            break;
        }
    }

    if count == 0 {
        return Ok(None);
    }
    if count == 1 {
        return Ok(found);
    }

    Err(Freeze::unstructured(
        "multiple top-level skeleton statements",
    ))
}

#[cfg(test)]
mod tests {
    use super::{infer_region_skeleton_facts, try_extract_skeleton_facts_from_stmt, SkeletonKind};
    use crate::ast::{ASTNode, LiteralValue, Span};

    #[test]
    fn skeleton_facts_loop_is_detected() {
        let loop_node = ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![],
            span: Span::unknown(),
        };
        let facts = try_extract_skeleton_facts_from_stmt(&loop_node)
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::Loop);
    }

    #[test]
    fn skeleton_facts_if_without_else_is_if2() {
        let if_node = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: None,
            span: Span::unknown(),
        };
        let facts = try_extract_skeleton_facts_from_stmt(&if_node)
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::If2);
    }

    #[test]
    fn skeleton_facts_if_with_else_is_if2() {
        let if_node = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: Some(vec![]),
            span: Span::unknown(),
        };
        let facts = try_extract_skeleton_facts_from_stmt(&if_node)
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::If2);
    }

    #[test]
    fn skeleton_facts_match_is_branchn() {
        let match_node = ASTNode::MatchExpr {
            scrutinee: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            arms: vec![(
                LiteralValue::Integer(1),
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                },
            )],
            else_expr: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let facts = try_extract_skeleton_facts_from_stmt(&match_node)
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::BranchN);
    }

    #[test]
    fn skeleton_facts_straight_line_is_none() {
        let assign = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let facts = try_extract_skeleton_facts_from_stmt(&assign).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn region_skeleton_empty_is_none() {
        let facts = infer_region_skeleton_facts(&[]).expect("Ok");
        assert!(facts.is_none());
    }

    #[test]
    fn region_skeleton_single_loop_is_some() {
        let loop_node = ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![],
            span: Span::unknown(),
        };
        let facts = infer_region_skeleton_facts(&[loop_node])
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::Loop);
    }

    #[test]
    fn region_skeleton_assignment_and_if_is_if2() {
        let assign = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_node = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: None,
            span: Span::unknown(),
        };
        let facts = infer_region_skeleton_facts(&[assign, if_node])
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::If2);
    }

    #[test]
    fn region_skeleton_multiple_is_unstructured() {
        let loop_node = ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![],
            span: Span::unknown(),
        };
        let if_node = ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: None,
            span: Span::unknown(),
        };
        let err = infer_region_skeleton_facts(&[loop_node, if_node]).unwrap_err();
        assert_eq!(err.tag, "unstructured");
    }
}
