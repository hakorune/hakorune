Status: Active
Date: 2026-06-09
Scope: one-shot C vs .hako mimalloc direct-exact comparison sweep.
Related:
  - tools/perf/bench_compare_c_vs_hako.sh
  - tools/allocator/hako_mimalloc_direct_exact_pair.sh
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md

# Hako vs C Mimalloc Direct-Exact Comparison

This note records the one-shot sweep that compared the current `.hako`
direct-exact front against C mimalloc across the benchmark pairs that exist in
`benchmarks/bench_*.hako` and `benchmarks/c/bench_*.c`.

## Command

```bash
PERF_SKIP_VM_PREFLIGHT=1 bash tools/perf/bench_compare_c_vs_hako.sh <bench_key> 1 3
```

The sweep ran for 32 matching benchmark pairs.

## Summary

```text
bench_count=32
hako_slower_than_c=32
median_slowdown=99.0x
mean_slowdown=394.2x
```

The direct-exact representative small-block object-lifecycle pair remains
slower than C, but much closer than the worst micro-benchmarks:

```text
representative-object-lifecycle-small-block-v0
hako_body_elapsed_ns=7000000
c_body_elapsed_ns=3386976
ratio_c_over_hako=2.067
summary=ok
```

## Worst Slowdowns

```text
3628.7x  kilo_micro_userbox_counter_step_chain
2315.0x  kilo_micro_userbox_point_sum
1939.0x  kilo_micro_userbox_counter_step
1314.7x  kilo_micro_userbox_point_add
 433.0x  kilo_leaf_map_getset_has
 369.7x  kilo_leaf_array_rmw_add1
 363.2x  chip8_kernel_small
 343.7x  kilo_leaf_map_get_missing
 302.2x  kilo_micro_array_getset
 221.3x  method_call_only
```

## Least-Bad Workloads

```text
0.40  kilo_micro_userbox_flag_toggle
0.27  kilo_micro_substring_concat
0.27  kilo_meso_substring_concat_array_set_loopcarry
0.25  method_call_only_small
0.21  box_create_destroy_small
```

## Current Observation

The current userbox counter-heavy leader is no longer the same shape as the
original sweep. `kilo_micro_userbox_counter_step_chain` now lowers through the
selected exact-slot bridge and the C-ABI exact-seed emitter returns a closed
form result, so it is no longer the clearest residual owner for codegen work.

The first exact front with a still-measurable kernel-only body is now
`kilo_micro_userbox_point_add`:

```text
point_add kernel-only:
  ratio_kernel_cycles=1.00
  ratio_kernel_ms=1.00
```

The remaining gap on `counter_step_chain` is dominated by startup / loader
noise, so the next design choice is whether to invest in startup/loader
reduction or move the userbox optimization lane to `point_add`.

## Measurement Floor Repair Decision

The next implementation target is the measurement floor, not another optimizer
rewrite. `counter_step_chain` has already reached the closed-form exact kernel,
and `point_add` is kernel-only equivalent to C. That makes the current blocker
an attribution problem: the direct-helper harness floor run must be trustworthy
before startup / loader work can be selected.

```text
task=PERF-USERBOX-001
scope=direct-helper measurement harness floor run repair
startup_loader_attribution_report=required
tool=tools/allocator/userbox_direct_helper_floor_attribution.py
guard=tools/checks/k2_wide_phase296x_perf_userbox_floor_attribution_guard.sh

counter_step_chain_role=startup_loader_sentinel
counter_step_chain_exact_kernel_target=0
point_add_role=kernel_equivalence_guard
point_add_next_optimizer_target=0

touch_perf_harness_measurement_tooling=1
touch_hako_source=0
touch_mirbuilder=0
touch_route_planner=0
touch_exact_helper_lowering=0
touch_runtime_object_representation=0
```

Acceptance:

```text
direct_helper_floor_run_status=ok
direct_helper_floor_invalid_arraybox_handle_count=0
counter_step_chain_helper_vs_floor_measured=1
point_add_helper_vs_floor_measured=1
startup_loader_attribution_report=1
measurement_harness_failure_count=0
```

## Startup / Loader Owner Split

`PERF-USERBOX-002` splits the exact-AOT startup / loader owner before changing
optimizer code. It joins the userbox floor/helper attribution report with a
`ret0` exact-AOT startup perf-top run.

```text
task=PERF-USERBOX-002
scope=exact-AOT startup/loader owner split
tool=tools/allocator/userbox_startup_loader_owner_split.sh
guard=tools/checks/k2_wide_phase296x_perf_userbox_startup_loader_owner_split_guard.sh

startup_probe=ret0_exact_aot_spawn_runner
source_truth_change=0
mirbuilder_change=0
route_planner_change=0
exact_helper_lowering_change=0
runtime_object_representation_change=0
```

Required report:

```text
output_contract=perf-userbox-startup-loader-owner-split-v0
ret0_perf_top_available=1
startup_loader_primary_owner_family
startup_loader_dynamic_loader_pct
startup_loader_libc_process_pct
startup_loader_nyash_kernel_runtime_pct
startup_loader_top_0_symbol
attribution_floor_run_status=ok
attribution_startup_loader_attribution_report=1
summary=ok
```

