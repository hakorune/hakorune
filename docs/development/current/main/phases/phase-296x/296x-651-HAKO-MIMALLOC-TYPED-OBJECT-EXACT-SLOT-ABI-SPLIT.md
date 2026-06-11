---
Status: Active
Date: 2026-06-10
Scope: split typed-object exact slot ABI from compat field access before the next C-speed user-box optimization.
Blocker: HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - crates/nyash_kernel/src/exports/typed_object.rs
  - crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs
  - tools/perf/bench_micro_aot_asm.sh
  - tools/perf/bench_micro_c_vs_aot_stat.sh
---

# 296x-651 Hako Mimalloc Typed Object Exact Slot ABI Split

## Purpose

The current user-box counter-heavy optimization moved the hot owner from broad
compat extraction to the exact-lane field ABI boundary. The next work must not
continue shaving `field_get_hii` as a unified helper. It must split exact slot
routes from the public/compat field route.

## Decision

```text
exact_lane_abi_separate_from_compat=1
field_get_hii_exact_ssot=0
field_get_hii_compat_legacy_adapter=1
i64_field_benchmark_primary_route=hako.typed_object.slot_load_i64
i64_field_helper_bridge=hako.object.exact_slot_get_i64_hii
handle_field_helper_bridge=hako.object.exact_slot_get_handle_hii
typed_object_exact_lowering_form=exact_helper_bridge
helper_internal_dispatch_keeper=0
native_direct_final_target=1
native_direct_open_in_002=0
```

## Required Output

```text
output_contract=hako-mimalloc-typed-object-exact-slot-abi-split-v0
typed_object_exact_slot_abi_split=1
typed_object_field_get_hii_compat_only=1
typed_object_get_compat_i64_count=0
typed_object_exact_internal_dispatch_count=0
typed_object_exact_silent_fallback_count=0
typed_object_exact_name_lookup_count=0
typed_object_exact_lowering_form=exact_helper_bridge
typed_object_exact_bridge_symbol=hako.object.exact_slot_get_i64_hii
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Task Ladder

```text
TYPEDOBJ-ABI-000:
  Land the typed-object exact slot ABI SSOT.

TYPEDOBJ-ABI-001:
  Add report/check vocabulary for typed_object.slot_load/store_* routes,
  compat field-get counts, exact helper counts, internal dispatch counts, and
  silent fallback counts.

TYPEDOBJ-ABI-002:
  Route the i64 user-box benchmark through hako.typed_object.slot_load_i64
  and hako.object.exact_slot_get_i64_hii when proof is selected.
  Keep lowering_form=exact_helper_bridge.
  Do not open NativeDirect.

TYPEDOBJ-ABI-003:
  Keep field_get_hii on compat/legacy only. It may exist, but it must not be
  accepted as exact-lane keeper evidence.

TYPEDOBJ-ABI-004:
  Select the first NativeDirect typed-object slot load/store pilot after route
  evidence proves the helper boundary is the remaining owner.
```

## First Implementation Slice

```text
target=TYPEDOBJ-ABI-002
behavior_change=selected exact route to helper-backed bridge
must_not_change=NativeDirect inline lowering
must_not_add=benchmark-name special cases
```

## TYPEDOBJ-ABI-002 Shape

```text
semantic_route=hako.typed_object.slot_load_i64
lowering_form=exact_helper_bridge
bridge_symbol=hako.object.exact_slot_get_i64_hii
fallback_policy=fail_fast
field_get_hii_used_as_exact_keeper=0
get_compat_i64_used_by_selected_exact_route=0
helper_internal_dispatch_keeper=0
native_direct_ready=0
```

`nyash.object.exact_slot_get_i64_hii` may remain as a legacy export alias during
the migration, but route/report truth stays in the `hako.*` namespace.

## First Commands

```bash
bash tools/checks/current_state_pointer_guard.sh
cargo fmt --check
git diff --check
```

## Quick Gate Repair

```text
mir_metadata_catalog_guard_owner=src/mir/function/metadata.rs
mir_root_facade_allowlist_sync=refresh_function_map_repr_plans,refresh_module_map_repr_plans
allowlist_rationale=refresh orchestration entry points only; MapReprPlan vocabulary stays in src/mir/map_repr_plan.rs
behavior_change=0
```

## Stop Line

- do not make `field_get_hii` the exact slot SSOT
- do not route i64 field benchmarks through `hako.object.exact_slot_get_handle_hii`
- do not hide selected exact routes inside helper-internal dispatch
- do not silently fall back after an exact route is selected
- do not reopen provider activation, hooks, global allocator claims, or winner
  claims

## Next

```text
HAKO-MIMALLOC-TYPED-OBJECT-EXACT-SLOT-ABI-SPLIT-296X-001:
  landed report/check vocabulary for exact slot versus compat field routes

