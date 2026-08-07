# LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2

Status: closed implementation row
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

## Closeout receipt (2026-08-08)

Implemented and closed. `LoopPhysicalSegmentBlockReceiptV1` is an adapter from
the closed R1 layout to the already allocated old canonical topology; it is
not the R1 segment allocator. It validates exact segment
coverage/owner/preheader/unique-block invariants, and rejects aliased segments
instead of silently sharing a block. The selected Callable canary now consumes
the complete layout through `prepare_loop_segment_operation_dispatch_v1`; its
operation targets are issued by exact segment key, with no logical-block-only
execution lookup in that caller.

The seven-row Callable parity remains `Pure=4 + Read=2 + Write=1`. Focused
negative evidence covers missing segment, foreign owner, duplicate/aliased
block, and the existing late-failure whole-session discard/fresh-session
reuse. The old logical dispatcher remains only for pre-existing test seams and
was not connected as a fallback or retry path.

Verified:

```text
cargo test -q mir::builder::resolved_lowering::loop_recipe_physicalizer --lib
  24 passed
cargo test -q mir::builder::resolved_lowering::loop_recipe_physicalizer::callable_production_canary_tests --lib
  2 passed
cargo test -q mir::builder::resolved_lowering::loop_recipe_physicalizer::segment_topology --lib
  2 passed
cargo test -q mir::builder::resolved_lowering::loop_recipe_physicalizer::segment_dispatcher --lib
  2 passed
rustfmt --edition 2021 --check <changed Rust files>
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

All touched source files remain below 800 lines. The exact reference,
resolved-lowering README, current state, 10-Now mirror, workstream, and next
R3 task were updated in the implementation closeout commit. R3 is now the
active boundary; no G0 physical emission or production selection is claimed.

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

R2 is closed: Callable segment placement is green and the old selected
logical-block-only execution lookup is gone from that caller. R2 does not
allocate R1 segments or retire the old Step/edge topology. R3 is a design
correction before implementation; no G0 physical or production claim is part
of this closeout.
