---
Status: Complete
Date: 2026-05-12
Scope: MIR metadata contract for dynamic exact numeric field-write range checks.
Related:
  - src/mir/function/types.rs
  - src/mir/verification/numeric_substrate.rs
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-06c Runtime Check Contract Metadata

## Goal

Give the verifier and later lowering rows a shared MIR metadata contract for
dynamic exact numeric field writes that require a runtime range check.

This row does not execute the check. It only defines and verifies the contract
shape so later VM/backend rows can consume one owner instead of inventing a
second ad hoc marker.

## Changes

- Added `ExactNumericRuntimeCheckContract` to function metadata.
- Added `ExactNumericRuntimeCheckContractKind::DynamicIntegerRange`.
- The contract is anchored to a `FieldSet` site by:
  - block;
  - instruction index;
  - field name;
  - value id;
  - declared exact numeric type name.
- The numeric verifier now accepts a dynamic exact numeric field write only when
  a matching `DynamicIntegerRange` contract is present.
- Mismatched contracts do not suppress
  `[mir/verify:numeric_dynamic_check_required]`.

## Accepted Surface

For a runtime-range-sensitive exact numeric field:

- no contract: verifier rejects unchecked dynamic writes;
- matching `DynamicIntegerRange` contract: verifier accepts the write as
  contract-backed;
- mismatched type/value/site contract: verifier still rejects.

The contract means "a later lowering/runtime row must materialize the range
check here." It is not a silent permission to use the old `Integer(i64)` lane.

## Stop Line

This row does not add:

- a new MIR instruction for the check;
- VM exact `usize` values;
- VM execution of the range check;
- backend lowering;
- checked arithmetic;
- unsigned comparison or logical shift;
- typed-object exact numeric storage;
- hako_alloc field migration.

## Next Rows

The next safe order is:

1. VM/runtime construction or check execution for `DynamicIntegerRange`;
2. backend unsupported-route fail-fast for exact numeric contracts;
3. checked arithmetic policy;
4. unsigned compare and logical shift;
5. PHI/Select exact numeric unification.

## Proof

```bash
cargo test -q numeric_substrate --lib
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
