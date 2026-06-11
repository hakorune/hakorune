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
  Guard: PERF-USERBOX-001

PERF-USERBOX-002:
  split the exact-AOT startup/loader owner with a ret0 startup perf-top probe
  before changing optimizer/runtime code. The report must keep source,
  MIRBuilder, route planner, exact helper lowering, and runtime object
  representation closed.

Guard:
  Guard: PERF-USERBOX-002

PERF-USERBOX-003:
  keep the first link/startup optimization as a probe: fix hyphen-prefixed
  `HAKO_AOT_LDFLAGS` forwarding, prove `-static-libgcc` removes dynamic
  `libgcc_s`, and do not change the default link mode or select full static
  linking.

Guard:
  Guard: PERF-USERBOX-003

PERF-USERBOX-004:
  add an explicit minimal system-libs probe mode for exact-AOT startup/link
  attribution. The default link mode stays `full`; the probe uses
  `NYASH_LLVM_LINK_SYSTEM_LIBS=minimal` plus `-static-libgcc` to prove ret0 can
  drop dynamic `libm` and `libgcc_s` without selecting full static linking.

Guard:
  Guard: PERF-USERBOX-004

PERF-USERBOX-005:
  add an explicit NyRT exact-EXE plugin-host-off probe mode for startup
  attribution. The default stays `auto`; the probe uses
  `HAKO_NYRT_PLUGIN_HOST=off` to skip plugin host initialization for ret0 /
  no-plugin seeds.

Guard:
  Guard: PERF-USERBOX-005

PERF-USERBOX-006:
  add an explicit NyRT runtime-hooks-off probe mode for startup attribution.
  The default stays `auto`; the probe uses `NYASH_NYRT_RUNTIME_HOOKS=off` to
  skip the runtime-hooks publication step for ret0 / no-plugin seeds.

Guard:
  Guard: PERF-USERBOX-006

PERF-USERBOX-007:
  add an explicit NyRT minimal-startup probe mode for startup attribution.
  The default stays full; the probe uses `NYASH_NYRT_MINIMAL_STARTUP=1` to
  bypass default registry startup for ret0 / no-plugin seeds.

Guard:
  Guard: PERF-USERBOX-007

PERF-USERBOX-008:
  add a loader / libc floor summary probe for exact-AOT startup attribution.
  The default link mode stays `full`; the probe packages
  `-static-libgcc` plus `NYASH_LLVM_LINK_SYSTEM_LIBS=minimal` to show that the
  remaining floor is still `libc` / `ld-linux` without selecting full static
  linking.

Guard:
  Guard: PERF-USERBOX-008

PERF-USERBOX-009:
  add a loader owner split summary probe for exact-AOT startup attribution.
  The probe joins the startup loader owner split and loader/libc floor summary
  so `dynamic_loader` stays the primary startup owner family while the loader
  floor remains `libc` / `ld-linux`, without changing the default link mode.

Guard:
  Guard: PERF-USERBOX-009

PERF-USERBOX-010:
  add an ld-linux / libc loader split probe for exact-AOT startup attribution.
  The probe checks that the startup loader split still shows both `ld-linux`
  and `libc` top rows while the loader/libc floor summary remains closed, again
  without changing the default link mode.

Guard:
  Guard: PERF-USERBOX-010

PERF-USERBOX-011:
  add an ld-linux / libc loader percent split probe for exact-AOT startup
  attribution. The probe exposes ld-linux versus libc percent sums on the
  startup loader report so the loader floor can be split more precisely while
  the closed loader/libc floor summary stays in place.

Guard:
  Guard: PERF-USERBOX-011

PERF-USERBOX-012:
  add an ld-linux / libc loader symbol split probe for exact-AOT startup
  attribution. The probe exposes the leading ld-linux and libc symbols on the
  startup loader report so the loader floor can be split at symbol granularity
  while the closed loader/libc floor summary stays in place.

Guard:
  Guard: PERF-USERBOX-012

PERF-USERBOX-013:
  add a startup executable split probe for exact-AOT startup attribution. The
  probe exposes the ret0.exe executable contribution on the startup loader
  report so the startup executable can be split from the loader floor while the
  closed loader/libc summary stays in place.

Guard:
  Guard: PERF-USERBOX-013

