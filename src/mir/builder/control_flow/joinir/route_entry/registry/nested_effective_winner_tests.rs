use super::{test_legacy_effective_winner_v1, LegacyEffectiveWinnerReceiptV1};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::MirType;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn less(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn assignment(target: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(target)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn nested_fixture() -> (ASTNode, Vec<ASTNode>) {
    let inner = ASTNode::Loop {
        condition: Box::new(less(variable("j"), integer(3))),
        body: vec![
            assignment("sum", add(variable("sum"), integer(1))),
            assignment("j", add(variable("j"), integer(1))),
        ],
        span: Span::unknown(),
    };
    (
        less(variable("i"), integer(3)),
        vec![
            ASTNode::Local {
                variables: vec!["j".into()],
                initial_values: vec![None],
                declared_type_names: Vec::new(),
                span: Span::unknown(),
            },
            assignment("j", integer(0)),
            inner,
            assignment("i", add(variable("i"), integer(1))),
        ],
    )
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("nested_loop_minimal/0".to_string());
    for name in ["i", "sum"] {
        let value = builder.alloc_typed(MirType::Integer);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
    }
    builder
}

#[test]
fn nested_legacy_route_executes_once_as_effective_winner() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let (condition, body) = nested_fixture();
    let ctx = LoopRouteContext::new(&condition, &body, "nested_loop_minimal/0", false, false);
    let mut builder = seeded_builder();

    let receipt = test_legacy_effective_winner_v1(&mut builder, &ctx, true, false)
        .expect("legacy Nested route must execute in strict shadow mode");
    assert_eq!(
        receipt,
        LegacyEffectiveWinnerReceiptV1::Succeeded {
            winner: LoopRouteId::NestedLoopMinimal,
            attempted: Box::new([LoopRouteId::NestedLoopMinimal]),
        }
    );
}
