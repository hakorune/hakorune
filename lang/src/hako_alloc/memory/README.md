# lang/src/hako_alloc/memory — Hako Alloc Memory Policy Plane

Scope
- Policy-plane helpers for the `hako_alloc` layer live here.
- This subdir hosts the first moved helpers from the historical `runtime/memory/` path.
- Future allocator policy helpers should follow the same root.

## Indexes

- `MODULE_INDEX.md`: file-level module list.
- `OWNER_CONTRACTS.md`: owner-specific responsibility and stop-line notes.
- `NUMERIC_FIELDS.md`: numeric field classification and current exact `usize`
  inventory.
- `NUMERIC_FIELD_GROUP_LEDGER.md`: detailed exact `usize` field-group
  selection/migration history.
- `page_map_release_invariant_box.hako`: the M173 pre-realloc release invariant
  observer module. It stays as an observer-only seam for release/realloc
  freeze evidence and does not take over page release execution.
- `purge_candidate_policy_box.hako` owns M211 purge candidate policy inventory.
  It keeps the candidate classification route narrow, exact, and explicit
  before any wider purge or decommit policy work opens.
- `heap_reuse_priority_box.hako` owns M208 heap reuse priority policy. It
  keeps the active reuse ranking route narrow, exact, and explicit before any
  wider heap reuse or fresh-page selection work opens.
- `segment_allocation_blocked_substrate_matrix_box.hako` owns MIMAP-149A
  segment allocation blocked-substrate matrix. It keeps the blocked-substrate
  classification route narrow, exact, and explicit before any real execution,
  concurrency, segment lookup, atomics, page-source/OS release seams, or
  backend-visible release work opens.
- `segment_map_local_free_reuse_ledger_box.hako` owns MIMAP-192A segment-map
  local-free reuse ledger bridge. It keeps the bridge route narrow, exact, and
  explicit before any released-token recycle, released-span observation, or
  closeout-pack work opens.
- `segment_map_lookup_guarded_readiness_composition_box.hako` owns MIMAP-153A
  segment-map lookup guarded readiness composition. It keeps the lookup route
  narrow, exact, and explicit before any real execution, raw pointer lookup,
  atomics, or page-source/OS release seams open.
- `object_lifecycle_facade_stats_box.hako` owns MIMAP-018A facade stats
  snapshot. It keeps the stats snapshot route narrow, exact, and explicit
  before any wider policy or backend-visible behavior work opens.
- `worker_tls_cache_box.hako`: the MIMAP-TLS-001 internal worker TLS cache-slot
  substrate. It keeps the worker identity, TLS cache-slot read/write, and
  cache-slot clear routes narrow, exact, and explicit before any wider worker
  or TLS substrate work opens.
- `page_map_realloc_alloc_copy_release_box.hako`: the M175 realloc
  alloc-copy-release fallback module. It keeps the fallback route narrow,
  exact, and explicit before any broader alloc-copy or release-order work
  opens.
- `page_map_release_box.hako`: the M172 page-map-backed release seam. It
  composes page-map lookup, page-local release, and ownership unregistering
  for the explicit release route and keeps the counter fields exact.
- `page_map_realloc_same_class_box.hako`: the M174 no-move realloc module. It
  owns the same-class path and keeps the exact counters for no-move evidence.
- `page_map_realloc_failure_contract_box.hako`: the M176 realloc failure-
  contract diagnostics owner. It freezes zero / oversized reject reporting and
  delegates same-class and grow handling back to M174 / M175.
- `purge_bounded_decommit_box.hako` owns M195 bounded decommit execution. It
  keeps the bounded decommit route narrow, exact, and explicit before any
  wider decommit policy work opens.
- `purge_bounded_scheduler_box.hako` owns M212 bounded purge/decommit scheduler.
  scheduler. It keeps the bounded scheduler route narrow, exact, and explicit
  before any closeout or wider purge policy work opens.
- `stats_box.hako` owns M191 allocator stats snapshots. It keeps the stats
  surface read-only, exact, and explicit before any wider stats or reporting
  work opens.
- `segment_allocation_modeled_local_free_integration_box.hako` owns M119A
  segment allocation modeled local-free integration. It keeps the integration
  route narrow, exact, and explicit before any wider local-free integration
  or page-apply work opens.
