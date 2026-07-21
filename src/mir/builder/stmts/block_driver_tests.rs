use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
use crate::mir::{ConstValue, MirBuilder, MirInstruction, ValueId};

#[derive(Default)]
struct RecordingAstChildPortV1 {
    statement_nodes: Vec<&'static str>,
}

impl RecursiveChildLoweringPortV1 for RecordingAstChildPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        Err("body should remain owned by block driver".to_string())
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        self.statement_nodes.push(input.node_type());
        crate::mir::builder::emission::constant::emit_integer(
            builder,
            100 + self.statement_nodes.len() as i64,
        )
    }

    fn lower_expression(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        Err("expression should remain owned by child port".to_string())
    }
}

fn integer(value: i64) -> ASTNode {
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

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: Vec::new(),
        span: Span::unknown(),
    }
}

fn return_(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

fn instructions(builder: &MirBuilder) -> Vec<&MirInstruction> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("current test function");
    let block = function
        .blocks
        .get(&function.entry_block)
        .expect("entry block");
    block.instructions.iter().collect()
}

fn integer_constants(builder: &MirBuilder) -> Vec<(ValueId, i64)> {
    instructions(builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(value),
            } => Some((*dst, *value)),
            _ => None,
        })
        .collect()
}

#[test]
fn empty_block_emits_one_void_and_restores_lexical_scope() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("block_driver_empty/0".to_string());

    let output = super::block_stmt::build_block(&mut builder, Vec::new()).unwrap();

    let void_outputs = instructions(&builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                dst,
                value: ConstValue::Void,
            } => Some(*dst),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(void_outputs, vec![output]);
    assert!(builder.function_state.scope.lexical_scope_stack.is_empty());
}

#[test]
fn statements_lower_once_in_source_order_and_return_the_last_value() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("block_driver_order/0".to_string());

    let output =
        super::block_stmt::build_block(&mut builder, vec![integer(11), integer(22)]).unwrap();
    let constants = integer_constants(&builder);

    assert_eq!(constants.len(), 2);
    assert_eq!(constants[0].1, 11);
    assert_eq!(constants[1].1, 22);
    assert_eq!(output, constants[1].0);
    assert!(builder.function_state.scope.lexical_scope_stack.is_empty());
}

#[test]
fn port_aware_block_reuses_the_supplied_statement_descent() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("block_driver_port/0".to_string());
    let mut child = RecordingAstChildPortV1::default();

    let output = super::block_stmt::build_block_with_port_v1(
        &mut builder,
        &mut child,
        vec![integer(11), integer(22)],
    )
    .unwrap();

    assert_eq!(child.statement_nodes, vec!["Literal", "Literal"]);
    assert_eq!(
        instructions(&builder)
            .iter()
            .filter(|instruction| matches!(instruction, MirInstruction::Const { .. }))
            .count(),
        2
    );
    assert_eq!(output, integer_constants(&builder).last().unwrap().0);
}

#[test]
fn termination_stops_before_an_invalid_trailing_statement() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("block_driver_termination/0".to_string());

    super::block_stmt::build_block(
        &mut builder,
        vec![return_(Some(integer(7))), variable("missing")],
    )
    .unwrap();

    assert_eq!(integer_constants(&builder).len(), 1);
    assert!(builder.is_current_block_terminated());
    assert!(builder.function_state.scope.lexical_scope_stack.is_empty());
}

#[test]
fn successful_local_scope_restores_variable_and_binding_views() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("block_driver_local/0".to_string());

    super::block_stmt::build_block(&mut builder, vec![local("inner", integer(1))]).unwrap();

    assert!(!builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("inner"));
    assert!(builder.function_state.binding_ctx.lookup("inner").is_none());
    assert!(builder.function_state.scope.lexical_scope_stack.is_empty());
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::KeepAlive { .. })));
}

#[test]
fn failure_after_local_restores_scope_state_without_retry() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("block_driver_failure/0".to_string());

    let error = super::block_stmt::build_block(
        &mut builder,
        vec![local("inner", integer(1)), variable("missing")],
    )
    .unwrap_err();

    assert!(error.contains("missing"));
    assert!(!builder
        .function_state
        .variable_ctx
        .variable_map
        .contains_key("inner"));
    assert!(builder.function_state.binding_ctx.lookup("inner").is_none());
    assert!(builder.function_state.scope.lexical_scope_stack.is_empty());
}