Next:
  repair the direct-helper measurement harness floor run before opening the
  next optimizer target. counter_step_chain is now a startup/loader sentinel,
  and point_add is a kernel equivalence guard.

PERF-USERBOX-001:
  direct-helper floor run must report ok, invalid ArrayBox handle count must be
  zero, and startup/loader attribution must be available for counter_step_chain
  and point_add. Do not change .hako, MIRBuilder, route planning, exact helper
  lowering, or runtime object representation in this measurement-floor slice.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_userbox_floor_attribution_guard.sh

PERF-USERBOX-002:
  split the exact-AOT startup/loader owner with a ret0 startup perf-top probe
  before changing optimizer/runtime code. The report must keep source,
  MIRBuilder, route planner, exact helper lowering, and runtime object
  representation closed.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_userbox_startup_loader_owner_split_guard.sh

PERF-USERBOX-003:
  keep the first link/startup optimization as a probe: fix hyphen-prefixed
  `HAKO_AOT_LDFLAGS` forwarding, prove `-static-libgcc` removes dynamic
  `libgcc_s`, and do not change the default link mode or select full static
  linking.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_aot_static_libgcc_probe_guard.sh

PERF-USERBOX-004:
  add an explicit minimal system-libs probe mode for exact-AOT startup/link
  attribution. The default link mode stays `full`; the probe uses
  `NYASH_LLVM_LINK_SYSTEM_LIBS=minimal` plus `-static-libgcc` to prove ret0 can
  drop dynamic `libm` and `libgcc_s` without selecting full static linking.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_aot_minimal_system_libs_probe_guard.sh

PERF-USERBOX-005:
  add an explicit NyRT exact-EXE plugin-host-off probe mode for startup
  attribution. The default stays `auto`; the probe uses
  `HAKO_NYRT_PLUGIN_HOST=off` to skip plugin host initialization for ret0 /
  no-plugin seeds.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_nyrt_plugin_host_off_probe_guard.sh

PERF-USERBOX-006:
  add an explicit NyRT runtime-hooks-off probe mode for startup attribution.
  The default stays `auto`; the probe uses `NYASH_NYRT_RUNTIME_HOOKS=off` to
  skip the runtime-hooks publication step for ret0 / no-plugin seeds.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_nyrt_runtime_hooks_off_probe_guard.sh

PERF-USERBOX-007:
  add an explicit NyRT minimal-startup probe mode for startup attribution.
  The default stays full; the probe uses `NYASH_NYRT_MINIMAL_STARTUP=1` to
  bypass default registry startup for ret0 / no-plugin seeds.

Guard:
  bash tools/checks/k2_wide_phase296x_perf_nyrt_minimal_startup_probe_guard.sh

NYRT-STARTUP-FLOOR-001:
  add a bare-entry floor A/B probe. The probe builds one ret0 `ny_main` object
  and links it both through the current minimal NyRT entry and through a tiny
  bare libc `main`, then reports the entry delta and top-symbol attribution.
  This is diagnostic-only evidence; it does not change the default NyRT entry
  or move ownership back to `.hako`, MIRBuilder, route planning, exact helper
  lowering, or runtime object representation.

Guard:
  bash tools/checks/k2_wide_phase296x_nyrt_startup_floor_bare_entry_ab_probe_guard.sh

NYRT-STARTUP-FLOOR-002:
  add a runtime-build-off diagnostic probe. The default stays `auto`; the probe
  uses `NYASH_NYRT_RUNTIME_BUILD=off` together with
  `NYASH_NYRT_RUNTIME_HOOKS=off` to skip `NyashRuntimeBuilder` / GC controller
  construction inside the current minimal NyRT entry. GC metrics still require
  runtime build and fail-fast if requested with this mode.

Guard:
  bash tools/checks/k2_wide_phase296x_nyrt_runtime_build_off_probe_guard.sh

NYRT-STARTUP-FLOOR-003:
  add an entry-path-prep-off diagnostic probe. The default stays `auto`; the
  probe uses `NYASH_NYRT_ENTRY_PATH_PREP=off` together with
  `HAKO_NYRT_PLUGIN_HOST=off` to skip `current_exe` / PATH / PYTHONHOME
  preparation inside the current minimal NyRT entry.

Guard:
  bash tools/checks/k2_wide_phase296x_nyrt_entry_path_prep_off_probe_guard.sh

Next:
  Use the bare-entry A/B delta to choose the next owner. Large delta opens
  NyRT entry decomposition (env / stdio / registry); runtime-build-off evidence
  separates `NyashRuntimeBuilder` / GC controller cost from the remaining
  loader / libc floor, and entry-path-prep-off isolates `current_exe` / path
  shaping from the rest of the entry cost.
```
