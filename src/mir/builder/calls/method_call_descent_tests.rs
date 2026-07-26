use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::{MirBuilder, MirInstruction, ValueId};

use super::super::recursive_child_lowering::{
    RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use super::call_argument_descent::CallArgumentDescentPortV1;
use super::method_call_descent::{
    lower_method_call_argument_v1, lower_method_call_arguments_v1, lower_method_call_receiver_v1,
    MethodCallDescentPortV1, MethodCallSyntaxViewV1, RawLegacyMethodCallInputV1,
};

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

#[test]
fn raw_method_input_exposes_one_borrowed_syntax_view() {
    let input = RawLegacyMethodCallInputV1::new(
        variable("text"),
        "substring".to_string(),
        vec![integer(1), integer(2)],
    );
    let port = RawLegacyChildLoweringPortV1;

    let view = port.method_call_syntax(&input).unwrap();
    assert!(matches!(view.receiver(), ASTNode::Variable { name, .. } if name == "text"));
    assert_eq!(view.method(), "substring");
    assert_eq!(view.arguments().len(), 2);
    assert_eq!(port.call_arguments_input(&input).unwrap().len(), 2);
}

#[test]
fn raw_receiver_and_arguments_use_existing_e0_and_arg0_ports() {
    let input = RawLegacyMethodCallInputV1::new(
        integer(7),
        "method".to_string(),
        vec![integer(8), integer(9)],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("method_call_descent/0".to_string());

    let receiver = lower_method_call_receiver_v1(&mut builder, &mut port, &input).unwrap();
    let arguments = lower_method_call_arguments_v1(&mut builder, &mut port, &input).unwrap();

    let integer_rows = instructions(&builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                dst,
                value: crate::mir::ConstValue::Integer(value),
            } => Some((dst, value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(integer_rows.len(), 3);
    assert_eq!(integer_rows[0], (receiver, 7));
    assert_eq!(integer_rows[1], (arguments[0], 8));
    assert_eq!(integer_rows[2], (arguments[1], 9));
}

struct DistinctArgumentsInput(Vec<ASTNode>);

struct DistinctMethodCallInput {
    receiver: ASTNode,
    method: String,
    arguments: DistinctArgumentsInput,
}

enum DistinctExpressionInput {
    Receiver(ASTNode),
    Argument(ASTNode),
}

#[derive(Default)]
struct DistinctMethodCallPort {
    receiver_descents: usize,
    argument_descents: usize,
}

impl RecursiveChildLoweringPortV1 for DistinctMethodCallPort {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = DistinctExpressionInput;

    fn lower_body(&mut self, _builder: &mut MirBuilder, _input: ()) -> Result<ValueId, String> {
        Err("body descent is outside this fixture".to_string())
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: (),
    ) -> Result<ValueId, String> {
        Err("statement descent is outside this fixture".to_string())
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: DistinctExpressionInput,
    ) -> Result<ValueId, String> {
        let syntax = match input {
            DistinctExpressionInput::Receiver(syntax) => {
                self.receiver_descents += 1;
                syntax
            }
            DistinctExpressionInput::Argument(syntax) => {
                self.argument_descents += 1;
                syntax
            }
        };
        builder.build_expression(syntax)
    }
}

impl CallArgumentDescentPortV1 for DistinctMethodCallPort {
    type ArgumentsInput = DistinctArgumentsInput;

    fn argument_count(&self, input: &Self::ArgumentsInput) -> usize {
        input.0.len()
    }

    fn argument_syntax<'input>(
        &self,
        input: &'input Self::ArgumentsInput,
        index: usize,
    ) -> Option<&'input ASTNode> {
        input.0.get(index)
    }

    fn argument_expression_input(
        &mut self,
        input: &Self::ArgumentsInput,
        index: usize,
    ) -> Result<Self::ExpressionInput, String> {
        input
            .0
            .get(index)
            .cloned()
            .map(DistinctExpressionInput::Argument)
            .ok_or_else(|| format!("missing distinct argument index={index}"))
    }
}

impl MethodCallDescentPortV1 for DistinctMethodCallPort {
    type MethodCallInput = DistinctMethodCallInput;

    fn method_call_syntax<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<MethodCallSyntaxViewV1<'input>, String> {
        Ok(MethodCallSyntaxViewV1::new(
            &input.receiver,
            &input.method,
            &input.arguments.0,
        ))
    }

    fn receiver_expression_input(
        &self,
        input: &Self::MethodCallInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(DistinctExpressionInput::Receiver(input.receiver.clone()))
    }

    fn call_arguments_input<'input>(
        &self,
        input: &'input Self::MethodCallInput,
    ) -> Result<&'input Self::ArgumentsInput, String> {
        Ok(&input.arguments)
    }
}

#[test]
fn raw_single_argument_descent_skips_syntax_only_neighbors() {
    let input = RawLegacyMethodCallInputV1::new(
        variable("__mir__"),
        "log".to_string(),
        vec![integer(10), integer(11), integer(12)],
    );
    let mut port = RawLegacyChildLoweringPortV1;
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("method_argument_range/0".to_string());

    let value = lower_method_call_argument_v1(&mut builder, &mut port, &input, 1).unwrap();
    let integer_values = instructions(&builder)
        .into_iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Const {
                value: crate::mir::ConstValue::Integer(value),
                ..
            } => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(integer_values, vec![11]);
    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Const { dst, .. } if *dst == value
    )));
}

#[test]
fn associated_inputs_keep_receiver_and_arguments_independent() {
    let input = DistinctMethodCallInput {
        receiver: integer(10),
        method: "method".to_string(),
        arguments: DistinctArgumentsInput(vec![integer(11), integer(12)]),
    };

    let mut receiver_builder = MirBuilder::new();
    receiver_builder.enter_function_for_test("method_receiver_only/0".to_string());
    let mut receiver_port = DistinctMethodCallPort::default();
    lower_method_call_receiver_v1(&mut receiver_builder, &mut receiver_port, &input).unwrap();
    assert_eq!(receiver_port.receiver_descents, 1);
    assert_eq!(receiver_port.argument_descents, 0);

    let mut argument_builder = MirBuilder::new();
    argument_builder.enter_function_for_test("method_arguments_only/0".to_string());
    let mut argument_port = DistinctMethodCallPort::default();
    lower_method_call_arguments_v1(&mut argument_builder, &mut argument_port, &input).unwrap();
    assert_eq!(argument_port.receiver_descents, 0);
    assert_eq!(argument_port.argument_descents, 2);
}
