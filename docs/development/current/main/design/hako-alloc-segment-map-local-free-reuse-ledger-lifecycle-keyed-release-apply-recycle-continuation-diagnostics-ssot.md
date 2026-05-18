# Hako Alloc Segment Map Local Free Reuse Ledger Lifecycle-Keyed Release Apply/Recycle Continuation Diagnostics SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Model diagnostics around the MIMAP-232A lifecycle-keyed release apply/recycle
continuation bridge.

MIMAP-233A observes the modeled reuse ledger after a lifecycle-keyed source
release apply, a missing-live-row apply reject, an unsupported apply reject, and
a post-continuation duplicate local-free reuse reject. It publishes scalar facts
only.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box.hako
```

The owner may:

- observe release-apply counters and reuse ledger counters;
- report whether a missing-live-row apply reject was seen;
- report whether an unsupported lifecycle-keyed apply reject was seen;
- report whether a post-continuation duplicate reuse reject was seen;
- preserve the MIMAP-232A continuation route shape.

The owner must not:

- mutate reuse or release ledgers;
- use the old modeled-reuse-token keyed release owner as the continuation owner;
- infer anything from owner names or backend matchers;
- define real generation/lifecycle semantics;
- execute real allocator behavior.

## Validation

MIMAP-233A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_lifecycle_keyed_release_apply_recycle_continuation_diagnostics_guard.sh --level L2
```

The guard must:

- prove missing-live-row apply diagnostics;
- prove unsupported lifecycle-keyed apply diagnostics;
- prove post-continuation duplicate reuse diagnostics;
- prove the diagnostic owner is observer-only;
- prove raw pointer, segment-map execution, arena, atomics, OSVM, worker,
  provider, and backend matcher seams remain inactive.

## Stop Lines

- No reuse/release ledger mutation from the diagnostic owner.
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
