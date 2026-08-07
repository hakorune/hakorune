# Generic G0 operation/effect anchor ledger S0

Status: `CLOSED`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EFFECT-CALLABLE-ADAPTER-S0`
Authority:
`docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md`

## Purpose

Issue an explicit Generic G0 item-to-source-anchor ledger before the existing
G0 producer drops its structural source facts. Adapt that ledger to the
neutral `LoopOperationSourceEvidenceV1` product exactly as the callable
adapter does. This row is still caller-zero and passive.

## Required item ledger

The producer must issue these exact item identities and anchors from its
resolver-backed source facts before any source view is consumed:

```text
item 0  outer condition lhs
item 1  outer condition rhs
item 2  outer condition result
item 3  C2 DerivedCarrierEntry / child entry
item 5  inner condition lhs
item 6  inner condition rhs
item 7  inner condition result
item 8  inner update lhs
item 9  inner update rhs
item 10 inner update value
item 11 inner update target
item 12 outer update lhs
item 13 outer update rhs
item 14 outer update value
item 15 outer update target
```

Item 4 is the nested `Loop` item and is outside operation evidence. C0/C1
carrier rows and Generic tail reads remain with their existing owners and must
not be silently relabeled as operation anchors.

## Contract

```text
source facts -> explicit item/anchor ledger -> neutral S0 product
```

Item keys are the only identity. Source preorder, role ordinal, fixture name,
or Generic profile label cannot match rows. The ledger must preserve owner,
loop, block, source-site, and (for binding operations) exact Core effect
relation. Duplicate, missing, foreign-owner, wrong-loop/block, dropped-before-
issuance, and operation/effect mismatch cases are typed `NoSafeSlice`.

Still closed:

```text
operation MIR / ValueId / BasicBlockId / PHI
cross-profile parity claim
production selector
retry / fallback
legacy deletion
```

## Acceptance gates

```text
cargo test --lib generic_g0 -- --nocapture
cargo test --lib operation_effect -- --nocapture
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

All touched source/test files remain below 800 lines. The row closes only when
the ledger is issued before source facts are dropped and the neutral product
passes positive, duplicate, missing, foreign, and wrong-placement checks.

## Same-commit documentation obligation

When this ledger lands, update in the same commit:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
docs/development/current/main/10-Now.md
```

References may claim only the Generic anchor ledger and passive adapter
evidence that actually landed. Cross-profile parity and physicalization need
separate receipts.

## Exit and next row

The next row after this ledger is cross-profile callable/G0 evidence parity.
Only after parity is sealed may the operation physicalizer be designed or
implemented.

## Implementation receipt (2026-08-07)

The caller-zero Generic G0 producer now issues the exact 15-row ledger before
its structural source facts leave the producer boundary. Item 3 is represented
as the existing child-loop `DerivedCarrierEntry` (carrier 2), while item 4,
C0/C1 carriers, and the Generic tail read remain outside this operation
product. The ledger is adapted directly into the neutral
`VerifiedLoopOperationEffectProductV1`; Recipe item placement and Core effect
relations remain the only semantic authorities.

The focused Generic G0 suite (42 tests) and operation/effect suite (6 tests)
are green. The neutral product's duplicate, missing, foreign-owner, and
wrong-placement rejects remain covered. No Builder/MIR, parity, selector,
retry/fallback, or legacy-deletion authority opened.

The next implementation-ready row is the cross-profile callable/G0 evidence
parity task. Its reference updates are required in the same commit as its
implementation.
