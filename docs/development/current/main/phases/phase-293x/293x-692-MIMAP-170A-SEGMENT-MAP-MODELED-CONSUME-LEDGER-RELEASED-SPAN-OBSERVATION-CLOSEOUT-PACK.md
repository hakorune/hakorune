# 293x-692 MIMAP-170A Segment Map Modeled Consume Ledger Released Span Observation Closeout Pack

Status: landed
Date: 2026-05-18

## Decision

Close the segment-map modeled consume-ledger released-span observation pack with
representative exact-MIR L3 EXE evidence.

## Context

MIMAP-168A proved the daily L2 behavior:

```text
segment-map release report
  -> modeled_block_start / modeled_block_end / released_blocks
  -> released-span ledger row
```

MIMAP-170A keeps that behavior in the
`segment-map-consume-ledger-released-span` pack and runs the representative L3
evidence for the pack.

## Scope

- Add closeout SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-modeled-consume-ledger-released-span-observation-closeout-ssot.md`.
- Add manifest-backed closeout guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_closeout_guard.sh`.
- Keep daily validation on L2 through
  `bash tools/checks/run_proof_app.sh --closeout-pack segment-map-consume-ledger-released-span --level L2`.

## Stop Lines

- No real segment allocation/free execution.
- No free-list mutation.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_released_span_observation_closeout_guard.sh
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-consume-ledger-released-span --level L2 --dry-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-171A post-segment-map-modeled-consume-ledger-released-span-observation-closeout row selection
```
