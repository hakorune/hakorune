//! O0-P0 product proof over the shared actual ParserBox fixture.

use crate::mir::builder::control_flow::generic_loop_canon::StepPlacement;
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopCarrierRoleV1, GenericLoopV1StepDispositionV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::product::{
    VerifiedLocatedGenericLoopBodyModeV1, VerifiedLocatedRecipeItemV1,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{IfContractKind, RecipeItem};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::policies::BodyLoweringPolicy;
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourcePathSegmentV1};

use super::{LocatedLoopPlanExpressionPortV1, VerifiedLocatedGenericLoopBodyRepresentationV1};

#[test]
fn actual_parser_loop_seals_default_and_strict_representations() {
    crate::runtime::ring0::ensure_global_ring0_initialized();

    with_default_and_strict_modes(|mode| {
        let plan = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&plan);
        let view =
            VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).expect("source view");
        let root_body = view.root_body();
        let loop_root = view
            .body_stmt(&root_body, 4)
            .expect("actual Loop is function Body(4)");
        let port = LocatedLoopPlanExpressionPortV1::new(view);
        let sealed =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, loop_root)
                .expect("actual located GenericLoop representation seals");

        assert_eq!(
            sealed.extraction.facts().carrier_role,
            GenericLoopCarrierRoleV1::NumericProgression
        );
        assert!(matches!(
            sealed.extraction.step(),
            GenericLoopV1StepDispositionV1::NumericProgression {
                placement: StepPlacement::Last,
                canonical_body_len: 6,
            }
        ));
        assert_expr_path(
            &sealed.condition,
            &[
                SourcePathSegmentV1::Body(4),
                SourcePathSegmentV1::LoopCondition,
            ],
        );

        match mode {
            GenericLoopTestModeV1::Default => assert_default_mode(&port, &sealed),
            GenericLoopTestModeV1::StrictPlannerRequired => assert_strict_mode(&port, &sealed),
        }
    });
}

fn assert_default_mode(
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    sealed: &VerifiedLocatedGenericLoopBodyRepresentationV1<'_>,
) {
    assert_eq!(
        sealed.extraction.facts().body_lowering_policy,
        BodyLoweringPolicy::RecipeOnly
    );
    assert!(sealed.extraction.facts().body_no_exit.is_none());
    let VerifiedLocatedGenericLoopBodyModeV1::DirectRecipeOnly { prefix, cleanup } = &sealed.mode
    else {
        panic!("default mode must retain the exact direct source prefix")
    };
    assert_eq!(prefix.len(), 5);
    assert_prefix_expr_path(
        port,
        &prefix[0],
        ExprChildRoleV1::LocalInitializer(0),
        0,
        SourcePathSegmentV1::Initializer(0),
    );
    assert_prefix_expr_path(
        port,
        &prefix[1],
        ExprChildRoleV1::LocalInitializer(0),
        1,
        SourcePathSegmentV1::Initializer(0),
    );
    assert_prefix_expr_path(
        port,
        &prefix[2],
        ExprChildRoleV1::IfCondition,
        2,
        SourcePathSegmentV1::IfCondition,
    );
    assert_prefix_expr_path(
        port,
        &prefix[3],
        ExprChildRoleV1::LocalInitializer(0),
        3,
        SourcePathSegmentV1::Initializer(0),
    );
    assert_prefix_expr_path(
        port,
        &prefix[4],
        ExprChildRoleV1::IfCondition,
        4,
        SourcePathSegmentV1::IfCondition,
    );
    assert_cleanup_site(port, cleanup);
}

