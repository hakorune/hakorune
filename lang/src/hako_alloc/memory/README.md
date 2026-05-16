# lang/src/hako_alloc/memory — Hako Alloc Memory Policy Plane

Scope
- Policy-plane helpers for the `hako_alloc` layer live here.
- This subdir hosts the first moved helpers from the historical `runtime/memory/` path.
- Future allocator policy helpers should follow the same root.

Current modules
- `abandoned_reclaim_inventory_box.hako`
- `options_inventory_box.hako`
- `thread_heap_owner_inventory_box.hako`
- `worker_identity_box.hako`
- `worker_tls_cache_box.hako`
- `alignment_policy_box.hako`
- `aligned_small_meta_store_box.hako`
- `allocator_metadata_records.hako`
- `allocator_facade_box.hako`
- `alloc_fast_path_heap_box.hako`
- `arc_box.hako`
- `layout_box.hako`
- `osvm_backed_fast_path_heap_box.hako`
- `page_box.hako`
- `page_heap_box.hako`
- `page_map_box.hako`
- `page_map_aligned_small_path_box.hako`
- `page_map_release_box.hako`
- `page_map_release_invariant_box.hako`
- `page_map_realloc_alloc_copy_release_box.hako`
- `page_map_realloc_failure_contract_box.hako`
- `page_map_realloc_same_class_box.hako`
- `heap_reuse_priority_box.hako`
- `lifecycle_stats_observer_box.hako`
- `page_queue_box.hako`
- `object_lifecycle_page_queue_box.hako`
- `object_lifecycle_facade_box.hako`
- `object_lifecycle_facade_reason_box.hako`
- `object_lifecycle_facade_result_box.hako`
- `object_lifecycle_facade_stats_box.hako`
- `object_lifecycle_facade_purge_policy_box.hako`
- `object_lifecycle_facade_page_source_box.hako`
- `object_lifecycle_facade_page_source_alloc_miss_box.hako`
- `object_lifecycle_facade_huge_failfast_box.hako`
- `object_lifecycle_facade_huge_page_model_box.hako`
- `object_lifecycle_facade_huge_page_source_box.hako`
- `object_lifecycle_facade_huge_decommit_box.hako`
- `object_lifecycle_facade_huge_decommit_failfast_box.hako`
- `object_lifecycle_facade_huge_unreserve_box.hako`
- `object_lifecycle_facade_huge_unreserve_failfast_box.hako`
- `object_lifecycle_facade_huge_backing_set_box.hako`
- `object_lifecycle_facade_huge_release_box.hako`
- `object_lifecycle_facade_huge_release_failfast_box.hako`
- `object_lifecycle_facade_huge_unregister_box.hako`
- `object_lifecycle_facade_huge_unregister_failfast_box.hako`
- `page_lifecycle_invariant_box.hako`
- `page_queue_lifecycle_box.hako`
- `page_source_policy_box.hako`
- `purge_bounded_decommit_box.hako`
- `purge_bounded_scheduler_box.hako`
- `purge_dry_run_box.hako`
- `purge_execution_box.hako`
- `purge_heap_decommit_box.hako`
- `purge_page_source_decommit_adapter_box.hako`
- `purge_page_source_unreserve_adapter_box.hako`
- `purge_candidate_policy_box.hako`
- `purge_policy_box.hako`
- `remote_free_page_integration_box.hako`
- `remote_free_abandoned_owner_policy_box.hako`
- `refcell_box.hako`
- `remote_free_policy_box.hako`
- `size_class_box.hako`
- `stats_box.hako`
- `usize_field_probe_box.hako`

Syntax/style contract
- New allocator state boxes should use Unified Members stored fields:
  `field`, `field: Type`, or `field: Type = expr`.
- Use stored field initializers for fixed defaults and owner construction.
  Initializers are evaluated per construction, so `new ArrayBox()` defaults are
  not shared between instances.
- Keep numeric allocator state on `i64` by default. Exact `usize` production
  fields are allowed only for field groups listed in `NUMERIC_FIELDS.md`.
  Current production `usize` scope is limited to facade-local monotonic stats.
- Numeric stored field migration is gated by
  [`NUMERIC_FIELDS.md`](./NUMERIC_FIELDS.md). Do not migrate a field to
  `usize` unless its category and sentinel behavior are recorded there first.
