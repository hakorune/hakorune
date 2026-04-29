//! ExitKind depth adapter (analysis-only).

use crate::mir::builder::control_flow::plan::recipe_tree::ExitKind;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct ExitKindDepthView {
    pub kind: ExitKind,
}

impl ExitKindDepthView {
    pub fn from_recipe_exit_kind(kind: ExitKind) -> Self {
        Self { kind }
    }
}