PERF-USERBOX-014:
  add a startup executable symbol split probe for exact-AOT startup
  attribution. The probe exposes the leading ret0.exe symbol on the startup
  loader report so the executable contribution can be split at symbol
  granularity while the closed loader/libc summary stays in place.

Guard:
  Guard: PERF-USERBOX-014

PERF-USERBOX-015:
  add a startup executable ret0 stack split probe for exact-AOT startup
  attribution. The probe exposes the leading ret0.exe symbol stack on the
  startup loader report so the executable contribution can be split as a small
  ret0 stack while the closed loader/libc summary stays in place.

Guard:
  Guard: PERF-USERBOX-015

PERF-USERBOX-016:
  add a startup executable ret0 family split probe for exact-AOT startup
  attribution. The probe exposes the ret0.exe family buckets on the startup
  loader report so the executable contribution can be split by family while
  the closed loader/libc summary stays in place.

Guard:
  Guard: PERF-USERBOX-016

PERF-USERBOX-017:
  add a startup executable ret0 bucket split probe for exact-AOT startup
  attribution. The probe exposes the ret0.exe bucket counts on the startup
  loader report so the executable contribution can be split by env/path/ffi/
  alloc/once/string buckets while the closed loader/libc summary stays in
  place.

Guard:
  Guard: PERF-USERBOX-017

PERF-USERBOX-018:
  add a startup executable ret0 bucket variability probe for exact-AOT startup
  attribution. The probe repeats the ret0 bucket split three times so the
  executable contribution can be read as a distribution before choosing the
  next owner.

Guard:
  Guard: PERF-USERBOX-018

PERF-USERBOX-019:
  add a startup executable ret0 bucket symbol variability probe for exact-AOT
  startup attribution. The probe repeats the ret0 bucket split and aggregates
  the representative bucket symbols so env / once / path / minimal_main /
  nyash_kernel_runtime can be read as stable owner signals before choosing the
  next owner.

Guard:
  Guard: PERF-USERBOX-019

PERF-USERBOX-020:
  add a startup executable ret0 bucket subkind variability probe for exact-AOT
  startup attribution. The probe repeats the ret0 bucket split and aggregates
  the representative bucket subkinds so env / once / path / minimal_main /
  nyash_kernel_runtime can be split into getenv / pathprep / futex /
  components / registry signals before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-020

PERF-USERBOX-021:
  add a startup executable ret0 bucket top-symbol variability probe for
  exact-AOT startup attribution. The probe repeats the ret0 bucket split and
  aggregates the exact top symbols so env / once / path / minimal_main /
  nyash_kernel_runtime can be split into their exact top-symbol owners before
  choosing the next owner.

Guard:
  Guard: PERF-USERBOX-021

PERF-USERBOX-022:
  add a startup executable ret0 env top-symbol family variability probe for
  exact-AOT startup attribution. The probe repeats the ret0 bucket split and
  aggregates the env top-symbol family counts so env can be split into
  var_os / getenv / current_exe / current_dir signals before choosing the
  next owner.

Guard:
  Guard: PERF-USERBOX-022

PERF-USERBOX-023:
  add a startup executable ret0 env exact top-symbol variability probe for
  exact-AOT startup attribution. The probe repeats the ret0 bucket split and
  aggregates the env exact top symbols so env can be split into
  current_exe / minimal_startup_enabled_from_env / gc_mode signals before
  choosing the next owner.

Guard:
  Guard: PERF-USERBOX-023

PERF-USERBOX-024:
  add a startup executable ret0 path exact top-symbol
  variability probe for exact-AOT startup attribution. The probe repeats the
  ret0 bucket split and aggregates the path exact top symbols so path can be
  split into components / current_dir / current_exe / related exact symbols
  before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-024

PERF-USERBOX-025:
  add a startup executable ret0 nyash_kernel_runtime exact top-symbol
  variability probe for exact-AOT startup attribution. The path exact probe
  flatlined, so the probe repeats the ret0 bucket split and aggregates the
  nyash_kernel_runtime exact top symbols so registry / runtime / once /
  other exact symbols can be split before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-025

PERF-USERBOX-026:
  add a startup executable ret0 nyash_kernel_runtime registry exact top-symbol
  variability probe for exact-AOT startup attribution. The registry exact
  symbols are now visible inside the nyash_kernel_runtime owner, so the probe
  repeats the ret0 bucket split and aggregates the registry exact top symbols
  before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-026

