---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env P2 centralization implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-p1-centralization-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - crates/nyash_kernel/src/entry.rs
  - src/config/env/stage1.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_p2_centralization_guard.sh
---

# NYRT-ENV-005 P2 Centralization Implementation

## Purpose

`NYRT-ENV-004` landed the shared P1 metrics helper. `NYRT-ENV-005` lands the
next slice: the post-`ny_main` GC telemetry / warning threshold cluster, so the
NyRT entry tail no longer owns the safepoint / allocation / threshold reads
directly.

This is still a narrow implementation slice. It centralizes the GC telemetry
cluster and keeps startup-floor gates and path shaping out of scope.

## Decision

```text
nyrt_p2_centralization_landed=1
nyrt_gc_telemetry_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_gc_telemetry_helper=1
nyrt_gc_telemetry_cluster_shared=1
nyrt_gc_warning_threshold_shared=1
direct_nyrt_gc_telemetry_reads_in_entry=0
```

## Implementation

- `src/config/env/stage1.rs` owns the shared P2 GC telemetry helper vocabulary.
- `crates/nyash_kernel/src/entry.rs` now reads the P2 telemetry state through
  the shared helpers instead of local `flag_on` / `u64_or` / `flag_default_on`
  calls.
- `src/config/env/mir_flags.rs` remains the baseline owner for the GC interval
  helpers that the stage1 helper delegates to.

## Stop Line

- do not move startup-floor gates or path discovery into this slice
- do not reintroduce direct `NYASH_GC_COLLECT_SP` / `NYASH_GC_COLLECT_ALLOC`
  / `NYASH_LLVM_AUTO_SAFEPOINT` / `NYASH_GC_ALLOC_THRESHOLD` reads in the
  NyRT entry
- do not turn the shared helper into an env-cache or snapshot design
- do not move ownership back to `.hako` or MIRBuilder

## Next Seam

The next seam after this slice is the P3 minimal-startup knob from
`nyrt-startup-env-centralization-priority-ssot.md`.
