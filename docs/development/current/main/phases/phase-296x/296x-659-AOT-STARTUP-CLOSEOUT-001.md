---
Status: Closeout
Date: 2026-06-14
Scope: exact-AOT startup closeout for folded micro kernels.
Blocker: HAKO-MIMALLOC-AOT-KERNEL-FRONT-SELECT-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-658-MIMALLOC-USERBOX-COUNTER-HEAVY-001A-OWNER-RECLASSIFICATION.md
  - docs/development/current/main/design/perf-userbox-link-startup-attribution-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - tools/allocator/nyrt_startup_floor_bare_entry_ab_probe.sh
  - tools/perf/bench_micro_c_vs_aot_lanes.sh
---

# 296x-659 AOT-STARTUP-CLOSEOUT-001

## Decision

Close the exact-AOT startup optimization lane for folded micro kernels.

Keep startup measurement as a diagnostic/profile surface, not as the next
default NyRT-entry optimization owner. The next implementation lane is
non-folded exact kernel front selection.

```text
startup_lane_closed=1
minimal_profile_kind=diagnostic_only
minimal_profile_product_default=0
product_default_nyrt_entry_changed=0
default_lazy_loading_changed=0
next_kernel_front_selection_required=1
```

## Interpretation

The current measurements support **A + B + C-lite**:

```text
A. close startup optimization lane for now
B. keep startup as diagnostic/profile surface
C. officialize a minimal exact-AOT diagnostic profile, but not as product
   default NyRT entry
```

Do not continue with:

```text
D. more default NyRT entry lazy-loading
E. symbol chasing for Arc::drop_slow / gc_mode::from_env / std::env::_var_os
```

Those can reopen only if a single residue owner reappears with a clear,
product-safe win estimate.

## Evidence

`kilo_micro_userbox_counter_step_chain` is no longer a kernel optimization
front. It is a startup sentinel.

Command:

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh \
  kilo_micro_userbox_counter_step_chain 1 5 50
```

Result:

```text
ny_total_instr=472399
ny_startup_instr=470376
ny_kernel_instr=6191
ny_total_cycles=749035
ny_startup_cycles=775172
ny_kernel_cycles=8867
```

Interpretation:

```text
bench_key=kilo_micro_userbox_counter_step_chain
bench_role=startup_sentinel
kernel_constant_folded=1
kernel_dispatch_boxing_hot_path=0
exact_kernel_optimization_front=0
nyrt_startup_dominates_total=1
```

## Startup A/B Rerun

Command:

```bash
HAKO_NYRT_PLUGIN_HOST=off \
NYASH_NYRT_RUNTIME_HOOKS=off \
NYASH_NYRT_MINIMAL_STARTUP=1 \
NYASH_NYRT_SILENT_RESULT=1 \
  tools/allocator/nyrt_startup_floor_bare_entry_ab_probe.sh \
    --out target/nyrt-startup-rerun-20260614/bare_ab_minimal_auto_50.kv \
    --startup-runs 50
```

Result:

```text
current_minimal_cycles=16299633
bare_entry_cycles=9964759
entry_delta_cycles=6334874
entry_delta_ratio=1.635728
current_minimal_instructions=11984904
bare_entry_instructions=6737300
current_minimal_primary_owner_family=nyash_kernel_runtime
bare_entry_primary_owner_family=dynamic_loader
current_minimal_nyash_kernel_runtime_pct=38.22
current_minimal_libc_process_pct=21.99
current_minimal_dynamic_loader_pct=15.86
current_minimal_other_pct=23.94
```

Top observed symbols:

```text
alloc::sync::Arc<T,A>::drop_slow
nyash_rust::runtime::gc_mode::GcMode::from_env
__strlen_evex
```

## Full-Off Diagnostic Rerun

Command:

```bash
HAKO_NYRT_PLUGIN_HOST=off \
NYASH_NYRT_RING0_INIT=off \
NYASH_NYRT_RUNTIME_HOOKS=off \
NYASH_NYRT_RUNTIME_BUILD=off \
NYASH_NYRT_ENTRY_PATH_PREP=off \
NYASH_NYRT_MINIMAL_STARTUP=1 \
NYASH_NYRT_SILENT_RESULT=1 \
  tools/allocator/nyrt_startup_floor_bare_entry_ab_probe.sh \
    --out target/nyrt-startup-rerun-20260614/bare_ab_ring0_off_50.kv \
    --startup-runs 50
```

Result:

```text
current_minimal_cycles=14993404
bare_entry_cycles=9793386
entry_delta_cycles=5200018
entry_delta_ratio=1.530972
current_minimal_instructions=11230599
bare_entry_instructions=6737782
current_minimal_nyash_kernel_runtime_pct=15.88
current_minimal_libc_process_pct=16.68
current_minimal_dynamic_loader_pct=26.48
current_minimal_other_pct=37.65
```

Read:

```text
startup_gate_off_delta_reduction_pct=17.9
startup_owner_mixed=1
single_nyrt_owner_remaining=0
loader_libc_floor_visible=1
```

Turning off runtime build, path prep, and ring0 init reduces the entry delta,
but not enough to justify continuing default-entry lazy-loading as the next
compiler/runtime cleanup owner.

## Minimal Exact-AOT Diagnostic Profile

The minimal exact-AOT entry/profile is useful as a measurement instrument.
It must not become product default behavior.

```text
exact_aot_micro_profile_enabled=1
exact_aot_micro_profile_name=minimal-diagnostic
exact_aot_micro_profile_claims_product_speedup=0
exact_aot_micro_profile_disables_plugin_host=allowed
exact_aot_micro_profile_disables_runtime_hooks=allowed
exact_aot_micro_profile_disables_ring0=allowed
exact_aot_micro_profile_disables_runtime_build=allowed
exact_aot_micro_profile_disables_path_prep=allowed
exact_aot_micro_profile_requires_explicit_opt_in=1
```

Use this profile to subtract startup noise and protect exact-kernel work from
loader / libc / entry-floor artifacts.

## Guard Vocabulary

```text
output_contract=exact-aot-startup-closeout-v0
bench_key=kilo_micro_userbox_counter_step_chain
bench_role=startup_sentinel
kernel_constant_folded=1
kernel_dispatch_boxing_hot_path=0
exact_kernel_optimization_front=0
nyrt_startup_dominates_total=1
startup_owner_mixed=1
single_nyrt_owner_remaining=0
loader_libc_floor_visible=1
minimal_profile_kind=diagnostic_only
minimal_profile_product_default=0
product_default_nyrt_entry_changed=0
default_lazy_loading_changed=0
startup_lane_closed=1
next_kernel_front_selection_required=1
summary=ok
```

## Stop Line

```text
do not use counter_step_chain as a kernel optimization front
do not optimize Counter.step_chain dispatch from startup sentinel evidence
do not continue default NyRT entry lazy-loading for this microbench
do not chase Arc::drop_slow / gc_mode::from_env / std::env::_var_os unless a
  single owner reappears with a clear product-safe win estimate
do not chase loader/libc floor as compiler/runtime hot-path work
do not claim product default speedup from diagnostic minimal profile results
do not touch .hako, MIRBuilder, route planner, exact helper lowering, or
  runtime object representation from this row
```

## Next

Proceed to:

```text
MIMALLOC-AOT-KERNEL-FRONT-SELECT-001:
  select the next non-folded exact-AOT kernel front.
```

Selection criteria:

```text
kernel_not_constant_folded=1
kernel_instr_share_threshold_met=1
startup_sentinel_excluded=1
c_pair_available=1
exact_front_owner_family_selected=1
product_default_behavior_changed=0
```
