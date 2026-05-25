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
- `segment_allocation_modeled_ledger_box.hako` owns MIMAP-094A segment
  allocation modeled ledger. It keeps the modeled ledger route narrow, exact,
  and explicit before any release/consume bridge work opens.
- `segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-094A report capsules for the modeled ledger route.
- `segment_allocation_modeled_local_free_reuse_ledger_box.hako` owns MIMAP-130A
  segment allocation modeled local-free reuse ledger. It keeps the local-free
  reuse ledger route narrow, exact, and explicit before any release-apply or
  release-applied-recycle bridge work opens.
- `segment_allocation_modeled_local_free_reuse_ledger_release_box.hako` owns
  MIMAP-196A segment-map local-free reuse ledger release bridge. It keeps the
  release bridge route narrow, exact, and explicit before any closeout pack
  opens.
- `object_lifecycle_facade_huge_unreserve_box.hako`: the MIMAP-034A facade
  huge unreserve owner. It composes MIMAP-029A huge decommit with the MIMAP-
  033A page-source unreserve adapter, then unreserves the exact decommitted
  backing range while still stopping before duplicate/stale unreserve
  diagnostics, recommit, provider activation, and allocator replacement.
- `purge_page_source_unreserve_adapter_box.hako` owns MIMAP-033A page-source
  unreserve adapter.
- `abandoned_reclaim_inventory_box.hako` owns M213 abandoned/reclaim inventory.
- `object_lifecycle_facade_page_source_box.hako` owns MIMAP-021B facade page-source fresh-page attach.
- `segment_arena_backing_modeled_source_accounting_diagnostic_box.hako` owns MIMAP-265A segment arena backing modeled source accounting diagnostics.
- `segment_arena_backing_modeled_allocation_ledger_release_candidate_diagnostic_box.hako` owns MIMAP-281A segment arena backing modeled allocation ledger release candidate diagnostics.
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