- `segment_allocation_modeled_local_free_page_apply_box.hako` owns M115A segment allocation modeled local-free page-model apply. It keeps the page-model apply route narrow, exact, and explicit before any wider local-free apply or page-model policy work opens.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-097A segment
  allocation modeled ledger. It keeps the modeled ledger route narrow, exact,
  and explicit before any release route work opens.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-097A report capsules for the modeled ledger route.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-100A segment
  allocation modeled ledger released-token recycle. It keeps the released-token
  recycle route narrow, exact, and explicit before any wider recycle bridge
  work opens.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-100A report capsules for the released-token recycle route.
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-094A segment
  allocation modeled ledger. It keeps the modeled ledger route narrow, exact,
  and explicit before any release/consume bridge work opens.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-094A report capsules for the modeled ledger route.
- `segment_allocation_modeled_local_free_reuse_ledger_box.hako` owns MIMAP-130A
  segment allocation modeled local-free reuse ledger. It keeps the local-free
  reuse ledger route narrow, exact, and explicit before any release-apply or
  release-applied-recycle bridge work opens.
- `segment_allocation_modeled_local_free_reuse_box.hako` owns MIMAP-188A segment-map local-free reuse bridge. It keeps the reuse bridge route narrow, exact, and explicit before any wider bridge work opens.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_shadow_box.hako` owns MIMAP-224A segment-map local-free reuse ledger lifecycle-keyed release shadow. It keeps the shadow route narrow, exact, and explicit before any migration work opens.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_box.hako` owns MIMAP-228A segment-map local-free reuse source release-ledger lifecycle-key migration. It keeps the migration route narrow, exact, and explicit before any wider migration work opens.
- `segment_allocation_modeled_local_free_reuse_lifecycle_keyed_release_ledger_diagnostic_box.hako` owns MIMAP-229A segment-map local-free reuse source lifecycle-keyed release ledger diagnostics. It keeps the diagnostics route narrow, exact, and explicit before any closeout pack opens.
- `segment_allocation_modeled_local_free_reuse_lifecycle_token_box.hako` owns MIMAP-212A segment-map local-free reuse ledger lifecycle-token pilot. It keeps the lifecycle-token derivation route narrow, exact, and explicit before any release-ledger key migration work opens.
- `segment_allocation_modeled_local_free_reuse_lifecycle_token_release_key_precondition_box.hako` owns MIMAP-220A segment-map local-free reuse ledger lifecycle-token release-key precondition observer. It keeps the precondition route narrow, exact, and explicit before any release-ledger key migration work opens.
- `segment_allocation_modeled_local_free_reuse_lifecycle_token_observer_box.hako` owns MIMAP-216A segment-map local-free reuse ledger lifecycle-token observer diagnostic. It keeps the observer route narrow, exact, and explicit before any release-ledger key migration work opens.
- `segment_map_accepted_readiness_modeled_consume_ledger_box.hako` owns MIMAP-164A segment-map modeled consume ledger released-token recycle. It keeps the released-token recycle route narrow, exact, and explicit before any released-span observation work opens.
- `segment_allocation_modeled_released_span_ledger_box.hako` owns MIMAP-168A segment-map modeled consume ledger released-span observation. It keeps the released-span observation route narrow, exact, and explicit before any wider observation work opens.
- `segment_map_accepted_readiness_modeled_consume_ledger_box.hako` owns MIMAP-184A segment-map local-free integration bridge. It keeps the integration bridge route narrow, exact, and explicit before any real free-list mutation, raw pointer residence, segment-map execution, atomics, OSVM/page-source, worker/TLS, provider activation, or global allocator work opens.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_box.hako` owns MIMAP-304A segment arena backing modeled allocation-ledger release/recycle continuation application bridge. It keeps the application bridge route narrow, exact, and explicit before any observer diagnostics or closeout pack opens.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_continuation_application_bridge_diagnostic_box.hako` owns MIMAP-305A segment arena backing modeled allocation-ledger release/recycle continuation application bridge diagnostics. It keeps the diagnostic route narrow, exact, and explicit before any closeout pack opens.
- `segment_allocation_modeled_local_free_reuse_ledger_release_apply_box.hako` owns MIMAP-200A segment-map local-free reuse ledger release apply bridge. It keeps the release apply bridge route narrow, exact, and explicit before any wider bridge work opens.
- `segment_allocation_modeled_local_free_reuse_ledger_release_box.hako` owns MIMAP-204A segment-map local-free reuse ledger release-applied recycle bridge. It keeps the release-applied recycle bridge route narrow, exact, and explicit before any wider bridge work opens.
- `segment_allocation_modeled_local_free_reuse_ledger_release_box.hako` owns
  MIMAP-196A segment-map local-free reuse ledger release bridge. It keeps the
  release bridge route narrow, exact, and explicit before any closeout pack
  opens.
