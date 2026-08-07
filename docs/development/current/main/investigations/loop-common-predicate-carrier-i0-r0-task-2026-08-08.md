# LOOP-COMMON-PREDICATE-CARRIER-I0-R0

Status: `closed — implementation receipt 2026-08-08`
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

## Implementation receipt (2026-08-08; Decision: accepted)

The common I0 is closed without opening G0 physical allocation. The neutral
After handoff now stores only owner, root After, and predecessor facts. Each
`LoopPhysicalTransferV1::Predicate` resolves and validates its own completed
Bool receipt against the transfer's physical source block; Callable's seven
row/count and condition-key proof remains in the outer profile close.

`DerivedCarrierEntry` is now a full-program projection in its own
`operation_carrier_demand` module. The common dispatcher admits it as a
profile-neutral `CarrierSeed` row and the private emitter uses canonical
identity `read_entry_receipt`; it does not fabricate an expression site or
create a G0-specific physical owner. Ordinary expression reads remain on the
existing read leaf. A focused Callable physicalizer suite remains green
(25/25), Generic demand evidence recognizes exactly one item-3 carrier row,
and all touched source files remain below 800 lines.

The next bounded row is `LOOP-CALLER-ZERO-PARITY-G0-I1-R0`. G0 allocation,
production selection, retry/fallback retirement, publication, and legacy
deletion remain closed. The exact reference, README, current pointers,
dashboard, and workstream are updated in this implementation commit; the
post-cutover reference closeout remains a required later task.
