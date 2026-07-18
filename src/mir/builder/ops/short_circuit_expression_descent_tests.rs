use std::cell::RefCell;

use crate::ast::BinaryOperator;
use crate::mir::loop_api::LoopBuilderApi;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, MirType, ValueId};

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::short_circuit_expression_descent::{
    drive_short_circuit_expression_v1, ShortCircuitExpressionDescentPortV1,
    ShortCircuitSyntaxViewV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpressionInputV1 {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventV1 {
    Syntax,
    LeftInput,
    LowerLeft(BasicBlockId),
    RightInput,
    LowerRight(BasicBlockId),
}

struct ShortCircuitInputV1 {
    operator: BinaryOperator,
}

struct RecordingShortCircuitPortV1 {
    events: RefCell<Vec<EventV1>>,
    left_value: bool,
    right_value: bool,
    fail_syntax: bool,
    fail_left_input: bool,
    fail_left_lower: bool,
    fail_right_input: bool,
    fail_right_lower: bool,
}

impl RecordingShortCircuitPortV1 {
    fn accepting(left_value: bool, right_value: bool) -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            left_value,
            right_value,
            fail_syntax: false,
            fail_left_input: false,
            fail_left_lower: false,
            fail_right_input: false,
            fail_right_lower: false,
        }
    }

    fn events(&self) -> Vec<EventV1> {
        self.events.borrow().clone()
    }
}

impl RecursiveChildLoweringPortV1 for RecordingShortCircuitPortV1 {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = ExpressionInputV1;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        Err("unexpected body descent".to_string())
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        Err("unexpected statement descent".to_string())
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        let block = builder.current_block()?;
        match input {
            ExpressionInputV1::Left => {
                self.events.borrow_mut().push(EventV1::LowerLeft(block));
                if self.fail_left_lower {
                    return Err("left-lower".to_string());
                }
                crate::mir::builder::emission::constant::emit_bool(builder, self.left_value)
            }
            ExpressionInputV1::Right => {
                self.events.borrow_mut().push(EventV1::LowerRight(block));
                if self.fail_right_lower {
                    return Err("right-lower".to_string());
                }
                crate::mir::builder::emission::constant::emit_bool(builder, self.right_value)
            }
        }
    }
}

impl ShortCircuitExpressionDescentPortV1 for RecordingShortCircuitPortV1 {
    type ShortCircuitInput = ShortCircuitInputV1;

