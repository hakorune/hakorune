//! Parser-owned transport for the first normal source-plan handoff.
//!
//! This is deliberately not a semantic plan or a `Verified*` product.  It
//! keeps the parser relations that the postpass already owns together while
//! the initial callable source borrows only the projected placement set.

use super::static_box_source::PreparedParserStaticBoxParentSourceV1;
use crate::parser::build_cfg::program_item_slots::ProjectedProgramItemSlotSetV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserNormalSourcePlanSeedIssueV1 {
    MissingProjectedProgramSlots,
    ForeignParserRelation,
    DuplicateStaticParentRelation,
}

/// One parser-invocation source relation.  The full prepared static-parent
/// rows have one owner here; the narrow static seal remains a separate
/// projection and must not retain this payload.
#[derive(Debug)]
pub(in crate::parser) struct ParserNormalSourcePlanSeedV1 {
    projected_program_slots: ProjectedProgramItemSlotSetV1,
    static_parent_sources: Box<[PreparedParserStaticBoxParentSourceV1]>,
}

impl ParserNormalSourcePlanSeedV1 {
    pub(in crate::parser) fn issue(
        projected_program_slots: Option<ProjectedProgramItemSlotSetV1>,
        static_parent_sources: Vec<PreparedParserStaticBoxParentSourceV1>,
    ) -> Result<Self, ParserNormalSourcePlanSeedIssueV1> {
        let projected_program_slots = projected_program_slots
            .ok_or(ParserNormalSourcePlanSeedIssueV1::MissingProjectedProgramSlots)?;
        let brand = projected_program_slots.brand();
        for (index, parent) in static_parent_sources.iter().enumerate() {
            if !parent.box_site().path().brand().same_as(brand) {
                return Err(ParserNormalSourcePlanSeedIssueV1::ForeignParserRelation);
            }
            if static_parent_sources[..index]
                .iter()
                .any(|previous| previous.box_site().path() == parent.box_site().path())
            {
                return Err(ParserNormalSourcePlanSeedIssueV1::DuplicateStaticParentRelation);
            }
        }
        Ok(Self {
            projected_program_slots,
            static_parent_sources: static_parent_sources.into_boxed_slice(),
        })
    }

    pub(in crate::parser) fn projected_program_slots(&self) -> &ProjectedProgramItemSlotSetV1 {
        &self.projected_program_slots
    }

    #[cfg(test)]
    pub(in crate::parser) fn static_parent_sources(
        &self,
    ) -> &[PreparedParserStaticBoxParentSourceV1] {
        &self.static_parent_sources
    }

    /// Explicit terminal for callers that intentionally choose an AST-only
    /// or compatibility projection before the future source-plan consumer is
    /// connected. This terminal intentionally discards the unconnected seed.
    pub(in crate::parser) fn discard_unconnected(self) {
        drop(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_projected_slots_are_not_an_empty_seed() {
        assert_eq!(
            ParserNormalSourcePlanSeedV1::issue(None, Vec::new()).unwrap_err(),
            ParserNormalSourcePlanSeedIssueV1::MissingProjectedProgramSlots
        );
    }
}

/// The completed postpass keeps this disposition total without using an
/// optional seed.  Compatibility is an explicit terminal for this staged
/// ordinary-only cell; it is not a missing source-plan row.
#[derive(Debug)]
pub(in crate::parser) enum ParserNormalSourcePlanSeedDispositionV1 {
    Ready(ParserNormalSourcePlanSeedV1),
    CompatibilityOutside,
}

impl ParserNormalSourcePlanSeedDispositionV1 {
    pub(in crate::parser) fn discard_unconnected(self) {
        match self {
            Self::Ready(seed) => seed.discard_unconnected(),
            Self::CompatibilityOutside => {}
        }
    }
}
