# Hako Alloc Memory Owner Contracts: segment allocation and local-free modeled ledger contracts

Status: Active
Scope: segment allocation and local-free modeled ledger contracts.
Related:
- `OWNER_CONTRACTS.md`
- `README.md`

- `segment_allocation_readiness_scalar_box.hako` owns MIMAP-088A. It may
  classify scalar segment/page allocation-readiness facts for a known page,
  including active-state support, page capacity, request block count, stable
  reject reasons, and inactive substrate flags. It must not execute segment
  allocation/free, allocate arena backing, use raw pointer residence, use a
  segment-map lookup, execute atomic bitmap claims, call page-source/OSVM
  seams, schedule workers, activate provider hooks, replace the host allocator,
  or add backend shortcuts.
- `segment_allocation_blocked_substrate_matrix_box.hako` owns MIMAP-149A. It
  may compose the existing scalar segment readiness, segment/page membership,
  and segment/arena/bitmap inventory facts into a proof-only blocked-substrate
  matrix. It may publish stable blocker reasons and inactive execution/provider
  flags. It must not execute real segment allocation/free, allocate arena
  backing, use raw pointer residence, perform segment-map lookup, execute
  atomic bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_map_scalar_lookup_boundary_inventory_box.hako` owns MIMAP-151A. It
  may prove one explicit-ID scalar lookup row for segment/page/slice/generation
  membership and stable reject reasons for unknown segment, wrong page, stale
  generation, out-of-range slice, and raw-pointer lookup requests. It must not
  use raw pointer residence, create a real segment-map lookup, allocate arena
  backing, execute atomic bitmap claims, call page-source/OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_map_lookup_guarded_readiness_composition_box.hako` owns MIMAP-153A.
  It may call the explicit-ID segment-map scalar lookup first, then compose
  accepted lookup rows with segment/page membership and allocation-readiness
  scalar facts. It must not use raw pointer residence, create real segment-map
  lookup execution, allocate arena backing, execute atomic bitmap claims, call
  page-source/OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_map_accepted_readiness_modeled_consume_ledger_box.hako` owns
  MIMAP-157A. It may compose an accepted MIMAP-153A readiness report into the
  existing MIMAP-091A modeled consume and MIMAP-094A modeled ledger owners. It
  must not use raw pointer residence, create real segment-map execution,
  allocate arena backing, execute atomic bitmap claims, call page-source/OSVM
  seams, schedule workers, activate provider hooks, replace the host allocator,
  or add backend shortcuts.
  It also owns MIMAP-161A. It may release a live modeled token through the same
  owner boundary by reusing the existing MIMAP-097A modeled ledger release
  substrate, while keeping real segment free execution closed.
  It also owns MIMAP-164A. It may prove that a released token can be accepted
  again as a new live modeled row by reusing the existing MIMAP-100A
  released-token recycle contract, while keeping real segment allocation/free
  execution closed.
  It also owns MIMAP-168A. It may expose the release report's scalar
  `modeled_block_end` and record successful segment-map release reports into
  the existing MIMAP-107A released-span ledger, while keeping real segment free
  execution and free-list mutation closed.
- `page_map_release_box.hako` owns M172. It may compose
  `HakoAllocPageMap.lookup(...)`, `HakoAllocPageModel.releaseLocal(...)`, and
  `HakoAllocPageMap.unregister(...)` into the explicit page-map-backed release
  seam. It must keep pointer registration owned by `page_map_box`, keep the
  release counters exact, and must not own pointer registration, execute real
  segment free, allocate arena backing, use raw pointer residence, perform
  segment-map lookup beyond the explicit page-map route, execute atomic bitmap
  claims, call page-source/OSVM seams, schedule workers, activate provider
  hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-094A report capsules. It may build the scalar modeled ledger reports and update the
  ledger observer counters. It must not execute real segment allocation/free,
  allocate arena backing, use raw pointer residence, perform segment-map
  lookup, execute atomic bitmap claims, call page-source/OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-097A report capsules. It may build the scalar release reports and update the
  release observer counters. It must not execute real segment free, allocate
  arena backing, use raw pointer residence, perform segment-map lookup,
  execute atomic bitmap claims, call page-source/OSVM seams, schedule workers,
  activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-100A report capsules. It may report the released-token recycle ledger results while
  keeping real segment allocation/free closed. It must not execute real
  segment allocation/free, allocate arena backing, use raw pointer residence,
  perform segment-map lookup, execute atomic bitmap claims, call page-source/
  OSVM seams, schedule workers, activate provider hooks, replace the host
  allocator, or add backend shortcuts.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-104A report capsules. It may enrich successful release span reports with scalar span
  facts. It must not execute real segment free, mutate a free-list, mutate
  page state outside the modeled ledger, allocate arena backing, use raw
  pointer residence, perform segment-map lookup, execute atomic bitmap claims,
  call page-source/OSVM seams, schedule workers, activate provider hooks,
  replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_consume_box.hako` owns MIMAP-091A. It may consume
  accepted scalar segment allocation-readiness facts and model the resulting
  `page_used` / `remaining_blocks` values plus a stable scalar modeled
  allocation token. It must not execute real segment allocation/free, allocate
  arena backing, use raw pointer residence, perform segment-map lookup, execute
  atomic bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-094A. It may record
  accepted MIMAP-091A modeled consume results as scalar token rows and expose
  deterministic token lookup/read facts. It must not execute real segment
  allocation/free, allocate arena backing, use raw pointer residence, perform
  segment-map lookup, execute atomic bitmap claims, call page-source/OSVM seams,
  schedule workers, activate provider hooks, replace the host allocator, or add
  backend shortcuts.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-097A as well. It may
  mark exactly one live modeled allocation token as released in the scalar
  ledger and expose deterministic release facts/counters. It must not execute
  real segment free, allocate arena backing, use raw pointer residence, perform
  segment-map lookup, execute atomic bitmap claims, call page-source/OSVM seams,
  schedule workers, activate provider hooks, replace the host allocator, or add
  backend shortcuts.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-100A as well. It may
  prove that a released modeled allocation token can be recorded again as the
  current live scalar ledger row while simultaneous live duplicates remain
  rejected. It must not execute real segment allocation/free, allocate arena
  backing, use raw pointer residence, perform segment-map lookup, execute atomic
  bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-104A as well. It may
  enrich successful modeled release reports with scalar block span facts from
  the ledger row. It must not execute real segment free, mutate a free-list,
  mutate page state outside the modeled ledger, allocate arena backing, use raw
  pointer residence, perform segment-map lookup, execute atomic bitmap claims,
  call page-source/OSVM seams, schedule workers, activate provider hooks,
  replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_released_span_ledger_box.hako` owns MIMAP-107A.
  It may consume successful MIMAP-104A release span reports and record
  deterministic scalar released-span rows. It must not execute real segment
  free, mutate a free-list, mutate page state outside the released-span ledger,
  allocate arena backing, use raw pointer residence, perform segment-map
  lookup, execute atomic bitmap claims, call page-source/OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_allocation_modeled_local_free_candidate_ledger_box.hako` owns
  MIMAP-109A. It may consume successful MIMAP-107A released-span ledger reports
  and record deterministic scalar local-free candidate rows. It must not
  execute real segment free, mutate a free-list, mutate page state outside the
  local-free candidate ledger, allocate arena backing, use raw pointer
  residence, perform segment-map lookup, execute atomic bitmap claims, call
  page-source/OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
  It also owns MIMAP-172A. It may consume released-span rows produced from the
  segment-map modeled consume-ledger owner boundary and record the same scalar
  local-free candidate facts, while keeping real free-list mutation and real
  segment free execution closed.
