//! Source-only demand for the bounded static-call result publication bridge.
//!
//! This product deliberately stops before Builder/MIR.  It normalizes the
//! already sealed exact-i64 source requirement into one AST-free identity that
//! a later physical receipt consumer may borrow.  It does not contain a
//! `ValueId`, a `MirType`, or a callable name selector.

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::VerifiedStaticExactI64RequirementV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedStaticCallResultPublicationDemandV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    _seal: VerifiedStaticCallResultPublicationDemandSealV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerifiedStaticCallResultPublicationDemandSealV1;

impl VerifiedStaticCallResultPublicationDemandV1 {
    pub(crate) fn from_exact_i64_requirement(
        requirement: VerifiedStaticExactI64RequirementV1<'_, '_>,
    ) -> Self {
        Self {
            caller: requirement.caller().clone(),
            site: requirement.site().clone(),
            target: requirement.target().clone(),
            _seal: VerifiedStaticCallResultPublicationDemandSealV1,
        }
    }

    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

#[cfg(test)]
impl VerifiedStaticCallResultPublicationDemandV1 {
    pub(crate) fn from_test_parts(
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    ) -> Self {
        Self {
            caller,
            site,
            target,
            _seal: VerifiedStaticCallResultPublicationDemandSealV1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::SourcePathSegmentV1;

    fn key(owner: &str, name: &str, arity: usize) -> CanonicalSameModuleCallableKeyV1 {
        CanonicalSameModuleCallableKeyV1::test_static_box_method(owner, name, arity)
    }

    #[test]
    fn source_demand_retains_only_sealed_identity() {
        let caller = key("StringHelpers", "int_to_str", 1);
        let target = key("StringHelpers", "to_i64", 1);
        let site = SourceExprSiteV1::from_node(
            crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ]),
        );
        let demand = VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
            caller.clone(),
            site.clone(),
            target.clone(),
        );
        assert_eq!(demand.caller(), &caller);
        assert_eq!(demand.site(), &site);
        assert_eq!(demand.target(), &target);
    }
}
