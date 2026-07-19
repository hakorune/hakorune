use std::cell::RefCell;

use crate::ast::BinaryOperator;
use crate::mir::{BinaryOp, CompareOp, MirBuilder, MirInstruction, MirType, ValueId};

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::binary_expression_descent::{
    drive_ordinary_binary_expression_v1, BinaryExpressionDescentPortV1, BinarySyntaxViewV1,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SideV1 {
    Left,
    Right,
}

struct BinaryInputV1 {
    operator: BinaryOperator,
}

struct ExpressionInputV1 {
    side: SideV1,
    value: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FailureV1 {
    None,
    Syntax,
    LeftInput,
    LeftLowering,
    RightInput,
    RightLowering,
}

struct RecordingBinaryPortV1 {
    events: RefCell<Vec<&'static str>>,
    failure: FailureV1,
    emit_child_values: bool,
}

impl RecordingBinaryPortV1 {
    fn new(failure: FailureV1) -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            failure,
            emit_child_values: true,
        }
    }

    fn detached_values() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            failure: FailureV1::None,
            emit_child_values: false,
        }
    }

    fn events(&self) -> Vec<&'static str> {
        self.events.borrow().clone()
    }
}

impl RecursiveChildLoweringPortV1 for RecordingBinaryPortV1 {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = ExpressionInputV1;

    fn lower_body(&mut self, _builder: &mut MirBuilder, _input: ()) -> Result<ValueId, String> {
        Err("body descent is outside BIN0-S0".to_string())
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: (),
    ) -> Result<ValueId, String> {
        Err("statement descent is outside BIN0-S0".to_string())
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: ExpressionInputV1,
    ) -> Result<ValueId, String> {
        match input.side {
            SideV1::Left => {
                self.events.borrow_mut().push("left-lower");
                if self.failure == FailureV1::LeftLowering {
                    return Err("left-lowering-failure".to_string());
                }
            }
            SideV1::Right => {
                self.events.borrow_mut().push("right-lower");
                if self.failure == FailureV1::RightLowering {
                    return Err("right-lowering-failure".to_string());
                }
            }
        }
        if self.emit_child_values {
            crate::mir::builder::emission::constant::emit_integer(builder, input.value)
        } else {
            Ok(ValueId(match input.side {
                SideV1::Left => 0,
                SideV1::Right => 1,
            }))
        }
    }
}

impl BinaryExpressionDescentPortV1 for RecordingBinaryPortV1 {
    type BinaryInput = BinaryInputV1;

    fn binary_syntax<'input>(
        &self,
        input: &'input Self::BinaryInput,
    ) -> Result<BinarySyntaxViewV1<'input>, String> {
        self.events.borrow_mut().push("syntax");
        if self.failure == FailureV1::Syntax {
            return Err("syntax-failure".to_string());
        }
        Ok(BinarySyntaxViewV1::new(&input.operator))
    }

    fn binary_left_input(
        &self,
        _input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.events.borrow_mut().push("left-input");
        if self.failure == FailureV1::LeftInput {
            return Err("left-input-failure".to_string());
        }
        Ok(ExpressionInputV1 {
            side: SideV1::Left,
            value: 7,
        })
    }

    fn binary_right_input(
        &self,
        _input: &Self::BinaryInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.events.borrow_mut().push("right-input");
        if self.failure == FailureV1::RightInput {
            return Err("right-input-failure".to_string());
        }
        Ok(ExpressionInputV1 {
            side: SideV1::Right,
            value: 3,
        })
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current BIN0 test function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

fn drive(
    builder: &mut MirBuilder,
    port: &mut RecordingBinaryPortV1,
    operator: BinaryOperator,
) -> Result<ValueId, String> {
    drive_ordinary_binary_expression_v1(builder, port, &BinaryInputV1 { operator })
}

#[test]
fn ordinary_arithmetic_descends_left_then_right_once_and_uses_existing_terminal() {
    let mut builder = builder("binary_descent_add/0");
    let mut port = RecordingBinaryPortV1::new(FailureV1::None);

    let output = drive(&mut builder, &mut port, BinaryOperator::Add).unwrap();

    assert_eq!(
        port.events(),
        vec![
            "syntax",
            "left-input",
            "left-lower",
            "right-input",
            "right-lower"
        ]
    );
    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            ..
        } if *dst == output
    )));
}

#[test]
fn ordinary_comparison_uses_same_order_and_existing_bool_terminal() {
    let mut builder = builder("binary_descent_compare/0");
    let mut port = RecordingBinaryPortV1::new(FailureV1::None);

    let output = drive(&mut builder, &mut port, BinaryOperator::Less).unwrap();

    assert_eq!(
        port.events(),
        vec![
            "syntax",
            "left-input",
            "left-lower",
            "right-input",
            "right-lower"
        ]
    );
    assert_eq!(
        builder.function_state.type_ctx.value_types.get(&output),
        Some(&MirType::Bool)
    );
    assert!(instructions(&builder).iter().any(|instruction| matches!(
        instruction,
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            ..
        } if *dst == output
    )));
}

