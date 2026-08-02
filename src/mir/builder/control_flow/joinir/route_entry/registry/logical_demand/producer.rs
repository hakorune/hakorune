//! Source-only, pre-effect disposition producer for the ordered loop registry.
//!
//! It consumes the existing selection exactly once. Only direct top-level
//! SimpleWhile source topology is borrowable in this pre-effect slice.

use super::product::{
    LoopDemandRejectionV1, LoopQualificationDispositionV1, VerifiedLoopRouteDemandV1,
};
use super::roles::{LogicalRoleSetV1, LogicalRoleV1};
use super::source::{LoopSourceViewErrorV1, LoopSourceViewV1};
use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::RecipeFirstRouteSelectionV1;
use crate::mir::builder::control_flow::plan::facts::loop_source_receipt::LoopSourceSlotV1;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;

pub(crate) fn qualify_selected_loop_route_v1<'src>(
    selection: &RecipeFirstRouteSelectionV1,
    source: Result<LoopSourceViewV1<'src>, LoopSourceViewErrorV1>,
    facts: &LoopFacts,
) -> LoopQualificationDispositionV1<'src> {
    let source = match source {
        Ok(source) => source,
        Err(error) => return LoopQualificationDispositionV1::Rejected(map_source_error(error)),
    };

    match selection.raw_execution_routes() {
        [] => LoopQualificationDispositionV1::NoRoute,
        [LoopRouteId::LoopSimpleWhile] => qualify_direct_simple_while(source, facts),
        [route] => reject_unlocated_route(*route),
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

fn qualify_direct_simple_while<'src>(
    source: LoopSourceViewV1<'src>,
    facts: &LoopFacts,
) -> LoopQualificationDispositionV1<'src> {
    let Some(simple_while) = facts.loop_simple_while() else {
        return reject_unlocated_route(LoopRouteId::LoopSimpleWhile);
    };
    let Some(topology) = simple_while.source_topology.as_ref() else {
        return reject_unlocated_route(LoopRouteId::LoopSimpleWhile);
    };
    if !topology.step().scope_box_children().is_empty() {
        return LoopQualificationDispositionV1::Rejected(
            LoopDemandRejectionV1::RouteSourceTopologyNotDirectlyBorrowable {
                route: LoopRouteId::LoopSimpleWhile,
            },
        );
    }

    let condition = match source.demand(LoopSourceSlotV1::Condition) {
        Ok(demand) => demand,
        Err(error) => return LoopQualificationDispositionV1::Rejected(map_source_error(error)),
    };
    let step = match source.demand(LoopSourceSlotV1::BodyStatement(
        topology.step().raw_body_index() as usize,
    )) {
        Ok(demand) => demand,
        Err(error) => return LoopQualificationDispositionV1::Rejected(map_source_error(error)),
    };
    let roles = match LogicalRoleSetV1::try_new(
        vec![
            LogicalRoleV1::LoopBinding,
            LogicalRoleV1::LoopBackContinuation,
        ]
        .into_boxed_slice(),
    ) {
        Ok(roles) => roles,
        Err(_) => {
            return LoopQualificationDispositionV1::Rejected(
                LoopDemandRejectionV1::RoutePayloadUnavailable,
            )
        }
    };

    LoopQualificationDispositionV1::Selected(VerifiedLoopRouteDemandV1::direct_simple_while(
        condition,
        step,
        topology.clone(),
        simple_while.loop_var.clone(),
        roles,
    ))
}

fn reject_unlocated_route<'src>(route: LoopRouteId) -> LoopQualificationDispositionV1<'src> {
    LoopQualificationDispositionV1::Rejected(
        LoopDemandRejectionV1::RouteSourceTopologyUnavailable { route },
    )
}

#[cfg(test)]
mod tests {
    use super::qualify_selected_loop_route_v1;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::logical_demand::{
        product::{LoopDemandRejectionV1, LoopQualificationDispositionV1, LoopRoutePayloadV1},
        roles::LogicalRoleV1,
        source::LoopSourceViewV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::joinir::route_entry::registry::selection::RecipeFirstRouteSelectionV1;
    use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;
    use crate::mir::builder::control_flow::plan::facts::LoopFacts;

    const ALL_ROUTES: &[LoopRouteId] = &[
        LoopRouteId::LoopBreakRecipe,
        LoopRouteId::IfPhiJoin,
        LoopRouteId::LoopContinueOnly,
        LoopRouteId::LoopTrueEarlyExit,
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

    struct Fixture {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
        facts: LoopFacts,
    }

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn direct_fixture(scope_boxed: bool) -> Fixture {
        let condition = Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(literal(3)),
            span: Span::unknown(),
        });
        let increment = ASTNode::Assignment {
            target: Box::new(variable("i")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("i")),
                right: Box::new(literal(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let body = if scope_boxed {
            vec![ASTNode::ScopeBox {
                body: vec![increment],
                span: Span::unknown(),
            }]
        } else {
            vec![increment]
        };
        let facts = try_build_loop_facts(condition.as_ref(), &body)
            .expect("facts build")
            .expect("simple while facts");
        Fixture {
            condition,
            body,
            facts,
        }
    }

    fn accum_fixture() -> Fixture {
        let condition = Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("i")),
            right: Box::new(literal(3)),
            span: Span::unknown(),
        });
        let body = vec![
            ASTNode::Assignment {
                target: Box::new(variable("sum")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("sum")),
                    right: Box::new(literal(1)),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(variable("i")),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(literal(1)),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ];
        let facts = try_build_loop_facts(condition.as_ref(), &body)
            .expect("facts build")
            .expect("accum facts");
        Fixture {
            condition,
            body,
            facts,
        }
    }

    fn source(fixture: &Fixture) -> LoopSourceViewV1<'_> {
        LoopSourceViewV1::try_new(
            fixture.condition.as_ref(),
            &fixture.body,
            fixture.facts.source_receipt(),
        )
        .expect("source")
    }

