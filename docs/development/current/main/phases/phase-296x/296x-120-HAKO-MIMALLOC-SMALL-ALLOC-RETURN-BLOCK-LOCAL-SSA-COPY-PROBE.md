---
Status: Current
Date: 2026-05-28
Scope: classify local SSA copy materialization inside objectLifecycleSmallAlloc return blocks.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-119-HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE.md
---

# 296x-120 Hako Mimalloc Small Alloc Return Block Local SSA Copy Probe

## Purpose

Row119 showed return blocks contain copy pressure, but not copies into the
return value itself:

```text
return_block_copy_count=23
copy_to_return_value_count=0
next_action=local_ssa_copy_probe
```

Classify whether these copies come from receiver materialization, argument
materialization, or duplicate reason-object calls before selecting another
MIR-builder change.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0
input_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
return_block_copy_count=23
receiver_copy_count
arg_copy_count
duplicate_reason_call_count
next_action=receiver_materialization_probe|arg_materialization_probe|reason_call_probe|stop_line
summary=ok
```

## Stop Line

Do not implement a MIR builder change in this row.
