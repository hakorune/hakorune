# 293x-745 MIMAP-222A Segment Map Local Free Reuse Ledger Lifecycle-Token Release-Key Precondition Closeout Pack

Status: landed
Date: 2026-05-19

## Decision

Close the segment-map local-free reuse ledger lifecycle-token release-key
precondition pack with representative exact-MIR L3 EXE evidence.

## Context

MIMAP-220A proved the daily L2 behavior:

```text
lifecycle-token observer accepted
  -> duplicate release seen while release key remains modeled reuse token
  -> lifecycle_count >= 2
  -> migration_candidate yes, migration execution no
```

MIMAP-222A keeps that behavior in the
`segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition`
pack and runs representative L3 evidence for the pack.

## Scope

- Add closeout SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout-ssot.md`.
- Add manifest-backed closeout guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_closeout_guard.sh`.
- Keep daily validation on L2 through:
  `bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition --level L2`.

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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_closeout_guard.sh
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition --level L2 --dry-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-223A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection
```
