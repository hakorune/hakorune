# lang/src/hako_alloc/memory — Hako Alloc Memory Policy Plane

Scope
- Policy-plane helpers for the `hako_alloc` layer live here.
- This subdir hosts the first moved helpers from the historical `runtime/memory/` path.
- Future allocator policy helpers should follow the same root.

Current modules
- `abandoned_reclaim_inventory_box.hako`
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
- `page_lifecycle_invariant_box.hako`
- `page_source_policy_box.hako`
- `purge_bounded_decommit_box.hako`
- `purge_bounded_scheduler_box.hako`
- `purge_dry_run_box.hako`
- `purge_execution_box.hako`
- `purge_heap_decommit_box.hako`
- `purge_page_source_decommit_adapter_box.hako`
- `purge_candidate_policy_box.hako`
- `purge_policy_box.hako`
- `remote_free_page_integration_box.hako`
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
- `usize_field_probe_box.hako` is a probe-only owner for exact `usize` stored
  field behavior. New production migrations still require a named field-group
  row and must not expand just because the probe is green.
- `alloc_fast_path_heap_box.hako` is the M167 orchestration owner. It may call
  `HakoAllocPageQueue.selectPage()` and `HakoAllocPageModel.acquire()`, but it
  must not source OS pages, collect local-free blocks, or implement remote-free
  policy.
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
