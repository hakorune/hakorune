# CALLABLE-LOOP-AFTER-CLOSURE-P0

Status: `Design correction accepted; implementation next`
Date: `2026-08-07`
Parent: `docs/development/current/main/investigations/callable-loop-physical-canary-p0-task-2026-08-07.md`
Authority: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Decision

The callable Tail cannot be lowered directly from the current
`LoopAfterContinuationReceiptV1`. That receipt proves only that physical loop
blocks were allocated. It does not prove CFG predecessors, backedges, or
canonical BindingSSA block sealing. A Tail read against that open After can
create a pending PHI or `MissingDefinition`, so a Tail-only slice is
`NoSafeSlice`.

The next implementation slice is therefore the continuation-closure
prerequisite:

```text
complete callable operation schedule
  -> canonical CFG edges
  -> block/identity sealing
  -> ReadyLoopAfterContinuationV1
```

This adds no semantic owner and does not touch Tail, Completion, or DraftSeal.
The existing open topology receipt is consumed once and cannot be fabricated
or reused as a sealed After receipt.

## Exact callable CFG closure

For the fixed caller-zero callable fixture, the outer callable adapter must
issue the already verified edges in this order after complete operation
emission:

```text
preheader -> header
header --condition--> body
header --false------> after
body -> step
step -> header                 # backedge before header sealing
```

The condition `ValueId` comes from the existing typed operation value ledger
(`LoopValueKeyV1`), never from a numeric guess or a binding/name lookup. The
fixed fixture's condition key is an implementation detail of its sealed Recipe
relation, not a new source authority.

After all edges exist, close blocks through the existing canonical owners:

```text
preheader, body, step:
  CanonicalCfgSessionV1::seal_block
  ResolvedSsaIdentityStateV2::seal_block

header:
  seal only after the step backedge is present

after:
  seal only after the header false edge is present
```

The adapter then selects the sealed After block and issues one private
`ReadyLoopAfterContinuationV1`. The receipt contains the exact owner, root
After block, logical loop key, and sealed predecessor witness. It is a
session-local temporal receipt, not a second CFG/SSA owner.

## Ownership and forbidden shortcuts

```text
canonical CFG session:
  edges, predecessors, block sealing

canonical identity/BindingSSA:
  block sealing and Tail reads

operation value ledger:
  LoopValueKey -> emitted ValueId transport only

outer callable adapter:
  fixed profile edge schedule and ReadyAfter issuance

common Loop physicalizer:
  operations only; no callable Tail/Completion
```

Forbidden:

```text
read an open After
fabricate ReadyLoopAfterContinuationV1 in the real canary
infer the condition from a ValueId number or source name
create a second CFG/SSA/PHI owner
seal header before the backedge exists
reuse a discarded session or receipt
add a fallback/retry route
```

## Acceptance evidence

```text
all callable operations are emitted before edge closure
each required edge is emitted exactly once
condition comes from the sealed operation ledger and has the expected Bool type
foreign owner/block/condition/edge is rejected before the affected effect
all blocks are CFG-sealed and identity-sealed in the documented order
header is sealed after the backedge; After after the false edge
open After cannot satisfy Tail-read APIs
ReadyLoopAfterContinuationV1 is one-shot and session-local
late failure discards the whole unpublished session and restores caller once
fresh session repeats equivalent semantic receipts
Tail, Completion, DraftSeal, production selection, and legacy deletion remain untouched
```

## Next slices after this one

The implementation is intentionally split into three commits:

1. `feat(mir): close callable loop continuation` — this task only.
2. `feat(mir): hand off callable tail and completion` — consume ReadyAfter,
   exact Tail binding read, ABI check, `mark_return`, and one
   `claim_explicit_return`; stop before DraftSeal.
3. `feat(mir): finish callable loop canary through DraftSeal` — consume the
   profile-close receipt, call only `finish_for_draft_seal`, and complete the
   existing DraftSeal prepare/commit path with discard/fresh-session evidence.

Each implementation commit must update the active card, `CURRENT_STATE.toml`,
the current mirrors, the owning `src/mir/.../README.md`, and the relevant
`docs/reference/mir/*` entry. A slice is not complete while its reference
claims or restart pointers are stale.

## Non-claims

```text
full callable physicalization = 0 until slices 1-3 all pass
Generic G0 parity = 0
production selector/switch = 0
retry/fallback/reselection retirement = 0
legacy scheduler/route deletion = 0
backend/performance parity = 0
```
