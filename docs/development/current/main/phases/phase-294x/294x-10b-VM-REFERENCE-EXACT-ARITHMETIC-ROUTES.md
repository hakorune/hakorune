---
Status: Complete
Date: 2026-05-12
Scope: VM reference execution consumes MIR-owned exact numeric Add/Sub/Mul
  route facts.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-10-VM-REFERENCE-EXACT-USIZE-ADD-ROUTE-V0.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-10b VM Reference Exact Arithmetic Routes

## Purpose

Close the checked arithmetic slice for the VM reference executor without
letting VM infer semantics from raw `Integer(i64)` values.

`294x-10` proved the execution hook with `BinOp::Add`. This row widens the
MIR-owned exact numeric binary-operation route facts to the remaining checked
plain arithmetic operations that already have numeric substrate policy:
`Add`, `Sub`, and `Mul`.

## Landed Shape

- Exact numeric binary-operation facts now carry a generic arithmetic source
  (`op + lhs + rhs`) instead of an Add-only source.
- MIR publishes route facts for `BinOp::Add`, `BinOp::Sub`, and `BinOp::Mul`
  only when both operands already share the same exact numeric value fact.
- Exact/dynamic and exact/exact mismatch rejection metadata applies to all
  three arithmetic ops.
- The VM reference executor consumes those MIR-owned route facts and uses
  `exact_numeric_checked_arithmetic(...)` before generic i64 fallback.

## Non-Goals

- No div/mod route consumption.
- No bitwise route consumption.
- No compare or shift route facts.
- No exact `VMValue` storage.
- No backend lowering.
- No hako_alloc field migration.

## Verification

- `cargo test -q --lib exact_numeric_`
- `cargo test -q --lib vm_reference_`
