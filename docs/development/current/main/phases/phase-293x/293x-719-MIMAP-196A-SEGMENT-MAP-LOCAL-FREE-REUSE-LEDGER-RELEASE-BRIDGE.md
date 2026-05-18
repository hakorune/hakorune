# 293x-719 MIMAP-196A Segment Map Local Free Reuse Ledger Release Bridge

Status: landed
Date: 2026-05-18

## Decision

Connect the segment-map local-free reuse ledger bridge to the existing modeled
local-free reuse ledger release owner.

## Context

MIMAP-194A closed the segment-map local-free reuse ledger bridge pack. MIMAP-196A
keeps the same scalar/model boundary and records a release fact for the
segment-map-derived reuse ledger row:

```text
segment-map local-free reuse ledger row
  -> modeled local-free reuse ledger release owner
```

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-bridge-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_guard.sh`.
- Add proof manifest row with `validation_profile = "scalar-mir"` and
  `exe = "deferred-to-closeout"`.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
- No mutation of the source reuse ledger by the release owner.
- No dependency on `segment_allocation_modeled_ledger_box.hako`,
  `recordModeledConsume`, or `releaseModeledToken` from the release owner.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-196A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-197A post-segment-map-local-free-reuse-ledger-release-bridge row selection
```
