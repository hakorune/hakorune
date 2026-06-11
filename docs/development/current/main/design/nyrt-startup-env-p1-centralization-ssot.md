---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env P1 centralization implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-p0-centralization-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - crates/nyash_kernel/src/entry.rs
  - src/config/env/stage1.rs
  - src/runner/stage1_bridge/env/runtime_defaults.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_p1_centralization_guard.sh
---

# NYRT-ENV-004 P1 Centralization Implementation

## Purpose

`NYRT-ENV-003` landed the shared P0 output toggle. `NYRT-ENV-004` lands the
next centralization slice: the post-`ny_main` metrics cluster and its shared
helpers, so the NyRT entry tail no longer owns the JSON/text toggles and the
GC threshold reads directly.
`NYRT-ENV-005` lands the follow-on P2 GC telemetry / warning threshold
cluster in the same helper module, so the NyRT entry tail no longer owns the
post-`ny_main` safepoint / allocation / warning-threshold reads directly.

This is still a narrow implementation slice. It centralizes the metrics
cluster and keeps startup-floor/path-shaping behavior out of scope.

## Decision

```text
nyrt_p1_centralization_landed=1
nyrt_metrics_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_metrics_helper=1
nyrt_json_metrics_cluster_shared=1
nyrt_text_metrics_cluster_shared=1
nyrt_threshold_metrics_cluster_shared=1
direct_nyrt_metrics_reads_in_entry=0
direct_nyrt_metrics_reads_in_runtime_defaults=0
```

## Implementation

- `src/config/env/stage1.rs` owns the shared P1 metrics helper vocabulary.
- `crates/nyash_kernel/src/entry.rs` now reads metrics state through the shared
  helpers instead of local `flag_on` / `u64_or` / `flag_default_on` calls.
- `src/config/env/mir_flags.rs` remains the baseline owner for the GC interval
  helpers that the stage1 helper delegates to.

## Stop Line

- do not move startup-floor gates or path discovery into this slice
- do not reintroduce direct `NYASH_GC_METRICS_JSON` / `NYASH_GC_METRICS` reads
  in the NyRT entry
- do not turn the shared helper into an env-cache or snapshot design
- do not move ownership back to `.hako` or MIRBuilder

## Next Seam

The next seam after this slice is the P3 minimal-startup knob from
`nyrt-startup-env-centralization-priority-ssot.md`.