- `huge_page_meta_store_box.hako` exposes scalar append/read/release methods
  with explicit `: i64` return contracts. These contracts keep same-module
  user-box routes stable for huge-page model reads. Scalar column reads should
  return through typed locals; do not rely on dynamic `ArrayBox.get` return
  inference for this store API.
- `usize_field_probe_box.hako` is a probe-only owner for exact `usize` stored
  field behavior. New production migrations still require a named field-group
  row and must not expand just because the probe is green.
- `alloc_fast_path_heap_box.hako` is the M167 orchestration owner. It may call
  `HakoAllocPageQueue.selectPage()` and `HakoAllocPageModel.acquire()`, but it
  must not source OS pages, collect local-free blocks, or implement remote-free
  policy.
- `object_lifecycle_page_queue_box.hako` is the MIMAP-012 object-backed
  lifecycle queue owner. It may retain `HakoAllocPageModel` objects in an
  `ArrayBox`, scan owned pages with a queue-length selection loop, call page
  lifecycle observers/methods, and return the selected page object directly
  from the queue owner. It must not source OS pages, own segment/TLS/atomic/
  remote-free policy, activate providers/hooks, or add backend shortcuts.
- `object_lifecycle_facade_box.hako` owns the MIMAP-013 thin facade object
  lifecycle queue seam plus the MIMAP-014A/MIMAP-014B/MIMAP-014C small
  allocation fast-path, the MIMAP-015A/MIMAP-015B release route, and the
  MIMAP-016A/MIMAP-016B alignment request metadata / aligned small allocation
  facade seam, and the MIMAP-017A/MIMAP-017B realloc shrink/grow observer
  routes. It may
  store one `HakoAllocObjectLifecyclePageQueue`, forward add/select object-page
  operations, prefer a selected reusable page, fall back to one selected active
  page, call `HakoAllocPageModel.acquire(size)`, release one known `(page id,
  block id)` through `HakoAllocPageModel.releaseLocal(block_id)`, find that page
  only through the facade-local `objectLifecycleKnownPageIndexById(page_id)`
  queue-length scan of already-owned queue slots, and expose read-only scalar
  observer data
  including miss/release reason and facade-local allocation counters. It may surface
  double-release and stale-page rejection as scalar fail-fast reasons without
  adding page-map lookup or arbitrary pointer resolution. It may record one
  alignment request, normalize it through `HakoAllocAlignmentPolicy`, expose
  requested/normalized/reason/supported scalar metadata, and route supported
  aligned small allocations through the existing small allocation path. It may
  fail fast before allocation for unsupported alignment. It may validate one
  known live page/block pair for same-page realloc shrink/no-move observation,
  and it may allocate a replacement block before releasing the old known block
  for grow/move observation. It must not use that facade seam to activate byte
  copy, native aligned pointer placement, OSVM/page-source execution, provider
  hooks, remote-free execution, host allocator replacement, arbitrary page-map
  lookup, padded pointer arithmetic, unregister/register behavior, or backend
  shortcuts.
