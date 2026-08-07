# LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2

Status: active execution row
Decision: accepted bounded follow-up to R1
Date: 2026-08-08

## Purpose

Bind the closed Builder-free `PreparedLoopPhysicalLayoutV1` to the existing
canonical physical-block services and migrate the Callable canary's operation
placement from logical-block lookup to exact segment placement.

R1 is the only logical/layout authority. R2 is a physical cutover only; it
does not redesign Recipe, JoinSig, After, Tail, Completion, or provider
selection.

## Allowed change

```text
PreparedLoopPhysicalLayoutV1
  -> exact segment key / logical item
  -> CanonicalCfgSessionV1 block allocation
  -> one physical-block receipt per segment
  -> existing canonical operation emitter services
```

The existing Callable fixture is the only production-shaped canary. The
layout must preserve its seven-row parity while proving that the new receipt
is segment-aware. The canonical CFG, Binding SSA, and PhiTxn owners remain the
only physical owners.

## Required contracts

- Every R1 segment receives exactly one owner-branded physical block.
- Every scheduled operation resolves through its exact segment, never by
  current block, item-name, ordinal-only, or a second Recipe traversal.
- Parent resume placement is retained even when the Callable fixture has no
  nested child; the Generic G0 split is tested at R1 only and remains closed
  for physical emission.
- A mismatch, foreign owner, duplicate segment, or missing placement fails
  before instruction emission where possible; after block allocation the
  unpublished function session is discarded as one transaction.
- No new CFG/SSA/PHI/transaction owner, selector, retry, fallback, or module
  publication path is introduced.

## Implementation order

1. Add the smallest physical segment receipt beside the existing canonical
   block receipt; keep `PreparedLoopPhysicalLayoutV1` as the sole input.
2. Add exact segment-to-physical-block binding and duplicate/missing/foreign
   rejects.
3. Switch the Callable canary's operation placement to that receipt and
   preserve the existing five operation-family dispatcher.
4. Add positive parity plus placement/owner/duplicate negatives and fresh
   unpublished-session discard/reuse evidence.
5. Remove the selected logical-block-only lookup from that canary in the same
   commit. Do not leave a fallback or retry edge.

## Explicit non-goals

```text
G0 physical emission
recursive After writer
Tail or Completion changes
DraftSeal or collector changes
production selector/caller switch
M8/M9 coverage activation
M10b/M11/M12 legacy retirement
```

If any non-goal is needed, stop and update the design SSOT before coding.

## Acceptance gates

```text
cargo test loop_recipe_contract --lib
cargo test loop_common --lib                 # if a focused R2 test target exists
rustfmt --edition 2021 --check <changed Rust files>
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The active canary and every changed source file remain below 800 lines. The
same implementation commit must update the exact `docs/reference/**` entry,
the affected module README, the active workstream/current pointers, and this
task's closeout receipt. After R2 implementation, update the reference
documentation again in the same commit; a code-only R2 commit is forbidden.

## Closeout rule

R2 closes only when Callable segment placement is green and the old selected
logical-block-only execution lookup is gone from that caller. R3 remains the
next design/implementation boundary. No G0 physical or production claim may
be added to the R2 closeout.
