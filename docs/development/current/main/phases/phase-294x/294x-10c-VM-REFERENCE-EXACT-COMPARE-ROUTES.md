---
Status: Complete
Date: 2026-05-12
Scope: VM reference execution consumes MIR-owned exact numeric compare route
  facts.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-10b-VM-REFERENCE-EXACT-ARITHMETIC-ROUTES.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-10c VM Reference Exact Compare Routes

## Purpose

Land unsigned compare reference execution without making VM infer exact numeric
semantics from raw `Integer(i64)` values.

MIR already owns exact numeric value facts and exact compare policy. This row
adds a Bool-producing compare route fact so the VM can consume that policy only
when the MIR metadata proves both operands are the same exact numeric type.

## Landed Shape

- MIR publishes exact numeric compare route facts for `Compare` instructions
  whose operands share the same exact numeric value fact.
- MIR records compare route rejections for exact/dynamic and exact/exact type
  mismatches.
- Compare route facts are separate from arithmetic route facts because compare
  produces canonical `Bool`, not another exact numeric value.
- The VM reference executor consumes the compare route fact, converts operands
  through exact numeric range checks, and runs `exact_numeric_compare(...)`
  before generic compare fallback.

## Non-Goals

- No shift route consumption.
- No div/mod or bitwise route consumption.
- No exact `VMValue` storage.
- No backend lowering.
- No hako_alloc field migration.

## Verification

- `cargo test -q --lib exact_numeric_`
- `cargo test -q --lib vm_reference_`
