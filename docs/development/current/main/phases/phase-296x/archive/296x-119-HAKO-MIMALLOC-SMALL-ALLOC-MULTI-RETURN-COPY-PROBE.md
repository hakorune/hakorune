---
Status: Landed
Date: 2026-05-28
Scope: classify the remaining multi-return/copy shape after single-pred PHI elision.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-118-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION.md
---

# 296x-119 Hako Mimalloc Small Alloc Multi-Return Copy Probe

## Purpose

Single-pred PHI elision removed the single-incoming PHI bloat
(`61 -> 0`) and reduced instruction count (`247 -> 191`), but copy count is
still high (`94 -> 99`) and the remaining candidate source is
`multi_return_join`.

Classify whether the next owner is return lowering, local SSA copy materialize,
or `.hako` control-shape cleanup.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0
input_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
return_count=6
copy_count=99
copy_from_phi_count=19
candidate_source=multi_return_join
next_action=return_lowering_probe|local_ssa_copy_probe|hako_shape_probe|stop_line
summary=ok
```

## Stop Line

Do not implement a second MIR builder change in this row.

## Landed Evidence

```text
output_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0
input_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
return_count=6
copy_count=99
copy_from_phi_count=19
candidate_source=multi_return_join
return_block_copy_count=23
failure_return_block_count=5
failure_return_copy_count=15
success_return_block_count=1
success_return_copy_count=8
copy_to_return_value_count=0
selected_reason=return_blocks_copy_call_receivers_and_args_not_return_values
next_action=local_ssa_copy_probe
next_diagnostic=small_alloc_return_block_local_ssa_copy_probe
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_multi_return_copy_probe_guard.sh
```