## Static-libgcc Link Probe

`PERF-USERBOX-003` keeps the first link/startup optimization as a probe rather
than changing the default link mode. The immediate fix is to preserve
hyphen-prefixed linker flags passed through `HAKO_AOT_LDFLAGS`, then prove that
`-static-libgcc` removes the dynamic `libgcc_s` dependency from a ret0 exact-AOT
executable.

```text
task=PERF-USERBOX-003
scope=exact-AOT static-libgcc link probe
guard=tools/checks/k2_wide_phase296x_perf_aot_static_libgcc_probe_guard.sh

linker_flag_forwarding_fix=tools/ny_mir_builder.sh passes --libs="$HAKO_AOT_LDFLAGS"
candidate_ldflags=-static-libgcc
dynamic_needed_libgcc_s=0
default_link_mode_changed=0
full_static_link_default=0
```

`-static` is intentionally not selected here. It removes the dynamic executable
surface, but it also shifts the ret0 owner to statically linked Rust/glibc init
work. That is a wider runtime/package decision and must stay separate from this
small `libgcc_s` dependency probe.

## Minimal System Libs Link Probe

`PERF-USERBOX-004` keeps the next link/startup step as an explicit diagnostic
mode. The default linker recipe remains compatibility-first, but
`NYASH_LLVM_LINK_SYSTEM_LIBS=minimal` lets exact-AOT startup probes omit `-lm`
when the active benchmark does not need math symbols.

```text
task=PERF-USERBOX-004
scope=exact-AOT minimal system libs link probe
guard=tools/checks/k2_wide_phase296x_perf_aot_minimal_system_libs_probe_guard.sh

link_system_libs=minimal
candidate_ldflags=-static-libgcc
dynamic_needed_libgcc_s=0
dynamic_needed_libm=0
default_link_mode_changed=0
```

This is not a language/runtime decision. Math-using programs can still pass
`-lm` explicitly through `HAKO_AOT_LDFLAGS`, and the default `full` mode keeps
the historical `-ldl -lpthread -lm` link surface.

## NyRT Plugin Host Off Probe

`PERF-USERBOX-005` isolates NyRT entry startup from plugin host initialization.
The default remains `HAKO_NYRT_PLUGIN_HOST=auto`; the opt-out mode is a
diagnostic exact-EXE probe for startup attribution.

```text
task=PERF-USERBOX-005
scope=exact-AOT NyRT plugin-host-off startup probe
guard=tools/checks/k2_wide_phase296x_perf_nyrt_plugin_host_off_probe_guard.sh

nyrt_plugin_host_mode=off
plugin_host_init_skipped=1
default_plugin_host_mode_changed=0
```

This does not remove plugin support from normal NyRT executables. It only gives
the perf lane a clean floor when the active seed does not use plugin-provided
objects.

Observed ret0 owner split with the current diagnostic floor
(`-static-libgcc`, `NYASH_LLVM_LINK_SYSTEM_LIBS=minimal`,
`HAKO_NYRT_PLUGIN_HOST=off`, `NYASH_NYRT_SILENT_RESULT=1`):

```text
startup_loader_primary_owner_family=libc_process
startup_loader_dynamic_loader_pct=20.56
startup_loader_libc_process_pct=30.58
startup_loader_nyash_kernel_runtime_pct=17.92
startup_loader_minimal_main_pct=22.36
top_runtime_symbol=nyash_rust::runtime::global_hooks::set_from_runtime
next_owner=NyRT runtime hooks/env/stdio startup floor
```

The follow-up probe for that floor is `PERF-USERBOX-006`, which keeps the
plugin-host-off floor and adds `NYASH_NYRT_RUNTIME_HOOKS=off` so the startup
sample can separate runtime-hooks publication from the remaining env / stdio
costs without changing the default runtime behavior.

`PERF-USERBOX-007` then adds `NYASH_NYRT_MINIMAL_STARTUP=1` on top of the
same floor. That probe bypasses default registry startup, which removes
`box_factory_policy_mode` from the startup top and keeps the next evidence on
the env / stdio surface instead of the registry policy path.

`NYRT-STARTUP-FLOOR-001` follows by treating env / stdio / registry as one
NyRT entry startup-floor owner first. It builds one ret0 `ny_main` object and
links it through both current minimal NyRT entry and bare libc `main` so the
next decision can separate NyRT entry delta from the remaining loader / libc
floor before any individual startup subsystem is optimized.

## Next Optimization Focus

The sweep suggests the next exact front should come from the userbox / counter
heavy family first, then the map / array leaf family:

```text
1. kilo_micro_userbox_counter_step_chain
2. kilo_micro_userbox_point_sum
3. kilo_micro_userbox_counter_step
4. kilo_micro_userbox_point_add
5. kilo_leaf_map_getset_has
```

These are the largest current gaps and are the most likely to produce a
meaningful exact-front win before widening to broader allocator surfaces.
