---
Status: Complete
Date: 2026-05-12
Scope: Split VM exact numeric reference execution internals by operation family.
Related:
  - docs/development/current/main/phases/phase-294x/294x-10b-VM-REFERENCE-EXACT-ARITHMETIC-ROUTES.md
  - docs/development/current/main/phases/phase-294x/294x-10c-VM-REFERENCE-EXACT-COMPARE-ROUTES.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-10d VM Exact Ops Module Split

## Purpose

Keep the VM semantic reference executor maintainable before the next exact
numeric operation rows land.

The exact numeric reference executor had grown past the local readability
threshold after arithmetic and compare route consumption landed. This row is a
behavior-preserving BoxShape cleanup: it does not add an accepted operation or
change VM semantics.

## Landed Shape

- `exec/exact_numeric_ops/mod.rs` is the module entry.
- `arithmetic.rs` owns checked Add/Sub/Mul route consumption.
- `compare.rs` owns exact compare route consumption.
- `helpers.rs` owns shared operand extraction and range diagnostics.
- `tests.rs` keeps the VM reference regression tests together.

## Non-Goals

- No new exact numeric operation.
- No shift route consumption.
- No backend lowering.
- No hako_alloc field migration.

## Verification

- `cargo test -q --lib exact_numeric_`
- `cargo test -q --lib vm_reference_`
