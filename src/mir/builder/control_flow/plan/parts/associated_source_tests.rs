//! D0-S0 disconnected projection proof for the Parts associated-source port.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::expression_port::{
    LocatedLoopPlanExprInputV1, LocatedLoopPlanExpressionPortV1, LocatedLoopPlanStmtInputV1,
    LoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::facts::extract::test_support::{
    with_default_and_strict_modes, GenericLoopTestModeV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::{
    VerifiedLocatedGenericLoopBodyRepresentationV1, VerifiedLocatedGenericLoopLoweringModeV1,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    ExitKind, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::{refs::StmtRef, RecipeBody};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultLegacySourceViewV1,
};

use super::associated_source::{
    LocatedPartsAssociatedSourceV1, PartsAssociatedRecipeItemV1, PartsAssociatedSourceV1,
    RawPartsAssociatedSourceV1,
};

#[test]
fn raw_provider_projects_stmt_exit_and_explicit_if_without_lowering() {
    let local = local_statement("value");
    let returning = return_statement(7);
    let condition = comparison("value", 7);
    let if_statement = ASTNode::If {
        condition: Box::new(condition.clone()),
        then_body: vec![return_statement(8)],
        else_body: None,
        span: Span::unknown(),
    };

    let mut arena = RecipeBodies::new();
    let root_id = arena.register(RecipeBody::new(vec![
        local.clone(),
        returning.clone(),
        if_statement.clone(),
    ]));
    let then_id = arena.register(RecipeBody::new(vec![return_statement(8)]));
    let then_block = RecipeBlock::new(
        then_id,
        vec![RecipeItem::Exit {
            kind: ExitKind::Return,
            stmt: StmtRef::new(0),
        }],
    );
    let root = RecipeBlock::new(
        root_id,
        vec![
            RecipeItem::Stmt(StmtRef::new(0)),
            RecipeItem::Exit {
                kind: ExitKind::Return,
                stmt: StmtRef::new(1),
            },
            RecipeItem::IfV2 {
                if_stmt: StmtRef::new(2),
                cond_view: CondBlockView::from_expr(&condition),
                contract: IfContractKind::ExitOnly {
                    mode: IfMode::ExitIf,
                },
                then_block: Box::new(then_block),
                else_block: None,
            },
        ],
    );

    let provider = RawPartsAssociatedSourceV1::new(&arena);
    let block = provider.root(&root);
    assert_eq!(provider.block_len(&block).expect("owned raw block"), 3);

    let PartsAssociatedRecipeItemV1::OpaqueStmt { source } = provider
        .item(&block, 0)
        .expect("raw statement projects")
        .test_parts()
        .1
    else {
        panic!("ordinal zero must stay an opaque statement")
    };
    assert_eq!(source, &local);

    let PartsAssociatedRecipeItemV1::OpaqueExit { source, kind } = provider
        .item(&block, 1)
        .expect("raw exit projects")
        .test_parts()
        .1
    else {
        panic!("ordinal one must stay an exit")
    };
    assert_eq!(source, &returning);
    assert!(matches!(kind, ExitKind::Return));

    let (raw_port, projected) = provider
        .item(&block, 2)
        .expect("raw If projects")
        .test_parts();
    let PartsAssociatedRecipeItemV1::ExplicitIfV2 {
        source,
        condition: projected_condition,
        then_body,
        else_body,
        contract,
        then_block,
        else_block,
    } = projected
    else {
        panic!("ordinal two must stay an explicit If")
    };
    assert_eq!(source, &if_statement);
    assert_eq!(&projected_condition.tail_expr, &condition);
    assert_eq!(raw_port.body_statements(&then_body).len(), 1);
    assert!(else_body.is_none());
    assert!(matches!(contract, IfContractKind::ExitOnly { .. }));
    assert_eq!(
        provider
            .block_len(&then_block)
            .expect("owned raw child block"),
        1
    );
    assert!(else_block.is_none());
}

#[test]
fn located_provider_projects_actual_strict_root_and_retained_join_bridge() {
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
            .expect("exact port binds");
        let VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { root } = bound.mode()
        else {
            panic!("strict actual root must use ExitAllowed representation")
        };
        let provider = LocatedPartsAssociatedSourceV1::new(&root);
        assert_eq!(provider.block_len(&root).expect("owned located root"), 5);

        for index in [0, 1, 3] {
            let PartsAssociatedRecipeItemV1::OpaqueStmt { source } = provider
                .item(&root, index)
                .expect("actual opaque statement projects")
                .test_parts()
                .1
            else {
                panic!("actual ordinal {index} must stay opaque")
            };
            assert!(matches!(
                source,
                LocatedLoopPlanStmtInputV1::BorrowedLocated(_)
            ));
        }

        let (exact_port, projected) = provider
            .item(&root, 2)
            .expect("actual exit If projects")
            .test_parts();
        let PartsAssociatedRecipeItemV1::ExplicitIfV2 {
            condition,
            then_body,
            else_body,
            contract,
            then_block,
            else_block,
            ..
        } = projected
        else {
            panic!("actual ordinal two must stay explicit IfV2")
        };
        assert!(matches!(
            condition,
            LocatedLoopPlanExprInputV1::BorrowedLocated(_)
        ));
        assert_eq!(exact_port.body_statements(&then_body).len(), 1);
        assert!(else_body.is_none());
        assert!(matches!(contract, IfContractKind::ExitOnly { .. }));
        assert_eq!(
            provider
                .block_len(&then_block)
                .expect("owned located child block"),
            1
        );
        assert!(else_block.is_none());

        let PartsAssociatedRecipeItemV1::StmtWrappedJoinIf { bridge } = provider
            .item(&root, 4)
            .expect("actual wrapped Join projects")
            .test_parts()
            .1
        else {
            panic!("actual ordinal four must retain the wrapped Join product")
        };
        assert!(matches!(
            bridge.condition(),
            LocatedLoopPlanExprInputV1::BorrowedLocated(_)
        ));
        assert_eq!(bridge.singleton_recipe().block.items.len(), 1);
        assert_eq!(
            provider
                .block_len(&bridge.singleton_root().then_block())
                .expect("owned wrapped then block"),
            1
        );
        assert_eq!(
            provider
                .block_len(
                    &bridge
                        .singleton_root()
                        .else_block()
                        .expect("wrapped Join exact else block")
                )
                .expect("owned wrapped else block"),
            1
        );
    });
}

