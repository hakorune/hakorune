//! Phase 100 P3-2: String Accumulator Emitter
//!
//! Dedicated emitter for string concatenation in JoinIR (VM/LLVM same semantics).
//!
//! # Responsibility
//!
//! Emit JoinIR instructions for string concatenation: `out = out + ch`
//! where ch is a Variable (not Literal, not MethodCall).
//!
//! # Design
//!
//! String concatenation uses BinOp(Add) just like integer addition.
//! Type semantics are enforced by:
//! 1. P3-1: AccumulatorKind detection (Int vs String)
//! 2. P3-3: Pattern2 wiring validation (RHS must be Variable, string-typed)
//! 3. This emitter: Emit BinOp(Add) for string operands
//!
//! The VM and LLVM backends handle BinOp(Add) polymorphically:
//! - Integer + Integer → integer addition
//! - String + String → string concatenation (via AddOperator.apply/2)
//!
//! # Phase 100 P3 Contract
//!
//! **Allowed**: `out = out + ch` where ch ∈ {Variable (string-typed)}
//! **Fail-Fast**: Literal RHS, MethodCall RHS, non-string RHS

use crate::mir::join_ir::{BinOpKind, JoinInst, MirLikeInst};
use crate::mir::ValueId;

/// Emit string concatenation in JoinIR
///
/// # Arguments
///
/// * `target_id` - ValueId of the accumulator variable (e.g., "out")
/// * `rhs_id` - ValueId of the RHS variable (e.g., "ch")
/// * `alloc_value` - ValueId allocator closure
/// * `instructions` - Output vector to append instructions to
///
/// # Returns
///
/// ValueId of the concatenation result
///
/// # Example
///
/// ```ignore
/// // For "out = out + ch":
/// let out_next = emit_string_concat(
///     out_param,     // ValueId of "out" parameter
///     ch_value,      // ValueId of "ch" body-local variable
///     &mut alloc_value,
///     &mut instructions,
/// )?;
/// // Generates:
/// //   out_next = BinOp(Add, out_param, ch_value)
/// ```
#[allow(dead_code)]
pub fn emit_string_concat(
    target_id: ValueId,
    rhs_id: ValueId,
    alloc_value: &mut dyn FnMut() -> ValueId,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String> {
    // Phase 100 P3-2: Emit BinOp(Add) for string concatenation
    // Type semantics enforced by Pattern2 wiring (P3-3)
    let result = alloc_value();
    instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
        dst: result,
        op: BinOpKind::Add,
        lhs: target_id,
        rhs: rhs_id,
    }));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_string_concat() {
        // Phase 100 P3-2: Unit test for string concat emission
        let mut value_counter = 10u32;
        let mut alloc_value = || {
            let id = ValueId(value_counter);
            value_counter += 1;
            id
        };

        let target_id = ValueId(1); // "out" parameter
        let rhs_id = ValueId(2);    // "ch" body-local

        let mut instructions = Vec::new();

        let result = emit_string_concat(
            target_id,
            rhs_id,
            &mut alloc_value,
            &mut instructions,
        ).unwrap();

        // Verify result
        assert_eq!(result, ValueId(10));

        // Verify instruction
        assert_eq!(instructions.len(), 1);
        match &instructions[0] {
            JoinInst::Compute(MirLikeInst::BinOp { dst, op, lhs, rhs }) => {
                assert_eq!(*dst, ValueId(10));
                assert_eq!(*op, BinOpKind::Add);
                assert_eq!(*lhs, target_id);
                assert_eq!(*rhs, rhs_id);
            }
            _ => panic!("Expected BinOp instruction"),
        }
    }
}