    #[test]
    fn every_other_single_route_remains_a_pre_effect_topology_rejection() {
        let fixture = direct_fixture(false);
        for route in ALL_ROUTES {
            let selection = RecipeFirstRouteSelectionV1::selection_for_test(&[*route]);
            assert!(matches!(
                qualify_selected_loop_route_v1(&selection, Ok(source(&fixture)), &fixture.facts),
                LoopQualificationDispositionV1::Rejected(
                    LoopDemandRejectionV1::RouteSourceTopologyUnavailable { route: rejected }
                ) if rejected == *route
            ));
        }
    }

    #[test]
    fn direct_simple_while_selects_only_raw_condition_and_step() {
        let fixture = direct_fixture(false);
        let selection =
            RecipeFirstRouteSelectionV1::selection_for_test(&[LoopRouteId::LoopSimpleWhile]);

        let disposition =
            qualify_selected_loop_route_v1(&selection, Ok(source(&fixture)), &fixture.facts);
        let LoopQualificationDispositionV1::Selected(selected) = disposition else {
            panic!("direct SimpleWhile must select")
        };
        assert_eq!(selected.route(), LoopRouteId::LoopSimpleWhile);
        assert_eq!(
            selected.roles().ordered(),
            &[
                LogicalRoleV1::LoopBinding,
                LogicalRoleV1::LoopBackContinuation,
            ]
        );
        let LoopRoutePayloadV1::SimpleWhile(payload) = selected.payload();
        assert!(std::ptr::eq(
            payload.condition().node(),
            fixture.condition.as_ref()
        ));
        assert!(std::ptr::eq(payload.step().node(), &fixture.body[0]));
        assert_eq!(
            payload.step_topology().step().scope_box_children(),
            &[] as &[u32]
        );
        assert_eq!(payload.loop_binding(), "i");
    }

    #[test]
    fn scope_box_simple_while_remains_typed_not_directly_borrowable() {
        let fixture = direct_fixture(true);
        let selection =
            RecipeFirstRouteSelectionV1::selection_for_test(&[LoopRouteId::LoopSimpleWhile]);

        assert!(matches!(
            qualify_selected_loop_route_v1(&selection, Ok(source(&fixture)), &fixture.facts),
            LoopQualificationDispositionV1::Rejected(
                LoopDemandRejectionV1::RouteSourceTopologyNotDirectlyBorrowable {
                    route: LoopRouteId::LoopSimpleWhile
                }
            )
        ));
    }

    #[test]
    fn accum_topology_remains_rejected_before_its_selection_row() {
        let fixture = accum_fixture();
        let selection =
            RecipeFirstRouteSelectionV1::selection_for_test(&[LoopRouteId::AccumConstLoop]);

        assert!(fixture
            .facts
            .accum_const_loop()
            .is_some_and(|facts| facts.source_topology.is_some()));
        assert!(matches!(
            qualify_selected_loop_route_v1(&selection, Ok(source(&fixture)), &fixture.facts),
            LoopQualificationDispositionV1::Rejected(
                LoopDemandRejectionV1::RouteSourceTopologyUnavailable {
                    route: LoopRouteId::AccumConstLoop
                }
            )
        ));
    }

    #[test]
    fn foreign_same_length_source_frame_is_rejected() {
        let fixture = direct_fixture(false);
        let foreign = direct_fixture(false);
        let selection =
            RecipeFirstRouteSelectionV1::selection_for_test(&[LoopRouteId::LoopSimpleWhile]);
        let foreign_source = LoopSourceViewV1::try_new(
            foreign.condition.as_ref(),
            &foreign.body,
            fixture.facts.source_receipt(),
        );

        assert!(matches!(
            qualify_selected_loop_route_v1(&selection, foreign_source, &fixture.facts),
            LoopQualificationDispositionV1::Rejected(LoopDemandRejectionV1::SourceFrameMismatch)
        ));
    }

    #[test]
    fn empty_and_overlapping_selection_are_terminal_not_retry() {
        let fixture = direct_fixture(false);
        assert!(matches!(
            qualify_selected_loop_route_v1(
                &RecipeFirstRouteSelectionV1::selection_for_test(&[]),
                Ok(source(&fixture)),
                &fixture.facts,
            ),
            LoopQualificationDispositionV1::NoRoute
        ));
        assert!(matches!(
            qualify_selected_loop_route_v1(
                &RecipeFirstRouteSelectionV1::selection_for_test(&[
                    LoopRouteId::LoopSimpleWhile,
                    LoopRouteId::GenericLoopV1,
                ]),
                Ok(source(&fixture)),
                &fixture.facts,
            ),
            LoopQualificationDispositionV1::Ambiguous { routes }
                if routes.as_ref() == [LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1]
        ));
    }
}
