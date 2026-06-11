---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env P3 centralization implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-p2-centralization-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - crates/nyash_kernel/src/entry.rs
  - src/config/env/stage1.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_p3_centralization_guard.sh
---

# NYRT-ENV-006 P3 Centralization Implementation

## Purpose

`NYRT-ENV-005` landed the shared P2 GC telemetry cluster. `NYRT-ENV-006`
lands the next slice: the `NYASH_NYRT_MINIMAL_STARTUP` knob, so the NyRT entry
head no longer owns the startup-floor toggle directly.

This is still a narrow implementation slice. It centralizes the minimal-startup
knob and keeps the startup gates and path shaping out of scope.

## Decision

```text
nyrt_p3_centralization_landed=1
nyrt_minimal_startup_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_minimal_startup_helper=1
nyrt_minimal_startup_knob_shared=1
direct_nyrt_minimal_startup_reads_in_entry=0
```

## Implementation

- `src/config/env/stage1.rs` owns the shared P3 minimal-startup helper.
- `crates/nyash_kernel/src/entry.rs` now reads `NYASH_NYRT_MINIMAL_STARTUP`
  through the shared helper instead of a local `flag_on` call.

## Stop Line

- do not move startup gates or path discovery into this slice
- do not reintroduce direct `NYASH_NYRT_MINIMAL_STARTUP` reads in the NyRT
  entry
- do not turn the shared helper into an env-cache or snapshot design
- do not move ownership back to `.hako` or MIRBuilder

## Next Seam

The next seam after this slice is the P4 startup gates from
`nyrt-startup-env-centralization-priority-ssot.md`.
