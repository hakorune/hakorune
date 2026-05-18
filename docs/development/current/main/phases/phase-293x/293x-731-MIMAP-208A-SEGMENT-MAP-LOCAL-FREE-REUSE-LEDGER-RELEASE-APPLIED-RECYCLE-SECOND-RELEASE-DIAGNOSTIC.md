# 293x-731 MIMAP-208A Segment Map Local Free Reuse Ledger Release-Applied Recycle Second-Release Diagnostic

Status: landed
Date: 2026-05-18

## Decision

Add a diagnostic proof for the current one-release-per-modeled-reuse-token
boundary after the segment-map local-free reuse ledger release-applied recycle
bridge closeout.

## Context

MIMAP-204A proved:

```text
source reuse ledger applies release
  -> old row becomes non-live
  -> source reuse ledger records the same modeled reuse token as a new live row
```

MIMAP-208A proves the next boundary without opening generation/lifecycle token
semantics:

```text
recycled live row
  -> second release owner record attempt
  -> duplicate reject
```

## Scope

- Add diagnostic SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-ssot.md`.
- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-proof`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_second_release_diagnostic_guard.sh`.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real allocator free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
- No release owner mutation by the source ledger.
- No generation/lifecycle token introduction in this row.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_second_release_diagnostic_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-208A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-209A post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic row selection
```
