use std::cell::RefCell;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{BindingId, ConstValue, MirBuilder, MirInstruction, ValueId};

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::variable_assignment_descent::{
    drive_raw_variable_assignment_v1, drive_variable_assignment_v1,
    VariableAssignmentDescentPortV1, VariableAssignmentSyntaxViewV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventV1 {
    Syntax,
    RhsInput,
    RhsLower,
}

struct VariableAssignmentInputV1 {
    variable_name: String,
}

struct RecordingAssignmentPortV1 {
    events: RefCell<Vec<EventV1>>,
    fail_syntax: bool,
    fail_input: bool,
    fail_lower: bool,
    remove_binding_during_lower: bool,
}

impl RecordingAssignmentPortV1 {
    fn accepting() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            fail_syntax: false,
            fail_input: false,
            fail_lower: false,
            remove_binding_during_lower: false,
        }
    }

    fn events(&self) -> Vec<EventV1> {
        self.events.borrow().clone()
    }
}

impl RecursiveChildLoweringPortV1 for RecordingAssignmentPortV1 {
    type BodyInput = ();
    type StatementInput = ();
    type ExpressionInput = ();

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
        _input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        self.events.borrow_mut().push(EventV1::RhsLower);
        let value = crate::mir::builder::emission::constant::emit_integer(builder, 41)?;
        if self.remove_binding_during_lower {
            builder.function_state.binding_ctx.remove("x");
        }
        if self.fail_lower {
            return Err("rhs-lower".to_string());
        }
        Ok(value)
    }
}

impl VariableAssignmentDescentPortV1 for RecordingAssignmentPortV1 {
    type VariableAssignmentInput = VariableAssignmentInputV1;

    fn variable_assignment_syntax<'input>(
        &self,
        input: &'input Self::VariableAssignmentInput,
    ) -> Result<VariableAssignmentSyntaxViewV1<'input>, String> {
        self.events.borrow_mut().push(EventV1::Syntax);
        if self.fail_syntax {
            return Err("syntax".to_string());
        }
        Ok(VariableAssignmentSyntaxViewV1::new(&input.variable_name))
    }

    fn assignment_rhs_expression_input(
        &self,
        _input: &Self::VariableAssignmentInput,
    ) -> Result<Self::ExpressionInput, String> {
        self.events.borrow_mut().push(EventV1::RhsInput);
        if self.fail_input {
            return Err("rhs-input".to_string());
        }
        Ok(())
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn binary(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn declare(builder: &mut MirBuilder, name: &str, value: ValueId, binding: u32) {
    builder
        .function_state
        .variable_ctx
        .variable_map
        .insert(name.to_string(), value);
    builder
        .function_state
        .binding_ctx
        .insert(name.to_string(), BindingId::new(binding));
}

fn instructions(builder: &MirBuilder) -> Vec<MirInstruction> {
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current ASN0 function")
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter().cloned())
        .collect()
}

#[test]
fn declared_target_preflights_then_descends_rhs_and_completes_once() {
    let mut builder = builder("asn0_order/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);
    let input = VariableAssignmentInputV1 {
        variable_name: "x".to_string(),
    };
    let mut port = RecordingAssignmentPortV1::accepting();

    let result = drive_variable_assignment_v1(&mut builder, &mut port, &input).unwrap();

    assert_eq!(
        port.events(),
        vec![EventV1::Syntax, EventV1::RhsInput, EventV1::RhsLower]
    );
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&result)
    );
    assert_ne!(result, old);
    assert_eq!(
        instructions(&builder)
            .iter()
            .filter(|row| matches!(row, MirInstruction::ReleaseStrong { .. }))
            .count(),
        1
    );
}