- `object_lifecycle_facade_reason_box.hako` is the MIMAP-FACADE-CLEAN-001
  Reason-code SSOT for `object_lifecycle_facade_box.hako` scalar observers. It
  may name stable integer result reasons for allocation, release, alignment, and
  realloc result capsules, but it must not scan pages, allocate, release,
  normalize alignment, mutate allocator state, read page-map ownership, or add
  fallback behavior. The current reason families are:

  | Family | Reason method | Code | Meaning |
  | --- | --- | ---: | --- |
  | common | `ok()` | 0 | Last operation succeeded or reset state is clear. |
  | small allocation | `small_no_page()` | 1 | No selected page or selected page object is unavailable. |
  | small allocation | `small_bad_selected_kind()` | 2 | Queue selected neither reusable nor active page kind. |
  | small allocation | `small_reuse_failed()` | 3 | Reusable page reactivation rejected. |
  | small allocation | `small_acquire_failed()` | 4 | Page-local acquire returned no block. |
  | small allocation | `small_alignment_unsupported()` | 5 | Alignment request failed before normal small allocation. |
  | small allocation | `small_huge_request()` | 6 | Facade page-source route rejected a huge request before small fallback. |
  | release | `release_no_page()` | 1 | Page id is invalid or not in the facade-owned known-page scan. |
  | release | `release_bad_block()` | 2 | Block id is invalid. |
  | release | `release_page_reject()` | 3 | Known page rejected the local release. |
  | release | `release_decommit_reject()` | 4 | Page-source decommit failed after huge unregister. |
  | alignment | `alignment_unsupported()` | 1 | Alignment policy rejected the request. |
  | realloc | `realloc_no_page()` | 1 | Page id is invalid or not in the facade-owned known-page scan. |
  | realloc | `realloc_bad_block()` | 2 | Block id is invalid. |
  | realloc | `realloc_bad_size()` | 3 | Requested size is invalid. |
  | realloc | `realloc_direction_unsupported()` | 4 | Requested size does not match the shrink/grow route direction. |
  | realloc | `realloc_stale_block()` | 5 | Old block is outside reserved range or not live. |
  | realloc | `realloc_alloc_failed()` | 6 | Replacement allocation failed. |
  | realloc | `realloc_release_failed()` | 7 | Old known block release failed after replacement allocation. |
- `object_lifecycle_facade_result_box.hako` is the
  MIMAP-FACADE-CLEAN-001 result capsule owner for facade scalar observer state.
  It owns the allocation, release, alignment, and realloc last-result fields and
  counters that used to live directly on `object_lifecycle_facade_box.hako`.
  The facade remains the orchestration owner and public observer API owner; the
  result boxes must not select pages, call page lifecycle methods, allocate,
  release, normalize alignment, read page maps, or add fallback behavior.
- `object_lifecycle_facade_stats_box.hako` owns the MIMAP-018A read-only stats
  snapshot for object-lifecycle facade observers. It may construct snapshots
  from already-recorded allocation/release result counters, but it must not
  trigger allocation, release, selection, page-map lookup, provider hooks,
  backend routes, or purge/decommit policy.
- `object_lifecycle_facade_purge_policy_box.hako` owns the MIMAP-019A
  read-only facade purge/reclaim/decommit policy route. It may adapt one
  facade stats snapshot and one scalar lifecycle view of a facade-known
  `HakoAllocPageModel` into the existing M211 purge candidate policy inventory
  and M213 abandoned reclaim inventory. It may expose a combined scalar
  decision and route counters. It must not execute decommit, recommit, reclaim, page-source calls, OSVM,
  provider hooks, remote-free behavior, page-map lookup, or backend shortcuts.
- `object_lifecycle_facade_page_source_box.hako` owns the MIMAP-021B
  facade page-source fresh-page attach seam. It may reserve/commit one backing
  range through `HakoAllocPageSourcePolicy`, construct one `HakoAllocPageModel`,
  and attach it through `HakoAllocObjectLifecycleFacade.objectLifecycleAddPage`
  with scalar proof counters. It must not allocate-on-miss, release, realloc,
  align, purge, reclaim, decommit, recommit, use page-map lookup, unreserve,
  release OSVM pages, call provider hooks, replace allocators, or add backend
  shortcuts.
- `object_lifecycle_facade_page_source_alloc_miss_box.hako` owns the MIMAP-021C
  facade page-source allocation-miss fallback. It may attempt one facade small
  allocation, check that the miss reason is `small_no_page()`, source exactly
  one fresh page through `HakoAllocObjectLifecycleFacadePageSourceAttach`, and
  retry the small allocation once with scalar proof counters. It must not call
  page-source/OSVM APIs directly, loop over multiple fresh pages, release,
  realloc, align, purge, reclaim, decommit, recommit, use page-map lookup,
  unreserve, release OSVM pages, call provider hooks, replace allocators, use
  TLS/atomics/remote-free, or add backend shortcuts.
- `object_lifecycle_facade_huge_failfast_box.hako` owns the MIMAP-022B facade
  huge-request fail-fast route. It may classify request size through
  `SizeClassBox`, reject huge requests before invoking the MIMAP-021C
  allocation-miss fallback, forward non-huge requests to that fallback, and
  expose scalar report fields/counters for the route decision. It must not own a
  huge page model, use page-map lookup, call page-source/OSVM APIs directly,
  alter release/realloc/alignment behavior, execute purge/reclaim/decommit/
  recommit, use remote-free/TLS/atomics, activate provider hooks, replace the
  host allocator, or add backend shortcuts.
