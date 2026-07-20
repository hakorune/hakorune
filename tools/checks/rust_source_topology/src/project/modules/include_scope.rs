//! INCLUDE-SCOPE0's disconnected two-lane include-identity vocabulary.
//!
//! The legacy `include_macro_ambiguity` bit conflates ordinary module-local
//! imports with textual `macro_rules!` visibility.  This product deliberately
//! models them independently, but has no production consumer until P0/I0.

#![allow(dead_code)] // INCLUDE-SCOPE0-S0 intentionally has zero production consumers.

use crate::SourceRangeV1;

/// Syntax evidence inside one already-selected source occurrence.
///
/// A range alone is not a global source-site identity.  The future scope
/// scanner seals it beneath the exact source occurrence that it scans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IncludeScopeSyntaxEvidenceV1 {
    pub(super) source_range: SourceRangeV1,
}

/// Order-independent ordinary-name state for exactly one module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ModuleLocalIncludeNameLaneV1 {
    BuiltinUnambiguous,
    PotentiallyShadowed(IncludeScopeSyntaxEvidenceV1),
}

/// Source-order textual visibility of a `macro_rules! include` definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TextualIncludeMacroLaneV1 {
    BuiltinVisible,
    UserMacroVisible(IncludeScopeSyntaxEvidenceV1),
}

/// The two independent scope lanes relevant to literal `include!` identity.
///
/// `child_module_entry` resets ordinary module-local name state while carrying
/// textual macro visibility forward.  Same-module included text will later
/// continue with this exact product instead of constructing a child scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct IncludeScopeLanesV1 {
    module_local: ModuleLocalIncludeNameLaneV1,
    textual: TextualIncludeMacroLaneV1,
}

impl IncludeScopeLanesV1 {
    pub(super) const fn root() -> Self {
        Self {
            module_local: ModuleLocalIncludeNameLaneV1::BuiltinUnambiguous,
            textual: TextualIncludeMacroLaneV1::BuiltinVisible,
        }
    }

    pub(super) fn child_module_entry(&self) -> Self {
        Self {
            module_local: ModuleLocalIncludeNameLaneV1::BuiltinUnambiguous,
            textual: self.textual.clone(),
        }
    }

    pub(super) fn with_module_local_shadow(
        mut self,
        evidence: IncludeScopeSyntaxEvidenceV1,
    ) -> Self {
        self.module_local = ModuleLocalIncludeNameLaneV1::PotentiallyShadowed(evidence);
        self
    }

    pub(super) fn with_textual_macro(mut self, evidence: IncludeScopeSyntaxEvidenceV1) -> Self {
        self.textual = TextualIncludeMacroLaneV1::UserMacroVisible(evidence);
        self
    }

    pub(super) const fn module_local(&self) -> &ModuleLocalIncludeNameLaneV1 {
        &self.module_local
    }

    pub(super) const fn textual(&self) -> &TextualIncludeMacroLaneV1 {
        &self.textual
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PositionV1;

    fn evidence(byte_start: usize) -> IncludeScopeSyntaxEvidenceV1 {
        IncludeScopeSyntaxEvidenceV1 {
            source_range: SourceRangeV1 {
                start: PositionV1 {
                    line: 1,
                    column: byte_start,
                },
                end: PositionV1 {
                    line: 1,
                    column: byte_start + 1,
                },
                byte_start,
                byte_end: byte_start + 1,
            },
        }
    }

    #[test]
    fn child_module_resets_module_local_lane_but_inherits_textual_lane() {
        let parent = IncludeScopeLanesV1::root()
            .with_module_local_shadow(evidence(3))
            .with_textual_macro(evidence(7));

        let child = parent.child_module_entry();
        assert!(matches!(
            child.module_local(),
            ModuleLocalIncludeNameLaneV1::BuiltinUnambiguous
        ));
        assert_eq!(child.textual(), parent.textual());
    }

    #[test]
    fn root_starts_with_two_independent_builtin_lanes() {
        let root = IncludeScopeLanesV1::root();
        assert!(matches!(
            root.module_local(),
            ModuleLocalIncludeNameLaneV1::BuiltinUnambiguous
        ));
        assert!(matches!(
            root.textual(),
            TextualIncludeMacroLaneV1::BuiltinVisible
        ));
    }
}
