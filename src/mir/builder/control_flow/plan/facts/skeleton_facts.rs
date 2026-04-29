//! Phase 29an P0: Loop SkeletonFacts SSOT (Loop/StraightLine)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum SkeletonKind {
    Loop,
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
mod tests {
    use super::{try_extract_loop_skeleton_facts, SkeletonKind};
    use crate::ast::{ASTNode, LiteralValue, Span};

    #[test]
    fn loop_skeleton_facts_are_loop() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let facts = try_extract_loop_skeleton_facts(&condition, &[])
            .expect("Ok")
            .expect("Some");
        assert_eq!(facts.kind, SkeletonKind::Loop);
    }
}