- `object_lifecycle_facade_huge_page_model_box.hako` owns the MIMAP-023A facade
  huge-page model route. It may classify request size through the existing
  MIMAP-022B threshold, route huge requests into the existing M180
  `HakoAllocHugePageModel`, forward non-huge requests through the MIMAP-022B /
  MIMAP-021C small path, and expose scalar report fields/counters. It must not
  add a new huge model, huge release/unregister/unreserve/decommit behavior,
  page-map lookup route, release/realloc/alignment behavior, purge/reclaim,
  remote-free/TLS/atomics, provider hooks, host allocator replacement, or
  backend shortcuts.
- `object_lifecycle_facade_huge_page_source_box.hako` owns the MIMAP-028A
  facade huge page-source backing route. It may reserve/commit one scalar
  backing range through `HakoAllocPageSourcePolicy`, then delegate the huge
  allocation/register step to the existing MIMAP-023A facade huge-page model
  route and expose scalar backing / huge metadata fields. It must not release or
  unregister the huge handle, decommit/unreserve/recommit, add small
  release/free, realloc, alignment, purge/reclaim, remote-free/TLS/atomics,
  provider hooks, host allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_decommit_box.hako` owns the MIMAP-029A facade
  huge decommit-after-unregister success route. It may allocate one
  page-source-backed huge handle through MIMAP-028A, bind M181
  `HakoAllocHugeReleaseSeam` to that same route's huge model, unregister that
  same live pointer, and decommit exactly the MIMAP-028A backing range through
  the M196 `HakoAllocPageSourceDecommitAdapter`. It must not add duplicate
  decommit diagnostics, unreserve/recommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free/TLS/atomics, provider hooks, host
  allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_decommit_failfast_box.hako` owns the
  MIMAP-030A facade huge-decommit fail-fast diagnostics route. It may compose
  the MIMAP-029A success owner, record the successful backing range in
  allocator-side state, and reject duplicate/stale decommit attempts before a
  second `HakoAllocPageSourceDecommitAdapter` call. It must not call the page
  source or OSVM directly, add unreserve/recommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free/TLS/atomics, provider hooks, host
  allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_unreserve_box.hako` owns the MIMAP-034A facade
  huge unreserve-after-decommit success route. It may compose the MIMAP-029A
  huge decommit route with the MIMAP-033A page-source unreserve adapter and
  unreserve exactly the decommitted backing range. It must not add
  duplicate/stale unreserve diagnostics, call page-source/OSVM directly,
  recommit, purge/reclaim, remote-free/TLS/atomics, provider hooks, host
  allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_unreserve_failfast_box.hako` owns the
  MIMAP-035A facade huge-unreserve fail-fast diagnostics route. It may compose
  the MIMAP-034A success owner, record the successful backing range in
  allocator-side state, and reject duplicate/stale unreserve attempts before a
  second `HakoAllocPageSourceUnreserveAdapter` call. It must not call the page
  source or OSVM directly, add recommit, purge/reclaim, remote-free/TLS/atomics,
  provider hooks, host allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_backing_set_box.hako` owns the MIMAP-037A
  facade huge backing-set helper. It may store and query exact `base + bytes`
  pairs for diagnostic routes. It must not own lifecycle behavior, call
  page-source/OSVM APIs, add new fail-fast vocabulary, provider hooks, host
  allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_release_box.hako` owns the MIMAP-024A facade
  huge-release metadata route. It may allocate one huge request through the
  MIMAP-023A facade huge-page model route, retire that same live huge pointer
  through `HakoAllocHugePageModel.markReleased(ptr)`, forward non-huge requests
  through the existing small fallback, and expose scalar report fields for the
  selected pointer, page id, requested/committed sizes, live-count transition,
  and release counters. It must not adopt `HakoAllocHugeReleaseSeam`, use
  page-map lookup/unregister, release OS pages, add small release/free, realloc,
  alignment, purge/reclaim, remote-free/TLS/atomics, provider hooks, host
  allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_release_failfast_box.hako` owns the MIMAP-025A
  facade huge-release fail-fast diagnostics route. It may compose the
  MIMAP-024A route, reject a second release of the same huge pointer, reject one
  stale/unknown huge pointer through `HakoAllocHugePageModel.markReleased(ptr)`,
  and expose scalar reject counters. It must not adopt `HakoAllocHugeReleaseSeam`,
  use page-map lookup/unregister, release OS pages, add small release/free,
  realloc, alignment, purge/reclaim, remote-free/TLS/atomics, provider hooks,
  host allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_unregister_box.hako` owns the MIMAP-026A
  facade huge-release page-map unregister route. It may allocate one huge
  request through the MIMAP-023A facade huge-page model route, release that same
  live huge pointer through the existing M181 `HakoAllocHugeReleaseSeam`, and
  expose scalar counters for huge-model live-state transition, page-map
  lookup/unregister transition, and M181 seam counters. It must not release OS
  pages, unreserve/decommit/recommit, add small release/free, realloc,
  alignment, purge/reclaim, remote-free/TLS/atomics, provider hooks, host
  allocator replacement, or backend shortcuts.