- `segment_allocation_modeled_local_free_apply_plan_box.hako` owns
  MIMAP-111A. It may consume successful MIMAP-109A local-free candidate reports
  and record deterministic scalar local-free apply-plan rows. It must not
  execute real segment free, mutate a free-list, mutate page state outside the
  local-free apply-plan ledger, allocate arena backing, use raw pointer
  residence, perform segment-map lookup, execute atomic bitmap claims, call
  page-source/OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
  It also owns MIMAP-176A. It may consume local-free candidate rows produced
  from the segment-map released-span bridge and record the same scalar
  apply-plan facts, while keeping real free-list mutation and page-state
  mutation closed.
- `segment_allocation_modeled_local_free_page_apply_box.hako` owns
  MIMAP-115A. It may consume successful MIMAP-111A local-free apply-plan reports
  and apply each block in the span to an explicit `HakoAllocPageModel` only by
  calling `HakoAllocPageModel.releaseLocal(block_id)`. It must not execute real
  segment free beyond the existing page-local model, mutate page arrays
  directly, use raw pointer residence, perform segment-map lookup, execute
  atomic bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
  It also owns MIMAP-180A. It may consume apply-plan rows produced from the
  segment-map bridge and apply the same modeled local-free block span through
  the explicit page model, while keeping real allocator free-list mutation,
  raw pointer residence, real segment-map execution, and backend shortcuts
  closed.
