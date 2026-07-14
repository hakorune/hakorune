use super::super::{MirInterpreter, StepTrace, VMError, VMValue};
use super::frame_transaction::with_function_frame;
use crate::mir::{
    BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirType, ValueId,
};

fn empty_function(name: &str) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: Vec::new(),
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn returning_integer(name: &str, value: i64) -> MirFunction {
    let mut function = empty_function(name);
    let result = function.next_value_id();
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: result,
        value: ConstValue::Integer(value),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    function
}

fn division_by_zero(name: &str) -> MirFunction {
    let mut function = empty_function(name);
    let lhs = function.next_value_id();
    let rhs = function.next_value_id();
    let result = function.next_value_id();
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: lhs,
        value: ConstValue::Integer(1),
    });
    block.add_instruction(MirInstruction::Const {
        dst: rhs,
        value: ConstValue::Integer(0),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: result,
        op: BinaryOp::Div,
        lhs,
        rhs,
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    function
}

fn phi_with_undefined_input(name: &str) -> MirFunction {
    let mut function = empty_function(name);
    let result = function.next_value_id();
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::Phi {
        dst: result,
        inputs: vec![(BasicBlockId::new(99), ValueId::new(999))],
        type_hint: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Return {
        value: Some(result),
    });
    function
}

fn missing_target(name: &str) -> MirFunction {
    let mut function = empty_function(name);
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(77),
            edge_args: None,
        });
    function
}

fn seeded_interpreter() -> MirInterpreter {
    let mut interpreter = MirInterpreter::new();
    interpreter.vm_trace_enabled = false;
    interpreter.vm_phi_tolerate_undefined_enabled = false;
    interpreter
        .regs
        .insert(ValueId::new(90), VMValue::Integer(90));
    interpreter.reg_fast_slots = vec![Some(VMValue::Integer(91))];
    interpreter
        .reg_copy_aliases
        .insert(ValueId::new(92), ValueId::new(90));
    interpreter.reg_i64_cache = vec![Some(93)];
    interpreter.reg_bool_cache = vec![Some(true)];
    interpreter.cur_fn = Some("Caller.outer/0".to_string());
    interpreter.call_depth = 2;
    interpreter.call_stack = vec!["Root.main/0".to_string(), "Caller.outer/0".to_string()];
    interpreter.last_block = Some(BasicBlockId::new(94));
    interpreter.last_inst = Some(MirInstruction::Safepoint);
    interpreter.last_inst_index = Some(95);
    interpreter.recent_steps.push_back(StepTrace {
        bb: BasicBlockId::new(96),
        inst_idx: Some(97),
        inst: Some("caller-step".to_string()),
    });
    interpreter
}

fn assert_caller_frame(interpreter: &MirInterpreter) {
    assert_eq!(
        interpreter.regs.get(&ValueId::new(90)),
        Some(&VMValue::Integer(90))
    );
    assert_eq!(interpreter.reg_fast_slots, vec![Some(VMValue::Integer(91))]);
    assert_eq!(
        interpreter.reg_copy_aliases.get(&ValueId::new(92)),
        Some(&ValueId::new(90))
    );
    assert_eq!(interpreter.reg_i64_cache, vec![Some(93)]);
    assert_eq!(interpreter.reg_bool_cache, vec![Some(true)]);
    assert_eq!(interpreter.cur_fn.as_deref(), Some("Caller.outer/0"));
    assert_eq!(interpreter.call_depth, 2);
    assert_eq!(
        interpreter.call_stack,
        vec!["Root.main/0".to_string(), "Caller.outer/0".to_string()]
    );
    assert_eq!(interpreter.last_block, Some(BasicBlockId::new(94)));
    assert!(matches!(
        interpreter.last_inst,
        Some(MirInstruction::Safepoint)
    ));
    assert_eq!(interpreter.last_inst_index, Some(95));
    assert_eq!(interpreter.recent_steps.len(), 1);
    let step = interpreter.recent_steps.front().unwrap();
    assert_eq!(step.bb, BasicBlockId::new(96));
    assert_eq!(step.inst_idx, Some(97));
    assert_eq!(step.inst.as_deref(), Some("caller-step"));
}

#[test]
fn success_restores_the_complete_caller_frame() {
    let mut interpreter = seeded_interpreter();
    let result = interpreter
        .exec_function_inner(&returning_integer("Frame.success/0", 42), None)
        .unwrap();
    assert_eq!(result, VMValue::Integer(42));
    assert_caller_frame(&interpreter);
}

#[test]
fn instruction_error_restores_the_complete_caller_frame() {
    let mut interpreter = seeded_interpreter();
    let error = interpreter
        .exec_function_inner(&division_by_zero("Frame.div_zero/0"), None)
        .unwrap_err();
    assert!(matches!(error, VMError::DivisionByZero));
    assert_caller_frame(&interpreter);
}

#[test]
fn phi_error_restores_the_complete_caller_frame() {
    let mut interpreter = seeded_interpreter();
    let error = interpreter
        .exec_function_inner(&phi_with_undefined_input("Frame.phi/0"), None)
        .unwrap_err();
    assert!(matches!(error, VMError::InvalidValue(_)));
    assert_caller_frame(&interpreter);
}

#[test]
fn step_budget_error_restores_the_complete_caller_frame() {
    let mut interpreter = seeded_interpreter();
    let function = returning_integer("Frame.step_budget/0", 1);
    let error = with_function_frame(&mut interpreter, &function, None, |_| {
        Err(VMError::StepBudgetExceeded {
            max_steps: 9,
            steps: 10,
            function: Some("Frame.step_budget/0".to_string()),
            current_block: BasicBlockId::new(7),
            last_block: Some(BasicBlockId::new(7)),
            last_inst: Some("loop".to_string()),
            last_inst_index: Some(8),
            span: None,
            source_file: None,
            mir_dump_path: None,
            mir_dump_snip_path: None,
            trace_tail: None,
            loop_signature: None,
        })
    })
    .unwrap_err();

    assert!(matches!(error, VMError::StepBudgetExceeded { .. }));
    assert_caller_frame(&interpreter);
}

#[test]
fn missing_block_restores_the_complete_caller_frame() {
    let mut interpreter = seeded_interpreter();
    let error = interpreter
        .exec_function_inner(&missing_target("Frame.missing/0"), None)
        .unwrap_err();
    assert!(matches!(error, VMError::InvalidBasicBlock(_)));
    assert_caller_frame(&interpreter);
}

#[test]
fn primary_error_is_retained_when_restore_validation_also_fails() {
    let mut interpreter = seeded_interpreter();
    let function = returning_integer("Frame.dual_error/0", 1);
    let error = with_function_frame(&mut interpreter, &function, None, |interpreter| {
        interpreter.call_depth = interpreter.call_depth.saturating_add(1);
        Err(VMError::DivisionByZero)
    })
    .unwrap_err();

    match error {
        VMError::DuringFrameRestore { primary, restore } => {
            assert!(matches!(*primary, VMError::DivisionByZero));
            assert!(restore.contains("fields=call_depth"), "{restore}");
        }
        other => panic!("unexpected error: {other:?}"),
    }
    assert_caller_frame(&interpreter);
}