- `object_lifecycle_facade_huge_unregister_failfast_box.hako` owns the
  MIMAP-027A facade huge-unregister fail-fast diagnostics route. It may compose
  the MIMAP-026A route, reject a second release of the same unregistered huge
  pointer, reject one stale/unknown pointer through the existing M181
  `HakoAllocHugeReleaseSeam`, and expose scalar lookup-miss / reject counters.
  It must not call page-map lookup/unregister or `HakoAllocHugePageModel`
  release directly, release OS pages, unreserve/decommit/recommit, add small
  release/free, realloc, alignment, purge/reclaim, remote-free/TLS/atomics,
  provider hooks, host allocator replacement, or backend shortcuts.
- `worker_identity_box.hako` owns the MIMAP-WORKER-001 allocator-facing worker
  identity observer. It may call `WorkerCoreBox.current_id_i64()`, store
  scalar `last_worker_id` / `call_count` proof state, and keep the single-worker
  lane deterministic. It must not open source-level worker identity,
  `worker_local` syntax, TLS/cache slots, atomics, remote-free, page ownership
  transfer, provider hooks, allocator replacement, task scheduling, or backend
  shortcuts.
- `worker_tls_cache_box.hako` owns the MIMAP-TLS-001 allocator-facing worker
  TLS cache-slot observer. It may compose `HakoAllocWorkerIdentity` with
  `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64`, store scalar slot/value/
  observed-worker/get-count/set-count proof state, and keep the single-worker
  lane deterministic. It must not open source-level worker-local syntax,
  generic TLS cells, atomics, remote-free, page ownership transfer, provider
  hooks, allocator replacement, task scheduling, or backend shortcuts.
- `osvm_backed_fast_path_heap_box.hako` is the M168 composition owner. It may
  reserve/commit/decommit through `HakoAllocPageSourcePolicy`, then reuse the
  same page queue and page-local free-list owners. It must not add OSVM metal,
  local-free retire, remote-free, page-map, provider, hook, or replacement
  behavior.
- `page_box.hako` owns M169 local-free collection and empty-page retire state.
  The row is page-local: remote-free atomics, abandoned reclaim, page-map lookup,
  OSVM release, provider hooks, and allocator replacement remain out of scope.
- `remote_free_page_integration_box.hako` owns M170 page-owned remote-free
  inbox composition. It may call `HakoAllocRemoteFreePolicy.pushRetry(...)` and
  `HakoAllocPageModel.releaseLocal(...)`, but it must not resolve arbitrary
  pointers to pages or add new pointer atomic vocabulary.
- `remote_free_abandoned_owner_policy_box.hako` owns MIMAP-REMOTE-001
  allocator remote-free / abandoned-owner policy composition. It may compose
  `HakoAllocWorkerTlsCache`, `HakoAllocRemoteFreePolicy`,
  `HakoAllocThreadHeapOwnerInventory`, and
  `HakoAllocAbandonedReclaimInventory` into scalar same-owner,
  remote-owner-publish, abandoned-owner-candidate, and reject decisions. It
  must not open source-level concurrency syntax, mutate page ownership, drain
  arbitrary remote-free queues, execute reclaim, call page-source APIs, use
  page-map lookup, add route rows, activate providers/hooks, or replace the
  host allocator.