- `segment_allocation_modeled_local_free_integration_box.hako` owns
  MIMAP-119A. It may compose the existing MIMAP-109A local-free candidate
  ledger, MIMAP-111A apply-plan ledger, and MIMAP-115A page-model apply route
  from a successful MIMAP-107A released-span report and an explicit
  `HakoAllocPageModel`. It must not execute real segment free beyond the
  existing page-local model, mutate page arrays directly, use raw pointer
  residence, perform segment-map lookup, execute atomic bitmap claims, call
  page-source/OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
  It also owns MIMAP-184A. It may consume released-span rows produced from the
  segment-map bridge and record the same modeled local-free integration facts,
  while keeping real allocator free-list mutation, raw pointer residence, real
  segment-map execution, arena backing, atomics, OSVM/page-source calls, and
  backend shortcuts closed.
- `segment_allocation_modeled_local_free_reuse_box.hako` owns MIMAP-126A. It
  may compose the existing MIMAP-119A local-free integration route with
  `HakoAllocPageModel.acquire(size)` and prove page-local reuse only when the
  ordinary free list is empty and `acquire` must collect from `local_free`.
  It must not execute real segment free beyond the existing page-local model,
  mutate page arrays directly, use raw pointer residence, perform segment-map
  lookup, allocate arena backing, execute atomic bitmap claims, call
  page-source/OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
  It also owns MIMAP-188A. It may consume released-span rows produced from the
  segment-map bridge and prove the same modeled local-free reuse facts, while
  keeping real allocator free-list mutation, raw pointer residence, real
  segment-map execution, arena backing, atomics, OSVM/page-source calls, and
  backend shortcuts closed.
