//! Read-only access and one-way constructor take of the existing New claim.
use super::*;

impl OrdinaryNewClaimCoreV1 {
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

    pub(crate) fn argument_rows(
        &self,
    ) -> Result<
        &[super::OrdinaryNewTrivialArgumentV1],
        &crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    > {
        self.argument_rows.as_deref()
    }

    /// Sealed owned `ArrayBox` field residences in declaration order.
    /// `Some` only on an `OwnedArrayFieldsNoHook` object whose every
    /// ArrayBox field proved its birth-side provider store; `None` there
    /// means unproven, never a plain teardown.
    pub(crate) fn array_children(&self) -> Option<&[CanonicalFieldRefV1]> {
        self.array_children.as_deref()
    }
}

impl OrdinaryNewAdmissionClaimV1 {
    pub(crate) fn object(&self) -> CanonicalObjectIdV1 {
        self.core.object()
    }

    pub(crate) fn destruction(&self) -> ObjectDestructionDispositionV1 {
        self.core.destruction()
    }

    pub(crate) fn construction(&self) -> &ConstructionEligibilityV1 {
        self.core.construction()
    }

    pub(crate) fn box_source(&self) -> &crate::parser::ParserOrdinaryBoxSourceRowV1 {
        self.core.box_source()
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        self.core.site()
    }

    pub(crate) fn class(&self) -> &str {
        self.core.class()
    }

    pub(crate) const fn arity(&self) -> usize {
        self.core.arity()
    }

    pub(crate) fn constructor(self) -> OrdinaryNewConstructorDispositionV1 {
        self.core.constructor()
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
        self.core.argument_rows()
    }

    pub(crate) fn array_children(&self) -> Option<&[CanonicalFieldRefV1]> {
        self.core.array_children()
    }
}

impl OrdinaryNewResultClaimV1 {
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        self.core.site()
    }

    pub(crate) fn class(&self) -> &str {
        self.core.class()
    }

    pub(crate) const fn arity(&self) -> usize {
        self.core.arity()
    }

    pub(crate) fn object(&self) -> CanonicalObjectIdV1 {
        self.core.object()
    }

    pub(crate) fn construction(&self) -> &ConstructionEligibilityV1 {
        self.core.construction()
    }

    pub(crate) fn box_source(&self) -> &crate::parser::ParserOrdinaryBoxSourceRowV1 {
        self.core.box_source()
    }

    pub(crate) fn constructor(self) -> OrdinaryNewConstructorDispositionV1 {
        self.core.constructor()
    }

    pub(crate) fn home_prefix(&self) -> Result<&ResultNewHomePrefixV1, &HomePrefixUnavailableV1> {
        self.home_prefix.as_ref()
    }

    pub(crate) fn argument_rows(
        &self,
    ) -> Result<
        &[super::OrdinaryNewTrivialArgumentV1],
        &crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    > {
        self.core.argument_rows()
    }
}
