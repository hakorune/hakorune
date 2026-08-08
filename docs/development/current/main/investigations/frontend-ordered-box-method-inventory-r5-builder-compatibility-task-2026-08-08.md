---
Status: active — R5-S1 landed; the next Builder compatibility caller remains
open
Date: 2026-08-08
Parent: `callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R5

## Goal

Remove the remaining Builder-side assumptions that Box methods are an
unordered AST map. Keep the ordered AST inventory and explicit compatibility
projections as the only inputs, then delete old map helpers when their callers
reach zero.

## Authority

```text
parser AST:
  BoxMethodInventoryV1 is the only ordered source carrier

compatibility projection:
  named lookup/order is allowed only at an audited legacy edge

resolver/source seal:
  not opened by R5

Builder/MIR:
  consumes an explicit projection and cannot reconstruct source order
```

## Implementation order

1. Inventory all remaining `HashMap`/name-sort method projections and classify
   each caller as durable, compatibility-only, or retire candidate.
2. Replace one named Builder edge with an explicit inventory projection; add a
   focused positive/negative gate and keep the old edge out of that caller.
3. Repeat only for the next named caller after the fast gate is green. Do not
   widen the AST contract or add a fallback.
4. Delete an old helper only after caller-zero evidence is recorded.
5. Update the owner README, affected language/reference receipt, this card,
   task map, and `CURRENT_STATE.toml` in the same implementation commit.

## Stop lines

```text
no resolver capability
no Hako parser parity claim
no CallableContract issuer
no source order from HashMap/name sorting
no compatibility-row promotion
no Builder retry/fallback
no broad cleanup mixed with a single caller migration
```

## Preflight census receipt — R5-S1

The first bounded Builder edge is selected, but implementation is not yet
opened because the worktree contains unrelated Loop/Recipe changes. R5 must
start from a clean boundary (commit or user-directed stash); those changes
must not be reset, folded into R5, or silently rewritten.

Selected first edge:

```text
PreparedProgramDeferredStaticBoxWorkV1
  -> into_compatibility_map()
  -> ProgramDeferredStaticBoxLifecycleV1::new()
  -> from_legacy_ast_map()
  -> PreparedNonMainStaticBoxMethodBatchV1
```

Audited locations:

```text
src/mir/builder/program_root_work_plan.rs:149
src/mir/builder/program_root_lowering.rs:90,385
src/mir/builder/nonmain_static_box_method_batch.rs:26
```

The implementation slice is intentionally narrow:

```text
PreparedProgramDeferredStaticBoxWorkV1::into_parts()
  returns (String, BoxMethodInventoryV1)

ProgramDeferredStaticBoxLifecycleV1::new()
  accepts BoxMethodInventoryV1 directly

PreparedNonMainStaticBoxMethodBatchV1
  uses one explicit named compatibility name-order view
```

The historical lowering order (`beta`, `alpha` source -> `alpha`, `beta`
execution) must remain unchanged. The slice must not claim source-order
promotion, resolver authority, Hako parser parity, or removal of the other
legacy projections. Acceptance requires a caller-zero guard proving that this
edge contains no `into_compatibility_map` / `from_legacy_ast_map` roundtrip,
plus focused order and context-restoration tests.

Retain for now: legacy JSON decoders, `declaration_order.rs`, raw static-Main
compatibility, normal/raw source-plan projections, and the callable declaration
catalog until their exact authority exists. Delete any helper only after its
complete production caller set reaches zero.

## R5-S1 landed receipt

Closed in the same implementation slice:

```text
BoxMethodInventoryV1
  -> PreparedProgramDeferredStaticBoxWorkV1
  -> ProgramDeferredStaticBoxLifecycleV1
  -> PreparedNonMainStaticBoxMethodBatchV1
```

The selected Program edge contains no production
`into_compatibility_map()` / `from_legacy_ast_map()` roundtrip. The batch keeps
the historical name-order result through the explicit
`into_compatibility_name_order()` projection. Focused context-restoration and
failure-prefix tests remain green, and the direct inventory ordering is covered
by the deferred static-Box fixture. This receipt does not close R5: the next
named Builder projection must be selected by a new census.

Caller-zero evidence for the migrated production edge:

```text
rg -n 'into_compatibility_map|from_legacy_ast_map' \
  src/mir/builder/program_root_work_plan.rs \
  src/mir/builder/program_root_lowering.rs \
  src/mir/builder/nonmain_static_box_method_batch.rs
=> no matches
```

## Acceptance

```text
each migrated caller names its inventory projection
compatibility projection remains visibly compatibility-only
caller-zero guard proves retired helper has no production callers
focused tests cover order/provenance preservation and malformed input
all touched source files remain below 800 lines
reference and README receipt land with the implementation
```
