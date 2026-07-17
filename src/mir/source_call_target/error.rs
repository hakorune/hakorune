use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::ReservedQualifiedReceiverRouteV1;

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
    ArityOverflow {
        receiver: Box<str>,
        method: Box<str>,
    },
    EmptyReceiver,
    EmptyMethod {
        receiver: Box<str>,
    },
    CallerOutsideCatalog {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    DuplicateCallSite {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    ReservedReceiverRoute {
        receiver: Box<str>,
        route: ReservedQualifiedReceiverRouteV1,
    },
    DirectReceiverLexicallyShadowed {
        receiver: Box<str>,
    },
    TargetOutsideCatalog {
        receiver: Box<str>,
        canonical_owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CurrentOwnerStaticCallTargetErrorV1 {
    SourceMethodCallRequired,
    CanonicalMeReceiverRequired,
    EmptyMethod,
    ArityOverflow {
        method: Box<str>,
    },
    CallerOutsideCatalog {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    CallerNotStaticBoxMethod {
        caller: CanonicalSameModuleCallableKeyV1,
    },
    DuplicateCallSite {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    TargetOutsideCatalog {
        owner: Box<str>,
        method: Box<str>,
        arity: u32,
    },
}
