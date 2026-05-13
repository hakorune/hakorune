# 293x-225 C208 Inline-Record Materialization / Escape Boundary

Status: Complete

## Purpose

C208 defines the compiler-visible boundary between future packed
`ArrayBox` inline-record auto-use and visible record values.

C207 can now emit `array_record_autouse_eligibility_plans`, but an eligible row
is not enough to use packed runtime storage. The next row must say which uses can
stay non-escaping and which uses require materialization.

## Decision

Decision: accepted.

Add metadata-only `array_record_materialization_boundary_plans` rows derived from
C207 eligible rows.

Allowed now:

- non-escaping direct indexed field reads
- metadata-only boundary rows for later C209 consumption
- stable fail-fast diagnostics for visible record materialization attempts

Rejected / closed now:

- public `ArrayBox.get(i)` returning a visible record value
- returned record elements
- host/backend boundary escape of record elements
- boxed fallback when materialization is missing
- production `ArrayStorage::InlineRecord` auto-use
- hako_alloc packed-store migration
- backend lowering

## Row Contract

For each C207 eligible row, C208 emits:

```text
boundary_kind = non_escaping_direct_field_reads_v0
direct_indexed_field_reads_allowed = true
visible_record_materialization_enabled = false
public_array_get_action = fail_fast_unmaterialized_record_value
returned_element_action = fail_fast_unmaterialized_record_value
host_backend_escape_action = fail_fast_unmaterialized_record_value
runtime_auto_use_enabled = false
```

Rejected C207 rows do not receive a C208 boundary row.

## Stop Lines

- Do not materialize record objects.
- Do not enable production packed ArrayBox auto-use.
- Do not migrate hako_alloc metadata stores.
- Do not add backend lowering or silent boxed fallback.
- Keep `ArrayInlineRecordProbe` / `ArrayInlineRecordPlanProbe` test-only.

## Acceptance

- MIR metadata includes `array_record_materialization_boundary_plans`.
- MIR JSON exposes the new rows.
- The C208 planner emits a boundary only for C207 eligible rows.
- Visible record materialization remains disabled with the stable inline-record
  unmaterialized diagnostic.
- The C208 guard stays local-run / index-listed and is not added to quick/dev
  gates.

## Verification

```bash
bash tools/checks/k2_wide_arraybox_inline_record_materialization_boundary_guard.sh
cargo test -q mir::array_record_materialization_boundary
cargo test -q runner::mir_json_emit::tests::decl_values::collect_array_record_materialization_boundary_plan_values_preserves_stop_line
```
