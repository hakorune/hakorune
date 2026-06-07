---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-115.
Related:
  - docs/development/current/main/phases/phase-296x/296x-613-MIM-PORT-FMEM-114-LOCAL-FREE-TO-FREE-REFILL-SOURCE-SYNTAX-MANIFEST-PROMOTION.md
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
---

# 296x-614 MIM-PORT-FMEM-115 Remaining FastMemory Migration Target Inventory

## Purpose

Organize the remaining hako_alloc FastMemory migration work after
`LOCAL_FREE_TO_FREE_REFILL_BODY` moved into the source-syntax manifest runner.

This is an inventory and task-ordering row. It does not add another fixture,
change `.hako` allocator behavior, or widen the FastMemory substrate.

## Already Manifest-Backed

The source-syntax manifest currently owns:

```text
SAME_REMOTE_FREE_PUBLISH_BODY
SAME_OWNER_FREE_BODY
LOCAL_FREE_ALLOC_BODY
LOCAL_FREE_TO_FREE_REFILL_BODY
FREE_HEAD
PILOT
OWNER
OWNER_EQ
LOCAL_FREE_HEAD
LOCAL_FREE_MEMOP
```

These live in:

```text
tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
tools/hako_check/manifests/fastmem_source_syntax_smoke/*.expected.kv
```

## Remaining Source Bodies In Legacy Shell Blocks

### Strong Body Targets

```text
lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako
lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako
lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako
```

### Substrate / Vocabulary / Precondition Targets

```text
lang/src/hako_alloc/memory/page_meta_local_free_push_precondition_box.hako
lang/src/hako_alloc/memory/page_meta_local_free_pop_precondition_box.hako
lang/src/hako_alloc/memory/page_meta_free_head_push_vocabulary_box.hako
lang/src/hako_alloc/memory/page_meta_free_head_push_precondition_box.hako
lang/src/hako_alloc/memory/page_meta_free_head_pop_vocabulary_box.hako
lang/src/hako_alloc/memory/page_meta_free_head_pop_precondition_box.hako
lang/src/hako_alloc/memory/page_meta_atomic_remote_head_push_vocabulary_box.hako
lang/src/hako_alloc/memory/page_meta_atomic_remote_head_drain_vocabulary_box.hako
lang/src/hako_alloc/memory/page_meta_drain_remote_list_to_local_vocabulary_box.hako
lang/src/hako_alloc/memory/page_meta_remote_owner_branch_routing_lowering_box.hako
lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako
lang/src/hako_alloc/memory/page_meta_fastmem_branch_return_scope_box.hako
```

## Not Source FastMemory Bodies Yet

These remain allocator model, runtime bridge, page-map/realloc seam, policy, or
diagnostic surfaces:

```text
lang/src/hako_alloc/memory/page_meta_same_remote_free_publish_body_runtime_box.hako
lang/src/hako_alloc/memory/page_map_release_box.hako
lang/src/hako_alloc/memory/page_map_bridge_box.hako
lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako
lang/src/hako_alloc/memory/page_map_realloc_alloc_copy_release_box.hako
lang/src/hako_alloc/memory/page_map_realloc_failure_contract_box.hako
lang/src/hako_alloc/memory/allocator_facade_box.hako
lang/src/hako_alloc/memory/page_box.hako
lang/src/hako_alloc/memory/page_heap_box.hako
lang/src/hako_alloc/memory/page_map_box.hako
lang/src/hako_alloc/memory/size_class_box.hako
lang/src/hako_alloc/memory/remote_free_policy_box.hako
lang/src/hako_alloc/memory/worker_tls_*.hako
lang/src/hako_alloc/memory/provider_*.hako
lang/src/hako_alloc/memory/segment_*.hako
lang/src/hako_alloc/memory/reclaim_*.hako
lang/src/hako_alloc/memory/purge_*.hako
```

They should not be swept into source-syntax manifest promotion rows without a
separate selection card, because most of them are not narrow FastMemory source
body fixtures.

## Next Three Rows

### 296x-615: Refill Counter Body Manifest Promotion

```text
file:
  lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_counter_body_box.hako

open:
  manifest fixture for LocalFreePop + FreeHeadPush plus refill counter
  field load/store evidence:
    local_free_collect_count
    local_free_collected_blocks

closed:
  multi-block refill
  refill-then-alloc
  route CFG execution
  PageMapRelease/realloc mutation
  TLS transfer
  product activation / hooks / global allocator / winner

guards:
  python3 tools/hako_check/fastmem_source_manifest_runner.py \
    --manifest tools/hako_check/manifests/fastmem_source_syntax_smoke.toml \
    --only LOCAL_FREE_TO_FREE_REFILL_COUNTER_BODY
  bash tools/hako_check/fastmem_source_syntax_smoke.sh
  bash tools/hako_check/fastmem_check_smoke.sh
  bash tools/checks/current_state_pointer_guard.sh
```

Risk: low-medium. It is mostly expectation migration, but counter field-group
evidence is easy to under-specify.

### 296x-616: Free-Head Alloc Body Manifest Promotion

```text
file:
  lang/src/hako_alloc/memory/page_meta_free_head_alloc_body_box.hako

open:
  manifest fixture for same-owner FreeHeadPop allocation body
  used + 1 field store evidence

closed:
  refill composition
  derived non-empty proof from FreeHeadPush
  branch route execution
  remote routing
  activation/product claims
```

Risk: low-medium. Narrower than refill-then-alloc; preserve FreeHeadPop
lowerable and non-empty expectations exactly.

### 296x-617: Refill-Then-Free-Head Alloc Body Manifest Promotion

```text
file:
  lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako

open:
  manifest fixture for LocalFreePop + FreeHeadPush + FreeHeadPop
  derived free-head non-empty evidence
  page_local_alloc_route_candidate=refill_then_free_head_alloc report keys

closed:
  page-local route branch execution
  multi-block refill
  TLS transfer
  product activation / hooks / global allocator / winner
```

Risk: medium. This composes refill transfer and allocation path; keep branch and
route claims closed.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
The remaining FastMemory migration targets are grouped into manifest-backed,
legacy shell-only source bodies, substrate/precondition rows, and non-source
FastMemory model/runtime surfaces. The next three rows are fixed as refill
counter body, free-head alloc body, and refill-then-free-head alloc body
manifest promotions.
```

## Closeout

```text
next: 296x-615 MIM-PORT-FMEM-116 refill counter body source-syntax manifest
promotion
```