PERF-USERBOX-027:
  add a startup executable ret0 nyash_kernel_runtime runtime exact-top-symbol
  variability probe for exact-AOT startup attribution. The registry exact
  probe is now closed, so the probe repeats the ret0 bucket split and
  aggregates the runtime exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-027

PERF-USERBOX-028:
  add a startup executable ret0 nyash_kernel_runtime build exact-top-symbol
  variability probe for exact-AOT startup attribution. The runtime exact
  probe is now closed, so the probe repeats the ret0 bucket split and
  aggregates the build exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-028

PERF-USERBOX-029:
  add a startup executable ret0 nyash_kernel_runtime build-registry
  exact-top-symbol variability probe for exact-AOT startup attribution. The
  build exact probe is now closed, so the probe repeats the ret0 bucket split
  and aggregates the build-registry exact top symbols before choosing the
  next owner.

Guard:
  Guard: PERF-USERBOX-029

PERF-USERBOX-030:
  add a startup executable ret0 nyash_kernel_runtime registry exact
  top-symbol variability probe for exact-AOT startup attribution. The
  build-registry exact probe is now closed, so the probe repeats the ret0
  bucket split and aggregates the registry exact top symbols before choosing
  the next owner.

Guard:
  Guard: PERF-USERBOX-030

PERF-USERBOX-031:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  exact-top-symbol variability probe for exact-AOT startup attribution. The
  registry exact probe is now closed, so the probe repeats the ret0 bucket
  split and aggregates the registry rebuild-cache exact top symbols before
  choosing the next owner.

Guard:
  Guard: PERF-USERBOX-031

PERF-USERBOX-032:
  add a startup executable ret0 nyash_kernel_runtime registry register-many
  exact-top-symbol variability probe for exact-AOT startup attribution. The
  registry rebuild-cache probe is now closed, so the probe repeats the ret0
  bucket split and aggregates the registry register-many exact top symbols
  before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-032

PERF-USERBOX-033:
  add a startup executable ret0 nyash_kernel_runtime registry
  create-default-registry exact-top-symbol variability probe for exact-AOT
  startup attribution. The registry register-many probe is now closed, so the
  probe repeats the ret0 bucket split and aggregates the registry
  create-default-registry exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-033

PERF-USERBOX-034:
  add a startup executable ret0 nyash_kernel_runtime registry
  create-default-registry rebuild-cache exact-top-symbol variability probe
  for exact-AOT startup attribution. The registry create-default-registry
  probe is now closed, so the probe repeats the ret0 bucket split and
  aggregates the registry create-default-registry rebuild-cache exact top
  symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-034

PERF-USERBOX-035:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  subkind variability probe for exact-AOT startup attribution. The registry
  create-default-registry rebuild-cache probe is now closed, so the probe
  repeats the ret0 bucket split and aggregates the registry rebuild-cache
  subkind counts before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-035

PERF-USERBOX-036:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  build exact-top-symbol variability probe for exact-AOT startup attribution.
  The registry rebuild-cache subkind probe is now closed, so the probe
  repeats the ret0 bucket split and aggregates the registry rebuild-cache
  build exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-036

PERF-USERBOX-037:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  gc_mode exact-top-symbol variability probe for exact-AOT startup
  attribution. The registry rebuild-cache build probe is now closed, so the
  probe repeats the ret0 bucket split and aggregates the registry
  rebuild-cache gc_mode exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-037

PERF-USERBOX-038:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  gc_mode_parse exact-top-symbol variability probe for exact-AOT startup
  attribution. The registry rebuild-cache gc_mode probe is now closed, so the
  probe repeats the ret0 bucket split and aggregates the registry
  rebuild-cache gc_mode_parse exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-038

PERF-USERBOX-039:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  tagged_checkpoint exact-top-symbol variability probe for exact-AOT startup
  attribution. The registry rebuild-cache gc_mode_parse probe is now closed,
  so the probe repeats the ret0 bucket split and aggregates the registry
  rebuild-cache tagged_checkpoint exact top symbols before choosing the next
  owner.

Guard:
  Guard: PERF-USERBOX-039