- `segment_allocation_modeled_local_free_reuse_ledger_box.hako` owns
  MIMAP-130A, MIMAP-138A, MIMAP-142A, MIMAP-192A, MIMAP-200A, and
  MIMAP-204A. It may consume successful MIMAP-126A local-free
  reuse reports and record deterministic scalar live reuse allocation rows
  keyed by `(segment_id, page_id, reused_block_id)`. It may also consume
  successful MIMAP-134A release facts and mark the matching source reuse ledger
  row non-live. After that release apply, it may record the same modeled reuse
  token again as a new live source row while still-live duplicates remain
  rejected. MIMAP-192A may consume segment-map-derived local-free reuse reports
  and record the same deterministic live reuse row shape. MIMAP-200A may apply
  segment-map-derived release facts to that source ledger and mark the matching
  row non-live. MIMAP-204A may then record the same segment-map-derived modeled
  reuse token again as a new live source row while still-live duplicates remain
  rejected. Its release-apply reporting and reject aggregation live in
  `segment_allocation_modeled_local_free_reuse_ledger_release_apply_box.hako`.
  It must not widen the bump-shaped
  `segment_allocation_modeled_ledger_box.hako` contract, execute real segment
  allocation/free, mutate page arrays, use raw pointer residence, perform
  segment-map lookup, allocate arena backing, execute atomic bitmap claims,
  call page-source/OSVM seams, schedule workers, activate provider hooks,
  replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_ledger_release_box.hako` owns
  MIMAP-134A. It may consume successful MIMAP-130A local-free reuse ledger
  reports and record one scalar release row per modeled reuse token. MIMAP-196A
  may feed it a segment-map-derived local-free reuse ledger row. It must not
  mutate the source reuse ledger, widen the bump-shaped modeled ledger contract,
  execute real segment allocation/free, mutate page arrays, use raw pointer
  residence, perform segment-map lookup, allocate arena backing, execute atomic
  bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako` owns
  MIMAP-212A. It may model a scalar lifecycle token derived from one
  modeled reuse token and one explicit lifecycle id, using branded helper
  parameters only inside the owner. It must not migrate the release ledger key,
  claim multi-cycle release/recycle semantics, mutate source ledger or release
  owner state, execute real segment allocation/free, use raw pointer residence,
  perform real segment-map execution, allocate arena backing, execute atomic
  bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako`
  owns MIMAP-216A. It may observe the lifecycle-token pilot owner and the
  release-owner duplicate diagnostic, reporting that the release ledger is
  still keyed by modeled reuse token. It must not migrate release-ledger keys,
  define real lifecycle semantics, mutate source ledger or release owner state,
  execute real segment allocation/free, use raw pointer residence, perform real
  segment-map execution, allocate arena backing, execute atomic bitmap claims,
  call page-source/OSVM seams, schedule workers, activate provider hooks,
  replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box.hako`
  owns MIMAP-220A. It may classify lifecycle-token observer reports as ready
  or blocked for a future release-key migration decision, but it must not
  migrate release-ledger keys, define real lifecycle semantics, mutate source
  ledger or release owner state, execute real segment allocation/free, use raw
  pointer residence, perform real segment-map execution, allocate arena
  backing, execute atomic bitmap claims, call page-source/OSVM seams, schedule
  workers, activate provider hooks, replace the host allocator, or add backend
  shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box.hako`
  owns MIMAP-224A. It may model a shadow release ledger keyed by reuse
  lifecycle token after the release-key precondition observer accepts. It must
  not migrate the source release ledger key, define real lifecycle semantics,
  mutate source ledger or release owner state, execute real segment
  allocation/free, use raw pointer residence, perform real segment-map
  execution, allocate arena backing, execute atomic bitmap claims, call
  page-source/OSVM seams, schedule workers, activate provider hooks, replace
  the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako`
  owns MIMAP-228A. It may model the migrated source release ledger keyed by
  reuse lifecycle token while preserving modeled reuse token as a backref. It
  must not mutate the old modeled-reuse-token keyed release owner, define real
  lifecycle semantics, execute real segment allocation/free, use raw pointer
  residence, perform real segment-map execution, allocate arena backing, execute
  atomic bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box.hako`
  owns MIMAP-229A. It may observe the lifecycle-keyed source release ledger and
  publish duplicate/precondition/lifecycle/mismatch/unsupported reject summary
  facts. It must not mutate either release ledger, define real lifecycle
  semantics, execute real segment allocation/free, use raw pointer residence,
  perform real segment-map execution, allocate arena backing, execute atomic
  bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_ledger_box.hako` owns the
  MIMAP-232A lifecycle-keyed release apply/recycle continuation entry
  `applyReuseLedgerLifecycleKeyedRelease`. It may apply a lifecycle-keyed source
  release report to the current live reuse-ledger row using modeled reuse token
  as an explicit backref. It must not use the old modeled-reuse-token keyed
  release owner as the continuation owner, define real lifecycle semantics,
  execute real segment allocation/free, use raw pointer residence, perform real
  segment-map execution, allocate arena backing, execute atomic bitmap claims,
  call page-source/OSVM seams, schedule workers, activate provider hooks,
  replace the host allocator, or add backend shortcuts.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_apply_recycle_diagnostic_box.hako`
  owns MIMAP-233A. It may observe the MIMAP-232A apply/recycle continuation and
  publish missing-live-row, unsupported-apply, and post-continuation duplicate
  facts. It must not mutate reuse/release ledgers, define real lifecycle
  semantics, execute real segment allocation/free, use raw pointer residence,
  perform real segment-map execution, allocate arena backing, execute atomic
  bitmap claims, call page-source/OSVM seams, schedule workers, activate
  provider hooks, replace the host allocator, or add backend shortcuts.
