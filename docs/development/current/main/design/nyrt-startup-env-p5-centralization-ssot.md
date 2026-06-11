---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env P5 centralization implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-p4-centralization-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - crates/nyash_kernel/src/entry.rs
  - src/config/env/paths.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_p5_centralization_guard.sh
---

# NYRT-ENV-008 P5 Centralization Implementation

## Purpose

`NYRT-ENV-007` landed the shared startup gates. `NYRT-ENV-008` lands the next
slice: the path-shaping helpers for executable discovery and Windows-specific
environment shaping, so the NyRT entry body no longer owns `current_exe`,
`current_dir`, `PATH`, or `PYTHONHOME` handling directly.

This is still a narrow implementation slice. It centralizes the path-shaping
helpers and keeps the startup gate parsing out of scope.

## Decision

```text
nyrt_p5_centralization_landed=1
nyrt_path_helper_owner=src/config/env/paths.rs
nyrt_entry_uses_shared_path_helpers=1
nyrt_path_shaping_shared=1
direct_nyrt_path_reads_in_entry=0
```

## Implementation

- `src/config/env/paths.rs` owns the shared P5 path helpers.
- `crates/nyash_kernel/src/entry.rs` now reads executable discovery and
  Windows path shaping through the shared helpers instead of local `current_*`
  / `PATH` / `PYTHONHOME` calls.

## Stop Line

- do not move startup gate parsing into this slice
- do not reintroduce direct `current_exe` / `current_dir` / `PATH` /
  `PYTHONHOME` handling in the NyRT entry
- do not turn the shared helper into an env-cache or snapshot design
- do not move ownership back to `.hako` or MIRBuilder

## Next Seam

The P2..P5 slices are complete once this doc and guard land, and any later
work should only reopen path shaping if a new probe proves the boundary moved.
