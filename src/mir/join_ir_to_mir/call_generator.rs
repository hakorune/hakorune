//! Call Generator - Unified call instruction emission
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Eliminates repeated call generation shapes (3x duplication).
//!
//! ## Structure Before
//!
//! ```ignore
//! instructions.push(MirInstruction::Const {
//!     dst: func_name_id,
//!     value: ConstValue::String(func_name),
//! });
//! instructions.push(MirInstruction::Call {
//!     dst: Some(call_result_id),
//!     func: func_name_id,
//!     callee: None,
//!     args: args.to_vec(),
//!     effects: EffectMask::PURE,
//! });
//! ```
//!
//! ## Structure After
//!
//! ```ignore
//! emit_call_pair(&mut instructions, func_name_id, call_result_id, &func_name, &args);
//! ```

use crate::ast::Span;
use crate::mir::{ConstValue, EffectMask, MirInstruction, ValueId};

/// Emit Const + Call instruction pair
///
/// Generates a function name constant followed by a call instruction.
/// Used for both tail calls and non-tail calls.
///
/// # Arguments
///
/// * `instructions` - Target instruction vector to append to
/// * `func_name_id` - ValueId for function name constant
/// * `call_result_id` - ValueId for call result
/// * `func_name` - Function name string
/// * `args` - Call arguments
///
/// # Example
///
/// ```ignore
/// let mut instructions = vec![];
/// emit_call_pair(
///     &mut instructions,
///     ValueId(100),  // func_name_id
///     ValueId(101),  // call_result_id
///     "my_function",
///     &[ValueId(1), ValueId(2)],
/// );
/// // Result:
/// // instructions[0] = Const { dst: ValueId(100), value: "my_function" }
/// // instructions[1] = Call { dst: Some(ValueId(101)), func: ValueId(100), ... }
/// ```
pub fn emit_call_pair(
    instructions: &mut Vec<MirInstruction>,
    func_name_id: ValueId,
    call_result_id: ValueId,
    func_name: &str,
    args: &[ValueId],
) {
    instructions.push(MirInstruction::Const {
        dst: func_name_id,
        value: ConstValue::String(func_name.to_string()),
    });

    // Phase 188.3 P2: Set callee field for JoinIR function calls
    // JoinIR functions (main, loop_step, inner_step, k_exit, etc.) are global functions
    instructions.push(MirInstruction::Call {
        dst: Some(call_result_id),
        func: func_name_id,
        callee: Some(crate::mir::definitions::Callee::Global(
            func_name.to_string(),
        )),
        args: args.to_vec(),
        effects: EffectMask::PURE,
    });
}

/// Emit Const + Call instruction pair with spans
///
/// Same as emit_call_pair but also appends unknown spans to a span vector.
/// Used when emitting directly to BasicBlock.
///
/// # Arguments
///
/// * `instructions` - Target instruction vector to append to
/// * `spans` - Target span vector to append to
/// * `func_name_id` - ValueId for function name constant
/// * `call_result_id` - ValueId for call result
/// * `func_name` - Function name string
/// * `args` - Call arguments
pub fn emit_call_pair_with_spans(
    instructions: &mut Vec<MirInstruction>,
    spans: &mut Vec<Span>,
    func_name_id: ValueId,
    call_result_id: ValueId,
    func_name: &str,
    args: &[ValueId],
) {
    emit_call_pair(instructions, func_name_id, call_result_id, func_name, args);
    spans.push(Span::unknown());
    spans.push(Span::unknown());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_call_pair() {
        let mut instructions = vec![];
        emit_call_pair(
            &mut instructions,
            ValueId(100),
            ValueId(101),
            "test_func",
            &[ValueId(1), ValueId(2)],
        );

        assert_eq!(instructions.len(), 2);

        // Check Const instruction
        if let MirInstruction::Const { dst, value } = &instructions[0] {
            assert_eq!(*dst, ValueId(100));
            if let ConstValue::String(s) = value {
                assert_eq!(s, "test_func");
            } else {
                panic!("Expected ConstValue::String");
            }
        } else {
            panic!("Expected Const instruction");
        }

        // Check Call instruction
        if let MirInstruction::Call {
            dst,
            func,
            callee,
            args,
            effects,
        } = &instructions[1]
        {
            assert_eq!(*dst, Some(ValueId(101)));
            assert_eq!(*func, ValueId(100));
            // Phase 188.3 P2: callee should be set to Global(func_name)
            assert_eq!(
                *callee,
                Some(crate::mir::definitions::Callee::Global(
                    "test_func".to_string()
                ))
            );
            assert_eq!(args, &[ValueId(1), ValueId(2)]);
            assert_eq!(*effects, EffectMask::PURE);
        } else {
            panic!("Expected Call instruction");
        }
    }

    #[test]
    fn test_emit_call_pair_with_spans() {
        let mut instructions = vec![];
        let mut spans = vec![];

        emit_call_pair_with_spans(
            &mut instructions,
            &mut spans,
            ValueId(200),
            ValueId(201),
            "another_func",
            &[],
        );

        assert_eq!(instructions.len(), 2);
        assert_eq!(spans.len(), 2);

        // Verify both spans are unknown
        assert_eq!(spans[0], Span::unknown());
        assert_eq!(spans[1], Span::unknown());
    }

    #[test]
    fn test_emit_call_pair_empty_args() {
        let mut instructions = vec![];
        emit_call_pair(
            &mut instructions,
            ValueId(300),
            ValueId(301),
            "no_args_func",
            &[],
        );

        assert_eq!(instructions.len(), 2);

        if let MirInstruction::Call { args, .. } = &instructions[1] {
            assert!(args.is_empty());
        } else {
            panic!("Expected Call instruction");
        }
    }
}
