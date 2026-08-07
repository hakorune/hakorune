# Loop operation physicalizer canary S0

Status: `IMPLEMENTATION-READY`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-PHYSICALIZER-DESIGN-STOP`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Change

Implement one caller-zero, test-only operation physicalizer canary using the
accepted move-only `VerifiedLoopOperationPhysicalDemandV1`. The canary may
lower exactly one `ConstI64` or `ReadBinding` operation through the existing
emitter and canonical BindingSSA service in a fresh unpublished function
session. It must not open production selection or legacy retirement.

## Contract

```text
profile adapter
  -> exact-move VerifiedLoopOperationPhysicalDemandV1
  -> session-local ReadyLoopEntryV1
  -> borrowed CanonicalCfgSessionV1 / BindingSSA / PhiTxn services
  -> one operation emission
  -> caller-zero receipt
```

The physicalizer owns no CFG/SSA/PHI truth. `CanonicalCfgSessionV1` owns block
creation and predecessor/seal state; the existing function session owns
BindingRef-to-ValueId and the sole `PhiTxn`; DraftSeal/collector remain
outside this canary. The canary must not receive AST, names, route/profile
labels, Tail, ABI, Completion, Return, DraftSeal, or the topology-only
`VerifiedLoopPhysicalBoundaryV1`.

All Stage-A checks run before Builder mutation:

```text
owner/frame/Scope/Region
item/effect membership and exact placement
entry/preheader/current continuation
supported operation/value class
available input/carrier obligation
```

After emission begins, any error poisons the unpublished function. Local
`PhiTxn` abort is diagnostic cleanup only; the enclosing session performs
whole-function discard and restores the caller once. Same-session repair,
retry, fallback, reselection, ID rollback, and product reuse are forbidden.

Generic item 3 is not part of the first one-operation canary unless the chosen
operation is that row. If it is selected, it remains a normal parent-body
`ReadBinding`, and the canary must issue a separate child-entry carrier-2 seed
receipt while preserving parent-block placement. It must not relabel the row
or infer placement from its `DerivedCarrierEntry` anchor.

## Done

- [ ] Add the private move-only demand and exact-move adapter boundary.
- [ ] Add one positive ConstI64 or ReadBinding canary and one fresh-session
      failure/discard canary.
- [ ] Add typed pre-effect rejects for missing/duplicate/foreign item/effect,
      wrong owner/frame/block/loop, unsupported value/op, missing input, and
      missing/terminated carrier entry.
- [ ] Assert no operation canary path uses AST/name/profile dispatch or the
      topology-only boundary.
- [ ] Keep every touched source/test file below 800 lines.
- [ ] Update code, focused tests, and the exact reference/README/current/
      workstream docs in the same commit.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib loop_recipe_physicalizer -- --nocapture
RUSTFLAGS='-Awarnings' cargo test --lib operation_effect -- --nocapture
RUSTFLAGS='-Awarnings' cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

## Stop

Return to design and record `NoSafeSlice` if the demand cannot bundle the
operation product and common continuation exactly once, if any second CFG/
SSA/PHI owner is needed, if a physical failure cannot discard the whole fresh
session, or if the canary needs profile/legacy dispatch. Do not add a second
physicalizer or a fallback route.

Explicit non-claims for this row:

```text
no production caller or selector
no Return/Completion/DraftSeal/publication
no M8/M9 all-route coverage
no retry/fallback removal
no legacy deletion
```

## Same-commit documentation obligation

The implementation commit must update:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
```

Reference docs may claim only the landed canary receipt. Production,
backend-performance, selector, retry/fallback, and legacy-deletion claims
remain closed until their own rows land.
