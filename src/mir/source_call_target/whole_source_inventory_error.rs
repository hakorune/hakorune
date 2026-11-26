use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::resolved_semantics::{ShadowResolveErrorV0, SourceExprSiteV1};

use super::{
    CurrentOwnerStaticCallTargetErrorV1, QualifiedCallRouteFactsErrorV1,
    QualifiedReceiverLexicalDispositionErrorV1, QualifiedStaticCallTargetErrorV1,
    SourceMethodCallSiteErrorV1,
};

#[derive(Debug)]
pub(crate) enum WholeSourceStaticCallTargetInventoryErrorV1 {
    ImportCatalogMismatch,
    MethodCallObservation {
        caller: CanonicalSameModuleCallableKeyV1,
        cause: ShadowResolveErrorV0,
    },
    MethodCallSite(SourceMethodCallSiteErrorV1),
    ObservationReceiverSiteMismatch {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    DuplicateObservedCall {
        caller: CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
    },
    QualifiedLexical(QualifiedReceiverLexicalDispositionErrorV1),
    QualifiedRoute(QualifiedCallRouteFactsErrorV1),
    QualifiedTarget(QualifiedStaticCallTargetErrorV1),
    CurrentOwnerTarget(CurrentOwnerStaticCallTargetErrorV1),
    StaticArityMismatch(super::error::StaticCallArityMismatchV1),
}

impl WholeSourceStaticCallTargetInventoryErrorV1 {
    pub(crate) fn call_diagnostic(&self) -> Option<String> {
        match self {
            Self::MethodCallObservation { cause, .. } => cause.call_diagnostic(),
            Self::StaticArityMismatch(issue)
            | Self::CurrentOwnerTarget(CurrentOwnerStaticCallTargetErrorV1::ArityMismatch(issue)) =>
                Some(format!(
                    "[call/arity-mismatch] {}.{} expects {:?} arguments, got {}; call in {}.{} at {:?}",
                    issue.target_owner,
                    issue.selector,
                    issue.expected,
                    issue.actual,
                    issue.caller.owner(),
                    issue.caller.name(),
                    issue.site.node()
                )),
            Self::CurrentOwnerTarget(CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
                owner, method, arity, site,
            }) => Some(format!("[call/target-missing] no declaration for {owner}.{method}/{arity} at {:?}", site.node())),
            _ => None,
        }
    }
}
