---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-113.
Related:
  - docs/development/current/main/phases/phase-296x/296x-611-MIM-PORT-FMEM-112-PAGEMAPRELEASE-REALLOC-GUARD-REFRESH-CLOSEOUT.md
  - docs/development/current/main/phases/phase-296x/296x-608-MIM-PORT-FMEM-109-SOURCE-SYNTAX-SMOKE-MANIFEST-RUNNER.md
  - lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako
  - lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako
  - tools/hako_check/fastmem_source_syntax_smoke.sh
  - tools/hako_check/manifests/fastmem_source_syntax_smoke.toml
---

# 296x-612 MIM-PORT-FMEM-113 hako_alloc Next Body Slice Selection

## Purpose

Select the next narrow `.hako hako_alloc` body slice after PageMapRelease
pointer lookup, same/remote publish routing, and the realloc guard refresh have
landed.

This is a selection row only. It does not add a new body, change allocator
semantics, reopen product activation, or retire the remaining legacy source
syntax smoke blocks by itself.

## Worker Inventory Summary

The worker inventory compared four candidate directions:

```text
1. page_meta_local_free_to_free_refill_body_box.hako
2. page_meta_refill_then_free_head_alloc_body_box.hako
3. page_map_release_box.hako / runtime publish bridge follow-up
4. page_map_realloc_*.hako follow-up
```

The narrowest next row is:

```text
lang/src/hako_alloc/memory/page_meta_local_free_to_free_refill_body_box.hako
```

## Decision

Select a manifest/promotion row for the single-block
`local_free_head -> free_head` refill body:

```text
296x-613:
  page_meta_local_free_to_free_refill_body source-syntax manifest promotion
```

The body already exists and composes only landed MemOps:

```text
LocalFreePop(page)
FreeHeadPush(page, block)
```

The next row should move this one body out of the remaining large legacy
`fastmem_source_syntax_smoke.sh` path and into the source-syntax manifest runner
shape introduced by 296x-608.

## Why This Slice

```text
uses existing verified MemOps only
keeps the body page-meta-local
does not reopen PageMapRelease or realloc semantics after 296x-610/611
does not require new pointer derivation, PageKey, AddressToken, or RawPtr<T>
does not require branch route execution, TLS transfer, activation, hooks, or
global allocator claims
reduces legacy smoke pressure without adding another broad cleanup row
```

## Rejected For This Row

### Refill-Then-Free-Head Alloc

```text
candidate:
  lang/src/hako_alloc/memory/page_meta_refill_then_free_head_alloc_body_box.hako

reason:
  valuable but wider. It composes LocalFreePop, FreeHeadPush, FreeHeadPop, and
  used update evidence. It should follow after the smaller refill transfer body
  has manifest coverage.
```

### PageMapRelease Follow-Up

```text
candidate:
  lang/src/hako_alloc/memory/page_map_release_box.hako

reason:
  296x-609 through 296x-611 just stabilized this seam. A new release follow-up
  would likely mix source-truth migration with page-map/realloc semantics.
```

### Realloc Body Follow-Up

```text
candidate:
  lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako
  lang/src/hako_alloc/memory/page_map_realloc_alloc_copy_release_box.hako
  lang/src/hako_alloc/memory/page_map_realloc_failure_contract_box.hako

reason:
  too broad for the next row. M173-M176 are stable as guards, but realloc body
  migration mixes lookup, release ordering, allocation, copy modeling, and
  failure matrix behavior.
```

## Required Boundary For 296x-613

```text
selected body:
  page_meta_local_free_to_free_refill_body_box.hako

open:
  source-syntax manifest fixture for the single-block refill body
  expected AST/MIR/report/check rows for LocalFreePop + FreeHeadPush

closed:
  multi-block refill
  refill counters
  refill-then-free-head alloc
  page-local route branch execution
  PageMapRelease/realloc semantic changes
  remote owner routing
  AtomicRemoteHead changes
  TLS transfer
  product activation
  hook install
  global allocator claim
  winner claim
```

## Acceptance For 296x-613

```text
The selected refill body is represented in
tools/hako_check/manifests/fastmem_source_syntax_smoke.toml.

Its expected KV files prove:
  fastmem_region_count=1
  fastmem_contract_id=PageMapV0
  fastmem_memop_local_free_pop_count=1
  fastmem_memop_free_head_push_count=1
  fastmem_local_free_pop_lowerable_count=1
  fastmem_free_head_push_lowerable_count=1
  type_abi_hot_lookup_count=0
  provider_abi_hot_dispatch_count=0
  product_activation=0
  global_allocator_claim=0
  winner_claim=0

The corresponding legacy bespoke block is removed or reduced to the manifest
runner entry point.
```

## Verification

```bash
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
296x-612 selects the single-block local_free -> free_head refill body as the
next hako_alloc implementation slice. The next work is manifest promotion for
that existing body, not PageMapRelease/realloc mutation and not a new fastmem
substrate.
```

## Closeout

```text
next: 296x-613 MIM-PORT-FMEM-114 local-free-to-free refill source-syntax
manifest promotion
```
