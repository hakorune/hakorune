---
Status: closed — implementation receipt
Date: 2026-08-09
Decision: repair the duplicate-definition fixture without widening verifier semantics
Parent: `loop-recipe-v2-typed-schema-callslot-i0-task-2026-08-08.md`
---

# LOOP-RECIPE-V2-TYPED-SCHEMA-CALLSLOT-I0-CLOSEOUT-R0

## Purpose

Close the already-landed V2 schema receipt. The focused failure was a test
fixture problem, not evidence that the verifier needs a new validation-order
authority.

The existing test changes a `TextEq` result to value key `2`, which is a
previously defined `Text` result of `CallSlot`. That mutation violates both:

```text
duplicate value definition
TextEq result must be Bool
```

The test therefore observes the deterministic `TextEqResultClassMismatch`
first. It is not a valid isolated duplicate-definition fixture.

## Minimal change

Change only the test mutation so the earlier `CallSlot` result is changed to
the existing `Bool` value key `3`, while the `TextEq` result remains key `3`.
This creates exactly one duplicate definition with matching value class.

```text
CallSlot result: key 3 (Bool)
TextEq result:   key 3 (Bool)
  -> DuplicateValueDefinition(key 3)
```

Do not reorder verifier checks, add a second identity phase, widen V2, or add
new operation/value vocabulary.

## Landed receipt

`typed_schema_v2_tests.rs` now has seven focused tests: round-trip, operand
class rejection, result class rejection, duplicate result rejection, unknown
wire fields, schema version rejection, and unknown CallSlot argument. No
verifier implementation or Recipe vocabulary changed.

## Acceptance

```text
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust typed_schema_v2_tests
  -> all focused V2 tests pass

git diff --check
bash tools/checks/current_state_pointer_guard.sh
all changed Rust files < 760 lines
```

The implementation remains Builder-free, source-observer-free, resolver-free,
and physicalization-free. The same commit updates the V2 task capsule, the
loop Recipe reference receipt, and this closeout card.

## Nonclaims

```text
no resolver instance target
no source-bound call relation
no ScanWithInit observer/producer
no Builder/MIR/CFG/PHI/ABI lowering
no production selection, fallback, retry, or legacy deletion
```
