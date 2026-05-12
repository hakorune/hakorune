# lang/src/hako_alloc/memory — Hako Alloc Memory Policy Plane

Scope
- Policy-plane helpers for the `hako_alloc` layer live here.
- This subdir hosts the first moved helpers from the historical `runtime/memory/` path.
- Future allocator policy helpers should follow the same root.

Current modules
- `allocator_facade_box.hako`
- `alloc_fast_path_heap_box.hako`
- `arc_box.hako`
- `layout_box.hako`
- `osvm_backed_fast_path_heap_box.hako`
- `page_box.hako`
- `page_heap_box.hako`
- `page_map_box.hako`
- `page_map_release_box.hako`
- `page_queue_box.hako`
- `page_source_policy_box.hako`
- `remote_free_page_integration_box.hako`
- `refcell_box.hako`
- `remote_free_policy_box.hako`
- `size_class_box.hako`
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
- Keep `birth(...)` for parameter-dependent initialization and ordering that
  cannot be expressed as a declaration-site default.
