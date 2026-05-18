# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Ledger Closeout SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Close out the source release-ledger lifecycle-key migration pack.

This closeout confirms that:

- MIMAP-228A opened a separate lifecycle-keyed source release ledger owner with
  first-pattern L3 exact-MIR EXE evidence;
- MIMAP-229A added observer-only diagnostics for the lifecycle-keyed source
  release ledger with L2 VM/MIR evidence;
- the old modeled-reuse-token keyed release owner remains unmigrated;
- raw pointer residence, real segment-map execution, arena backing, atomics,
  OSVM/page-source calls, worker scheduling, provider activation, hooks,
  `#[global_allocator]`, cross-function `Result` direct ABI, runtime sum
  materialization, and backend matchers remain closed.

## Pack

```text
closeout_pack = source-release-ledger-lifecycle-key-migration
```

## Required Evidence

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_ledger_closeout_guard.sh
bash tools/checks/run_proof_app.sh --closeout-pack source-release-ledger-lifecycle-key-migration --level L2 --dry-run
```

The closeout guard must run:

- `MIMAP-228A` at L3 for exact-MIR EXE representative evidence;
- `MIMAP-229A` at L2 for observer-only diagnostics evidence.

## Next

```text
MIMAP-231A post source release-ledger lifecycle-key migration closeout row selection
```

The likely next bridge is release/recycle lifecycle continuation. Raw pointer
residence, arena backing, real segment-map execution, and atomic bitmap behavior
remain after that bridge unless a later row explicitly opens them.