- `object_lifecycle_facade_huge_unreserve_box.hako`: the MIMAP-034A facade
  huge unreserve owner. It composes MIMAP-029A huge decommit with the MIMAP-
  033A page-source unreserve adapter, then unreserves the exact decommitted
  backing range while still stopping before duplicate/stale unreserve
  diagnostics, recommit, provider activation, and allocator replacement.
- `osvm_fast_path_purge_route_box.hako` owns MIMAP-042A OSVM-backed fast-path bounded purge route. It keeps the bounded purge route narrow, exact, and explicit before any recommit, reuse, unreserve, provider activation, or concurrency work opens.
- `osvm_fast_path_reuse_route_box.hako` owns MIMAP-043A OSVM-backed fast-path recommit/reuse route. It keeps the recommit/reuse route narrow, exact, and explicit before any page-source, OSVM, unreserve, provider activation, or concurrency work opens.
- `purge_page_source_unreserve_adapter_box.hako` owns MIMAP-033A page-source
  unreserve adapter.
- `abandoned_reclaim_inventory_box.hako` owns M213 abandoned/reclaim inventory.
- `remote_free_abandoned_owner_policy_box.hako` owns MIMAP-REMOTE-001
  remote-free / abandoned-owner policy composition. It keeps the
  same-owner / remote-owner / abandoned-owner decision route narrow, exact,
  and explicit before any wider reclaim, worker/TLS, atomic, or policy
  expansion opens.
