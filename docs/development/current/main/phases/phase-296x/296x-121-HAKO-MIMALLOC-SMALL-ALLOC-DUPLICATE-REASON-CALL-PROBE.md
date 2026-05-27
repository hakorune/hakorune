---
Status: Current
Date: 2026-05-28
Scope: classify duplicate reason global calls in objectLifecycleSmallAlloc failure return blocks.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-120-HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE.md
---

# 296x-121 Hako Mimalloc Small Alloc Duplicate Reason Call Probe

## Purpose

Row120 found that failure return blocks call the same reason global twice:

```text
reason_call_count=10
duplicate_reason_call_count=5
next_action=reason_call_probe
```

Classify whether the next keeper should bind the reason once in `.hako`, add a
MIR call-CSE probe, or introduce a narrow reason-singleton lowering rule.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
input_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
duplicate_reason_call_count=5
reason_call_count=10
failure_return_block_count=5
next_action=hako_reason_bind_probe|mir_call_cse_probe|reason_singleton_lowering_probe|stop_line
summary=ok
```

## Stop Line

Do not implement a `.hako` keeper or MIR builder change in this row.
