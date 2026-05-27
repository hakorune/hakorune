---
Status: Current
Date: 2026-05-28
Scope: probe whether binding failure reasons once in .hako removes duplicate MIR reason calls.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-121-HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE.md
---

# 296x-122 Hako Mimalloc Small Alloc Hako Reason Bind Probe

## Purpose

Row121 showed that nested failure calls duplicate reason global calls in MIR:

```text
source_reason_call_count=5
reason_call_count=10
duplicate_unused_reason_call_count=5
next_action=hako_reason_bind_probe
```

Probe whether the narrow `.hako` source shape:

```text
local reason = HakoAllocObjectLifecycleFacadeReason.small_no_page()
return me.recordSmallAllocFailure(reason)
```

removes duplicate reason calls before selecting a keeper.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0
input_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
before_reason_call_count=10
after_reason_call_count
after_duplicate_reason_call_count
next_action=apply_hako_reason_bind_keeper|reason_singleton_lowering_probe|stop_line
summary=ok
```

## Stop Line

This row may use a temporary probe copy or patch-and-restore workflow, but must
not land the `.hako` keeper unless the row is explicitly converted into an
implementation row.
