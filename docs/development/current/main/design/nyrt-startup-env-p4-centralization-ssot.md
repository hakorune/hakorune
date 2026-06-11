---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env P4 centralization implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-centralization-priority-ssot.md
  - docs/development/current/main/design/nyrt-startup-env-p3-centralization-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - crates/nyash_kernel/src/entry.rs
  - src/config/env/stage1.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_p4_centralization_guard.sh
---

# NYRT-ENV-007 P4 Centralization Implementation

## Purpose

`NYRT-ENV-006` landed the shared minimal-startup knob. `NYRT-ENV-007` lands
the next slice: the startup gates for plugin host, runtime hooks, runtime
builder, entry-path prep, and ring0 init.

This is still a narrow implementation slice. It centralizes the gate parsing
and keeps path shaping out of scope.

## Decision

```text
nyrt_p4_centralization_landed=1
nyrt_startup_gate_helper_owner=src/config/env/stage1.rs
nyrt_entry_uses_shared_startup_gate_helpers=1
nyrt_startup_gates_shared=1
direct_nyrt_startup_gate_reads_in_entry=0
```

## Implementation

- `src/config/env/stage1.rs` owns the shared P4 gate parsers.
- `crates/nyash_kernel/src/entry.rs` now reads the startup gates through the
  shared helpers instead of local `auto|off` parsing functions.

## Stop Line

- do not move path shaping into this slice
- do not reintroduce direct startup-gate parsing in the NyRT entry
- do not turn the shared helper into an env-cache or snapshot design
- do not move ownership back to `.hako` or MIRBuilder

## Next Seam

The next seam after this slice is the P5 path-shaping helpers from
`nyrt-startup-env-centralization-priority-ssot.md`.
