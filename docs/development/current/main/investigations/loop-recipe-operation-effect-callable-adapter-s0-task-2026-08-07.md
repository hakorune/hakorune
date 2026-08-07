# Callable operation/effect adapter S0

Status: `IMPLEMENTATION-READY`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EFFECT-S0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Adapt the existing callable `VerifiedLoopOperationSourceRelationV1` rows to
the neutral `LoopOperationSourceEvidenceV1` ledger accepted by the passive
operation/effect product. This is a caller-zero source adapter. It must not
open operation MIR, a physicalizer, a selector, retry/fallback, or production
publication.

## Sole ownership and conversion

```text
callable source-ledger relation
  -> exact item/anchor/loop/block evidence
  -> VerifiedLoopOperationEffectProductV1::issue
```

The callable relation is the source producer for this profile. Its transient
operation view is consumed only to prove equality with the already sealed
`LoopRecipeV1` item. The neutral evidence row retains no copied operation,
operand, binding-effect catalog, or alternative Recipe truth.

The adapter must reject duplicate, missing, foreign-owner, foreign-loop,
wrong-block, invalid-source, and operation/item mismatches before the neutral
product is issued. Item identity comes from the Recipe key; source preorder,
role ordinal, method name, or profile-specific labels are not matching keys.

## Allowed scope

```text
callable source relation -> neutral evidence adapter
focused positive/negative tests
exact reference receipts
```

Still closed:

```text
Generic G0 anchor ledger
cross-profile parity claim
operation physicalization
Builder / ValueId / BasicBlockId / PHI
Return / DraftSeal / module publication
production selector
retry / fallback / legacy deletion
```

## Acceptance gates

```text
cargo test --lib callable_single_loop -- --nocapture
cargo test --lib operation_effect -- --nocapture
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

The adapter and test sources remain below 800 lines. A focused failure must
return to the callable source/evidence contract; no workaround route may be
added.

## Same-commit documentation obligation

When this adapter lands, update in the same commit:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
docs/development/current/main/10-Now.md
```

References may claim only callable-adapter evidence that actually landed.
Generic G0 parity and physical operation emission require their own receipts.

## Exit and next row

Close this row only after the existing callable ledger is converted once,
neutral-product positive/negative parity is green, and no physical caller is
introduced. The next row is the explicit Generic G0 anchor ledger, followed by
cross-profile parity; operation physicalization remains closed until both are
complete.
