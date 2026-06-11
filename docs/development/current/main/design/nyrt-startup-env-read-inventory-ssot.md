---
Status: SSOT
Decision: current
Date: 2026-06-11
Scope: NyRT startup env read inventory and path-probe boundary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - docs/reference/environment-variables.md
  - crates/nyash_kernel/src/entry.rs
  - src/runner/plugin_init.rs
  - src/runtime/plugin_loader_unified.rs
  - src/runner/modes/common_util/resolve/root.rs
  - src/main.rs
  - src/config/env.rs
  - tools/checks/k2_wide_phase296x_nyrt_startup_env_read_inventory_guard.sh
---

# NYRT-ENV-001 NyRT Startup Env Read Inventory

## Purpose

The current startup attribution lane showed that `getenv` and env string scan
activity point at the NyRT entry path. This document inventories that surface
so we can keep the owner boundary explicit before any env cache or snapshot
design is discussed.

This is an inventory-only SSOT. It does not change startup behavior and does
not move ownership back to `.hako`, MIRBuilder, route planning, or allocator
replacement.

## Decision

```text
nyrt_startup_env_inventory=1
env_cache_design=0
env_snapshot_design=0
direct_std_env_surface_kept_visible=1
src_config_env_is_control_surface=1
```

## Inventory

| File | Functions / surface | Classification | Access pattern | Notes |
| --- | --- | --- | --- | --- |
| `crates/nyash_kernel/src/entry.rs` | `plugin_host_mode_from_env`, `runtime_hooks_mode_from_env`, `runtime_build_mode_from_env`, `entry_path_prep_mode_from_env`, `ring0_init_mode_from_env`, `main` | `startup-before-ny_main` / `after-ny_main/result/metrics` | direct `std::env::var` + `crate::env_flags` + `current_exe` / `current_dir` | Primary NyRT owner. The startup half reads `HAKO_NYRT_PLUGIN_HOST`, `NYASH_NYRT_RUNTIME_HOOKS`, `NYASH_NYRT_RUNTIME_BUILD`, `NYASH_NYRT_ENTRY_PATH_PREP`, and `NYASH_NYRT_RING0_INIT`. The tail reads GC metrics knobs and path/env shaping keys such as `PATH` and `PYTHONHOME`; `NYASH_NYRT_SILENT_RESULT` is now routed through the shared stage1 helper. |
| `src/runner/plugin_init.rs` | `resolve_plugin_toml`, `init_bid_plugins` | `runtime helper` | `src/config/env` plus path fallback | Startup helper for plugin-host path discovery. It checks `hako.toml` / `nyash.toml` in CWD and falls back to `crate::config::env::hako_root()`. |
| `src/runner/stage1_bridge/env/runtime_defaults.rs` | `apply` | `runtime helper` | shared stage1 helper + child-env defaulting | Stage-1 bridge runtime defaults helper. It seeds `NYASH_NYRT_SILENT_RESULT=1` through the shared stage1 helper only when the parent process has not set the toggle. |
| `src/runtime/plugin_loader_unified.rs` | plugin loader unified entry | `runtime helper` | `src/config/env` | Helper surface for plugin loading. It reads `disable_plugins()` through the centralized env layer. |
| `src/runner/modes/common_util/resolve/root.rs` | `resolve_repo_root` | `runtime helper` | mixed `src/config/env` + direct `current_dir` / `current_exe` | Path discovery helper. This is adjacent to startup, not the NyRT entrypoint itself. |
| `src/config/env/stage1.rs` | Stage-1 / selfhost CLI env helpers | `control surface helper` | shared stage1 helper + direct env presence check for `NYASH_NYRT_SILENT_RESULT` | Shared owner for the output-only toggle. `NYASH_NYRT_SILENT_RESULT` is centralized here so the NyRT tail and Stage-1 bridge defaults share the same helper. |
| `src/main.rs` | bootstrap wrapper and `current_exe` bootstrap | `startup wrapper` / `compile-time-tooling` boundary | direct `std::env::var` + `current_exe` | Optional adjacent host wrapper. It reads `HAKO_PROGRAM_JSON`, `HAKO_PROGRAM_JSON_FILE`, `NYASH_VERIFY_JSON`, `HAKO_VERIFY_V1_FORCE_HAKOVM`, `HAKO_ALLOW_NYASH`, and `NYASH_ALLOW_NYASH` before the main runtime path. |
| `src/config/env.rs` | env aggregator / bootstrap | `control surface` | centralized `src/config/env` wrappers and `bootstrap_from_toml_env` | Comparison baseline. This file is the intended env SSOT, but it does not yet absorb the NyRT entry's direct startup reads. |

## Reading Order

1. `crates/nyash_kernel/src/entry.rs`
2. `src/runner/plugin_init.rs`
3. `src/runner/modes/common_util/resolve/root.rs`
4. `src/runtime/plugin_loader_unified.rs`
5. `src/main.rs`
6. `src/config/env.rs`
7. `docs/reference/environment-variables.md`

## Stop Line

- do not introduce env cache or env snapshot behavior from this inventory
- do not move the owner back to `.hako` or MIRBuilder
- do not treat `src/config/env` as evidence that the NyRT entry no longer has direct reads
- do not silently merge the startup wrapper into the primary NyRT entry owner

## Next Seam

This inventory is meant to be used together with `PERF-USERBOX-064` and the
NyRT startup floor probes.
The practical next questions are:

- which of the direct NyRT entry reads should be centralized first
- whether the adjacent startup helpers need separate inventory rows later
- whether any future env cache proposal can preserve the current fail-fast
  boundary without changing default behavior
