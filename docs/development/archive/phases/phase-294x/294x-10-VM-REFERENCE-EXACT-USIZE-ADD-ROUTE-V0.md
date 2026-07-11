---
Status: Complete
Date: 2026-05-12
Scope: VM reference execution consumes MIR-owned exact numeric `BinOp::Add`
  route facts.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-10 VM Reference Exact Usize Add Route V0

## Purpose

Move the first exact numeric operation from diagnostic metadata into VM
reference execution without making VM the product/backend owner.

The VM now consumes the existing MIR-owned exact numeric `BinOp::Add` route
fact and runs it through the numeric substrate checked arithmetic policy before
the legacy generic `Integer(i64)` fast path can observe the instruction.

## Landed Shape

- `src/backend/mir_interpreter/exec/exact_numeric_ops.rs` owns the VM reference
  exact numeric operation prehook.
- The prehook matches `block + instruction_index + dst + op + lhs + rhs`
  against `FunctionMetadata.exact_numeric_binary_op_route_facts`.
- Exact operands are converted through
  `exact_numeric_value_from_dynamic_integer(...)`.
- The add result is produced through `exact_numeric_checked_arithmetic(...)`.
- Negative unsigned operands and exact numeric overflow fail fast with stable
  VM diagnostics instead of falling through to generic i64 arithmetic.
- Results still publish to the current VM `Integer(i64)` lane only when the
  checked exact result fits that lane.
- A `usize` result above the current VM `Integer(i64)` publication lane fails
  fast until a later row adds exact numeric VM value/storage.

## Non-Goals

- No new `VMValue` variant.
- No VM-only type inference from raw `Integer(i64)` values.
- No `Compare`, shift, bitwise, div/mod, `Sub`, or `Mul` route consumption.
- No typed-object exact numeric storage.
- No non-VM backend lowering.
- No hako_alloc field migration.

## Diagnostics

- `[vm/exact_numeric_op_type]`
- `[vm/exact_numeric_op_range]`
- `[vm/exact_numeric_op_overflow]`
- `[vm/exact_numeric_op_result_unrepresentable]`

## Verification

- `cargo test -q --lib vm_reference_`
- `cargo test -q --lib exact_numeric_`
- `cargo check --release --bin hakorune`