PERF-USERBOX-040:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  scheduler exact-top-symbol variability probe for exact-AOT startup
  attribution. The registry rebuild-cache tagged_checkpoint probe is now
  closed, so the probe repeats the ret0 bucket split and aggregates the
  registry rebuild-cache scheduler exact top symbols before choosing the next
  owner.

Guard:
  Guard: PERF-USERBOX-040

PERF-USERBOX-041:
  add a startup executable ret0 nyash_kernel_runtime registry rebuild-cache
  register-many exact-top-symbol variability probe for exact-AOT startup
  attribution. The registry rebuild-cache scheduler probe is now closed, so
  the probe repeats the ret0 bucket split and aggregates the registry
  rebuild-cache register-many exact top symbols before choosing the next
  owner.

Guard:
  Guard: PERF-USERBOX-041

PERF-USERBOX-042:
  add a startup executable ret0 nyash_kernel_runtime box_factory exact-top-
  symbol variability probe for exact-AOT startup attribution. The raw ret0
  report now shows PluginBoxFactory / BoxFactory ahead of registry lookup
  inside nyash_kernel_runtime, so the probe repeats the ret0 bucket split and
  aggregates the box_factory exact top symbols before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-042

PERF-USERBOX-043:
  add a startup executable ret0 nyash_kernel_runtime box_factory plugin/
  factory_type presence probe for exact-AOT startup attribution. The box_
  factory exact probe is now closed, so the probe repeats the ret0 bucket
  split and records whether a stable box_factory plugin/factory_type focus is
  actually present before choosing the next owner.

Guard:
  Guard: PERF-USERBOX-043

PERF-USERBOX-044:
  registry-focus presence probe for the ret0 startup report.
  Confirms registry focus stays visible after box_factory is absent.

Guard:
  PERF-USERBOX-044

PERF-USERBOX-045:
  rebuild-cache/register-many dominance probe for the ret0 startup report.
  Current runs favor rebuild-cache as the next registry owner.

Guard:
  PERF-USERBOX-045

PERF-USERBOX-046:
  rebuild-cache dominance probe with runner-up preservation.
  Keeps rebuild-cache dominant while preserving runner-up signals.

Guard:
  PERF-USERBOX-046

PERF-USERBOX-047:
  scheduler presence probe under the rebuild-cache-dominant report.
  Keeps scheduler visible as a presence signal.

Guard:
  PERF-USERBOX-047

PERF-USERBOX-048:
  register-many presence probe under the rebuild-cache-dominant report.
  Keeps register-many visible as a presence signal.

Guard:
  PERF-USERBOX-048

PERF-USERBOX-049:
  create-default-registry presence probe under the rebuild-cache-dominant report.
  Keeps create-default-registry visible as a presence signal.

Guard:
  PERF-USERBOX-049

PERF-USERBOX-050:
  ring0 presence probe under the rebuild-cache-dominant report.
  Keeps ring0 visible as a presence signal.

Guard:
  PERF-USERBOX-050

PERF-USERBOX-051:
  global_hooks presence probe under the rebuild-cache-dominant report.
  Keeps `global_hooks::set_from_runtime` visible as a presence signal.

Guard:
  PERF-USERBOX-051

PERF-USERBOX-052:
  exact-top-symbol probe for `global_hooks::set_from_runtime`.
  Keeps the exact signal visible under the rebuild-cache-dominant report.

Guard:
  PERF-USERBOX-052

PERF-USERBOX-053:
  exact-top-symbol probe for `Ring0Registry::build_with_fs`.
  Keeps the exact signal visible under the rebuild-cache-dominant report.

Guard:
  PERF-USERBOX-053

PERF-USERBOX-054:
  absence probe for `get_factory_order_by_policy`.
  Confirms the exact symbol stays absent under the rebuild-cache report.

Guard:
  PERF-USERBOX-054

PERF-USERBOX-056:
  exact-top-symbol variability probe for libc_process.
  Repeats the ret0 bucket split and keeps the family distribution visible.

Guard:
  PERF-USERBOX-056

PERF-USERBOX-057:
  exact-top-symbol split probe for libc_process malloc/getenv.
  Keeps the malloc versus getenv evidence visible before choosing the owner.

Guard:
  PERF-USERBOX-057

