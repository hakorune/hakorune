---
Status: Complete
Date: 2026-05-12
Scope: MIR refresh owner for dynamic exact numeric field-write range-check contracts.
Related:
  - src/mir/exact_numeric_field_contracts.rs
  - src/mir/compiler/mod.rs
  - src/mir/semantic_refresh.rs
  - src/mir/verification/numeric_substrate.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-06e Dynamic Range Contract Refresh

## Goal

Attach `DynamicIntegerRange` contracts from real MIR `FieldSet` producers after
the MIR shape is stable, instead of requiring hand-written metadata in tests or
future lowerers.

This row keeps the verifier and the contract refresher on one shared
field-assignment facts owner so the exact numeric acceptance rule is not copied
between phases.

## Changes

- Added `src/mir/exact_numeric_field_contracts.rs` as the MIR owner for exact
  numeric field-write facts and runtime-check contract refresh.
- Added
  `refresh_module_exact_numeric_runtime_check_contracts(module: &mut MirModule)`.
- `MirCompiler` now refreshes those contracts after optimization and before
  `MirVerifier::verify_module(...)`, avoiding instruction-index drift.
- `refresh_module_semantic_metadata(...)` also refreshes the contracts for
  module-level metadata refresh callers.
- The verifier now consumes the shared findings instead of carrying a duplicate
  field-assignment scanner.

## Accepted Surface

For a real `FieldSet` into an exact numeric declared field:

- statically known integer writes still use the verifier's range violation
  path and are not hidden by a runtime contract;
- dynamic writes to `usize`, unsigned, or narrower signed fields receive a
  `DynamicIntegerRange` contract when the dynamic `Integer(i64)` lane does not
  fit the target range;
- dynamic writes to `i64` do not receive a contract because the current dynamic
  lane already fits;
- repeated refresh is idempotent.

## Stop Line

This row does not add:

- exact `usize` runtime value variants;
- checked arithmetic;
- unsigned comparison or logical shift;
- non-VM backend execution of the contracts;
- backend fail-fast for exact numeric contracts;
- typed-object exact numeric storage;
- hako_alloc field migration.

## Next Rows

The next safe order is:

1. fail fast on unsupported non-VM backend routes when exact numeric contracts
   are present;
2. add exact numeric value/operation rows;
3. add checked arithmetic / unsigned compare / logical shift;
4. resume hako_alloc non-negative field migration only after the relevant
   backend/runtime rows are green.

## Proof

```bash
cargo test -q exact_numeric_field_contracts --lib
cargo test -q compile_attaches_dynamic_integer_range_contract_before_verify --lib
cargo test -q numeric_substrate --lib
cargo test -q numeric_contracts --lib
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
