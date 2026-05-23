# Hako Alloc Memory Owner Contracts: core/object lifecycle/provider contracts

Status: Active
Scope: core/object lifecycle/provider contracts.
Related:
- `OWNER_CONTRACTS.md`
- `README.md`

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
- `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_box.hako`
  owns MIMAP-292A. It may record scalar/model release-applied recycle facts
  from accepted modeled allocation-ledger release-apply reports and expose a
  report object plus counters. It must not recycle real arena backing, mutate
  segment-map state, execute atomic bitmap operations, call OSVM/page-source,
  open pointer residence, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_diagnostic_box.hako`
  owns MIMAP-293A. It may observe MIMAP-292A release-applied recycle inventory
  counters and last-report facts, but it must not create additional
  release-applied recycle rows, recycle real arena backing, mutate segment-map
  state, execute atomic bitmap operations, call OSVM/page-source, open pointer
  residence, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_box.hako`
  owns MIMAP-296A. It may observe the MIMAP-292A release-applied recycle
  inventory/report and publish scalar facts that a second release after modeled
  recycle is rejected, but it must not create additional release-applied recycle
  rows, introduce lifecycle generation, recycle real arena backing, mutate
  segment-map state, execute atomic bitmap operations, call OSVM/page-source,
  open pointer residence, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_box.hako`
  owns MIMAP-300A. It may record one scalar/model lifecycle-continuation bridge
  row from an accepted release-applied recycle report, keyed by an explicit
  model-only continuation token. It must not introduce real lifecycle
  generation, recycle real arena backing, mutate segment-map state, execute
  atomic bitmap operations, call OSVM/page-source, open pointer residence,
  activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_diagnostic_box.hako`
  owns MIMAP-301A. It may observe MIMAP-300A lifecycle-continuation bridge
  inventory/report state and publish scalar diagnostic facts. It must not
  record a new continuation row, introduce real lifecycle generation, recycle
  real arena backing, mutate segment-map state, execute atomic bitmap
  operations, call OSVM/page-source, open pointer residence, activate
  providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_box.hako`
  owns MIMAP-304A. It may record one scalar/model continuation application
  bridge row from an accepted lifecycle-continuation bridge report, keyed by an
  explicit model-only application token. It must not introduce real lifecycle
  generation, recycle real arena backing, mutate segment-map state, execute
  atomic bitmap operations, call OSVM/page-source, open pointer residence,
  activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_diagnostic_box.hako`
  owns MIMAP-305A. It may observe MIMAP-304A continuation application bridge
  inventory/report state and publish scalar diagnostic facts. It must not
  record a new application row, introduce real lifecycle generation, recycle
  real arena backing, mutate segment-map state, execute atomic bitmap
  operations, call OSVM/page-source, open pointer residence, activate
  providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_box.hako`
  owns MIMAP-308A. It may observe an accepted MIMAP-304A continuation
  application report and publish compact scalar/model applied-state summary
  facts. It must not record a new application row, introduce real lifecycle
  generation, release or recycle real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, open pointer
  residence, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_diagnostic_box.hako`
  owns MIMAP-309A. It may observe MIMAP-308A applied-state summary facts and
  publish scalar diagnostic counters/reports. It must not record a new summary
  row, introduce real lifecycle generation, release or recycle real arena
  backing, mutate segment-map state, execute atomic bitmap operations, call
  OSVM/page-source, open pointer residence, activate providers, or add backend
  matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako`
  owns MIMAP-312A. It may observe MIMAP-308A applied-state summary facts and
  publish a model-only readiness matrix for future release/recycle execution.
  It must not create lifecycle generation, open pointer residence, release or
  recycle real arena backing, mutate segment-map state, execute atomic bitmap
  operations, call OSVM/page-source, activate providers, or add backend
  matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostic_box.hako`
  owns MIMAP-313A. It may observe MIMAP-312A execution readiness matrix facts
  and publish scalar diagnostic counters/reports. It must not record new matrix
  rows, create lifecycle generation, open pointer residence, release or recycle
  real arena backing, mutate segment-map state, execute atomic bitmap
  operations, call OSVM/page-source, activate providers, or add backend
  matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_box.hako`
  owns MIMAP-316A. It may record explicit model-only release/recycle execution
  intent from accepted readiness matrix evidence and publish unsupported
  execution facts. It must not execute release/recycle behavior, create
  lifecycle generation, open pointer residence, release or recycle real arena
  backing, mutate segment-map state, execute atomic bitmap operations, call
  OSVM/page-source, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_diagnostic_box.hako`
  owns MIMAP-317A. It may observe MIMAP-316A intent marker facts and publish
  scalar diagnostic counters/reports. It must not record new intent marker
  rows, execute release/recycle behavior, create lifecycle generation, open
  pointer residence, release or recycle real arena backing, mutate segment-map
  state, execute atomic bitmap operations, call OSVM/page-source, activate
  providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_box.hako`
  owns MIMAP-320A. It may record model-only unsupported execution outcomes from
  accepted MIMAP-316A intent marker facts. It must not execute release/recycle
  behavior, create lifecycle generation, open pointer residence, release or
  recycle real arena backing, mutate segment-map state, execute atomic bitmap
  operations, call OSVM/page-source, activate providers, or add backend
  matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_diagnostic_box.hako`
  owns MIMAP-321A. It may observe MIMAP-320A unsupported outcome ledger facts
  and publish scalar diagnostic counters/reports. It must not record new
  outcome rows, execute release/recycle behavior, create lifecycle generation,
  open pointer residence, release or recycle real arena backing, mutate
  segment-map state, execute atomic bitmap operations, call OSVM/page-source,
  activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako`
  owns MIMAP-324A. It may record a model-only release/recycle execution support
  gate from accepted MIMAP-320A unsupported outcome facts. The gate remains
  closed and blocked by the unsupported outcome. It must not execute
  release/recycle behavior, create lifecycle generation, open pointer
  residence, release or recycle real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, activate providers,
  or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostic_box.hako`
  owns MIMAP-325A. It may observe MIMAP-324A support gate facts and publish
  scalar diagnostic counters/reports. It must not record new support gate rows,
  execute release/recycle behavior, create lifecycle generation, open pointer
  residence, release or recycle real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, activate providers,
  or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_box.hako`
  owns MIMAP-328A. It may record a model-only release/recycle execution support
  requirement matrix from closed MIMAP-324A support gate facts. It must not
  satisfy requirements, execute release/recycle behavior, create lifecycle
  generation, open pointer residence, release or recycle real arena backing,
  mutate segment-map state, execute atomic bitmap operations, call
  OSVM/page-source, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostic_box.hako`
  owns MIMAP-329A. It may observe MIMAP-328A requirement matrix facts and
  publish scalar diagnostic counters/reports. It must not record new
  requirement matrix rows, satisfy requirements, execute release/recycle
  behavior, create lifecycle generation, open pointer residence, release or
  recycle real arena backing, mutate segment-map state, execute atomic bitmap
  operations, call OSVM/page-source, activate providers, or add backend
  matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_box.hako`
  owns MIMAP-332A. It may record a model-only lifecycle generation
  prerequisite from MIMAP-328A requirement matrix facts. It must not generate
  real lifecycle tokens, execute release/recycle behavior, open pointer
  residence, release or recycle real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, activate providers,
  or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_generation_prerequisite_diagnostic_box.hako`
  owns MIMAP-333A. It may observe MIMAP-332A lifecycle generation prerequisite
  facts and publish scalar diagnostic counters/reports. It must not record new
  prerequisite rows, generate real lifecycle tokens, execute release/recycle
  behavior, open pointer residence, release or recycle real arena backing,
  mutate segment-map state, execute atomic bitmap operations, call
  OSVM/page-source, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_box.hako`
  owns MIMAP-336A. It may record a model-only pointer residence prerequisite
  from MIMAP-332A lifecycle generation prerequisite facts. It must not create
  raw pointer residence, perform pointer-derived lookup, execute
  release/recycle behavior, release or recycle real arena backing, mutate
  segment-map state, execute atomic bitmap operations, call OSVM/page-source,
  activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_residence_prerequisite_diagnostic_box.hako`
  owns MIMAP-337A. It may observe MIMAP-336A pointer residence prerequisite
  facts and publish scalar diagnostic counters/reports. It must not record new
  prerequisite rows, create raw pointer residence, perform pointer-derived
  lookup, execute release/recycle behavior, release or recycle real arena
  backing, mutate segment-map state, execute atomic bitmap operations, call
  OSVM/page-source, activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box.hako`
  owns MIMAP-340A. It may record a model-only pointer-derived lookup
  prerequisite from MIMAP-336A pointer residence prerequisite facts. It must not
  create raw pointer residence, perform pointer-derived lookup, execute
  release/recycle behavior, release or recycle real arena backing, mutate
  segment-map state, execute atomic bitmap operations, call OSVM/page-source,
  activate providers, or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_diagnostic_box.hako`
  owns MIMAP-341A. It may observe MIMAP-340A pointer-derived lookup prerequisite
  facts and publish scalar diagnostic counters/reports. It also closes the
  model-only pointer-derived lookup prerequisite pack. It must not record new
  prerequisite rows, perform pointer-derived lookup, execute release/recycle
  behavior, release or recycle real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, activate providers,
  or add backend matchers.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_remaining_execution_prerequisite_ledger_box.hako`
  owns MIMAP-342A. It may record the remaining model-only release/recycle
  execution requirements from the requirement matrix as one bundled ledger. It
  must not execute release/recycle behavior, create raw pointer residence,
  perform pointer-derived lookup, release or recycle real arena backing, mutate
  segment-map state, execute atomic bitmap operations, call OSVM/page-source,
  run worker/TLS behavior, activate providers, or add backend matchers.
- `segment_arena_backing_no_escape_pointer_residence_pilot_box.hako`
  owns MIMAP-344A. It may record a private proof-scope no-escape pointer
  residence token from the accepted remaining execution prerequisite ledger. It
  must not perform pointer-derived lookup, dereference, execute release/recycle
  behavior, release or recycle real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, run worker/TLS
  behavior, activate providers, or add backend matchers.
- `segment_arena_backing_handle_pilot_box.hako` owns MIMAP-345A. It may record
  a bounded arena backing handle token from an accepted no-escape pointer
  residence report. It must not perform pointer-derived lookup, dereference,
  execute release/recycle behavior, release or recycle real arena backing,
  mutate segment-map state, execute atomic bitmap operations, call
  OSVM/page-source, run worker/TLS behavior, activate providers, or add backend
  matchers.
- `segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako`
  owns MIMAP-346A. It may derive one bounded pointer-derived lookup fact from an
  accepted arena backing handle and keep that fact non-dereferenceable. It must
  not dereference, execute release/recycle behavior, release or recycle real
  arena backing, mutate segment-map state, execute atomic bitmap operations,
  call OSVM/page-source, run worker/TLS behavior, activate providers, or add
  backend matchers.
- `segment_map_mutation_pilot_box.hako` owns MIMAP-347A. It may record one
  bounded segment-map mutation fact from an accepted pointer-derived lookup
  report. It must not dereference, execute release/recycle behavior, release or
  recycle real arena backing, execute atomic bitmap operations, call
  OSVM/page-source, run worker/TLS behavior, activate providers, or add backend
  matchers.
- `atomic_bitmap_pilot_box.hako` owns MIMAP-348A. It may record one bounded
  atomic bitmap fact from an accepted segment-map mutation report. It must not
  use real atomic primitives, dereference, execute release/recycle behavior,
  release or recycle real arena backing, call OSVM/page-source, run worker/TLS
  behavior, activate providers, or add backend matchers.
- `osvm_page_source_pilot_box.hako` owns MIMAP-349A. It may record one bounded
  OSVM/page-source fact from an accepted atomic bitmap report. It must not
  dereference, execute release/recycle behavior, release or recycle real arena
  backing, run worker/TLS behavior, activate providers, replace the host
  allocator, expose hooks, or add backend matchers.
- `worker_tls_pilot_box.hako` owns MIMAP-350A. It may record one bounded
  worker/TLS fact from an accepted OSVM/page-source report through the existing
  internal `HakoAllocWorkerTlsCache` seam. It must not expose source-level
  worker-local syntax, spawn or schedule workers, execute release/recycle
  behavior, activate providers, replace the host allocator, expose hooks, or
  add backend matchers.
- `provider_inactive_boundary_inventory_box.hako` owns MIMAP-352A. It may
  record that provider activation, host allocator replacement, hooks, and
  backend matchers remain inactive after an accepted worker/TLS report. It must
  not select a provider, activate provider behavior, install hooks, replace the
  host allocator, or add backend matchers.
- `provider_boundary_diagnostic_vocabulary_box.hako` owns MIMAP-360A. It may
  inventory provider boundary diagnostic reason codes after an accepted
  provider inactive boundary report. It must not select a provider, activate
  provider behavior, install hooks, replace the host allocator, or add backend
  matchers.
- `provider_readiness_preflight_box.hako` owns MIMAP-362A. It may preflight
  provider readiness from accepted provider boundary diagnostic vocabulary. It
  must not select a provider, activate provider behavior, install hooks,
  replace the host allocator, or add backend matchers.
- `provider_selection_inventory_box.hako` owns MIMAP-364A. It may record one
  provider candidate token and provider kind after accepted provider readiness
  preflight. It must not activate provider behavior, install hooks, replace the
  host allocator, or add backend matchers.
- `provider_activation_unsupported_outcome_ledger_box.hako` owns MIMAP-370A.
  It may ledger a provider activation request as an unsupported outcome after
  accepted provider selection inventory. It must not activate provider
  behavior, call provider APIs, install hooks, replace the host allocator, run
  worker/TLS behavior, or add backend matchers.
- `provider_activation_input_bundle_inventory_box.hako` owns MIMAP-376A. It
  may inventory an explicit provider activation input bundle after an accepted
  unsupported-outcome ledger report. It must not activate provider behavior,
  call provider APIs, install hooks, replace the host allocator, run worker/TLS
  behavior, or add backend matchers.
- `provider_activation_dry_run_unsupported_behavior_box.hako` owns MIMAP-378A.
  It may consume an accepted explicit provider activation input bundle and
  record an unsupported dry-run activation outcome. It must not activate
  provider behavior, call provider APIs, install hooks, replace the host
  allocator, run worker/TLS behavior, or add backend matchers.
- `provider_activation_modeled_open_pilot_box.hako` owns MIMAP-380A. It may
  consume an accepted dry-run unsupported outcome and record modeled provider
  activation as open. It must not call provider APIs, install hooks, replace
  the host allocator, run worker/TLS behavior, or add backend matchers.
- `provider_call_capability_gate_inventory_box.hako` owns MIMAP-382A. It may
  consume a modeled-open activation report and inventory the provider-call
  capability gate. It must not call provider APIs, install hooks, replace the
  host allocator, run worker/TLS behavior, or add backend matchers.
- `provider_call_dry_run_unsupported_behavior_box.hako` owns MIMAP-384A. It
  may consume an accepted provider-call capability gate and record an
  unsupported provider-call dry-run outcome. It must not call provider APIs,
  install hooks, replace the host allocator, run worker/TLS behavior, or add
  backend matchers.
- `provider_call_modeled_open_pilot_box.hako` owns MIMAP-386A. It may consume
  an accepted provider-call dry-run unsupported outcome and record modeled
  provider-call open state. It must not call provider APIs, install hooks,
  replace the host allocator, run worker/TLS behavior, or add backend matchers.
- `provider_call_execution_capability_preflight_box.hako` owns MIMAP-388A. It
  may consume an accepted provider-call modeled-open report and inventory the
  explicit execution capability preflight. It must not call provider APIs,
  install hooks, replace the host allocator, run worker/TLS behavior, or add
  backend matchers.
- `provider_call_noop_execution_seam_pilot_box.hako` owns MIMAP-390A. It may
  consume an accepted provider-call execution capability preflight and record a
  no-op execution seam crossing. It must not call provider APIs, install hooks,
  replace the host allocator, run worker/TLS behavior, or add backend matchers.
- `provider_call_real_api_execution_preflight_box.hako` owns MIMAP-392A. It may
  consume an accepted provider-call no-op execution seam report and inventory
  future real provider API readiness. It must not call provider APIs, install
  hooks, replace the host allocator, run worker/TLS behavior, or add backend
  matchers.
- `provider_call_real_api_stub_execution_pilot_box.hako` owns MIMAP-396A. It
  may consume an accepted provider-call real API execution preflight report and
  record model-space stub provider API call execution evidence. It must not
  call actual provider APIs, install hooks, replace the host allocator, run
  worker/TLS behavior, or add backend matchers.
- `provider_call_external_api_adapter_inventory_box.hako` owns MIMAP-400A. It
  may consume an accepted provider-call real API stub execution report and
  inventory external provider API adapter presence/readiness. It must not call
  external provider APIs, install hooks, replace the host allocator, run
  worker/TLS behavior, or add backend matchers.
- `provider_call_external_api_adapter_preflight_box.hako` owns MIMAP-402A. It
  may consume an accepted provider-call external API adapter inventory report
  and record preflight readiness for a future external provider API call. It
  must not call external provider APIs, install hooks, replace the host
  allocator, run worker/TLS behavior, or add backend matchers.
- `provider_call_external_api_call_stub_execution_pilot_box.hako` owns
  MIMAP-406A. It may consume an accepted provider-call external API adapter
  preflight report and record model-space external provider API call stub
  execution evidence. It must not call actual external provider APIs, install
  hooks, replace the host allocator, run worker/TLS behavior, or add backend
  matchers.
- `real_external_provider_api_adapter_execution_preflight_box.hako` owns
  MIMAP-410A. It may consume an accepted external provider API call stub
  execution report and record readiness for a future real external provider API
  adapter execution. It must not call actual external provider APIs, install
  hooks, replace the host allocator, run worker/TLS behavior, or add backend
  matchers.
- `real_external_provider_api_call_first_pattern_pilot_box.hako` owns
  MIMAP-415A. It may consume an accepted real external provider API adapter
  execution preflight report and record first-pattern real external provider API
  call pilot evidence. It must not install hooks, replace the host allocator,
  run worker/TLS behavior, add backend matchers, or install a global allocator.
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
