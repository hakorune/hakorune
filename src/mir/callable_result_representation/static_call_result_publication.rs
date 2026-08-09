//! Source-only demand for the bounded static-call result publication bridge.
//!
//! This product deliberately stops before Builder/MIR.  It normalizes the
//! already sealed exact-i64 source requirement into one AST-free identity that
//! a later physical receipt consumer may borrow.  It does not contain a
//! `ValueId`, a `MirType`, or a callable name selector.

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::{VerifiedCallableResultCallSiteV1, VerifiedStaticExactI64RequirementV1};

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

/// Owned, source-bound activation handoff for one exact static result row.
///
/// This is deliberately smaller than the disconnected activation plan: the
/// declaration/target/result proofs are borrowed only while issuing it, and
/// no AST, Builder, ValueId, or catalog clone crosses the handoff boundary.
/// The caller scope owns its lifetime and consumes it at most once.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedStaticCallResultPublicationHandoffV1 {
    catalog_identity: usize,
    demand: VerifiedStaticCallResultPublicationDemandV1,
    required_i64_arguments: Box<[u32]>,
}

impl VerifiedStaticCallResultPublicationHandoffV1 {
    pub(crate) fn from_exact_i64_requirement(
        requirement: VerifiedStaticExactI64RequirementV1<'_, '_>,
    ) -> Self {
        let catalog_identity = requirement.catalog_identity();
        let required_i64_arguments = requirement
            .required_i64_arguments()
            .to_vec()
            .into_boxed_slice();
        let demand =
            VerifiedStaticCallResultPublicationDemandV1::from_exact_i64_requirement(requirement);
        Self {
            catalog_identity,
            demand,
            required_i64_arguments,
        }
    }

    pub(super) fn from_general_call_result(
        catalog_identity: usize,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
        result: &VerifiedCallableResultCallSiteV1<'_>,
    ) -> Option<Self> {
        let target = result.static_target_key()?.clone();
        Some(Self {
            catalog_identity,
            demand: VerifiedStaticCallResultPublicationDemandV1 {
                caller: caller.clone(),
                site: site.clone(),
                target,
                _seal: VerifiedStaticCallResultPublicationDemandSealV1,
            },
            required_i64_arguments: result.required_i64_arguments().to_vec().into_boxed_slice(),
        })
    }

    pub(crate) const fn catalog_identity(&self) -> usize {
        self.catalog_identity
    }

    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.demand.caller()
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        self.demand.site()
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.demand.target()
    }

    pub(crate) fn required_i64_arguments(&self) -> &[u32] {
        &self.required_i64_arguments
    }

    pub(crate) fn is_branded_by(
        &self,
        declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        self.catalog_identity == declarations as *const _ as usize
    }

    pub(crate) fn consume(self) -> (VerifiedStaticCallResultPublicationDemandV1, Box<[u32]>) {
        (self.demand, self.required_i64_arguments)
    }
}

#[cfg(test)]
impl VerifiedStaticCallResultPublicationHandoffV1 {
    fn from_test_parts(
        catalog_identity: usize,
        demand: VerifiedStaticCallResultPublicationDemandV1,
        required_i64_arguments: &[u32],
    ) -> Self {
        Self {
            catalog_identity,
            demand,
            required_i64_arguments: required_i64_arguments.to_vec().into_boxed_slice(),
        }
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

    #[test]
    fn handoff_is_owned_and_single_use_by_move() {
        let caller = key("StringHelpers", "int_to_str", 1);
        let target = key("StringHelpers", "to_i64", 1);
        let site = SourceExprSiteV1::from_node(
            crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ]),
        );
        let handoff = VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(
            17,
            VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
                caller.clone(),
                site.clone(),
                target.clone(),
            ),
            &[0, 2],
        );
        assert_eq!(handoff.catalog_identity(), 17);
        assert_eq!(handoff.caller(), &caller);
        assert_eq!(handoff.site(), &site);
        assert_eq!(handoff.target(), &target);
        assert_eq!(handoff.required_i64_arguments(), &[0, 2]);
        let (demand, ordinals) = handoff.consume();
        assert_eq!(demand.caller(), &caller);
        assert_eq!(ordinals.as_ref(), &[0, 2]);
    }
}
