//! Terminal pre-effect qualification vocabulary.

use super::roles::LogicalRoleSetV1;
use super::source::LoopSourceDemandV1;
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::plan::facts::loop_simple_while_facts::LoopSimpleWhileSourceTopologyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopDemandRejectionV1 {
    SourceReceiptUnavailable,
    SourceReceiptMismatch,
    SourceFrameMismatch,
    SourceSlotUnavailable,
    RouteSourceTopologyUnavailable { route: LoopRouteId },
    RouteSourceTopologyNotDirectlyBorrowable { route: LoopRouteId },
    PreEffectPhysicalAdmissibilityUnavailable { route: LoopRouteId },
    RoutePayloadUnavailable,
}

/// A selected logical demand. It intentionally does not implement `Clone`.
#[derive(Debug)]
pub(crate) struct VerifiedLoopRouteDemandV1<'src> {
    route: LoopRouteId,
    payload: LoopRoutePayloadV1<'src>,
    roles: LogicalRoleSetV1,
}

/// Route-local raw borrows issued by the pre-effect logical producer.
///
/// This intentionally exposes no general source view: a later physicalizer
/// may consume only the source locations the selected route proved.
#[derive(Debug)]
pub(crate) enum LoopRoutePayloadV1<'src> {
    SimpleWhile(VerifiedSimpleWhileDemandV1<'src>),
}

#[derive(Debug)]
pub(crate) struct VerifiedSimpleWhileDemandV1<'src> {
    condition: LoopSourceDemandV1<'src>,
    step: LoopSourceDemandV1<'src>,
    step_topology: LoopSimpleWhileSourceTopologyV1,
    loop_binding: String,
}

#[derive(Debug)]
pub(crate) enum LoopQualificationDispositionV1<'src> {
    NoRoute,
    Rejected(LoopDemandRejectionV1),
    Ambiguous { routes: Box<[LoopRouteId]> },
    Selected(VerifiedLoopRouteDemandV1<'src>),
}

impl<'src> VerifiedLoopRouteDemandV1<'src> {
    pub(crate) fn direct_simple_while(
        condition: LoopSourceDemandV1<'src>,
        step: LoopSourceDemandV1<'src>,
        step_topology: LoopSimpleWhileSourceTopologyV1,
        loop_binding: String,
        roles: LogicalRoleSetV1,
    ) -> Self {
        Self {
            route: LoopRouteId::LoopSimpleWhile,
            payload: LoopRoutePayloadV1::SimpleWhile(VerifiedSimpleWhileDemandV1 {
                condition,
                step,
                step_topology,
                loop_binding,
            }),
            roles,
        }
    }

    pub(crate) fn route(&self) -> LoopRouteId {
        self.route
    }

    pub(crate) fn payload(&self) -> &LoopRoutePayloadV1<'src> {
        &self.payload
    }

    pub(crate) fn roles(&self) -> &LogicalRoleSetV1 {
        &self.roles
    }
}

impl<'src> VerifiedSimpleWhileDemandV1<'src> {
    pub(crate) fn condition(&self) -> LoopSourceDemandV1<'src> {
        self.condition
    }

    pub(crate) fn step(&self) -> LoopSourceDemandV1<'src> {
        self.step
    }

    pub(crate) fn step_topology(&self) -> &LoopSimpleWhileSourceTopologyV1 {
        &self.step_topology
    }

    pub(crate) fn loop_binding(&self) -> &str {
        &self.loop_binding
    }
}

#[cfg(test)]
mod tests {
    use super::{LoopDemandRejectionV1, LoopQualificationDispositionV1};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;

    #[test]
    fn disposition_is_typed_without_selector_or_retry() {
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
