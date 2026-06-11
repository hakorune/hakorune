---
Status: SSOT
Decision: current
Date: 2026-06-10
Scope: exact-AOT link option contract and startup attribution ladder for PERF-USERBOX rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md
  - docs/development/current/main/design/optimization-tag-flow-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/reference/environment-variables.md
  - docs/tools/check-scripts-index.md
  - tools/perf/lib/aot_helpers.sh
  - tools/ny_mir_builder.sh
  - lang/c-abi/shims/hako_aot_shared_impl.inc
  - crates/nyash_kernel/src/entry.rs
  - tools/hako_check/README.md
---

# PERF-USERBOX Link / Startup Attribution SSOT

## Goal

Keep the exact-AOT link / startup probe lane readable in one page.

This lane is not a `hako_check` optimization surface. `hako_check` stays
read-only and source/MIR oriented; link / startup attribution lives in
`tools/perf`, `tools/allocator`, and `tools/checks`.

## Contract Surface

These are the only knobs this lane treats as first-class:

| Knob | Default | Reading |
| --- | --- | --- |
| `HAKO_AOT_LDFLAGS` | unset | Compat append ldflags. Use for explicit `-static-libgcc` or other caller-owned additions. |
| `NYASH_LLVM_LINK_SYSTEM_LIBS` | `full` | `full` keeps the historical `-ldl -lpthread -lm` surface; `minimal` drops `-lm` for diagnostic probes only. |
| `HAKO_NYRT_PLUGIN_HOST` | `auto` | `auto` keeps normal NyRT plugin-host init; `off` skips plugin-host init for no-plugin floor probes. |
| `NYASH_NYRT_RUNTIME_HOOKS` | `auto` | `auto` publishes runtime hooks into the global safepoint bridge; `off` skips the runtime-hooks publication step for diagnostic startup-floor probes. |
| `NYASH_NYRT_RUNTIME_BUILD` | `auto` | `auto` builds the normal minimal runtime; `off` skips `NyashRuntimeBuilder` / GC controller construction for diagnostic startup-floor probes and requires `NYASH_NYRT_RUNTIME_HOOKS=off`. |
| `NYASH_NYRT_ENTRY_PATH_PREP` | `auto` | `auto` keeps the executable-path preparation step (`current_exe`, PATH / PYTHONHOME shaping); `off` skips that prep for diagnostic startup-floor probes and requires `HAKO_NYRT_PLUGIN_HOST=off`. |
| `NYASH_NYRT_RING0_INIT` | `auto` | `auto` keeps the ring0 bootstrap step; `off` skips `Ring0Context` initialization for diagnostic startup-floor probes and requires `HAKO_NYRT_PLUGIN_HOST=off`, `NYASH_NYRT_RUNTIME_HOOKS=off`, `NYASH_NYRT_RUNTIME_BUILD=off`, and `NYASH_NYRT_ENTRY_PATH_PREP=off`. |
| `NYASH_NYRT_SILENT_RESULT` | off | Suppresses the standard `Result: <code>` line so startup probes can keep stdout clean. |

Rules:

- default link mode stays compatibility-first
- `minimal` is a probe mode, not a new public ABI
- `off` is a diagnostic plugin-host mode, not a runtime policy change
- if a program needs math symbols, it must opt back in explicitly
- do not move this lane into `hako_check`; the surface is evidence packaging, not route ownership

## One-Page Ladder

