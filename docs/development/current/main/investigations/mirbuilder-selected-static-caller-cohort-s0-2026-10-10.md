# Selected Static caller cohort S0

Status: selected execution
Date: 2026-10-10
Scope: MIRBUILDER-SELECTED-STATIC-CALLER-COHORT-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-object-guarded-outgoing-transport-d0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-closed-app-static-selection-s0-2026-10-10.md
  - docs/development/current/main/investigations/mirbuilder-explicit-exe-static-selection-s0-2026-10-10.md

## Contract

Prerequisite: `8b4339ecfb` confines closed-App physical selection to
explicit EXE requests. The reusable package issuer retains the complete
source cohort; this selected incoming view applies only after the EXE scope
was chosen. Existing uncommitted implementation in a separate checkout must
be reconciled with that boundary before this S0 can close.

Keep the complete original Static claim index and declarations. Derive the
exact omitted App Static caller batch slots from the already co-sealed
source-backed catalog and resolved batch. The ordinary incoming walk must
exclude only those caller slots from executable observations. It must still
reject an unknown or unmatched unselected batch declaration and every
unsupported selected caller. No new source scan, physical selection map,
transport packet, or Static seed policy is allowed.

The same-brand omitted loan needs parser identity, canonical Static key, and
batch slot correspondence. The original `accepts -> class_id` row remains in
`QualifiedStaticCallClaimIndexV1`; `BorrowedIncomingInventoryV1` is the
selected executable caller view. Any current method called a complete
original cohort must be renamed or documented as selected-only.

## Acceptance

- On unchanged mimalloc-lite, retain all 15 Heap source callers and the
  selected `Heap -> LayoutBox.class_id` edge. Retain the original
  `LayoutBox.accepts -> class_id` claim row, but it contributes no executable
  incoming/context veto. The selected class_id seed may advance only if all
  other existing conditions pass.
- Adding a real incoming call to `LayoutBox.accepts` reselects it and restores
  its CurrentOwner/context veto. A selected unsupported caller continues to
  veto; foreign or mismatched identity fails closed. Script/library behavior
  remains total.
- Run focused positive/negative tests and the required family guard, then
  probe unchanged mimalloc-lite to record the actual first stop. A zero-test
  filter or prior EXE probe is not acceptance.

Non-claims: no executable tagged actual, result Completion, Page edge, Loop
Mul, old-edge retirement, or MirBuilder goal completion.
