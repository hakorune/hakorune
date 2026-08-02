use std::cell::RefCell;

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::{ConstValue, MirBuilder, MirInstruction, ValueId};

use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::return_statement_descent::{
    drive_value_return_statement_v1, ReturnStatementDescentPortV1, ReturnStatementSyntaxViewV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventV1 {
    Syntax,
    MatchProbe,
    ValueInput,
    ValueLower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchResultV1 {
    Decline,
    Select,
    Fail,
}

struct ReturnInputV1 {
    value: ASTNode,
}

struct RecordingReturnPortV1 {
    events: RefCell<Vec<EventV1>>,
    fail_syntax: bool,
    fail_input: bool,
    fail_lower: bool,
    match_result: MatchResultV1,
}

impl RecordingReturnPortV1 {
    fn accepting() -> Self {
        Self {
            events: RefCell::new(Vec::new()),
            fail_syntax: false,
            fail_input: false,
            fail_lower: false,
            match_result: MatchResultV1::Decline,
        }
    }

    fn events(&self) -> Vec<EventV1> {
        self.events.borrow().clone()
    }
}

impl RecursiveChildLoweringPortV1 for RecordingReturnPortV1 {
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
        self.events.borrow_mut().push(EventV1::ValueLower);
        let value = crate::mir::builder::emission::constant::emit_integer(builder, 41)?;
        if self.fail_lower {
            return Err("value-lower".to_string());
        }
        Ok(value)
    }
}

impl ReturnStatementDescentPortV1 for RecordingReturnPortV1 {
    type ReturnInput = ReturnInputV1;

    fn return_value_syntax<'input>(
        &self,
        input: &'input Self::ReturnInput,
    ) -> Result<ReturnStatementSyntaxViewV1<'input>, String> {
        self.events.borrow_mut().push(EventV1::Syntax);
        if self.fail_syntax {
            return Err("syntax".to_string());
        }
        Ok(ReturnStatementSyntaxViewV1::new(&input.value))
    }

    fn try_match_return_optimization(
        &mut self,
        builder: &mut MirBuilder,
        _input: &Self::ReturnInput,
        _value: &ASTNode,
    ) -> Result<Option<ValueId>, String> {
        self.events.borrow_mut().push(EventV1::MatchProbe);
        match self.match_result {
            MatchResultV1::Decline => Ok(None),
            MatchResultV1::Select => {
                crate::mir::builder::emission::constant::emit_integer(builder, 99).map(Some)
            }
            MatchResultV1::Fail => Err("match-probe".to_string()),
        }
    }
}

fn lower_recorded_value_after_probe_v1(
    builder: &mut MirBuilder,
    port: &mut RecordingReturnPortV1,
    _input: ReturnInputV1,
) -> Result<ValueId, String> {
    port.events.borrow_mut().push(EventV1::ValueInput);
    if port.fail_input {
        return Err("value-input".to_string());
    }
    super::super::recursive_child_lowering::drive_legacy_expression_v1(builder, port, ())
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
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
        .expect("current RET0 function")
        .blocks
        .values()
        .flat_map(|block| {
            block
                .instructions
                .iter()
                .chain(block.terminator.iter())
                .cloned()
        })
        .collect()
}

fn return_count(builder: &MirBuilder) -> usize {
    instructions(builder)
        .iter()
        .filter(|row| matches!(row, MirInstruction::Return { .. }))
        .count()
}

fn current_terminator(builder: &MirBuilder) -> Option<MirInstruction> {
    let block = builder
        .function_state
        .current_block
        .expect("current RET0 block");
    builder
        .function_state
        .current_function
        .as_ref()
        .expect("current RET0 function")
        .blocks
        .get(&block)
        .expect("current RET0 block body")
        .terminator
        .clone()
}

#[test]
fn cleanup_precedes_match_child_and_return_effects() {
    let mut builder = builder("ret0_cleanup/0");
    builder.function_state.protected_region.cleanup.active = true;
    builder.function_state.protected_region.cleanup.allow_return = false;
    let input = ReturnInputV1 { value: integer(1) };
    let mut port = RecordingReturnPortV1::accepting();

    let error = drive_value_return_statement_v1(
        &mut builder,
        &mut port,
        input,
        lower_recorded_value_after_probe_v1,
    )
    .unwrap_err();

    assert!(error.contains("return is not allowed inside cleanup block"));
    assert!(port.events().is_empty());
    assert!(instructions(&builder).is_empty());
}