#[test]
fn ordinary_operator_boundary_rejects_only_and_or_before_child_effects() {
    let ordinary = [
        BinaryOperator::Add,
        BinaryOperator::Subtract,
        BinaryOperator::Multiply,
        BinaryOperator::Divide,
        BinaryOperator::Modulo,
        BinaryOperator::BitAnd,
        BinaryOperator::BitOr,
        BinaryOperator::BitXor,
        BinaryOperator::Shl,
        BinaryOperator::Shr,
        BinaryOperator::Equal,
        BinaryOperator::NotEqual,
        BinaryOperator::Less,
        BinaryOperator::Greater,
        BinaryOperator::LessEqual,
        BinaryOperator::GreaterEqual,
    ];
    for (index, operator) in ordinary.into_iter().enumerate() {
        let mut builder = builder(&format!("binary_descent_operator_{index}/0"));
        let mut port = RecordingBinaryPortV1::new(FailureV1::None);
        drive(&mut builder, &mut port, operator).unwrap();
        assert_eq!(port.events().len(), 5);
    }

    for operator in [BinaryOperator::And, BinaryOperator::Or] {
        let mut builder = builder("binary_descent_logical_reject/0");
        let before = instructions(&builder);
        let mut port = RecordingBinaryPortV1::new(FailureV1::None);
        let error = drive(&mut builder, &mut port, operator).unwrap_err();
        assert!(error.contains("logical-short-circuit-owned-by-sc0"));
        assert_eq!(port.events(), vec!["syntax"]);
        assert_eq!(instructions(&builder), before);
    }
}

#[test]
fn syntax_and_input_failures_precede_later_child_effects() {
    let cases = [
        (FailureV1::Syntax, vec!["syntax"], "syntax-failure"),
        (
            FailureV1::LeftInput,
            vec!["syntax", "left-input"],
            "left-input-failure",
        ),
        (
            FailureV1::RightInput,
            vec!["syntax", "left-input", "left-lower", "right-input"],
            "right-input-failure",
        ),
    ];
    for (index, (failure, expected_events, expected_error)) in cases.into_iter().enumerate() {
        let mut builder = builder(&format!("binary_descent_input_failure_{index}/0"));
        let mut port = RecordingBinaryPortV1::new(failure);
        let error = drive(&mut builder, &mut port, BinaryOperator::Add).unwrap_err();
        assert_eq!(error, expected_error);
        assert_eq!(port.events(), expected_events);
    }
}

#[test]
fn child_failure_stops_later_descent_and_fresh_driver_is_independent() {
    let mut left_builder = builder("binary_descent_left_failure/0");
    let mut left_port = RecordingBinaryPortV1::new(FailureV1::LeftLowering);
    assert_eq!(
        drive(&mut left_builder, &mut left_port, BinaryOperator::Add).unwrap_err(),
        "left-lowering-failure"
    );
    assert_eq!(
        left_port.events(),
        vec!["syntax", "left-input", "left-lower"]
    );

    let mut right_builder = builder("binary_descent_right_failure/0");
    let mut right_port = RecordingBinaryPortV1::new(FailureV1::RightLowering);
    assert_eq!(
        drive(&mut right_builder, &mut right_port, BinaryOperator::Add).unwrap_err(),
        "right-lowering-failure"
    );
    assert_eq!(
        right_port.events(),
        vec![
            "syntax",
            "left-input",
            "left-lower",
            "right-input",
            "right-lower"
        ]
    );

    let mut fresh_builder = builder("binary_descent_fresh/0");
    let mut fresh_port = RecordingBinaryPortV1::new(FailureV1::None);
    drive(&mut fresh_builder, &mut fresh_port, BinaryOperator::Add).unwrap();
    assert_eq!(fresh_port.events().len(), 5);
}

#[test]
fn terminal_failure_occurs_after_both_children_without_retry() {
    let mut detached_builder = MirBuilder::new();
    let mut port = RecordingBinaryPortV1::detached_values();

    let error = drive(&mut detached_builder, &mut port, BinaryOperator::Add).unwrap_err();

    assert!(!error.is_empty());
    assert_eq!(
        port.events(),
        vec![
            "syntax",
            "left-input",
            "left-lower",
            "right-input",
            "right-lower"
        ]
    );

    let mut fresh_builder = builder("binary_descent_after_terminal_failure/0");
    let mut fresh_port = RecordingBinaryPortV1::new(FailureV1::None);
    drive(&mut fresh_builder, &mut fresh_port, BinaryOperator::Add).unwrap();
    assert_eq!(fresh_port.events().len(), 5);
}