- `page_map_box.hako` owns M171 pointer-to-page ownership lookup. It may record
  and resolve caller-visible pointer identity to page/block ids, but it must not
  perform page release, realloc, pointer arithmetic, or native metal work.
- `page_map_release_box.hako` owns M172 page-map-backed release orchestration.
  It may call `HakoAllocPageMap.lookup(...)`,
  `HakoAllocPageModel.releaseLocal(...)`, and
  `HakoAllocPageMap.unregister(...)`, but it must not own registration, realloc,
  aligned/huge allocation, OSVM release, provider hooks, or allocator
  replacement.
- `page_map_release_invariant_box.hako` owns M173 pre-realloc release
  observation. It may call `HakoAllocPageMap.lookup(...)` and
  `HakoAllocPageMapReleaseSeam.releasePtr(...)` to freeze handle lifetime and
  release/unregister timing, but it must not own registration, page-local
  mutation, unregister execution, realloc, or byte copy.
- `page_map_realloc_alloc_copy_release_box.hako` owns M175 grow fallback. It
  may allocate a replacement block from current pages, call
  `HakoAllocPageMap.register(...)`, and release the old ptr through
  `HakoAllocPageMapReleaseSeam.releasePtr(...)` only after allocation succeeds,
  but it must not own byte copy, raw `unregister(...)`, same-class/no-move
  routing, or aligned/huge behavior.
- `page_map_realloc_failure_contract_box.hako` owns M176 realloc diagnostics. It
  may classify zero/oversized rejects and delegate to the existing M174/M175
  owners, but it must not own raw registration, release, unregister,
  page-local mutation, byte copy, or aligned/huge behavior.
- `page_map_realloc_same_class_box.hako` owns M174 same-class/no-move realloc.
  It may call `HakoAllocPageMap.lookup(...)` and inspect the current page block
  to decide whether the same live ptr can be reused, but it must not own
  release, unregister, alloc-copy-release fallback, or byte copy.
- `alignment_policy_box.hako` owns M177 alignment policy. It may normalize
  alignment, check power-of-two validity, and compute padded-size policy, but
  it must not start aligned allocation execution or huge-page routing.
- `page_map_aligned_small_path_box.hako` owns M178 aligned small-path execution.
  It may attach alignment metadata to normal page-map-backed small allocations,
  but it must not start huge-page routing or native alignment claims.
- `aligned_small_meta_store_box.hako` owns C205c aligned-small metadata storage
  behind a record-shaped append/read seam. It may construct
  `HakoAllocAlignedSmallMeta` and read its fields locally, but it must not
  enable `ArrayStorage::InlineRecord`, backend lowering, huge metadata
  migration, provider hooks, or native allocator behavior. C206a adds a single
  `findIndex(ptr)` lookup seam for its read APIs; this is cleanup only. C210
  adds compiler-side packed-store pilot metadata for this shape, but this
  source file must still not name compiler internals.
- `huge_threshold_router_box.hako` owns M179 huge threshold/routing. It may
  classify padded requests and fail fast for huge-unsupported requests, but it
  must not implement a huge page model or OS release.
- `huge_page_model_box.hako` owns M180 huge page modeling. It may register huge
  handles and track requested/committed/live metadata, but it must not implement
  huge release, unregister, or OS release.
- `huge_page_meta_store_box.hako` owns C205d huge-page metadata storage behind a
  record-shaped append/read seam. It may construct `HakoAllocHugePageMeta` and
  read its fields locally, but it must not enable `ArrayStorage::InlineRecord`,
  backend lowering, small-page state migration, provider hooks, or native
  allocator behavior. C211 adds compiler-side packed-store pilot metadata for
  this shape, but this source file must still not name compiler internals.
- `huge_release_seam_box.hako` owns M181 huge release composition. It may mark
  huge model state released and unregister page-map ownership, but it must not
  call small page `releaseLocal(...)` or OS release.
- `secure_free_list_diagnostics_box.hako` owns M183 secure-list diagnostics. It
  may observe page-local free/local_free shape, but it must not implement
  encode/decode, cookies, or hardening policy.