#[test]
fn ordinary_value_probes_then_descends_once_and_completes_once() {
    let mut builder = builder("ret0_order/0");
    let input = ReturnInputV1 { value: integer(1) };
    let mut port = RecordingReturnPortV1::accepting();

    let result = drive_value_return_statement_v1(
        &mut builder,
        &mut port,
        input,
        lower_recorded_value_after_probe_v1,
    )
    .unwrap();

    assert_eq!(
        port.events(),
        vec![
            EventV1::Syntax,
            EventV1::MatchProbe,
            EventV1::ValueInput,
            EventV1::ValueLower,
        ]
    );
    assert_eq!(return_count(&builder), 1);
    assert!(
        matches!(instructions(&builder).last(), Some(MirInstruction::Return { value: Some(value) }) if *value == result)
    );
}

#[test]
fn selected_match_bypasses_value_demand_and_ordinary_completion() {
    let mut builder = builder("ret0_match_selected/0");
    let input = ReturnInputV1 { value: integer(1) };
    let mut port = RecordingReturnPortV1::accepting();
    port.match_result = MatchResultV1::Select;

    let selected = drive_value_return_statement_v1(
        &mut builder,
        &mut port,
        input,
        lower_recorded_value_after_probe_v1,
    )
    .unwrap();

    assert_eq!(port.events(), vec![EventV1::Syntax, EventV1::MatchProbe]);
    assert_eq!(return_count(&builder), 0);
    assert!(matches!(
        instructions(&builder).as_slice(),
        [MirInstruction::Const {
            dst,
            value: ConstValue::Integer(99),
        }] if *dst == selected
    ));
}

#[test]
fn syntax_match_input_and_child_failures_emit_no_return_completion() {
    for stage in ["syntax", "match", "input", "child"] {
        let mut builder = builder(&format!("ret0_failure_{stage}/0"));
        let input = ReturnInputV1 { value: integer(1) };
        let mut port = RecordingReturnPortV1::accepting();
        port.fail_syntax = stage == "syntax";
        port.match_result = if stage == "match" {
            MatchResultV1::Fail
        } else {
            MatchResultV1::Decline
        };
        port.fail_input = stage == "input";
        port.fail_lower = stage == "child";

        drive_value_return_statement_v1(
            &mut builder,
            &mut port,
            input,
            lower_recorded_value_after_probe_v1,
        )
        .unwrap_err();

        assert_eq!(return_count(&builder), 0, "stage={stage}");
        assert!(current_terminator(&builder).is_none(), "stage={stage}");
        assert_eq!(builder.recursion_depth, 0, "stage={stage}");
        if stage == "child" {
            assert!(instructions(&builder).iter().any(|row| matches!(
                row,
                MirInstruction::Const {
                    value: ConstValue::Integer(41),
                    ..
                }
            )));
        }
    }

    let mut fresh = builder("ret0_failure_reuse/0");
    let input = ReturnInputV1 { value: integer(1) };
    drive_value_return_statement_v1(
        &mut fresh,
        &mut RecordingReturnPortV1::accepting(),
        input,
        lower_recorded_value_after_probe_v1,
    )
    .unwrap();
    assert_eq!(return_count(&fresh), 1);
}

#[test]
fn configured_defer_reuses_copy_and_jump_completion_without_direct_return() {
    let mut builder = builder("ret0_defer/0");
    let slot = builder.next_value_id();
    let target = builder.next_block_id();
    builder.function_state.protected_region.return_defer.active = true;
    builder.function_state.protected_region.return_defer.slot = Some(slot);
    builder.function_state.protected_region.return_defer.target = Some(target);
    let input = ReturnInputV1 { value: integer(1) };
    let mut port = RecordingReturnPortV1::accepting();

    let result = drive_value_return_statement_v1(
        &mut builder,
        &mut port,
        input,
        lower_recorded_value_after_probe_v1,
    )
    .unwrap();
    let rows = instructions(&builder);

    assert!(builder.function_state.protected_region.return_defer.emitted);
    assert_eq!(return_count(&builder), 0);
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row, MirInstruction::Copy { dst, src } if *dst == slot && *src == result))
            .count(),
        1
    );
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row, MirInstruction::Jump { target: row_target, .. } if *row_target == target))
            .count(),
        1
    );
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(
                row,
                MirInstruction::Jump { .. } | MirInstruction::Return { .. }
            ))
            .count(),
        1
    );
    assert!(matches!(
        current_terminator(&builder),
        Some(MirInstruction::Jump {
            target: row_target,
            ..
        }) if row_target == target
    ));
}

#[test]
fn value_return_input_excludes_void() {
    let input = ReturnInputV1 { value: integer(1) };
    let syntax = RecordingReturnPortV1::accepting()
        .return_value_syntax(&input)
        .unwrap();

    assert!(matches!(syntax.value(), ASTNode::Literal { .. }));
    let _no_optional_value: &ASTNode = syntax.value();
}
