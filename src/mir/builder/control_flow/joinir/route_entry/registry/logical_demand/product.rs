//! Terminal pre-effect qualification vocabulary.

use super::roles::LogicalRoleSetV1;
use super::source::LoopSourceViewV1;
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopDemandRejectionV1 {
    SourceReceiptUnavailable,
    SourceReceiptMismatch,
    SourceFrameMismatch,
    SourceSlotUnavailable,
    RouteSourceTopologyUnavailable { route: LoopRouteId },
    PreEffectPhysicalAdmissibilityUnavailable { route: LoopRouteId },
    RoutePayloadUnavailable,
}

/// A selected logical demand. It intentionally does not implement `Clone`.
#[derive(Debug)]
pub(crate) struct VerifiedLoopRouteDemandV1<'src> {
    route: LoopRouteId,
    source: LoopSourceViewV1<'src>,
    roles: LogicalRoleSetV1,
}

#[derive(Debug)]
pub(crate) enum LoopQualificationDispositionV1<'src> {
    NoRoute,
    Rejected(LoopDemandRejectionV1),
    Ambiguous { routes: Box<[LoopRouteId]> },
    Selected(VerifiedLoopRouteDemandV1<'src>),
}

impl<'src> VerifiedLoopRouteDemandV1<'src> {
    pub(crate) fn new(
        route: LoopRouteId,
        source: LoopSourceViewV1<'src>,
        roles: LogicalRoleSetV1,
    ) -> Self {
        Self {
            route,
            source,
            roles,
        }
    }

    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }

    pub(crate) fn source(&self) -> LoopSourceViewV1<'src> {
        self.source
    }

    pub(crate) fn roles(&self) -> &LogicalRoleSetV1 {
        &self.roles
    }
}

#[cfg(test)]
mod tests {
    use super::{LoopDemandRejectionV1, LoopQualificationDispositionV1, VerifiedLoopRouteDemandV1};
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::logical_demand::roles::{
        LogicalRoleSetV1, LogicalRoleV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::logical_demand::source::LoopSourceViewV1;
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::plan::facts::loop_source_receipt::{
        LoopSourceReceiptV1, LoopSourceSlotV1,
    };

    #[test]
    fn disposition_is_typed_without_selector_or_retry() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        };
        let body = vec![];
        let receipt = LoopSourceReceiptV1::from_raw_loop(&condition, &body);
        let source = LoopSourceViewV1::try_new(&condition, &body, &receipt).expect("source");
        let roles = LogicalRoleSetV1::try_new(vec![LogicalRoleV1::LoopBinding].into_boxed_slice())
            .expect("roles");
        let selected = VerifiedLoopRouteDemandV1::new(LoopRouteId::LoopSimpleWhile, source, roles);

        assert_eq!(selected.route(), LoopRouteId::LoopSimpleWhile);
        assert!(selected
            .source()
            .demand(LoopSourceSlotV1::Condition)
            .is_ok());
        assert_eq!(selected.roles().ordered(), &[LogicalRoleV1::LoopBinding]);
        assert!(matches!(
            LoopQualificationDispositionV1::Selected(selected),
            LoopQualificationDispositionV1::Selected(_)
        ));
        assert!(matches!(
            LoopQualificationDispositionV1::NoRoute,
            LoopQualificationDispositionV1::NoRoute
        ));
        assert!(matches!(
            LoopQualificationDispositionV1::Rejected(
                LoopDemandRejectionV1::SourceReceiptUnavailable
            ),
            LoopQualificationDispositionV1::Rejected(
                LoopDemandRejectionV1::SourceReceiptUnavailable
            )
        ));
        assert!(matches!(
            LoopQualificationDispositionV1::Ambiguous {
                routes: vec![LoopRouteId::LoopSimpleWhile].into_boxed_slice(),
            },
            LoopQualificationDispositionV1::Ambiguous { .. }
        ));
    }
}
