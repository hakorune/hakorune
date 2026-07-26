use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{MirBuilder, MirInstruction, ValueId};

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::call_argument_descent::{drive_call_arguments_v1, CallArgumentDescentPortV1};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
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

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current test function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

struct ArgumentTokenV1 {
    syntax: ASTNode,
    emitted: i64,
    index: usize,
}

struct ExpressionTokenV1 {
    emitted: i64,
    index: usize,
}

#[derive(Default)]
struct CountingArgumentPortV1 {
    lowered: Vec<usize>,
    projected: Vec<usize>,
    fail_at: Option<usize>,
}

impl RecursiveChildLoweringPortV1 for CountingArgumentPortV1 {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = ExpressionTokenV1;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        unreachable!("argument test port does not lower bodies")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        unreachable!("argument test port does not lower statements")
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.lowered.push(input.index);
        if self.fail_at == Some(input.index) {
            return Err(format!("argument-failure-{}", input.index));
        }
        crate::mir::builder::emission::constant::emit_integer(builder, input.emitted)
    }
}

impl CallArgumentDescentPortV1 for CountingArgumentPortV1 {
    type ArgumentsInput = [ArgumentTokenV1];

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        input.len()
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        input.get(index).map(|argument| &argument.syntax)
    }

    fn argument_expression_input(
        &mut self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        self.projected.push(index);
        input
            .get(index)
            .map(|argument| ExpressionTokenV1 {
                emitted: argument.emitted,
                index: argument.index,
            })
            .ok_or_else(|| format!("missing-test-argument-{index}"))
    }
}

fn inputs(values: &[i64]) -> Vec<ArgumentTokenV1> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| ArgumentTokenV1 {
            syntax: integer(*value),
            emitted: *value,
            index,
        })
        .collect()
}

#[test]
fn associated_inputs_descend_once_in_source_order() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("call_argument_order/0".to_string());
    let mut port = CountingArgumentPortV1::default();

    let values = drive_call_arguments_v1(&mut builder, &mut port, &inputs(&[4, 5, 6])).unwrap();

    assert_eq!(port.lowered, vec![0, 1, 2]);
    assert_eq!(port.projected, vec![0, 1, 2]);
    assert_eq!(values.len(), 3);
    assert!(values.windows(2).all(|pair| pair[0].0 < pair[1].0));
}

#[test]
fn empty_arguments_publish_no_child_calls() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("call_argument_empty/0".to_string());
    let mut port = CountingArgumentPortV1::default();

    assert!(drive_call_arguments_v1(&mut builder, &mut port, &[])
        .unwrap()
        .is_empty());
    assert!(port.lowered.is_empty());
}

#[test]
fn argument_failure_stops_later_descent_without_retry() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("call_argument_failure/0".to_string());
    let mut port = CountingArgumentPortV1 {
        fail_at: Some(1),
        ..CountingArgumentPortV1::default()
    };

    assert_eq!(
        drive_call_arguments_v1(&mut builder, &mut port, &inputs(&[7, 8, 9])).unwrap_err(),
        "argument-failure-1"
    );
    assert_eq!(port.lowered, vec![0, 1]);
    assert_eq!(port.projected, vec![0, 1]);
}

#[test]
fn failed_port_does_not_poison_fresh_argument_descent() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("call_argument_reuse/0".to_string());
    let mut port = CountingArgumentPortV1 {
        fail_at: Some(0),
        ..CountingArgumentPortV1::default()
    };
    assert!(drive_call_arguments_v1(&mut builder, &mut port, &inputs(&[1])).is_err());

    port.fail_at = None;
    assert_eq!(
        drive_call_arguments_v1(&mut builder, &mut port, &inputs(&[2, 3]))
            .unwrap()
            .len(),
        2
    );
    assert_eq!(port.lowered, vec![0, 0, 1]);
}

#[test]
fn selected_raw_facade_preserves_nested_argument_mir() {
    let arguments = vec![add(integer(1), integer(2)), integer(3)];

    let mut selected = MirBuilder::new();
    selected.enter_function_for_test("call_argument_raw_selected/0".to_string());
    let selected_values = selected.build_call_args(&arguments).unwrap();

    let mut manual = MirBuilder::new();
    manual.enter_function_for_test("call_argument_raw_selected/0".to_string());
    let manual_values = arguments
        .into_iter()
        .map(|argument| manual.build_expression(argument).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(selected_values, manual_values);
    assert_eq!(instructions(&selected), instructions(&manual));
}
