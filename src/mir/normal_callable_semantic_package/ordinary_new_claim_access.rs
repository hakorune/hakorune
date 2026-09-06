//! Read-only access and one-way constructor take of the existing New claim.
use super::*;

impl OrdinaryNewAdmissionClaimV1 {
    pub(crate) fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn destruction(&self) -> ObjectDestructionDispositionV1 {
        self.destruction
    }

    pub(crate) fn construction(&self) -> &ConstructionEligibilityV1 {
        &self.construction
    }

    pub(crate) fn box_source(&self) -> &crate::parser::ParserOrdinaryBoxSourceRowV1 {
        &self.box_source
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }

    pub(crate) fn class(&self) -> &str {
        &self.class
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) fn constructor(self) -> OrdinaryNewConstructorDispositionV1 {
        self.constructor
    }

    pub(crate) fn home_prefix(&self) -> Result<&CallerNewHomePrefixV1, &HomePrefixUnavailableV1> {
        self.home_prefix.as_ref()
    }

    pub(crate) fn argument_rows(
        &self,
    ) -> Result<
        &[super::OrdinaryNewTrivialArgumentV1],
        &crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    > {
        self.argument_rows.as_deref()
    }
}