NYRT-STARTUP-FLOOR-001:
  add a bare-entry floor A/B probe. The probe builds one ret0 `ny_main` object
  and links it both through the current minimal NyRT entry and through a tiny
  bare libc `main`, then reports the entry delta and top-symbol attribution.
  This is diagnostic-only evidence; it does not change the default NyRT entry
  or move ownership back to `.hako`, MIRBuilder, route planning, exact helper
  lowering, or runtime object representation.

Guard:
  Guard: NYRT-STARTUP-FLOOR-001

NYRT-STARTUP-FLOOR-002:
  add a runtime-build-off diagnostic probe. The default stays `auto`; the probe
  uses `NYASH_NYRT_RUNTIME_BUILD=off` together with
  `NYASH_NYRT_RUNTIME_HOOKS=off` to skip `NyashRuntimeBuilder` / GC controller
  construction inside the current minimal NyRT entry. GC metrics still require
  runtime build and fail-fast if requested with this mode.

Guard:
  Guard: NYRT-STARTUP-FLOOR-002

NYRT-STARTUP-FLOOR-003:
  add an entry-path-prep-off diagnostic probe. The default stays `auto`; the
  probe uses `NYASH_NYRT_ENTRY_PATH_PREP=off` together with
  `HAKO_NYRT_PLUGIN_HOST=off` to skip `current_exe` / PATH / PYTHONHOME
  preparation inside the current minimal NyRT entry.

Guard:
  Guard: NYRT-STARTUP-FLOOR-003

NYRT-STARTUP-FLOOR-004:
  add a ring0-init-off diagnostic probe. The default stays `auto`; the probe
  uses `NYASH_NYRT_RING0_INIT=off` together with
  `HAKO_NYRT_PLUGIN_HOST=off`, `NYASH_NYRT_RUNTIME_HOOKS=off`,
  `NYASH_NYRT_RUNTIME_BUILD=off`, and `NYASH_NYRT_ENTRY_PATH_PREP=off` to
  skip Ring0Context initialization inside the current minimal NyRT entry.

Guard:
  Guard: NYRT-STARTUP-FLOOR-004

Next:
  Use the ret0.exe nyash_kernel_runtime registry rebuild-cache/register-many/
  create-default-registry exact top-symbol distribution to choose the next
  owner. The path exact top-symbol
  distribution flatlined in PERF-USERBOX-024, PERF-USERBOX-026 isolated the
  registry exact top symbols inside nyash_kernel_runtime, PERF-USERBOX-027
  isolated the runtime exact top symbols inside nyash_kernel_runtime,
  PERF-USERBOX-028 isolated the build exact top symbols inside
  nyash_kernel_runtime, PERF-USERBOX-029 isolated the build-registry exact
  top symbols inside nyash_kernel_runtime, PERF-USERBOX-030 isolated the
  registry exact top symbols inside nyash_kernel_runtime, PERF-USERBOX-031
  isolated the registry rebuild-cache exact top symbols inside
  nyash_kernel_runtime, PERF-USERBOX-032 now isolates the registry
  register-many exact top symbols inside nyash_kernel_runtime, PERF-USERBOX-033
  now isolates the registry create-default-registry exact top symbols inside
  nyash_kernel_runtime, PERF-USERBOX-034 now isolates the registry
  create-default-registry rebuild-cache exact top symbols inside
  nyash_kernel_runtime, PERF-USERBOX-035 now isolates the registry
  rebuild-cache subkind counts inside nyash_kernel_runtime, and
  PERF-USERBOX-036 now isolates the registry rebuild-cache build exact top
  symbols inside nyash_kernel_runtime, PERF-USERBOX-037 now isolates the
  registry rebuild-cache gc_mode exact top symbols inside nyash_kernel_runtime,
  PERF-USERBOX-038 now isolates the registry rebuild-cache gc_mode_parse exact
  top symbols inside nyash_kernel_runtime, PERF-USERBOX-039 now isolates the
  registry rebuild-cache tagged_checkpoint exact top symbols inside
  nyash_kernel_runtime, PERF-USERBOX-040 now isolates the registry
  PERF-USERBOX-041..054 keep the registry branch split readable.
  PERF-USERBOX-056..059 move the libc_process ladder from variability to
  malloc-family dominance.
  Keep env / stdio / path closed.
  Keep the entry-floor probes closed unless the executable owner flatlines.
```
