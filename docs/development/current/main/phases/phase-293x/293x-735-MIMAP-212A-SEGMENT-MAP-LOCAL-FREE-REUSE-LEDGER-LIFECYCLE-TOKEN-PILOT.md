# 293x-735 MIMAP-212A Segment Map Local Free Reuse Ledger Lifecycle-Token Pilot

Status: landed
Date: 2026-05-19

## Decision

Add a narrow scalar lifecycle-token pilot after the release-applied recycle
second-release diagnostic closeout.

## Context

MIMAP-208A proved the current release-owner boundary:

```text
recycled source reuse ledger row
  -> second release owner record attempt
  -> duplicate reject for the same modeled reuse token
```

MIMAP-210A closed that diagnostic pack with representative exact-MIR L3 EXE
evidence, and MIMAP-211A selected a lifecycle-token sidecar as the next
smallest row.

MIMAP-212A keeps the source reuse ledger and release owner unchanged. It adds a
dedicated owner that derives a scalar reuse-lifecycle token from:

```text
modeled_reuse_token
explicit lifecycle_id
```

The row proves valid, duplicate, invalid-shape, and unsupported-requirement
branches in model space only.

## Scope

- Add lifecycle-token owner:
  `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh`.
- Add accepted design SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-ssot.md`.

## Stop Lines

- No release ledger key migration.
- No generation/lifecycle semantics for real allocator cycles.
- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real allocator free-list mutation.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_pilot_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-212A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-213A post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot row selection
```
