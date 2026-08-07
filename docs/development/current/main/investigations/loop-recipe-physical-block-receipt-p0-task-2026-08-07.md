# Loop recipe physical block receipt P0

Status: `landed logical-to-physical receipt 2026-08-07; leaf emission remains closed`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-PHYSICALIZER-DESIGN-STOP / Decision B`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Change

Add one private receipt that binds the existing canonical CFG allocation to
the logical Loop topology. The receipt is issued by the current topology
physicalizer and is the only placement authority exposed to a later leaf
emitter:

```text
LoopPhysicalBlockReceiptV1 {
  owner
  preheader
  rows: logical Loop + role (+ optional logical Block) -> BasicBlockId
}
```

Roles are the fixed topology roles `Preheader`, `Header`, `Body`, `Step`, and
`After`. The receipt owns no CFG state; it only records the exact blocks
created by `CanonicalCfgSessionV1`. The existing topology/After canary keeps
its observable API by querying this receipt rather than storing a second block
map.

## Non-claims

```text
no new operation shape
no operation instruction emission
no BindingSSA/PHI mutation
no function session / Completion / DraftSeal
no continuation-to-leaf handoff
no production selector / retry / fallback
no legacy deletion
```

Instruction emission must not use `current_block` as an implicit placement.
The later Const leaf row must bind its expected owner, Loop, logical Block,
role, preheader, and function state against this receipt before emitting.

## Acceptance

- [x] Add a move-only/private `LoopPhysicalBlockReceiptV1` and fixed role
      vocabulary under the topology facade.
- [x] Build the receipt from the existing canonical CFG allocation with one
      exact row per Loop/role and no duplicate physical-map owner.
- [x] Preserve nested Generic G0 topology and pre-allocation entry rejection.
- [x] Add positive lookup coverage and typed duplicate/missing/foreign placement
      rejection before any later emitter is opened.
- [x] Keep operation demand, leaf emitter, BindingSSA, PHI, session, and
      production callers unchanged/zero.
- [x] Keep every touched source/test file below 800 lines.
- [x] Update current state, workstream, design/reference README, and this card
      in the same implementation commit.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib loop_recipe_physicalizer -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo test --lib operation_physical_demand -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

## Stop

Return to the physicalizer design stop if receipt construction needs a second
CFG/SSA/PHI owner, infers a logical Block from operation evidence, or requires
an operation emitter/Completion/session change. Those belong to later rows.
