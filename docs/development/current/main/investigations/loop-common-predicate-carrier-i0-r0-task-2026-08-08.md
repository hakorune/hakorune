# LOOP-COMMON-PREDICATE-CARRIER-I0-R0

Status: `next implementation row after D1`
Date: `2026-08-08`
Design SSOT: `docs/development/current/main/investigations/loop-caller-zero-parity-g0-i1-design-d1-2026-08-08.md`

## Objective

Add the two common contracts required before Generic G0 physicalization:

1. resolve each `Predicate` transfer from its own completed condition value;
2. emit `DerivedCarrierEntry` through a profile-neutral prepared operation
   variant and canonical identity's `read_entry_receipt`.

This is one BoxShape implementation row. It does not allocate or emit
physical G0 and does not open a G0-specific physicalizer.

## Required changes

- remove the Callable-only single-condition assumption from the neutral After
  receipt and recursive transfer writer;
- build and validate a per-transfer condition receipt table before emission;
- preserve Callable's profile-close count/condition proof outside the neutral
  receipt;
- add the common prepared carrier-seed operation/emitter path;
- keep `VerifiedLoopOperationPhysicalDemandV1` full-program and move-only;
  add no first/select/filter extraction API;
- preserve canonical CFG, identity/Binding SSA, and PhiTxn as the sole
  physical owners.

## Acceptance

```text
Callable R3 focused gate remains green
root and child Predicate receipts reject wrong owner/type/block
DerivedCarrierEntry emits through canonical identity
expression-anchor behavior is unchanged
missing/duplicate/foreign/stale receipts reject before instruction emission
no G0 allocation, selector, fallback/retry, publication, or legacy removal
all touched source/check files < 800 lines
```

The same commit must update the affected `docs/reference/**` page, README,
focused tests/guards, current state, `10-Now.md`, and the active workstream.
The commit must state that the following G0 row is
`LOOP-CALLER-ZERO-PARITY-G0-I1-R0`; it must not claim G0 physical parity.
