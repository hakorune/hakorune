---
Status: Active
Date: 2026-06-09
Scope: optimize the userbox/counter-heavy exact front family selected by the direct-exact comparison sweep.
Blocker: HAKO-MIMALLOC-USERBOX-COUNTER-HEAVY-OPTIMIZATION-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/hako-optimization-toolbox-usability-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - tools/perf/bench_compare_c_vs_hako.sh
  - tools/perf/bench_micro_c_vs_aot_stat.sh
  - tools/perf/bench_micro_aot_asm.sh
---

# 296x-649 Hako Mimalloc Userbox Counter-Heavy Optimization

## Purpose

Start the exact-front optimization pass from the hottest remaining userbox
counter family.

The direct-exact comparison sweep showed that the current Hako implementation
is slower than C on every matching benchmark pair, and the worst slowdowns are
concentrated in the userbox/counter-heavy family.

This row does not reopen provider activation, allocator replacement, hooks, or
global allocator claims. It only narrows the next optimization owner and keeps
the measurement surfaces fixed.

## Exact Front

```text
primary_exact_front=kilo_micro_userbox_counter_step_chain
family=userbox_counter_heavy
next_exact_front_candidates=
  kilo_micro_userbox_point_sum,
  kilo_micro_userbox_counter_step,
  kilo_micro_userbox_point_add,
  kilo_leaf_map_getset_has
```

## Required Input

```text
output_contract=hako-vs-c-mimalloc-direct-exact-comparison-v0
bench_count=32
hako_slower_than_c=32
median_slowdown=99.0x
mean_slowdown=394.2x
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-userbox-counter-heavy-optimization-v0
selected_exact_front=kilo_micro_userbox_counter_step_chain
selected_family=userbox_counter_heavy
source_surface_ready=1
mir_shape_ready=1
measurement_ready=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## First Diagnostic Facts

```text
source_perf_surface:
  target_method_0=step_chain
  target_method_0_method_call_count=1
  target_method_1=step
  target_method_1_field_get_count=1

mir_shape:
  selected_method=Counter.step_chain/0
  mir_instruction_count=3
  call_count=1
  copy_count=1
  return_count=1

direct_exact_pair:
  hako_body_elapsed_ns=7000000
  c_body_elapsed_ns=3386976
  ratio_c_over_hako=2.067
```

## First Commands

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/hako_check.sh perf-surface --target benchmarks/bench_kilo_micro_userbox_counter_step_chain.hako --contract-version v1
python3 tools/mir_check/method_shape_report.py --mir-json <mir-json> --method Counter.step_chain/0
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_userbox_counter_step_chain 1 5
```

## Stop Line

- do not reopen provider activation, hooks, global allocator claims, or
  winner claims
- do not rewrite `kilo` benchmark semantics just to chase a measurement without
  MIR/source evidence
- do not mix userbox counter optimization with unrelated allocator-api work

## Next

```text
MIMALLOC-USERBOX-COUNTER-HEAVY-001:
  reduce step_chain dispatch/boxing cost with source and MIR evidence fixed
```