- `object_lifecycle_facade_page_source_box.hako` owns MIMAP-021B facade page-source fresh-page attach.
- `segment_arena_backing_modeled_source_accounting_diagnostic_box.hako` owns MIMAP-265A segment arena backing modeled source accounting diagnostics.
- `segment_arena_backing_modeled_allocation_plan_box.hako` owns MIMAP-268A segment arena backing modeled allocation plan. It keeps the allocation-plan route narrow, exact, and explicit before any wider apply or ledger work opens.
- `allocator_comparison_benchmark_execution_preflight_inventory_box.hako` owns MIMAP-436A allocator comparison benchmark execution preflight inventory. It keeps the benchmark execution preflight inventory route narrow, exact, and explicit before any benchmark execution, process replacement, hook, backend matcher, or global allocator work opens.
- `segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako` owns MIMAP-281A segment arena backing modeled allocation ledger release candidate diagnostics.
- `segment_arena_backing_modeled_source_bridge_box.hako` owns MIMAP-260A segment arena backing modeled source bridge. It keeps the scalar/model source bridge route narrow, exact, and explicit before any real pointer residence, pointer lookup, arena backing, segment-map, atomic bitmap, OSVM, worker, provider, or backend matcher work opens.
- `object_lifecycle_facade_huge_release_box.hako` owns MIMAP-024A facade huge-release metadata route. It keeps the huge-release metadata route narrow, exact, and explicit before any release execution, fail-fast diagnostics, provider activation, process replacement, hook installation, or allocator replacement work opens.
- `object_lifecycle_facade_huge_release_failfast_box.hako` owns MIMAP-025A facade huge-release fail-fast route. It keeps the huge-release fail-fast route narrow, exact, and explicit before any release execution, provider activation, process replacement, hook installation, or allocator replacement work opens.
- `allocator_comparison_benchmark_execution_preflight_diagnostic_box.hako` owns MIMAP-437A allocator comparison benchmark execution preflight diagnostics. It keeps the benchmark execution preflight diagnostics route narrow, exact, and explicit before any benchmark execution, process replacement, hook, backend matcher, or global allocator work opens.
- `allocator_comparison_controlled_benchmark_execution_inventory_box.hako` owns MIMAP-440A allocator comparison controlled benchmark execution inventory. It keeps the controlled benchmark execution inventory route narrow, exact, and explicit before any benchmark execution, process replacement, hook, backend matcher, or global allocator work opens.
- `allocator_comparison_controlled_benchmark_execution_diagnostic_box.hako` owns MIMAP-441A allocator comparison controlled benchmark execution diagnostics. It keeps the controlled benchmark execution diagnostics route narrow, exact, and explicit before any benchmark execution, process replacement, hook, backend matcher, or global allocator work opens.
- `segment_arena_backing_modeled_allocation_plan_diagnostic_box.hako` owns MIMAP-269A segment arena backing modeled allocation plan diagnostics. It keeps the observer-only allocation-plan route narrow, exact, and explicit before any real pointer residence, pointer lookup, arena backing, segment-map, atomic bitmap, OSVM, worker, provider, or backend matcher work opens.
- `segment_arena_backing_modeled_allocation_apply_box.hako` owns MIMAP-272A segment arena backing modeled allocation apply. It keeps the allocation-apply route narrow, exact, and explicit before any wider ledger or bridge work opens.
- `segment_arena_backing_modeled_allocation_apply_diagnostic_box.hako` owns MIMAP-273A segment arena backing modeled allocation apply diagnostics. It keeps the observer-only allocation-apply route narrow, exact, and explicit before any real pointer residence, pointer lookup, arena backing, segment-map, atomic bitmap, OSVM, worker, provider, or backend matcher work opens.
- `segment_arena_backing_modeled_allocation_ledger_diagnostic_box.hako` owns MIMAP-277A segment arena backing modeled allocation ledger diagnostics.
- `segment_arena_backing_modeled_allocation_ledger_release_apply_box.hako` owns MIMAP-288A segment arena backing modeled allocation ledger release apply.
- `segment_arena_backing_modeled_allocation_ledger_release_apply_diagnostic_box.hako` owns MIMAP-289A segment arena backing modeled allocation ledger release apply diagnostics.
- `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_box.hako` owns MIMAP-292A segment arena backing modeled allocation ledger release-applied recycle. It keeps the release-applied recycle route narrow, exact, and explicit before any closeout pack opens.
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_lifecycle_continuation_bridge_diagnostic_box.hako` owns MIMAP-301A segment arena backing modeled allocation ledger release/recycle lifecycle continuation bridge diagnostics.
- `segment_arena_backing_requirement_matrix_box.hako` owns MIMAP-240A segment arena backing scalar requirement matrix.
- `segment_arena_backing_modeled_allocation_ledger_release_intent_box.hako` owns MIMAP-284A segment arena backing modeled allocation ledger release intent.
- `segment_arena_backing_modeled_allocation_ledger_release_intent_diagnostic_box.hako` owns MIMAP-285A segment arena backing modeled allocation-ledger release intent diagnostics.
- `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_diagnostic_box.hako` owns MIMAP-293A segment arena backing modeled allocation-ledger release-applied recycle diagnostics.
- `segment_arena_backing_modeled_allocation_ledger_release_applied_recycle_second_release_diagnostic_box.hako` owns MIMAP-296A segment arena backing modeled allocation-ledger release-applied recycle second-release diagnostic.

Syntax/style contract
- New allocator state boxes should use Unified Members stored fields:
  `field`, `field: Type`, or `field: Type = expr`.
- Use stored field initializers for fixed defaults and owner construction.
  Initializers are evaluated per construction, so `new ArrayBox()` defaults are
  not shared between instances.
- Keep numeric allocator state on `i64` by default. Exact `usize` production
  fields are allowed only for field groups listed in `NUMERIC_FIELDS.md` and
  advanced by a named phase-294x field-group row.
- Numeric stored field migration is gated by
  [`NUMERIC_FIELDS.md`](./NUMERIC_FIELDS.md). Do not migrate a field to
  `usize` unless its category and sentinel behavior are recorded there first.

Owner-specific responsibility notes moved to `OWNER_CONTRACTS.md`. Keep this
README as the layer entry and style contract only.
