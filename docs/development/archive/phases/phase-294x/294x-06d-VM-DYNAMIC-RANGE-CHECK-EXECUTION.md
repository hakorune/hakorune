---
Status: Complete
Date: 2026-05-12
Scope: VM execution of existing dynamic exact numeric field-write range-check contracts.
Related:
  - src/backend/mir_interpreter/exec/numeric_contracts.rs
  - src/backend/mir_interpreter/exec/block.rs
  - src/mir/function/types.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-06d VM Dynamic Range Check Execution

## Goal

Consume the existing `DynamicIntegerRange` function metadata contract in the VM
interpreter so dynamic exact numeric field writes fail fast at the `FieldSet`
site instead of relying on a later ad hoc checker.

This row is intentionally narrow: it executes contracts that are already present
in MIR function metadata. It does not create those contracts from source or
claim exact `usize` runtime values.

## Changes

- Added `src/backend/mir_interpreter/exec/numeric_contracts.rs` as the VM owner
  for exact numeric runtime-check contract execution.
- `FieldSet` block execution now checks for a matching
  `ExactNumericRuntimeCheckContractKind::DynamicIntegerRange` contract before
  mutating the field.
- The VM rejects:
  - non-integer runtime values with `[vm/numeric_dynamic_range_type]`;
  - negative dynamic values for unsigned exact numeric fields with
    `[vm/numeric_dynamic_range] reason=negative-to-unsigned`;
  - out-of-range dynamic values with
    `[vm/numeric_dynamic_range] reason=out-of-range`.
- If no matching contract exists, VM execution remains unchanged. The verifier
  remains responsible for requiring contracts where exact numeric dynamic field
  writes need them.

## Accepted Surface

For a `FieldSet` with a matching `DynamicIntegerRange` contract:

- the runtime value must be the current dynamic `Integer(i64)` value;
- the value must fit the contract's declared exact numeric type;
- failure is reported before field mutation.

This gives the VM a real fail-fast execution point while keeping exact numeric
value representation deferred.

## Stop Line

This row does not add:

- automatic contract insertion from source lowering;
- a new MIR instruction for the check;
- VM exact `usize` value variants;
- checked arithmetic;
- unsigned comparison or logical shift;
- backend/LLVM lowering of exact numeric checks;
- typed-object exact numeric storage;
- hako_alloc field migration.

## Next Rows

The next safe order is:

1. attach/insert `DynamicIntegerRange` contracts from real MIR field-write
   producers;
2. make unsupported backend routes fail fast when exact numeric contracts are
   present;
3. add exact numeric value/operation rows;
4. add checked arithmetic / unsigned compare / logical shift;
5. resume hako_alloc non-negative field migration only after the relevant
   backend/runtime rows are green.

## Proof

```bash
cargo test -q numeric_contracts --lib
cargo test -q numeric_substrate --lib
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
