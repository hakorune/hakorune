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
- `purge_page_source_unreserve_adapter_box.hako` owns MIMAP-033A page-source
  unreserve adapter.
- `abandoned_reclaim_inventory_box.hako` owns M213 abandoned/reclaim inventory.

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
