# lang/src/hako_alloc/memory — Hako Alloc Memory Policy Plane

Scope
- Policy-plane helpers for the `hako_alloc` layer live here.
- This subdir hosts the first moved helpers from the historical `runtime/memory/` path.
- Future allocator policy helpers should follow the same root.

Current modules
- `alignment_policy_box.hako`
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
- `huge_threshold_router_box.hako` owns M179 huge threshold/routing. It may
  classify padded requests and fail fast for huge-unsupported requests, but it
  must not implement a huge page model or OS release.
- `huge_page_model_box.hako` owns M180 huge page modeling. It may register huge
  handles and track requested/committed/live metadata, but it must not implement
  huge release, unregister, or OS release.
- `huge_release_seam_box.hako` owns M181 huge release composition. It may mark
  huge model state released and unregister page-map ownership, but it must not
  call small page `releaseLocal(...)` or OS release.
- `secure_free_list_diagnostics_box.hako` owns M183 secure-list diagnostics. It
  may observe page-local free/local_free shape, but it must not implement
  encode/decode, cookies, or hardening policy.
- `secure_free_list_policy_box.hako` owns M184 secure-list encoded-next policy.
  It may encode/decode next indices and validate decoded capacity, but it must
  not source entropy, mutate page state, or claim hardening policy.
- Keep `birth(...)` for parameter-dependent initialization and ordering that
  cannot be expressed as a declaration-site default.
