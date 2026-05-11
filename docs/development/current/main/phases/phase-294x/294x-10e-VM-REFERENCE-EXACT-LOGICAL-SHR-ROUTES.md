---
Status: Complete
Date: 2026-05-12
Scope: VM reference execution consumes MIR-owned exact numeric logical right
  shift route facts.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-10d-VM-EXACT-OPS-MODULE-SPLIT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-10e VM Reference Exact Logical Shr Routes

## Purpose

Land the first exact numeric shift execution path needed by `usize`-oriented
allocator code while preserving the existing dynamic `Integer(i64)` lane.

Generic `Shr` keeps its current signed `i64` behavior. This row only changes
execution when MIR has published an exact unsigned left-operand route fact.

## Landed Shape

- MIR publishes exact logical `Shr` route facts when the left operand has an
  exact unsigned numeric value fact.
- MIR propagates the exact numeric value fact through that `Shr` result.
- MIR records a signed-left rejection instead of silently treating signed exact
  numeric values as logical shifts.
- The VM reference executor consumes the route fact, range-checks the exact
  left operand, checks the dynamic shift count, and runs
  `exact_numeric_logical_shr(...)` before generic `i64` fallback.

## Non-Goals

- No `Shl` route consumption.
- No bitwise route consumption.
- No exact `VMValue` storage.
- No backend lowering.
- No hako_alloc field migration.

## Verification

- `cargo test -q --lib exact_numeric_`
- `cargo test -q --lib vm_reference_`
