---
Status: SSOT
Decision: current
Date: 2026-06-12
Scope: NyRT startup env centralization priority table.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - docs/reference/environment-variables.md
  - crates/nyash_kernel/src/entry.rs
  - src/runner/stage1_bridge/env/runtime_defaults.rs
  - src/config/env/mir_flags.rs
  - src/config/env/runner_flags.rs
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_centralization_priority_guard.sh
---

# NYRT-ENV-002 NyRT Startup Env Centralization Priority

## Purpose

`NYRT-ENV-001` fixed the inventory surface. This document orders that surface
by centralization priority so the next code move is obvious and the fail-fast
boundary stays visible.

This is a ranking SSOT, not a behavior change. The current direct reads remain
until a later implementation slice moves them.

## Decision

```text
nyrt_startup_env_centralization_priority=1
first_centralization_surface=NYASH_NYRT_SILENT_RESULT
baseline_surface=src/config/env.rs
top_priority_is_output_only=1
path_shape_knob_last=1
next_centralization_surface=NYASH_NYRT_MINIMAL_STARTUP
```

## Priority Table

| Priority | Surface | Why it comes here | Current owner boundary |
| --- | --- | --- | --- |
| `P0` | `NYASH_NYRT_SILENT_RESULT` | Output-only toggle. It is shared by the NyRT exact-EXE tail and the Stage-1 bridge defaults, so it is the safest first seam to centralize. | `crates/nyash_kernel/src/entry.rs` tail + `src/runner/stage1_bridge/env/runtime_defaults.rs` |
| `P1` | `NYASH_GC_METRICS_JSON` / `NYASH_GC_METRICS` | Post-`ny_main` result/metrics surface. The metrics family is the next low-risk cluster after the output-only toggle. | `crates/nyash_kernel/src/entry.rs` tail |
| `P2` | `NYASH_GC_COLLECT_SP` / `NYASH_GC_COLLECT_ALLOC` / `NYASH_LLVM_AUTO_SAFEPOINT` / `NYASH_GC_ALLOC_THRESHOLD` | Still post-`ny_main`, but this cluster affects GC telemetry and warning thresholds, so it stays behind the result/metrics toggles. | `crates/nyash_kernel/src/entry.rs` tail |
| `P3` | `NYASH_NYRT_MINIMAL_STARTUP` | Pre-runtime-builder policy knob. It is already a single boolean, but it changes the startup floor, so it stays after the output/metrics cluster. | `crates/nyash_kernel/src/entry.rs` head |
| `P4` | `HAKO_NYRT_PLUGIN_HOST` / `NYASH_NYRT_RUNTIME_HOOKS` / `NYASH_NYRT_RUNTIME_BUILD` / `NYASH_NYRT_ENTRY_PATH_PREP` / `NYASH_NYRT_RING0_INIT` | These define the startup-floor gates and must remain explicit until the floor probe lane says otherwise. | `crates/nyash_kernel/src/entry.rs` head |
| `P5` | `current_exe` / `current_dir` / `PATH` / `PYTHONHOME` | Path shaping and platform-specific discovery are the last seam because they are the most coupled to executable layout and OS behavior. | `crates/nyash_kernel/src/entry.rs` head / helper adjacencies |

## Notes

- `src/config/env/mir_flags.rs` already provides the centralized baseline for
  `gc_metrics`, `gc_collect_sp_interval`, and `gc_collect_alloc_bytes`.
- `P0` is the first seam because it can be centralized without widening the
  path discovery surface or changing startup-floor behavior.
- `NYRT-ENV-003` lands that P0 seam in `src/config/env/stage1.rs`, so the
  NyRT entry tail and the Stage-1 bridge runtime defaults share the same
  helper.
- `NYRT-ENV-004` lands the P1 metrics cluster in the same helper module, so
  the NyRT entry tail no longer owns the post-`ny_main` JSON/text toggle reads
  directly.
- `NYRT-ENV-005` lands the P2 GC telemetry / warning threshold cluster in the
  same helper module, so the NyRT entry tail no longer owns the post-`ny_main`
  safepoint / allocation / threshold reads directly.
- `NYRT-ENV-006` lands the P3 minimal-startup knob in the same helper module,
  keeping the startup floor toggle centralized without widening the startup
  gates.
- `NYRT-ENV-007` lands the P4 startup gates in the same helper module, so the
  entry head no longer owns the plugin-host / runtime / ring0 mode parsers
  directly.
- `NYRT-ENV-008` lands the P5 path-shaping helpers in `src/config/env/paths.rs`
  so `current_exe` / `current_dir` / `PATH` / `PYTHONHOME` shaping no longer
  lives inside the NyRT entry body.

## Reading Order

1. `docs/development/current/main/design/nyrt-startup-env-read-inventory-ssot.md`
2. this SSOT
3. `crates/nyash_kernel/src/entry.rs`
4. `src/runner/stage1_bridge/env/runtime_defaults.rs`
5. `src/config/env/mir_flags.rs`
6. `docs/reference/environment-variables.md`

## Stop Line

- do not turn the ranking into an env-cache design
- do not move path shaping ahead of the output-only and metrics clusters
- do not pretend the inventory changed behavior just because the priority table is written down
- do not move the owner back to `.hako` or MIRBuilder

## Next Seam

This priority table is meant to feed the next implementation slice, which can
centralize `P3` next as the remaining startup-floor knob after `P0`, `P1`,
and `P2` have landed as shared helpers.
