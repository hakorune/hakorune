//! Source-only, pre-effect disposition producer for the ordered loop registry.
//!
//! It consumes the existing selection exactly once. Until facts retain
//! route-specific raw source topology, every exactly-one selection is a typed
//! rejection instead of a cloned-AST rematch or a post-effect `None` retry.

use super::product::{LoopDemandRejectionV1, LoopQualificationDispositionV1};
use super::source::{LoopSourceViewErrorV1, LoopSourceViewV1};
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::RecipeFirstRouteSelectionV1;

pub(crate) fn qualify_selected_loop_route_v1<'src>(
    selection: &RecipeFirstRouteSelectionV1,
    source: Result<LoopSourceViewV1<'src>, LoopSourceViewErrorV1>,
) -> LoopQualificationDispositionV1<'src> {
    let source = match source {
        Ok(source) => source,
        Err(error) => return LoopQualificationDispositionV1::Rejected(map_source_error(error)),
    };

    match selection.raw_execution_routes() {
        [] => LoopQualificationDispositionV1::NoRoute,
        [route] => reject_unlocated_route(*route, source),
        routes => LoopQualificationDispositionV1::Ambiguous {
            routes: routes.into(),
        },
    }
}

fn map_source_error(error: LoopSourceViewErrorV1) -> LoopDemandRejectionV1 {
    match error {
        LoopSourceViewErrorV1::ReceiptUnavailable => {
            LoopDemandRejectionV1::SourceReceiptUnavailable
        }
        LoopSourceViewErrorV1::ReceiptBodyLengthMismatch => {
            LoopDemandRejectionV1::SourceReceiptMismatch
        }
        LoopSourceViewErrorV1::SourceFrameMismatch => LoopDemandRejectionV1::SourceFrameMismatch,
        LoopSourceViewErrorV1::SlotOutOfBounds => LoopDemandRejectionV1::SourceSlotUnavailable,
    }
}

fn reject_unlocated_route<'src>(
    route: crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId,
    _source: LoopSourceViewV1<'src>,
) -> LoopQualificationDispositionV1<'src> {
    LoopQualificationDispositionV1::Rejected(
        LoopDemandRejectionV1::RouteSourceTopologyUnavailable { route },
    )
}

#[cfg(test)]
mod tests {
    use super::qualify_selected_loop_route_v1;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::logical_demand::{
        product::{LoopDemandRejectionV1, LoopQualificationDispositionV1},
        source::LoopSourceViewV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::RecipeFirstRouteSelectionV1;
    use crate::mir::builder::control_flow::plan::facts::loop_source_receipt::LoopSourceReceiptV1;

    const ALL_ROUTES: &[LoopRouteId] = &[
        LoopRouteId::LoopBreakRecipe,
        LoopRouteId::IfPhiJoin,
        LoopRouteId::LoopContinueOnly,
        LoopRouteId::LoopTrueEarlyExit,
        LoopRouteId::LoopSimpleWhile,
        LoopRouteId::LoopCharMap,
        LoopRouteId::LoopArrayJoin,
        LoopRouteId::ScanWithInit,
        LoopRouteId::SplitScan,
        LoopRouteId::BoolPredicateScan,
        LoopRouteId::AccumConstLoop,
        LoopRouteId::NestedLoopMinimal,
        LoopRouteId::LoopTrueBreakContinue,
        LoopRouteId::LoopCondBreakContinue,
        LoopRouteId::LoopCondContinueOnly,
        LoopRouteId::LoopCondContinueWithReturn,
        LoopRouteId::LoopCondReturnInBody,
        LoopRouteId::GenericLoopV0,
        LoopRouteId::GenericLoopV1,
    ];

    fn source() -> LoopSourceViewV1<'static> {
        let condition = Box::leak(Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }));
        let body = Box::leak(Box::<[ASTNode]>::default());
        let receipt = Box::leak(Box::new(LoopSourceReceiptV1::from_raw_loop(
            condition, body,
        )));
        LoopSourceViewV1::try_new(condition, body, receipt).expect("source")
    }

    #[test]
    fn every_single_route_is_a_pre_effect_topology_rejection_until_located() {
        for route in ALL_ROUTES {
            let selection = RecipeFirstRouteSelectionV1::selection_for_test(&[*route]);
            assert!(matches!(
                qualify_selected_loop_route_v1(&selection, Ok(source())),
                LoopQualificationDispositionV1::Rejected(
                    LoopDemandRejectionV1::RouteSourceTopologyUnavailable { route: rejected }
                ) if rejected == *route
            ));
        }
    }

    #[test]
    fn empty_and_overlapping_selection_are_terminal_not_retry() {
        assert!(matches!(
            qualify_selected_loop_route_v1(
                &RecipeFirstRouteSelectionV1::selection_for_test(&[]),
                Ok(source()),
            ),
            LoopQualificationDispositionV1::NoRoute
        ));
        assert!(matches!(
            qualify_selected_loop_route_v1(
                &RecipeFirstRouteSelectionV1::selection_for_test(&[
                    LoopRouteId::LoopSimpleWhile,
                    LoopRouteId::GenericLoopV1,
                ]),
                Ok(source()),
            ),
            LoopQualificationDispositionV1::Ambiguous { routes }
                if routes.as_ref() == [LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1]
        ));
    }
}