| Row | Focus | Guard | What changed | What stayed closed |
| --- | --- | --- | --- | --- |
| `PERF-USERBOX-001` | direct-helper floor attribution | `k2_wide_phase296x_perf_userbox_floor_attribution_guard.sh` | floor run becomes measurable and valid | `.hako`, MIRBuilder, route planning, exact helper lowering, runtime object representation |
| `PERF-USERBOX-002` | startup / loader owner split | `k2_wide_phase296x_perf_userbox_startup_loader_owner_split_guard.sh` | ret0 startup top symbols are classified | source, MIRBuilder, route planner, exact helper lowering, runtime object representation |
| `PERF-USERBOX-003` | `-static-libgcc` probe | `k2_wide_phase296x_perf_aot_static_libgcc_probe_guard.sh` | dynamic `libgcc_s` drops on ret0 | default link mode, full static linking |
| `PERF-USERBOX-004` | minimal system libs probe | `k2_wide_phase296x_perf_aot_minimal_system_libs_probe_guard.sh` | dynamic `libm` also drops on ret0 | default link mode, full static linking |
| `PERF-USERBOX-005` | NyRT plugin-host-off probe | `k2_wide_phase296x_perf_nyrt_plugin_host_off_probe_guard.sh` | plugin-host init is skipped for the floor seed | normal NyRT plugin support, default runtime behavior |
| `PERF-USERBOX-006` | NyRT runtime-hooks-off probe | `k2_wide_phase296x_perf_nyrt_runtime_hooks_off_probe_guard.sh` | runtime-hooks publication is skipped for the floor seed | normal runtime hooks, default runtime behavior |
| `PERF-USERBOX-007` | NyRT minimal-startup probe | `k2_wide_phase296x_perf_nyrt_minimal_startup_probe_guard.sh` | default registry startup is bypassed for the floor seed | normal runtime registry behavior |
| `PERF-USERBOX-008` | loader / libc floor summary probe | `k2_wide_phase296x_perf_userbox_loader_libc_floor_guard.sh` | `-static-libgcc` plus `NYASH_LLVM_LINK_SYSTEM_LIBS=minimal` package the loader/libc floor without changing default link mode | default link mode, full static linking |
| `PERF-USERBOX-009` | loader owner split summary probe | `k2_wide_phase296x_perf_userbox_loader_owner_split_guard.sh` | the startup owner split and loader/libc floor summary are joined into one evidence row with `dynamic_loader` as the primary startup owner family | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-010` | ld-linux / libc loader split probe | `k2_wide_phase296x_perf_userbox_loader_dso_split_guard.sh` | the startup loader split is now checked for both `ld-linux` and `libc` top rows while the loader/libc floor summary stays closed | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-011` | ld-linux / libc loader percent split probe | `k2_wide_phase296x_perf_userbox_loader_dso_percent_guard.sh` | the startup loader split now exposes ld-linux versus libc percent sums so the loader floor can be split more precisely while the closed floor summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-012` | ld-linux / libc loader symbol split probe | `k2_wide_phase296x_perf_userbox_loader_symbol_split_guard.sh` | the startup loader split now exposes the leading ld-linux and libc symbols so the loader floor can be split at symbol granularity while the closed floor summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-013` | startup executable split probe | `k2_wide_phase296x_perf_userbox_startup_executable_split_guard.sh` | the startup report now exposes the ret0.exe executable contribution so the startup executable can be split from the loader floor while the closed loader/libc summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-014` | startup executable symbol split probe | `k2_wide_phase296x_perf_userbox_startup_executable_symbol_split_guard.sh` | the startup report now exposes the leading ret0.exe symbol so the executable contribution can be split at symbol granularity while the closed loader/libc summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-015` | startup executable ret0 stack split probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_stack_split_guard.sh` | the startup report now exposes the leading ret0.exe symbol stack so the executable contribution can be split as a small ret0 stack while the closed loader/libc summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-016` | startup executable ret0 family split probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_family_split_guard.sh` | the startup report now exposes the ret0.exe family buckets so the executable contribution can be split by family while the closed loader/libc summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-017` | startup executable ret0 bucket split probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_split_guard.sh` | the startup report now exposes the ret0.exe bucket counts so the executable contribution can be split by env/path/ffi/alloc/once/string buckets while the closed loader/libc summary stays in place | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-018` | startup executable ret0 bucket variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_variability_guard.sh` | the startup report now packages repeated ret0 bucket trials so the executable contribution can be read as a distribution before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-019` | startup executable ret0 bucket symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_symbol_variability_guard.sh` | the startup report now packages repeated ret0 bucket representative symbols so the executable contribution can be read as stable owner signals before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-020` | startup executable ret0 bucket subkind variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_subkind_variability_guard.sh` | the startup report now packages repeated ret0 bucket representative subkinds so the executable contribution can be split into getenv / pathprep / futex / components / registry signals before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-021` | startup executable ret0 bucket top-symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_top_symbol_variability_guard.sh` | the startup report now packages repeated ret0 bucket exact top symbols so the executable contribution can be split into exact top-symbol owners before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-022` | startup executable ret0 env top-symbol family variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_env_top_symbol_family_variability_guard.sh` | the startup report now packages repeated env top-symbol family counts so env can be split into var_os / getenv / current_exe / current_dir signals before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-023` | startup executable ret0 env exact top-symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_env_exact_top_symbol_variability_guard.sh` | the startup report now packages repeated env exact top symbols so env can be split into current_exe / minimal_startup_enabled_from_env / gc_mode signals before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-024` | startup executable ret0 path exact top-symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_path_exact_top_symbol_variability_guard.sh` | the startup report now packages repeated path exact top symbols so path can be split into components / current_dir / current_exe / related exact symbols before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-025` | startup executable ret0 nyash_kernel_runtime exact top-symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_nyash_kernel_runtime_exact_top_symbol_variability_guard.sh` | the startup report now packages repeated nyash_kernel_runtime exact top symbols so registry / runtime / once / other exact symbols can be split before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-026` | startup executable ret0 nyash_kernel_runtime registry exact top-symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_nyash_kernel_runtime_registry_exact_top_symbol_variability_guard.sh` | the startup report now packages repeated nyash_kernel_runtime registry exact top symbols so registry exact owner signals can be split before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `PERF-USERBOX-027` | startup executable ret0 nyash_kernel_runtime runtime exact-top-symbol variability probe | `k2_wide_phase296x_perf_userbox_startup_executable_ret0_bucket_nyash_kernel_runtime_runtime_exact_top_symbol_variability_guard.sh` | the startup report now packages repeated nyash_kernel_runtime runtime exact top symbols so runtime exact owner signals can be split before choosing the next owner | default link mode, exact-AOT runtime surface, exact helper lowering |
| `NYRT-STARTUP-FLOOR-001` | bare-entry floor A/B probe | `k2_wide_phase296x_nyrt_startup_floor_bare_entry_ab_probe_guard.sh` | one ret0 `ny_main` object is linked through current minimal NyRT entry and bare libc `main` | default entry, `.hako`, MIRBuilder, route planner, exact helper lowering |
| `NYRT-STARTUP-FLOOR-002` | runtime-build-off probe | `k2_wide_phase296x_nyrt_runtime_build_off_probe_guard.sh` | `NyashRuntimeBuilder` / GC controller construction is skipped inside the current minimal NyRT entry | default runtime build, runtime hooks, GC metrics semantics |
| `NYRT-STARTUP-FLOOR-003` | entry-path-prep-off probe | `k2_wide_phase296x_nyrt_entry_path_prep_off_probe_guard.sh` | `current_exe` / PATH / PYTHONHOME preparation is skipped inside the current minimal NyRT entry | default plugin-host path prep, default entry path discovery |
| `NYRT-STARTUP-FLOOR-004` | ring0-init-off probe | `k2_wide_phase296x_nyrt_ring0_init_off_probe_guard.sh` | Ring0Context initialization is skipped inside the current minimal NyRT entry | default ring0 bootstrap, default ring0-dependent startup services |

## Reading Order

1. `docs/development/current/main/CURRENT_STATE.toml`
2. this SSOT
3. `docs/development/current/main/phases/phase-296x/296x-651-HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT.md`
4. `docs/reference/environment-variables.md`
5. `tools/checks/current_state_pointer_guard.sh`

## Stop Line

- do not change the default link mode while evaluating the probe rows
- do not introduce a new public ABI for this lane
- do not turn `hako_check` into the owner of link/startup attribution
- do not split env / stdio / registry into separate optimization owners until the bare-entry A/B delta is available
- do not silently fallback from `minimal` to `full`
- do not silently ignore invalid knob values

## Next Seam

The current floor is now below loader, libgcc, libm, plugin-host init,
runtime-hooks publication, default registry startup, and the first ret0.exe
executable contribution row.
The path exact top-symbol distribution flatlined in `PERF-USERBOX-024`, then
`PERF-USERBOX-025` showed the nyash_kernel_runtime exact top symbols,
`PERF-USERBOX-026` split the registry exact top symbols inside that owner, and
`PERF-USERBOX-027` now splits the runtime exact top symbols inside that owner.
The startup executable owner still has signal, so keep the nyash_kernel_runtime
lane closed from env / stdio / path and only split within the exact runtime
subfamily when a finer owner is worth landing.
