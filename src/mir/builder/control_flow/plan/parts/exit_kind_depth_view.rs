//! ExitKind depth adapter (analysis-only).

use crate::mir::builder::control_flow::plan::facts::feature_facts::ExitKindFacts;
use crate::mir::builder::control_flow::plan::recipe_tree::common::ExitKind;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct ExitKindDepthView {
    pub kind: ExitKind,
    pub depth: Option<usize>,
}

impl ExitKindDepthView {
    pub fn from_facts_exit_kind(kind: ExitKindFacts) -> Option<Self> {
        match kind {
            ExitKindFacts::Return => Some(Self {
                kind: ExitKind::Return,
                depth: None,
            }),
            ExitKindFacts::Break => Some(Self {
                kind: ExitKind::Break { depth: 1 },
                depth: None,
            }),
            ExitKindFacts::Continue => Some(Self {
                kind: ExitKind::Continue { depth: 1 },
                depth: None,
            }),
            ExitKindFacts::Unwind => None,
        }
    }

    pub fn from_recipe_exit_kind(kind: ExitKind) -> Self {
        let depth = match kind {
            ExitKind::Break { depth } | ExitKind::Continue { depth } => Some(depth as usize),
            ExitKind::Return => None,
        };
        Self { kind, depth }
    }
}