- `secure_free_list_policy_box.hako` owns M184 secure-list encoded-next policy.
  It may encode/decode next indices and validate decoded capacity, but it must
  not source entropy, mutate page state, or claim hardening policy.
- `stats_box.hako` owns M191 allocator stats snapshots. It may construct a
  read-only `HakoAllocStatsSnapshot` from existing facade/page observers, but
  it must not mutate allocator options, add environment toggles, source
  purge/decommit, or change allocation behavior.
- `purge_policy_box.hako` owns M192 purge/decommit policy inventory. It may
  classify empty retired pages as future decommit candidates and return a
  read-only decision object, but it must not call `HakoAllocPageSourcePolicy`,
  mutate heap/page state, unreserve pages, or perform OS release behavior.
- `purge_dry_run_box.hako` owns M193 purge/decommit dry-run observation. It may
  read existing OSVM-backed heap page/backing state and delegate to
  `HakoAllocPurgePolicyInventory`, but it must not call page-source APIs,
  mutate heap/page state, decommit, unreserve, or release OSVM pages.
- `purge_execution_box.hako` owns M194 purge/decommit execution fail-fast
  entry. It may accept a purge decision and return a structured blocked report,
  but it must not call page-source APIs, mutate heap/page state, decommit,
  unreserve, or release OSVM pages.
- `purge_bounded_decommit_box.hako` owns M195 bounded decommit execution
  policy. It may call a caller-provided `decommitPage(base, bytes)` executor at
  most once after validating an eligible decision and byte bound, but it must
  not directly call OSVM/page-source APIs, mutate heap/page state, unreserve, or
  release OSVM pages.
- `purge_page_source_decommit_adapter_box.hako` owns M196 page-source decommit
  adapter. It may implement `decommitPage(base, bytes)` by delegating to
  `HakoAllocPageSourcePolicy.decommitPage`, but it must not reserve, commit,
  unreserve, release OSVM pages, or mutate heap/page state.
- `purge_page_source_unreserve_adapter_box.hako` owns MIMAP-033A page-source
  unreserve adapter. It may implement `unreservePage(base, bytes)` by
  delegating to `HakoAllocPageSourcePolicy.unreservePage`, but it must not
  reserve, commit, decommit, recommit, call facade lifecycle owners, mutate
  heap/page state, activate provider hooks, or replace allocators.
- `purge_heap_decommit_box.hako` owns M197 purge decommit heap integration. It
  may compose dry-run observation, bounded decommit policy, and the page-source
  decommit adapter for an existing heap page/backing, but it must not mutate
  heap/page state, unreserve, release OSVM pages, or replace allocators.
- `purge_decommit_state_marker_box.hako` owns M198 purge decommit state marker.
  It may record page ids from successful decommit reports and reject duplicate
  or widened release reports, but it must not call page-source APIs, mutate
  heap/page state, unreserve, release OSVM pages, or replace allocators.
- `purge_decommit_state_marker_box.hako` also owns M204 recommit marker
  transition. It records recommitted page ids as a separate generation lane and
  treats a page as marked only while marked generations outnumber recommitted
  generations. It must not physically remove marker entries or mutate heap/page
  state.
- `purge_state_aware_decommit_box.hako` owns M199 purge state-aware duplicate
  guard. It may consult the M198 marker before delegating to M197 heap decommit
  integration, but it must not call page-source APIs directly, mutate heap/page
  state, unreserve, release OSVM pages, or replace allocators.
- `purge_decommitted_page_reuse_precondition_box.hako` owns M200 decommitted
  page reuse precondition. It may classify committed/unmarked pages as reusable
  and decommitted pages as requiring future recommit, but it must not call
  page-source APIs, mutate heap/page state, recommit, unreserve, release OSVM
  pages, or replace allocators.
- `purge_recommit_failfast_box.hako` owns M201 recommit fail-fast entry. It may
  read the M200 precondition and return a structured blocked/no-op report, but
  it must not call page-source APIs, mutate heap/page state, clear the decommit
  marker, recommit, unreserve, release OSVM pages, or replace allocators.
