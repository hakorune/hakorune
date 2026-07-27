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
}
