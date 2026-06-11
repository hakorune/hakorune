---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env P0 centralization implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - crates/nyash_kernel/src/entry.rs
  - src/config/env/stage1.rs
  - src/runner/stage1_bridge/env/runtime_defaults.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_p0_centralization_guard.sh
---

# NYRT-ENV-003 P0 Centralization Implementation

## Purpose

`NYRT-ENV-002` ranked the startup env reads. `NYRT-ENV-003` lands the first
implementation seam from that ranking: the shared `NYASH_NYRT_SILENT_RESULT`
helper used by both the NyRT exact-EXE tail and the Stage-1 bridge runtime
defaults.

This is still a narrow implementation slice. It centralizes the P0 output-only
toggle and does not widen the env inventory into cache/snapshot design.

## Decision

```text
nyrt_p0_centralization_landed=1
nyrt_silent_result_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_helper=1
stage1_runtime_defaults_use_shared_helper=1
direct_nyrt_silent_result_reads_in_entry=0
direct_nyrt_silent_result_reads_in_runtime_defaults=0
```

## Implementation

- `src/config/env/stage1.rs` owns the shared P0 helper vocabulary.
- `crates/nyash_kernel/src/entry.rs` reads the effective toggle through the
  shared helper instead of a local `flag_on(...)` call.
- `src/runner/stage1_bridge/env/runtime_defaults.rs` checks the same shared
  helper before seeding the default child env value.

## Stop Line

- do not move GC metrics, startup-floor gates, or path-shaping knobs into this
  slice
- do not reintroduce direct `NYASH_NYRT_SILENT_RESULT` reads in the NyRT
  entry or the Stage-1 bridge defaults
- do not turn the shared helper into an env-cache or snapshot design
- do not widen ownership back to `.hako` or MIRBuilder

## Next Seam

The next seam remains the P1 metrics cluster from
`nyrt-startup-env-centralization-priority-ssot.md` if a later slice chooses to
land it.