#[test]
fn raw_provider_rejects_a_block_issued_by_a_foreign_arena() {
    let mut first_arena = RecipeBodies::new();
    let first_body = first_arena.register(RecipeBody::new(vec![local_statement("first")]));
    let first_root = RecipeBlock::new(first_body, vec![RecipeItem::Stmt(StmtRef::new(0))]);
    let first_provider = RawPartsAssociatedSourceV1::new(&first_arena);
    let first_block = first_provider.root(&first_root);

    let mut second_arena = RecipeBodies::new();
    second_arena.register(RecipeBody::new(vec![local_statement("second")]));
    let second_provider = RawPartsAssociatedSourceV1::new(&second_arena);

    assert!(matches!(
        second_provider.block_len(&first_block),
        Err(super::associated_source::PartsAssociatedSourceErrorV1::ForeignRawBlock)
    ));
    assert!(matches!(
        second_provider.item(&first_block, usize::MAX),
        Err(super::associated_source::PartsAssociatedSourceErrorV1::ForeignRawBlock)
    ));
}

#[test]
fn located_provider_rejects_a_block_bound_to_a_foreign_port() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    with_default_and_strict_modes(|mode| {
        if mode != GenericLoopTestModeV1::StrictPlannerRequired {
            return;
        }
        let plan = actual_parser_add_fixture::plan();
        let (first_port, first_loop) = located_loop(&plan);
        let first_representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(
                &first_port,
                first_loop,
            )
            .expect("first representation");
        let first_bound = first_representation
            .bind_lowering_port(&first_port)
            .expect("first exact port binds");
        let VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { root: first_root } =
            first_bound.mode()
        else {
            panic!("strict first root")
        };

        let (second_port, second_loop) = located_loop(&plan);
        let second_representation =
            VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(
                &second_port,
                second_loop,
            )
            .expect("second representation");
        let second_bound = second_representation
            .bind_lowering_port(&second_port)
            .expect("second exact port binds");
        let VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { root: second_root } =
            second_bound.mode()
        else {
            panic!("strict second root")
        };

        let first_provider = LocatedPartsAssociatedSourceV1::new(&first_root);
        assert!(matches!(
            first_provider.block_len(&second_root),
            Err(super::associated_source::PartsAssociatedSourceErrorV1::ForeignLocatedBlock)
        ));
        assert!(matches!(
            first_provider.item(&second_root, usize::MAX),
            Err(super::associated_source::PartsAssociatedSourceErrorV1::ForeignLocatedBlock)
        ));
    });
}

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

fn local_statement(name: &str) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn return_statement(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        })),
        span: Span::unknown(),
    }
}

fn comparison(name: &str, value: i64) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }),
        right: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}
