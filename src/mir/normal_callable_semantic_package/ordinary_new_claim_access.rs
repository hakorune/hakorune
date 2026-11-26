//! Read-only access and one-way constructor take of the existing New claim.
use super::*;

impl OrdinaryNewClaimCoreV1 {
    pub(crate) fn execution_profile(&self) -> Result<NewExecutionProfileRefV1<'_>, ConstructionUnavailableV1> {
        new_execution_profile_v1(&self.construction, self.allocation_execution.as_ref(), self.object)
    }

    pub(crate) fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn destruction(&self) -> ObjectDestructionDispositionV1 {
        self.destruction
    }

    pub(crate) fn construction(&self) -> &ConstructionEligibilityV1 {
        &self.construction
    }

    pub(crate) fn construction_source(&self) -> &ConstructionSourceEligibilityV1 {
        &self.construction_source
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

    /// Sealed owned field residences in declaration order. `Some` only
    /// on an owned-field-disposition object whose every residence-capable
    /// field proved its birth-side provider store; `None` there means
    /// unproven, never a plain teardown.
    pub(crate) fn children(&self) -> Option<&[OwnedFieldChildV1]> {
        self.children.as_deref()
    }
}

impl OrdinaryNewAdmissionClaimV1 {
    pub(crate) fn core(&self) -> &OrdinaryNewClaimCoreV1 {
        &self.core
    }

    pub(crate) fn into_core(self) -> OrdinaryNewClaimCoreV1 {
        self.core
    }

    pub(crate) fn home_prefix(&self) -> Result<&CallerNewHomePrefixV1, &HomePrefixUnavailableV1> {
        self.home_prefix.as_ref()
    }
}

impl OrdinaryNewResultClaimV1 {
    pub(crate) fn core(&self) -> &OrdinaryNewClaimCoreV1 {
        &self.core
    }

    pub(crate) fn into_core(self) -> OrdinaryNewClaimCoreV1 {
        self.core
    }

    pub(crate) fn home_prefix(&self) -> Result<&ResultNewHomePrefixV1, &HomePrefixUnavailableV1> {
        self.home_prefix.as_ref()
    }
}