#[test]
fn undeclared_binding_missing_and_pin_targets_reject_before_rhs_effects() {
    for case in ["undeclared", "binding-missing", "pin"] {
        let mut builder = builder(&format!("asn0_preflight_{case}/0"));
        let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
        let target = match case {
            "binding-missing" => {
                builder
                    .function_state
                    .variable_ctx
                    .variable_map
                    .insert("x".to_string(), old);
                "x"
            }
            "pin" => {
                builder.repl_mode = true;
                "__pin$1$recv"
            }
            _ => "x",
        };
        let input = VariableAssignmentInputV1 {
            variable_name: target.to_string(),
        };
        let mut port = RecordingAssignmentPortV1::accepting();
        let before = instructions(&builder);

        let error = drive_variable_assignment_v1(&mut builder, &mut port, &input).unwrap_err();

        assert_eq!(port.events(), vec![EventV1::Syntax]);
        assert_eq!(instructions(&builder), before);
        match case {
            "binding-missing" => assert!(error.contains("local_contract_binding_missing")),
            "pin" => assert!(error.contains("pin_named_assignment_forbidden")),
            _ => assert!(error.contains("Undefined variable: x")),
        }
    }
}

#[test]
fn syntax_and_rhs_input_failures_publish_no_rhs_or_assignment_effects() {
    for fail_syntax in [true, false] {
        let mut builder = builder("asn0_input_failure/0");
        let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
        declare(&mut builder, "x", old, 0);
        let input = VariableAssignmentInputV1 {
            variable_name: "x".to_string(),
        };
        let mut port = RecordingAssignmentPortV1::accepting();
        port.fail_syntax = fail_syntax;
        port.fail_input = !fail_syntax;
        let before = instructions(&builder);

        drive_variable_assignment_v1(&mut builder, &mut port, &input).unwrap_err();

        assert_eq!(
            builder.function_state.variable_ctx.variable_map.get("x"),
            Some(&old)
        );
        assert_eq!(instructions(&builder), before);
        assert_eq!(
            port.events(),
            if fail_syntax {
                vec![EventV1::Syntax]
            } else {
                vec![EventV1::Syntax, EventV1::RhsInput]
            }
        );
    }
}

#[test]
fn rhs_failure_keeps_old_assignment_and_emits_no_completion_effect() {
    let mut builder = builder("asn0_rhs_failure/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);
    let input = VariableAssignmentInputV1 {
        variable_name: "x".to_string(),
    };
    let mut port = RecordingAssignmentPortV1::accepting();
    port.fail_lower = true;

    let error = drive_variable_assignment_v1(&mut builder, &mut port, &input).unwrap_err();

    assert_eq!(error, "rhs-lower");
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&old)
    );
    assert!(!instructions(&builder)
        .iter()
        .any(|row| matches!(row, MirInstruction::ReleaseStrong { .. })));
}

#[test]
fn completion_recheck_rejects_lost_binding_and_fresh_attempt_succeeds() {
    let mut builder = builder("asn0_completion_recheck/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);
    let input = VariableAssignmentInputV1 {
        variable_name: "x".to_string(),
    };
    let mut port = RecordingAssignmentPortV1::accepting();
    port.remove_binding_during_lower = true;

    let error = drive_variable_assignment_v1(&mut builder, &mut port, &input).unwrap_err();
    assert!(error.contains("local_contract_binding_missing"));
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&old)
    );
    assert!(!instructions(&builder)
        .iter()
        .any(|row| matches!(row, MirInstruction::ReleaseStrong { .. })));

    builder
        .function_state
        .binding_ctx
        .insert("x".to_string(), BindingId::new(0));
    let mut fresh = RecordingAssignmentPortV1::accepting();
    let result = drive_variable_assignment_v1(&mut builder, &mut fresh, &input).unwrap();
    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&result)
    );
}

#[test]
fn raw_facade_reuses_recursive_binary_rhs_and_existing_completion() {
    let mut builder = builder("asn0_raw_facade/0");
    let old = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    declare(&mut builder, "x", old, 0);

    let result = drive_raw_variable_assignment_v1(
        &mut builder,
        "x".to_string(),
        binary(integer(2), integer(3)),
    )
    .unwrap();

    assert_eq!(
        builder.function_state.variable_ctx.variable_map.get("x"),
        Some(&result)
    );
    assert!(instructions(&builder)
        .iter()
        .any(|row| matches!(row, MirInstruction::BinOp { .. })));
    let constants = instructions(&builder)
        .iter()
        .filter_map(|row| match row {
            MirInstruction::Const {
                value: ConstValue::Integer(value),
                ..
            } => Some(*value),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(constants.ends_with(&[2, 3]));
}
