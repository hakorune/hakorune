---
Status: Active
Date: 2026-06-09
Scope: select the next exact front for direct-exact Hako vs C mimalloc optimization.
Blocker: HAKO-MIMALLOC-DIRECT-EXACT-OPTIMIZATION-SWEEP-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - tools/perf/bench_compare_c_vs_hako.sh
  - tools/allocator/hako_mimalloc_direct_exact_pair.sh
---

# 296x-648 Hako Mimalloc Direct-Exact Optimization Sweep Selection

## Purpose

Select the next exact front for the direct-exact Hako vs C mimalloc
optimization lane from the one-shot comparison sweep.

This row does not change allocator semantics. It turns the comparison evidence
into a narrow next-front selection so the optimization phase can start from a
known hot workload instead of reopening the whole queue.

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
output_contract=hako-mimalloc-direct-exact-optimization-sweep-selection-v0
selected_exact_front=kilo_micro_userbox_counter_step_chain
selected_next_front_family=userbox_counter_heavy
comparison_doc_linked=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
representative-object-lifecycle-small-block-v0:
  hako_body_elapsed_ns=7000000
  c_body_elapsed_ns=3386976
  ratio_c_over_hako=2.067

worst_10_exact_fronts:
  kilo_micro_userbox_counter_step_chain
  kilo_micro_userbox_point_sum
  kilo_micro_userbox_counter_step
  kilo_micro_userbox_point_add
  kilo_leaf_map_getset_has
```

## Guard

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mimalloc-current.md
docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md
```

## Stop Line

Do not add provider activation, replacement, hooks, global allocator claims, or
allocator-replacement semantics in this row.

## Next

```text
MIMALLOC-DIRECT-EXACT-OPTIMIZATION-001:
  userbox_counter_heavy exact front optimization ladder
```
