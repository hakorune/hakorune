---
Status: Current
Date: 2026-05-27
Scope: inline the small-alloc success result updates on the hot success path.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-108-HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH.md
---

# 296x-109 Hako Mimalloc Small-Alloc Inline Success Result Fast Path Keeper

## Purpose

Apply one BoxCount keeper selected by row108:

```text
keeper=small_alloc_inline_success_result_fast_path
keeper_kind=box_count
```

`objectLifecycleSmallAlloc/1` remains the top active owner after the accepted
direct select keeper. The next narrow candidate is to inline the hot success
result updates in `objectLifecycleSmallAlloc/1`, while preserving the generic
failure helpers and observer-compatible result capsule state.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-inline-success-result-fast-path-keeper-v0
input_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0
keeper=small_alloc_inline_success_result_fast_path
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
inline_success_result_used=1
failure_helpers_preserved=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not change failure semantics, observer getters, release behavior, provider
activation, replacement, hooks, globals, or winner claims in this row.
