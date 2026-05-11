# lang/src/hako_alloc/memory — Hako Alloc Memory Policy Plane

Scope
- Policy-plane helpers for the `hako_alloc` layer live here.
- This subdir hosts the first moved helpers from the historical `runtime/memory/` path.
- Future allocator policy helpers should follow the same root.

Current modules
- `allocator_facade_box.hako`
- `arc_box.hako`
- `layout_box.hako`
- `page_box.hako`
- `page_heap_box.hako`
- `page_queue_box.hako`
- `page_source_policy_box.hako`
- `refcell_box.hako`
- `remote_free_policy_box.hako`
- `size_class_box.hako`

Syntax/style contract
- New allocator state boxes should use Unified Members stored fields:
  `field`, `field: Type`, or `field: Type = expr`.
- Use stored field initializers for fixed defaults and owner construction.
  Initializers are evaluated per construction, so `new ArrayBox()` defaults are
  not shared between instances.
- Keep numeric allocator state on `i64` annotations for now. `usize` is
  accepted as annotation metadata by the language surface, but exact
  pointer-sized unsigned semantics are not live yet.
- Keep `birth(...)` for parameter-dependent initialization and ordering that
  cannot be expressed as a declaration-site default.
