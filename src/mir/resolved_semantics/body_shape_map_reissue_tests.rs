//! Test-only access to the private rows of a sealed inventory, for the tests of
//! the limited empty-Map re-issue that live beside its only caller.

use super::*;

impl VerifiedResolvedBodyShapeInventoryV1 {
    pub(crate) fn duplicate_for_test(&self) -> Self {
        Self {
            owner: self.owner,
            body_root: self.body_root.clone(),
            statements: self.statements.clone(),
            expressions: self.expressions.clone(),
            effects: self.effects.clone(),
            relations: self.relations.clone(),
            assignment_sources: self.assignment_sources.clone(),
        }
    }

    /// Forge a Map entry relation from `parent` to `child`.
    pub(crate) fn with_forged_entry_for_test(
        mut self,
        parent: &SourceExprSiteV1,
        child: &SourceExprSiteV1,
    ) -> Self {
        let forged = BodyShapeRelationV1 {
            parent: parent.node().clone(),
            role: SourcePathSegmentV1::EntryValue(0),
            child: child.clone(),
        };
        self.relations = self.relations.iter().cloned().chain([forged]).collect();
        self
    }
}
