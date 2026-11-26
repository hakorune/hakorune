use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{ShadowResolveErrorV0, SourceExprSiteV1};

use crate::mir::policies::source_method_reserved_route::{
    SourceMethodReservedRouteDispositionV1, SourceMethodReservedRouteFailureV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceMethodCallSiteErrorV1 {
    CallerOutsideCatalog {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    SiteOutsideCallerBody {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    SiteCrossesNestedCallableBoundary {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    MethodCallRequired {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    ArityOverflow {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        method: Box<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QualifiedReceiverLexicalDispositionErrorV1 {
    EmptyRequestSet,
    MixedCaller {
        expected: CanonicalSameModuleCallableKeyV1,
        actual: CanonicalSameModuleCallableKeyV1,
    },
    MixedCallerDeclaration {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    QualifiedReceiverVariableRequired {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    DuplicateReceiverSite {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    ShadowTraversal(ShadowResolveErrorV0),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StaticImportAliasViewErrorV1 {
    EmptyAlias,
    EmptyCanonicalOwner {
        alias: Box<str>,
    },
    DuplicateAlias {
        alias: Box<str>,
    },
    TargetOwnerOutsideCatalog {
        alias: Box<str>,
        canonical_owner: Box<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QualifiedStaticCallTargetErrorV1 {
    RouteFactCatalogMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    RouteFactImportViewMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    DuplicateCallSite {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    TargetOutsideCatalog {
        receiver: Box<str>,
        canonical_owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QualifiedCallRouteFactsErrorV1 {
    LexicalDispositionUnavailable {
        caller: CanonicalSameModuleCallableKeyV1,
        receiver_site: SourceExprSiteV1,
    },
    ImportCatalogMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    ReservedRouteSelected {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        disposition: SourceMethodReservedRouteDispositionV1,
    },
    ReservedRouteRejected {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        reason: SourceMethodReservedRouteFailureV1,
    },
    DirectReceiverLexicallyBound {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        receiver: Box<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StaticCallArityMismatchV1 {
    pub(super) caller: CanonicalSameModuleCallableKeyV1,
    pub(super) site: SourceExprSiteV1,
    pub(super) target_owner: Box<str>,
    pub(super) selector: Box<str>,
    pub(super) expected: Vec<u32>,
    pub(super) actual: u32,
}

impl StaticCallArityMismatchV1 {
    pub(super) fn for_call(
        call: &super::VerifiedSourceMethodCallSiteV1<'_>,
        target_owner: &str,
        expected: Vec<u32>,
    ) -> Self {
        Self {
            caller: call.caller().clone(),
            site: call.site().clone(),
            target_owner: target_owner.into(),
            selector: call.method().into(),
            expected,
            actual: call.arity(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CurrentOwnerStaticCallTargetErrorV1 {
    CallCatalogMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    CanonicalMeReceiverRequired {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    CallerNotStaticBoxMethod {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    DuplicateCallSite {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    ArityMismatch(StaticCallArityMismatchV1),
    TargetOutsideCatalog {
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
        site: SourceExprSiteV1,
    },
}
