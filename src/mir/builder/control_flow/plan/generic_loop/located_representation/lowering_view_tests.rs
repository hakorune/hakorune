//! R0-V0 disconnected proof for the lifetime-bound O0 lowering view.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::{
    LocatedLoopPlanExprInputV1, LocatedLoopPlanExpressionPortV1, LocatedLoopPlanStmtInputV1,
    LoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultLegacySourceViewV1,
};

use super::{
    LocatedGenericLoopRepresentationErrorV1, VerifiedLocatedGenericLoopBodyRepresentationV1,
    VerifiedLocatedGenericLoopLoweringModeV1, VerifiedLocatedRecipeItemLoweringViewV1,
};

fn located_loop<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
) -> (
    LocatedLoopPlanExpressionPortV1<'plan>,
    crate::mir::callable_result_representation::LegacyStmtInputV1<'plan>,
) {
    let caller = actual_parser_add_fixture::caller(plan);
    let view =
        VerifiedCallableResultLegacySourceViewV1::verify(plan, &caller).expect("source view");
    let root = view.root_body();
    let loop_root = view
        .body_stmt(&root, 4)
        .expect("actual Loop is function Body(4)");
    (LocatedLoopPlanExpressionPortV1::new(view), loop_root)
}

#[test]
fn bound_view_borrows_default_prefix_condition_and_cleanup() {
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::Default {
            return;
        }
        let plan = actual_parser_add_fixture::plan();
        let (port, loop_root) = located_loop(&plan);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("O0 representation");
        let bound = representation
            .bind_lowering_port(&port)
            .expect("same source port binds");

        let condition = bound.condition();
        assert!(matches!(
            condition,
            LocatedLoopPlanExprInputV1::BorrowedLocated(_)
        ));
        assert!(matches!(
            port.expr_syntax(&condition),
            ASTNode::BinaryOp { .. }
        ));
        let cleanup = bound.cleanup();
        assert!(matches!(
            cleanup,
            LocatedLoopPlanStmtInputV1::BorrowedLocated(_)
        ));
        assert!(matches!(
            port.stmt_syntax(&cleanup),
            ASTNode::Assignment { .. }
        ));

        let VerifiedLocatedGenericLoopLoweringModeV1::DirectRecipeOnly { body } = bound.mode()
        else {
            panic!("default representation must retain direct prefix")
        };
        assert_eq!(body.len(), 5);
        for index in 0..body.len() {
            assert!(matches!(
                body.statement(index),
                Some(LocatedLoopPlanStmtInputV1::BorrowedLocated(_))
            ));
        }
        assert!(body.statement(body.len()).is_none());
    });
}

#[test]
fn bound_view_retains_strict_items_and_wrapped_join_product() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::StrictPlannerRequired {
            return;
        }
        let plan = actual_parser_add_fixture::plan();
        let (port, loop_root) = located_loop(&plan);
        let representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("strict O0 representation");
        let bound = representation
            .bind_lowering_port(&port)
            .expect("same source port binds");
        let VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { root } = bound.mode()
        else {
            panic!("strict representation must retain ExitAllowed root")
        };
        assert_eq!(root.len(), 5);
        assert!(matches!(
            root.item(2),
            Some(VerifiedLocatedRecipeItemLoweringViewV1::ExplicitIfV2 {
                condition: LocatedLoopPlanExprInputV1::BorrowedLocated(_),
                else_body: None,
                ..
            })
        ));

        let Some(VerifiedLocatedRecipeItemLoweringViewV1::StmtWrappedJoinIf { bridge }) =
            root.item(4)
        else {
            panic!("strict ordinal 4 must retain wrapped Join")
        };
        assert!(matches!(
            bridge.condition(),
            LocatedLoopPlanExprInputV1::BorrowedLocated(_)
        ));
        assert_eq!(bridge.singleton_recipe().block.items.len(), 1);
        let singleton = bridge.singleton_root();
        assert_eq!(singleton.then_block().len(), 1);
        assert_eq!(singleton.else_block().expect("exact else block").len(), 1);
    });
}

#[test]
fn foreign_port_rejects_before_a_bound_view_is_published() {
    let primary = actual_parser_add_fixture::plan();
    let (primary_port, loop_root) = located_loop(&primary);
    let representation = VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(
        &primary_port,
        loop_root,
    )
    .expect("primary O0 representation");

    let foreign = actual_parser_add_fixture::plan();
    let (foreign_port, _) = located_loop(&foreign);
    assert!(matches!(
        representation.bind_lowering_port(&foreign_port),
        Err(LocatedGenericLoopRepresentationErrorV1::Port(_))
    ));
}
