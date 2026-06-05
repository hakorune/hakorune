---
Status: SSOT
Date: 2026-06-05
Scope: Mimalloc fidelity guard for Hakorune replacement-front execution shape.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-benchmark-route-taxonomy-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
  - tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  - tools/allocator/replacement_front_bins_templates.py
---

# Mimalloc Replacement-Front Fidelity

## Decision

Hakorune must not accept a fast replacement front as a keeper merely because it
is fast. A keeper must be fast through a mimalloc-shaped execution route.

Current reading:

```text
.hako model:
  mimalloc policy/model/source truth is present in pieces

generated C replacement front:
  benchmark-only hot-path bridge
  not yet full mimalloc execution
```

Therefore, the active owner is not broad `.hako` language optimization. The
active owner is replacement-front execution shape.

## Fidelity Guard

Required for a mimalloc-fidelity keeper:

```text
mimalloc_fidelity_guard=1

per_thread_tls_heap_or_arena=1
per_sizeclass_active_page=1
page_local_free_list=1
page_local_local_free_list=1
cross_thread_remote_free_list=1
owner_thread_fast_free=1
remote_free_atomic_push=1
owner_side_remote_drain=1
global_lock_hot_path_count=0
global_lock_refill_or_reclaim_count=allowed
```

Forbidden as final keeper shape:

```text
global lock on malloc/free hot path
one global per-bin free stack as final route
range_scan pointer ownership on hot free path
product claim before remote/abandoned counters
Type ABI lookup on replacement-front hot path
Provider ABI dispatch on replacement-front hot path
```

The current locked global page-bins/HotCore route is useful evidence, but it is
not a mimalloc-fidelity keeper because its malloc/free/realloc hot paths enter a
single global critical section.

## Step Order

### Step 0: Claim Boundary

Keep all product and full-algorithm claims closed while the bridge is
benchmark-only:

```text
replacement_front_is_full_hako_algorithm=0
hako_mimalloc_algorithm_claim=0
replacement_front_product_activation_ready=0
replacement_front_product_pages_consumer_enabled=0
benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
```

### Step 1: TLS Page Arena

First implementation owner:

```text
ReplacementFrontTlsPageArenaPlanV0
BenchmarkPageBinsHotCoreTlsRouteV0
```

Responsibilities:

```text
thread-local arena / heap substrate
per-sizeclass active page
same-thread alloc/free through page-local lists
cold refill under global/bin lock
global lock absent from same-thread hot malloc/free
remote-free seam present but may be report-only or disabled in first slice
```

Hot malloc target:

```text
malloc(size):
  class = size_to_class(size)
  arena = tls_arena()
  page = arena.active_page[class]
  if page.free has block:
      return pop(page.free)
  if page.local_free has block:
      migrate local_free to free
      return pop(page.free)
  return slow_refill_tls_page(class)
```

Hot free target:

```text
free(ptr):
  page = page_from_ptr(ptr)
  if page.owner_thread_id == current_thread_id:
      push page.local_free
      return
  remote_push(page, ptr)
```

### Step 1.5: Ptr-To-Page Fast Bridge

Do not open full product pages just to remove `range_scan`. Add a narrow
benchmark hot-free bridge first:

```text
BenchmarkPageFromPtrBridgeV0
replacement_front_page_from_ptr_route=page_base_mask|side_table_direct|backptr_header
replacement_front_page_from_ptr_range_scan_count=0
replacement_front_product_pages_consumer_enabled=0
```

This bridge is not product ownership, not a Type ABI lookup, and not Provider
ABI dispatch. Its job is only to remove generated range scans from the hot
free/usable-size/realloc ownership path.

### Step 2: Remote-Free Queue

Add cross-thread free as a mimalloc-shaped seam:

```text
RemoteFreeQueuePlanV0
remote_push(page, ptr) = atomic push to page remote head
owner drain = atomic exchange remote head and move to local/page list
```

Mixed-ws may legitimately report near-zero remote frees. A separate
remote-heavy benchmark is required before product activation.

### Step 3: PageMap / Product-Pages Bridge

Only after TLS page arena and hot ptr-to-page bridge are visible:

```text
replacement_front_page_map_route=indexed_page_table
replacement_front_owner_lookup_route=page_map_bridge
usable_size/realloc/free correctness consumes page ownership
replacement_front_product_pages_consumer_enabled may remain 0
```

### Step 4: Product Pages / Segment Backing / Arena Reclaim

Connect the product allocator substrate:

```text
segment backing
arena page supply
page retire
abandoned owner reclaim
large allocation route
metadata ownership
```

### Step 5: Product Activation Preflight

Only after fidelity evidence exists:

```text
provider activation
production replacement
hook install
global allocator claim
winner claim
```

## Acceptance Fields

Route identity:

```text
replacement_front_thread_local_page_bins_mode=1
replacement_front_thread_local_hotcore_route=benchmark_page_bins_hotcore_tls
replacement_front_page_bins_route=benchmark_page_bins_hotcore_page_model
replacement_front_page_bins_lookup_route=page_from_ptr_bridge
replacement_front_page_from_ptr_route=page_base_mask|side_table_direct|backptr_header
replacement_front_remote_free_route=disabled|atomic_page_remote_head|heap_delayed
replacement_front_product_pages_consumer_enabled=0
replacement_front_product_activation_ready=0
replacement_front_is_full_hako_algorithm=0
hako_mimalloc_algorithm_claim=0
```

Hot path counts:

```text
replacement_front_malloc_tls_fast_count
replacement_front_malloc_tls_refill_slow_count
replacement_front_malloc_global_lock_count
replacement_front_same_thread_alloc_local_count
replacement_front_same_thread_free_local_count
replacement_front_cross_thread_free_remote_push_count
replacement_front_remote_free_drain_count
replacement_front_remote_free_cas_retry_count
replacement_front_global_lock_hot_path_count=0
replacement_front_global_lock_refill_count
replacement_front_global_lock_reclaim_count
```

Owner/page lookup:

```text
replacement_front_owner_thread_id_lookup_count
replacement_front_owner_thread_id_same_count
replacement_front_owner_thread_id_remote_count
replacement_front_page_from_ptr_count
replacement_front_page_from_ptr_miss_count
replacement_front_page_from_ptr_invalid_count
replacement_front_page_from_ptr_range_scan_count=0
```

Abandoned/thread lifecycle:

```text
replacement_front_abandoned_owner_count
replacement_front_abandoned_remote_free_count
replacement_front_abandoned_reclaim_attempt_count
replacement_front_abandoned_reclaim_success_count
replacement_front_thread_exit_arena_flush_count
replacement_front_tls_arena_count
replacement_front_tls_arena_peak_count
```

Workload interpretation:

```text
benchmark_thread_origin=c_pthread
benchmark_front_class=replacement_front_c_shim
hako_source_thread_support_claim=0
replacement_front_same_thread_free_rate
replacement_front_remote_free_rate
replacement_front_tls_fast_alloc_rate
replacement_front_refill_per_1m_ops
replacement_front_global_lock_per_1m_ops
```

## Keeper Rule

Keeper wording must include both route and fidelity:

```text
accepted_keeper=
  replacement_front_thread_local_page_bins_hotcore_tls

accepted_reason=
  mimalloc_fidelity_guard_passed
  and measurement_quality_ok
  and product_activation_closed
```

Rejected wording:

```text
accepted_reason=fastest
accepted_reason=beats_mimalloc_without_fidelity_fields
accepted_reason=global_locked_route_is_good_enough
```
