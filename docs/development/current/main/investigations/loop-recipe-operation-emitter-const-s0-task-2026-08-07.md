# Loop recipe ConstI64 leaf-emitter canary S0

Status: `landed`
Date: 2026-08-07
Parent: `LOOP-RECIPE-PHYSICAL-BLOCK-RECEIPT-P0 / Decision B`
Authority: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Change

Add one private leaf emitter for a prepared `ConstI64` operation. The emitter
must bind an already prepared operation to one exact
`LoopPhysicalBlockReceiptV1` row before emitting one MIR `Const` through the
existing canonical Builder emission/type-fact path.

The first canary uses an explicit test-only prepared constructor. It must not
extract a row from a seven-operation Callable or fifteen-operation Generic G0
full demand. The emitter receives no Recipe, profile, Tail, ABI, Completion,
Return, DraftSeal, publication, or Loop continuation.

## Contract

```text
PreparedLoopOperationEmissionV1
  owner
  item
  LoopOperationV1::ConstI64 { result, value }
  expected loop/block/role

emit_prepared_operation_v1
  -> exact owner and placement validation
  -> one canonical Const instruction
  -> one emitted ValueId receipt
```

The physical block receipt is the only placement authority. `current_block`
may not silently select the destination. Emission must use the existing
Builder/type-fact owner; no second CFG, SSA, PHI, transaction, or operation
route may be introduced. A late canary failure is handled by the harness after
emission and the whole unpublished session is discarded; the production
emitter has no retry/fallback branch.

## Done

- [x] Add the private prepared Const payload and one leaf emitter under the
      physicalizer directory; keep every touched source/test file below 800
      lines.
- [x] Validate owner, preheader, Loop, logical Block, role, and destination
      function membership before emission.
- [x] Prove one `ConstValue::Integer` instruction appears in the exact target
      block, with an exact `i64` type fact and one result receipt.
- [x] Add negative placement/owner/preheader tests with zero instruction
      emission on pre-emission rejection.
- [x] Add a harness-only post-emission failure, whole-session discard, and
      fresh-session repeat proof without a production test branch.
- [x] Update current state, workstream, design/reference README, and this card
      in the implementation commit. After implementation, update the relevant
      reference documentation with the landed contract and non-claims.

Implementation receipt: focused tests, lib check, source guards, and the
reference/current documentation update are landed in the implementation
commit for this row. The next operation kind is intentionally not selected by
this canary.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib loop_recipe_physicalizer -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo test --lib operation_physical_demand -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo check --lib
rustfmt --edition 2021 --check <touched Rust files>
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

## Stop

Return to the physicalizer design stop if the row needs full-demand
single-operation extraction, source/AST rereading, BindingSSA/PHI mutation,
continuation handling, function session/Completion/DraftSeal changes,
production selection, retry/fallback, or legacy deletion. Those are later
rows and must not be smuggled into this canary.