fn assert_strict_mode(
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    sealed: &VerifiedLocatedGenericLoopBodyRepresentationV1<'_>,
) {
    assert!(matches!(
        sealed.extraction.facts().body_lowering_policy,
        BodyLoweringPolicy::ExitAllowed { .. }
    ));
    let VerifiedLocatedGenericLoopBodyModeV1::ExitAllowedRecipe { root, cleanup } = &sealed.mode
    else {
        panic!("strict mode must retain the verified ExitAllowed recipe")
    };
    assert_cleanup_site(port, cleanup);
    assert_eq!(root.items.len(), 5);
    assert!(matches!(
        root.items[0],
        VerifiedLocatedRecipeItemV1::OpaqueStmt { .. }
    ));
    assert!(matches!(
        root.items[1],
        VerifiedLocatedRecipeItemV1::OpaqueStmt { .. }
    ));
    assert!(matches!(
        root.items[3],
        VerifiedLocatedRecipeItemV1::OpaqueStmt { .. }
    ));

    let VerifiedLocatedRecipeItemV1::ExplicitIfV2 {
        condition,
        then_body,
        else_body,
        contract,
        ..
    } = &root.items[2]
    else {
        panic!("strict ordinal 2 must be explicit IfV2")
    };
    assert!(matches!(contract, IfContractKind::ExitOnly { .. }));
    assert!(else_body.is_none());
    assert_expr_path(
        condition,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(2),
            SourcePathSegmentV1::IfCondition,
        ],
    );
    let then_return = port
        .exact_body_stmt(then_body, 0)
        .expect("strict ordinal 2 then branch retains exact Return");
    let then_value = port
        .exact_child_expr_from_stmt(&then_return, ExprChildRoleV1::ReturnValue)
        .expect("strict ordinal 2 Return retains exact value");
    assert_expr_path(
        &then_value,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(2),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Value,
        ],
    );

    let VerifiedLocatedRecipeItemV1::StmtWrappedJoinIf { bridge } = &root.items[4] else {
        panic!("strict ordinal 4 must own the StmtWrappedJoinIf bridge")
    };
    assert!(bridge.else_body.is_some());
    assert_expr_path(
        &bridge.condition,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(4),
            SourcePathSegmentV1::IfCondition,
        ],
    );
    assert_eq!(bridge.singleton_recipe.block.items.len(), 1);
    assert!(matches!(
        bridge.singleton_recipe.block.items.first(),
        Some(RecipeItem::IfV2 {
            if_stmt,
            contract: IfContractKind::Join,
            ..
        }) if if_stmt.index() == 0
    ));
    assert_eq!(bridge.singleton_root.then_block.items.len(), 1);
    assert!(bridge.singleton_root.else_block.is_some());
    assert_eq!(
        bridge
            .singleton_root
            .else_block
            .as_ref()
            .expect("singleton Join retains sealed else block")
            .items
            .len(),
        1
    );

    let join_then = port
        .exact_body_stmt(&bridge.then_body, 0)
        .expect("wrapped Join then branch retains exact Assignment");
    let join_then_value = port
        .exact_child_expr_from_stmt(&join_then, ExprChildRoleV1::AssignmentValue)
        .expect("wrapped Join then Assignment retains exact value");
    assert_expr_path(
        &join_then_value,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(4),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Value,
        ],
    );
    let join_else = port
        .exact_body_stmt(
            bridge
                .else_body
                .as_ref()
                .expect("wrapped Join retains exact else body"),
            0,
        )
        .expect("wrapped Join else branch retains exact Assignment");
    let join_else_value = port
        .exact_child_expr_from_stmt(&join_else, ExprChildRoleV1::AssignmentValue)
        .expect("wrapped Join else Assignment retains exact value");
    assert_expr_path(
        &join_else_value,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(4),
            SourcePathSegmentV1::IfElse(0),
            SourcePathSegmentV1::Value,
        ],
    );
}

fn assert_prefix_expr_path(
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    statement: &crate::mir::callable_result_representation::LegacyStmtInputV1<'_>,
    role: ExprChildRoleV1,
    ordinal: u32,
    child: SourcePathSegmentV1,
) {
    let expression = port
        .exact_child_expr_from_stmt(statement, role)
        .expect("direct prefix statement retains its exact expression");
    assert_expr_path(
        &expression,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(ordinal),
            child,
        ],
    );
}

fn assert_cleanup_site(
    port: &LocatedLoopPlanExpressionPortV1<'_>,
    cleanup: &crate::mir::callable_result_representation::LegacyStmtInputV1<'_>,
) {
    let cleanup_value = port
        .exact_child_expr_from_stmt(cleanup, ExprChildRoleV1::AssignmentValue)
        .expect("cleanup assignment value is exact");
    assert_eq!(
        cleanup_value
            .activation_site()
            .expect("cleanup expression is located")
            .1,
        &actual_parser_add_fixture::selected_static_sites()[1]
    );
}

fn assert_expr_path(
    expr: &crate::mir::callable_result_representation::LegacyExprInputV1<'_>,
    expected: &[SourcePathSegmentV1],
) {
    assert_eq!(
        expr.activation_site()
            .expect("expression is located")
            .1
            .node()
            .segments(),
        expected
    );
}