- `purge_bounded_recommit_box.hako` owns M202 bounded recommit policy. It may
  execute at most one caller-provided `commitPage(base, bytes)` source call
  after M200 reports `requires_recommit`, but must not call page-source APIs
  directly, clear markers, or mutate heap/page state.
- `purge_page_source_recommit_adapter_box.hako` owns M203 page-source recommit
  adapter. It may delegate `commitPage(base, bytes)` to
  `HakoAllocPageSourcePolicy.commitPage` only, but must not expose reserve,
  decommit, unreserve, OS release, marker transition, heap/page mutation, or
  allocator replacement behavior.
- `purge_recommit_heap_integration_box.hako` owns M205 recommit heap
  integration. It may compose M200/M202/M203/M204 and call
  `HakoAllocPageModel.reactivate()` after successful recommit, but must not
  source pages, mutate heap/backing arrays, unreserve, release OSVM pages, or
  replace allocators.
- `page_lifecycle_invariant_box.hako` owns M207 page lifecycle invariant
  freeze. It may read heap page/backing state and marker generation counts to
  classify active/retired/decommitted/recommitted-active states, but it must not
  allocate, release, decommit, recommit, reactivate, source pages, unreserve,
  release OSVM pages, or replace allocators.
- `heap_reuse_priority_box.hako` owns M208 heap reuse priority policy. It may
  read `HakoAllocPageQueue` page order and M207 lifecycle observer facts to rank
  active, recommitted-active, retired-reactivate, and fresh fallback routes, but
  it must not acquire/release/reactivate pages, decommit/recommit, source pages,
  unreserve, release OSVM pages, or replace allocators.
- `lifecycle_stats_observer_box.hako` owns M209 lifecycle stats observer surface.
  It may snapshot existing M207 lifecycle observer counters and M208 reuse
  priority policy counters, but it must not trigger observation/selection,
  mutate heap/page/marker/page-source state, add mutable options, or replace
  allocators.
- `purge_candidate_policy_box.hako` owns M211 purge candidate policy inventory.
  It may classify already-built M207 lifecycle reports as future purge
  candidates, but it must not observe heap pages, scan queues, schedule purge,
  decommit, recommit, call page-source APIs, mutate heap/page/marker state,
  unreserve, release OSVM pages, or replace allocators.
- `purge_bounded_scheduler_box.hako` owns M212 bounded purge/decommit scheduler
  small path. It may scan at most a caller-provided page count, observe M207
  lifecycle facts, classify them through M211, and call the M199 state-aware
  guard for at most one eligible page, but it must not call M197/M195/M196 or
  page-source APIs directly, mutate heap/page/backing state, recommit,
  unreserve, release OSVM pages, or replace allocators.
- `abandoned_reclaim_inventory_box.hako` owns M213 abandoned/reclaim inventory.
- `options_inventory_box.hako` owns M214 allocator options/defaults inventory. It may classify static option/default facts and report inactive mutable options, env toggles, provider/hook/replacement, and reclaim execution, but it must not parse process configuration or change allocation behavior.
- `thread_heap_owner_inventory_box.hako` owns M215 thread heap owner-token inventory. It may classify scalar owner-token facts for future abandoned/reclaim rows, but it must not schedule threads, use atomics, drain remote frees, mutate ownership, call page-source APIs, unreserve, or release OSVM pages.
  It may classify scalar owner/page facts into read-only abandoned and reclaim
  candidate vocabulary, but it must not schedule threads, add atomics, execute
  reclaim, call page-source APIs, decommit, recommit, unreserve, release OSVM
  pages, or replace allocators.
- `allocator_metadata_records.hako` owns C205a allocator metadata record
  declarations. It may declare identity-free shapes for aligned-small and
  huge-page metadata. C205c consumes aligned-small metadata through a
  record-shaped store, and C205d consumes huge-page metadata the same way.
  `ArrayStorage::InlineRecord` compiler auto-use remains future work.
- D195 checkpoint: after M184, secure-list state remains split between
  observation (`secure_free_list_diagnostics_box.hako`) and encoded-next policy
  (`secure_free_list_policy_box.hako`). Page mutation stays with
  `page_box.hako` and release/realloc owners.
- Keep `birth(...)` for parameter-dependent initialization and ordering that
  cannot be expressed as a declaration-site default.
