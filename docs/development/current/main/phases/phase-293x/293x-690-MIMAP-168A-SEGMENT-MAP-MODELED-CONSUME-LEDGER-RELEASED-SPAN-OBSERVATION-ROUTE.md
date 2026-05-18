# 293x-690 MIMAP-168A Segment Map Modeled Consume Ledger Released Span Observation Route

Status: landed
Date: 2026-05-18

## Decision

Add a segment-map owner-boundary released-span observation proof that records a
successful segment-map modeled release report into the existing released-span
ledger.

## Context

MIMAP-166A closed released-token recycle at the segment-map modeled
consume-ledger boundary. MIMAP-168A keeps the same model/scalar lane and proves
that the release report now carries enough span facts for the downstream
released-span ledger:

```text
release report
  -> modeled_block_start / modeled_block_end / released_blocks
  -> released-span ledger row
```

This prepares the next free-list / local-free bridge without opening real
segment free execution.

## Scope

- Add `modeled_block_end` to
  `HakoAllocSegmentMapModeledConsumeLedgerReleaseReport`.
- Add proof app:
  `apps/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh`.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No free-list mutation.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-168A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-169A post-segment-map-modeled-consume-ledger-released-span-observation row selection
```