    fn short_circuit_syntax<'input>(
        &self,
        input: &'input Self::ShortCircuitInput,
    ) -> Result<ShortCircuitSyntaxViewV1<'input>, String> {
        self.events.borrow_mut().push(EventV1::Syntax);
        if self.fail_syntax {
            return Err("syntax".to_string());
        }
        Ok(ShortCircuitSyntaxViewV1::new(&input.operator))
    }

    fn short_circuit_left_input(
        &self,
        _input: &Self::ShortCircuitInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.events.borrow_mut().push(EventV1::LeftInput);
        if self.fail_left_input {
            return Err("left-input".to_string());
        }
        Ok(ExpressionInputV1::Left)
    }

    fn short_circuit_right_input(
        &self,
        _input: &Self::ShortCircuitInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.events.borrow_mut().push(EventV1::RightInput);
        if self.fail_right_input {
            return Err("right-input".to_string());
        }
        Ok(ExpressionInputV1::Right)
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn drive(
    builder: &mut MirBuilder,
    port: &mut RecordingShortCircuitPortV1,
    operator: BinaryOperator,
) -> Result<ValueId, String> {
    drive_short_circuit_expression_v1(builder, port, &ShortCircuitInputV1 { operator })
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn logical_driver_requests_left_before_rhs_and_rhs_only_in_eval_block() {
    let mut builder = builder("sc0_order/0");
    let entry = builder.current_block().unwrap();
    let mut port = RecordingShortCircuitPortV1::accepting(true, false);

    let result = drive(&mut builder, &mut port, BinaryOperator::And).unwrap();
    let events = port.events();

    assert_eq!(events[0], EventV1::Syntax);
    assert_eq!(events[1], EventV1::LeftInput);
    assert_eq!(events[2], EventV1::LowerLeft(entry));
    assert_eq!(events[3], EventV1::RightInput);
    let EventV1::LowerRight(rhs_block) = events[4] else {
        panic!("missing RHS lowering event: {events:?}");
    };
    assert_ne!(rhs_block, entry, "RHS must lower inside the eval-RHS block");
    assert_eq!(
        builder.type_ctx.value_types.get(&result),
        Some(&MirType::Bool)
    );
    assert!(instructions(&builder)
        .iter()
        .any(|instruction| matches!(instruction, MirInstruction::Phi { .. })));
}

#[test]
fn and_and_or_share_the_existing_short_circuit_completion() {
    for operator in [BinaryOperator::And, BinaryOperator::Or] {
        let mut builder = builder(&format!("sc0_operator/{operator}"));
        let mut port = RecordingShortCircuitPortV1::accepting(false, true);

        let result = drive(&mut builder, &mut port, operator).unwrap();

        assert_eq!(
            builder.type_ctx.value_types.get(&result),
            Some(&MirType::Bool)
        );
        assert_eq!(
            instructions(&builder)
                .iter()
                .filter(|instruction| matches!(instruction, MirInstruction::Phi { .. }))
                .count(),
            1
        );
    }
}

#[test]
fn ordinary_operator_rejects_before_child_input_or_cfg_effects() {
    let mut builder = builder("sc0_ordinary_reject/0");
    let before_block_count = builder
        .scope_ctx
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .len();
    let mut port = RecordingShortCircuitPortV1::accepting(true, true);

    let error = drive(&mut builder, &mut port, BinaryOperator::Add).unwrap_err();

    assert!(error.contains("ordinary-binary-owned-by-bin0"));
    assert_eq!(port.events(), vec![EventV1::Syntax]);
    assert_eq!(
        builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len(),
        before_block_count
    );
    assert!(instructions(&builder).is_empty());
}

#[test]
fn syntax_and_left_failures_stop_before_short_circuit_cfg() {
    let mut syntax_builder = builder("sc0_syntax_failure/0");
    let mut syntax_port = RecordingShortCircuitPortV1::accepting(true, true);
    syntax_port.fail_syntax = true;
    assert_eq!(
        drive(&mut syntax_builder, &mut syntax_port, BinaryOperator::And),
        Err("syntax".to_string())
    );
    assert_eq!(syntax_port.events(), vec![EventV1::Syntax]);

    let mut left_builder = builder("sc0_left_failure/0");
    let mut left_port = RecordingShortCircuitPortV1::accepting(true, true);
    left_port.fail_left_lower = true;
    assert_eq!(
        drive(&mut left_builder, &mut left_port, BinaryOperator::Or),
        Err("left-lower".to_string())
    );
    assert_eq!(
        left_port.events(),
        vec![
            EventV1::Syntax,
            EventV1::LeftInput,
            EventV1::LowerLeft(BasicBlockId::new(0)),
        ]
    );
    assert_eq!(
        left_builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len(),
        1
    );
}

#[test]
fn rhs_input_and_lowering_fail_only_after_entering_eval_block() {
    let mut input_builder = builder("sc0_rhs_input_failure/0");
    let entry = input_builder.current_block().unwrap();
    let mut input_port = RecordingShortCircuitPortV1::accepting(true, true);
    input_port.fail_right_input = true;
    assert_eq!(
        drive(&mut input_builder, &mut input_port, BinaryOperator::And),
        Err("right-input".to_string())
    );
    assert_eq!(input_port.events().last(), Some(&EventV1::RightInput));
    assert!(
        input_builder
            .scope_ctx
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .len()
            > 1
    );

    let mut lower_builder = builder("sc0_rhs_lower_failure/0");
    let mut lower_port = RecordingShortCircuitPortV1::accepting(true, true);
    lower_port.fail_right_lower = true;
    assert_eq!(
        drive(&mut lower_builder, &mut lower_port, BinaryOperator::Or),
        Err("right-lower".to_string())
    );
    let EventV1::LowerRight(rhs_block) = lower_port.events().last().cloned().unwrap() else {
        panic!("RHS lowering was not requested");
    };
    assert_ne!(rhs_block, entry);
}

#[test]
fn failed_driver_does_not_poison_a_fresh_driver() {
    let mut failed_builder = builder("sc0_failed/0");
    let mut failed_port = RecordingShortCircuitPortV1::accepting(true, true);
    failed_port.fail_left_input = true;
    assert_eq!(
        drive(&mut failed_builder, &mut failed_port, BinaryOperator::And),
        Err("left-input".to_string())
    );

    let mut fresh_builder = builder("sc0_fresh/0");
    let mut fresh_port = RecordingShortCircuitPortV1::accepting(true, true);
    let result = drive(&mut fresh_builder, &mut fresh_port, BinaryOperator::And).unwrap();
    assert_eq!(
        fresh_builder.type_ctx.value_types.get(&result),
        Some(&MirType::Bool)
    );
}
