---
Status: Active
Date: 2026-06-13
Scope: MIMALLOC-USERBOX-COUNTER-HEAVY-001 baseline refresh and build-seam unblock.
Blocker: HAKO-MIMALLOC-USERBOX-COUNTER-HEAVY-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-649-HAKO-MIMALLOC-USERBOX-COUNTER-HEAVY-OPTIMIZATION.md
  - tools/perf/build_perf_release.sh
  - tools/perf/bench_micro_c_vs_aot_stat.sh
  - tools/hako_check.sh
---

# 296x-657 MIMALLOC-USERBOX-COUNTER-HEAVY-001 Baseline Refresh

## Decision

Resume the userbox/counter-heavy exact-front optimization lane from the
selected front:

```text
selected_exact_front=kilo_micro_userbox_counter_step_chain
selected_method=Counter.step_chain/0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

This row records the current source/MIR/perf evidence before changing lowering
or route behavior.

## Release Build Seam

The first perf run failed because the default release artifacts were out of
sync. `tools/perf/build_perf_release.sh` then exposed a `nyash_kernel` release
compile seam:

```text
typed_object dispatch helper duplicated between parent module and types.rs
typed_layouts/from_tag visibility did not match the split
entry.rs referenced private config::env::paths module
```

The seam is fixed before taking the baseline. This is a build hygiene repair,
not an optimization keeper.

## Source Surface

Command:

```bash
bash tools/hako_check.sh perf-surface \
  --target benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako \
  --methods step_chain,step \
  --contract-version v1
```

Result:

```text
output_contract=hako-check-perf-surface-v1
target_method_0=step_chain
target_method_0_method_call_count=1
target_method_1=step
target_method_1_field_get_count=1
winner_claim=0
replacement_active=0
summary=ok
```

## MIR Shape

Command:

```bash
NYASH_FEATURES=stage3,rune target/release/hakorune --backend mir \
  --emit-mir-json target/mimalloc-userbox-counter-heavy/counter_step_chain.mir.json \
  benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako

python3 tools/mir_check/method_shape_report.py \
  --mir-json target/mimalloc-userbox-counter-heavy/counter_step_chain.mir.json \
  --method Counter.step_chain/0
```

Result:

```text
output_contract=hako-mir-method-shape-v0
selected_method=Counter.step_chain/0
mir_instruction_count=3
call_count=1
field_get_count=0
field_set_count=0
copy_count=1
return_count=1
summary=ok
```

## Direct Exact Baseline

Command:

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh \
  kilo_micro_userbox_counter_step_chain 1 5
```

Result:

```text
name=kilo_micro_userbox_counter_step_chain
c_instr=129071
c_cycles=210353
c_cache_miss=4018
c_ms=3
ny_aot_instr=476590
ny_aot_cycles=756612
ny_aot_cache_miss=10496
ny_aot_ms=4
ratio_instr=0.27
ratio_cycles=0.28
ratio_ms=0.75
c_ipc=0.61
ny_aot_ipc=0.63
aot_status=ok
```

Interpretation:

```text
hako_instr_over_c=3.69x
hako_cycles_over_c=3.60x
ipc_shape=similar
current_owner=step_chain_call_dispatch_and_boxing_cost
```

## Next

```text
MIMALLOC-USERBOX-COUNTER-HEAVY-001A:
  inspect the selected AOT route for Counter.step_chain/0 and Counter.step/0.
  Prefer a route/lowering owner over MIRBuilder changes unless evidence shows
  selected route loss.
```

Stop line:

```text
do not reopen provider activation
do not claim allocator replacement
do not change benchmark semantics
do not optimize startup/loader from this row
do not add source-level thread/worker semantics
```
