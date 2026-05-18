# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Token Release-Key Precondition Closeout SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Close the
`segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition`
pack with representative exact-MIR L3 EXE evidence.

MIMAP-220A remains the daily L2 behavior row. MIMAP-222A only proves that the
same MIR artifact used for preflight can lower to an executable and produce the
same release-key precondition diagnostic output.

## Pack

```text
segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition
```

Included daily row:

```text
MIMAP-220A segment-map local-free reuse ledger lifecycle-token release-key precondition observer
```

## Validation

Daily L2:

```bash
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition --level L2
```

Representative L3:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_token_release_key_precondition_closeout_guard.sh
```

The closeout guard must:

- dry-run the closeout pack selection and find MIMAP-220A;
- run the MIMAP-220A L2 guard;
- emit one MIR JSON artifact and use that exact artifact for EXE build;
- prove VM and EXE output both keep precondition diagnostic behavior and
  inactive stop lines stable.

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

## Next

```text
MIMAP-223A post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection
```
