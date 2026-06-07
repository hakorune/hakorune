# Hako Alloc Memory Owner Contracts: segment arena, worker/TLS, reclaim, and metadata contracts

Status: Active
Scope: segment arena, worker/TLS, reclaim, and metadata contracts.
Related:
- `OWNER_CONTRACTS.md`
- `README.md`

- `segment_arena_backing_readiness_inventory_box.hako` owns MIMAP-236A. It may
  observe lifecycle-keyed release apply/recycle continuation diagnostics and
  publish scalar arena backing readiness requirements. It must not allocate
  arena backing, use raw pointer residence, mutate a real segment-map, execute
  real segment allocation/free, execute atomic bitmap claims, call page-source
  or OSVM seams, schedule workers, activate provider hooks, replace the host
  allocator, or add backend shortcuts.
- `segment_arena_backing_readiness_diagnostic_box.hako` owns MIMAP-237A. It may
  observe MIMAP-236A readiness counters and publish scalar diagnostic summary
  facts for missing continuation, invalid shape, and blocked requirement
  categories. It must not classify readiness itself, allocate arena backing,
  use raw pointer residence, mutate a real segment-map, execute atomic bitmap
  claims, call page-source or OSVM seams, schedule workers, activate provider
  hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_requirement_matrix_box.hako` owns MIMAP-240A. It may
  consume arena readiness and diagnostics reports and publish scalar requirement
  matrix facts for arena id, segment id, slice geometry, page size, alignment,
  and blocked substrate categories. It must not allocate arena backing, use raw
  pointer residence, mutate a real segment-map, execute atomic bitmap claims,
  call page-source or OSVM seams, schedule workers, activate provider hooks,
  replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_requirement_matrix_diagnostic_box.hako` owns
  MIMAP-241A. It may observe MIMAP-240A requirement matrix counters and publish
  scalar diagnostic summary facts. It must not record requirement matrix rows,
  allocate arena backing, use raw pointer residence, mutate a real segment-map,
  execute atomic bitmap claims, call page-source or OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_arena_backing_no_escape_address_capability_box.hako` owns
  MIMAP-244A. It may observe an accepted requirement matrix report and publish
  scalar owner/lifetime/address-carrier facts plus escape blockers. It must not
  create pointer residence, perform pointer-derived lookup, allocate arena
  backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_no_escape_address_capability_diagnostic_box.hako` owns
  MIMAP-245A. It may observe MIMAP-244A no-escape address capability counters
  and publish scalar diagnostic summary facts. It must not record capability
  rows, create pointer residence, perform pointer-derived lookup, allocate
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_no_escape_address_residence_box.hako` owns
  MIMAP-248A. It may record accepted no-escape address capability reports as
  scalar/model residence rows. The address carrier remains non-dereferenceable.
  It must not create real pointer residence, perform pointer-derived lookup,
  allocate arena backing, mutate a real segment-map, execute atomic bitmap
  claims, call page-source or OSVM seams, schedule workers, activate provider
  hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_no_escape_address_residence_diagnostic_box.hako`
  owns MIMAP-249A. It may observe MIMAP-248A modeled no-escape address
  residence counters and publish scalar diagnostic summary facts. It must not
  record residence rows, create real pointer residence, perform pointer-derived
  lookup, allocate arena backing, mutate a real segment-map, execute atomic
  bitmap claims, call page-source or OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_residence_arena_binding_box.hako` owns
  MIMAP-252A. It may bind accepted modeled no-escape address residence reports
  to accepted scalar requirement matrix reports for the same segment and arena.
  It must not create real pointer residence, perform pointer-derived lookup,
  allocate arena backing, mutate a real segment-map, execute atomic bitmap
  claims, call page-source or OSVM seams, schedule workers, activate provider
  hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_residence_arena_binding_diagnostic_box.hako`
  owns MIMAP-253A. It may observe MIMAP-252A binding counters and publish
  scalar diagnostic summary facts. It must not record binding rows, create real
  pointer residence, perform pointer-derived lookup, allocate arena backing,
  mutate a real segment-map, execute atomic bitmap claims, call page-source or
  OSVM seams, schedule workers, activate provider hooks, replace the host
  allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_arena_slot_box.hako` owns MIMAP-256A. It may
  record scalar/model arena-slot facts from an accepted modeled residence
  arena-binding report. It must not create real pointer residence, perform
  pointer-derived lookup, allocate arena backing, mutate a real segment-map,
  execute atomic bitmap claims, call page-source or OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_arena_backing_modeled_arena_slot_diagnostic_box.hako` owns
  MIMAP-257A. It may observe MIMAP-256A arena-slot counters and publish scalar
  diagnostic summary facts. It must not record arena-slot rows, create real
  pointer residence, perform pointer-derived lookup, allocate arena backing,
  mutate a real segment-map, execute atomic bitmap claims, call page-source or
  OSVM seams, schedule workers, activate provider hooks, replace the host
  allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_source_bridge_box.hako` owns MIMAP-260A. It
  may record scalar/model backing source facts from an accepted modeled
  arena-slot report. It must not create real pointer residence, perform
  pointer-derived lookup, allocate arena backing, mutate a real segment-map,
  execute atomic bitmap claims, call page-source or OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_arena_backing_modeled_source_bridge_diagnostic_box.hako` owns
  MIMAP-261A. It may observe MIMAP-260A source bridge counters and publish
  scalar diagnostic summary facts. It must not record source bridge rows,
  create real pointer residence, perform pointer-derived lookup, allocate
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_source_accounting_box.hako` owns MIMAP-264A.
  It may record scalar/model source-backed arena accounting from accepted
  modeled source bridge reports. It must not create real pointer residence,
  perform pointer-derived lookup, allocate arena backing, mutate a real
  segment-map, execute atomic bitmap claims, call page-source or OSVM seams,
  schedule workers, activate provider hooks, replace the host allocator, or
  add backend shortcuts.
- `segment_arena_backing_modeled_source_accounting_diagnostic_box.hako` owns
  MIMAP-265A. It may observe MIMAP-264A source accounting counters and publish
  scalar diagnostic summary facts. It must not record source accounting rows,
  create real pointer residence, perform pointer-derived lookup, allocate
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_plan_box.hako` owns MIMAP-268A.
  It may record scalar/model allocation-plan facts from accepted source
  accounting reports. It must not create real pointer residence, perform
  pointer-derived lookup, allocate arena backing, mutate a real segment-map,
  execute atomic bitmap claims, call page-source or OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_arena_backing_modeled_allocation_plan_diagnostic_box.hako` owns
  MIMAP-269A. It may observe MIMAP-268A allocation-plan counters and publish
  scalar diagnostic summary facts. It must not record allocation-plan rows,
  create real pointer residence, perform pointer-derived lookup, allocate
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_apply_box.hako` owns MIMAP-272A.
  It may record scalar/model allocation apply facts from accepted modeled
  allocation-plan reports. It must not create real pointer residence, perform
  pointer-derived lookup, allocate arena backing, mutate a real segment-map,
  execute atomic bitmap claims, call page-source or OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_arena_backing_modeled_allocation_apply_diagnostic_box.hako` owns
  MIMAP-273A. It may observe MIMAP-272A allocation-apply counters and publish
  scalar diagnostic summary facts. It must not record allocation-apply rows,
  create real pointer residence, perform pointer-derived lookup, allocate
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_box.hako` owns MIMAP-276A.
  It may record scalar/model allocation ledger facts from accepted modeled
  allocation-apply reports. It must not create real pointer residence, perform
  pointer-derived lookup, allocate real arena backing, mutate a real
  segment-map, execute atomic bitmap claims, call page-source or OSVM seams,
  schedule workers, activate provider hooks, replace the host allocator, or add
  backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_diagnostic_box.hako` owns
  MIMAP-277A. It may observe MIMAP-276A allocation-ledger counters and publish
  scalar diagnostic summary facts. It must not record allocation-ledger rows,
  create real pointer residence, perform pointer-derived lookup, allocate real
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_release_candidate_box.hako`
  owns MIMAP-280A. It may record scalar/model release-candidate facts from
  accepted modeled allocation-ledger reports. It must not create real pointer
  residence, perform pointer-derived lookup, allocate or release real arena
  backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako`
  owns MIMAP-281A. It may observe MIMAP-280A release-candidate counters and
  publish scalar diagnostic summary facts. It must not record release-candidate
  rows, create real pointer residence, perform pointer-derived lookup, allocate
  or release real arena backing, mutate a real segment-map, execute atomic
  bitmap claims, call page-source or OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_release_intent_box.hako`
  owns MIMAP-284A. It may record scalar/model release-intent facts from
  accepted modeled allocation-ledger release-candidate reports. It must not
  create real pointer residence, perform pointer-derived lookup, allocate or
  release real arena backing, mutate a real segment-map, execute atomic bitmap
  claims, call page-source or OSVM seams, schedule workers, activate provider
  hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_release_intent_diagnostic_box.hako`
  owns MIMAP-285A. It may observe MIMAP-284A release-intent counters and
  publish scalar diagnostic summary facts. It must not record release-intent
  rows, create real pointer residence, perform pointer-derived lookup, allocate
  or release real arena backing, mutate a real segment-map, execute atomic
  bitmap claims, call page-source or OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_release_apply_box.hako`
  owns MIMAP-288A. It may record scalar/model release-apply facts from accepted
  modeled allocation-ledger release-intent reports. It must not create real
  pointer residence, perform pointer-derived lookup, allocate or release real
  arena backing, mutate a real segment-map, execute atomic bitmap claims, call
  page-source or OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_arena_backing_modeled_allocation_ledger_release_apply_diagnostic_box.hako`
  owns MIMAP-289A. It may observe MIMAP-288A release-apply counters and
  publish scalar diagnostic summary facts. It must not record release-apply
  rows, create real pointer residence, perform pointer-derived lookup, allocate
  or release real arena backing, mutate a real segment-map, execute atomic
  bitmap claims, call page-source or OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
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
  It may call `HakoAllocPageMapBridge.lookup(...)`,
  `HakoAllocPageModel.releaseLocal(...)`, and
  `HakoAllocPageMapBridge.unregister(...)`, but it must not own registration,
  realloc, aligned/huge allocation, OSVM release, provider hooks, or allocator
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
- `secure_entropy_inventory_box.hako` owns MIMAP-049A secure entropy inventory.
  It may classify deterministic proof-key facts and rejected runtime entropy
  requests, but it must not source entropy, call random/OS/provider helpers,
  mutate secure-list behavior, or claim cryptographic hardening.
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
- `osvm_fast_path_purge_route_box.hako` owns MIMAP-042A OSVM-backed fast-path
  bounded purge route. It may compose M168 allocation/release, M199 duplicate
  guard state, and M212 bounded scheduling, but it must not call page-source or
  OSVM APIs directly, unreserve, recommit, release OSVM pages, activate
  providers, install hooks, replace allocators, or add user-facing concurrency.
- `osvm_fast_path_reuse_route_box.hako` owns MIMAP-043A OSVM-backed fast-path
  recommit/reuse route. It may compose the MIMAP-042A route with M205 recommit
  heap integration and perform one post-recommit allocation through the same
  route, but it must not call page-source or OSVM APIs directly, unreserve,
  release OSVM pages, activate providers, install hooks, replace allocators,
  change scheduler/page-queue policy, or add user-facing concurrency.
- `osvm_fast_path_unreserve_route_box.hako` owns MIMAP-045A OSVM-backed
  fast-path unreserve route. It may compose the MIMAP-043A route with the
  MIMAP-033A page-source unreserve adapter and prove one
  allocate/release/purge/unreserve sequence, but it must not call page-source
  or OSVM APIs directly, add post-unreserve reuse, release OSVM pages outside
  the adapter seam, activate providers, install hooks, replace allocators,
  change scheduler/page-queue policy, or add user-facing concurrency.
- `osvm_fast_path_unreserve_failfast_box.hako` owns MIMAP-046A OSVM-backed
  fast-path unreserve diagnostics. It may reject duplicate, unknown, and
  not-decommitted fast-path unreserve requests before adapter execution, but it
  must not call page-source or OSVM APIs directly, add post-unreserve reuse,
  release OSVM pages outside the adapter seam, activate providers, install
  hooks, replace allocators, change scheduler/page-queue policy, or add
  user-facing concurrency.
- `abandoned_reclaim_inventory_box.hako` owns M213 abandoned/reclaim inventory.
- `reclaim_atomic_claim_contract_box.hako` owns MIMAP-054A reclaim atomic-claim
  contract. It may model scalar owner-token compare-and-claim success/failure
  and publish a hypothetical `owner_after` for a future execution row, but it
  must not call real atomic substrate routes, mutate production page ownership,
  drain remote frees, schedule threads, call page-source APIs, unreserve,
  release OSVM pages, activate providers, install hooks, or replace the
  process allocator.
- `reclaim_owner_transfer_execution_box.hako` owns MIMAP-055A first guarded
  owner-transfer execution route. It may compose the MIMAP-051A readiness
  contract and MIMAP-054A atomic-claim contract, then update only an
  executor-local modeled owner token for one ready page. It must not mutate the
  production page map, execute full reclaim, drain remote frees, schedule
  threads, call page-source or OSVM seams, activate providers, install hooks,
  replace the process allocator, or add backend shortcuts.
- `reclaim_remote_free_drain_contract_box.hako` owns MIMAP-056A reclaim
  remote-free drain contract inventory. It may classify scalar pending/head
  facts and report whether reclaim can proceed without drain work, but it must
  not drain remote-free queues, traverse remote-free pointer lists, schedule
  threads, call page-source/OSVM seams, mutate production page ownership,
  activate providers, install hooks, replace the process allocator, or add
  backend shortcuts.
- `reclaim_remote_free_drain_execution_box.hako` owns MIMAP-057A first modeled
  remote-free drain execution route. It may compose the MIMAP-056A contract and
  decrement one executor-local modeled pending count, but it must not traverse
  remote-free pointer lists, call `releaseLocal`, use real atomics, schedule
  threads, call page-source/OSVM seams, execute full reclaim, activate
  providers, install hooks, replace the process allocator, or add backend
  shortcuts.
- `reclaim_post_drain_owner_transfer_box.hako` owns MIMAP-058A post-drain
  owner-transfer integration. It may compose the MIMAP-057A modeled drain route
  and MIMAP-055A modeled owner-transfer route, and it may attempt transfer only
  when pending remote-free work is gone. It must not execute full reclaim,
  schedule threads, call page-source/OSVM seams, activate providers, install
  hooks, replace the process allocator, or add backend shortcuts.
- `reclaim_completion_marker_box.hako` owns MIMAP-060A scalar reclaim
  completion marker route. It may compose MIMAP-058A and set only an
  executor-local completion marker after integration success. It must not call
  page-source/OSVM seams, schedule threads, activate providers, install hooks,
  replace the process allocator, or add backend shortcuts.
- `reclaim_scheduler_request_marker_box.hako` owns MIMAP-064A scalar reclaim
  scheduler request marker contract. It may compose MIMAP-060A and set only an
  executor-local request marker when completion succeeds and scheduler request
  is enabled. It must not execute real scheduling, expose source-level
  concurrency semantics, call page-source/OSVM seams, activate providers,
  install hooks, replace the process allocator, or add backend shortcuts.
- `reclaim_scheduler_request_ledger_box.hako` owns MIMAP-068A scalar reclaim
  scheduler request ledger route. It may compose MIMAP-064A, record at most
  one pending modeled scheduler request, and report marker-blocked,
  scheduler-disabled, and already-pending suppressions. It must not execute
  real scheduling, spawn workers, expose source-level concurrency, call
  page-source/OSVM seams, activate providers, install hooks, replace the
  process allocator, or add backend shortcuts.
  `reclaim_scheduler_request_ledger_box.hako` also owns MIMAP-071A scalar
  ledger consume route. It may clear one pending modeled scheduler request when
  the requested page id matches the pending page and report no-pending /
  page-mismatch suppressions. It must not run a scheduler, spawn workers, add
  source-level concurrency, call page-source/OSVM seams, activate providers,
  install hooks, replace the process allocator, or add backend shortcuts.
- `reclaim_scheduler_request_ledger_roundtrip_box.hako` owns MIMAP-074A
  scalar scheduler request ledger roundtrip route. It may compose the
  scheduler request ledger, record one modeled request, and consume the same
  pending page id to prove a local record->consume lifecycle. It must not run a
  scheduler, spawn workers, add source-level concurrency, call page-source/OSVM
  seams, activate providers, install hooks, replace the process allocator, or
  add backend shortcuts.
- `reclaim_owner_transfer_contract_box.hako` owns MIMAP-051A reclaim
  owner-transfer contract inventory. It may compose M213 abandoned/reclaim
  facts with M215 thread owner-token facts and report contract-ready vs blocked
  preconditions for a future reclaim execution row, but it must not schedule
  threads, use atomics, drain remote frees, mutate ownership, execute reclaim,
  call page-source APIs, unreserve, release OSVM pages, activate providers,
  install hooks, or replace the process allocator.
- `options_inventory_box.hako` owns M214 allocator options/defaults inventory. It may classify static option/default facts and report inactive mutable options, env toggles, provider/hook/replacement, and reclaim execution, but it must not parse process configuration or change allocation behavior.
- `thread_heap_owner_inventory_box.hako` owns M215 thread heap owner-token inventory. It may classify scalar owner-token facts for future abandoned/reclaim rows, but it must not schedule threads, use atomics, drain remote frees, mutate ownership, call page-source APIs, unreserve, or release OSVM pages.
  It may classify scalar owner/page facts into read-only abandoned and reclaim
  candidate vocabulary, but it must not schedule threads, add atomics, execute
  reclaim, call page-source APIs, decommit, recommit, unreserve, release OSVM
  pages, or replace allocators.
- `segment_arena_bitmap_inventory_box.hako` owns MIMAP-079A segment / arena /
  bitmap boundary inventory. It may classify tiny scalar proof-only facts and
  explicit blocked reasons for raw pointer, atomic bitmap, OSVM, provider, and
  invalid-shape requests, but it must not allocate segments, route arena
  memory, execute bitmap claims, call page-source APIs, activate providers,
  replace the process allocator, or add backend shortcuts.
- `segment_lifecycle_scalar_state_box.hako` owns MIMAP-082A segment lifecycle
  scalar state contract. It may classify proof-only segment state transitions
  and blocked substrate requirements, but it must not allocate/free segments,
  route arena backing, execute bitmap claims, call page-source/OSVM APIs, run
  threads, activate providers, replace the process allocator, or add backend
  shortcuts.
- `segment_page_membership_scalar_box.hako` owns MIMAP-085A segment page
  membership scalar contract. It may classify proof-only segment/page/slice
  facts and blocked substrate requirements, but it must not allocate/free
  segments, route arena backing, look up pointer membership, execute bitmap
  claims, call page-source/OSVM APIs, run threads, activate providers, replace
  the process allocator, or add backend shortcuts.
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
