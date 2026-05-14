---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: PACKED-002 source PackedArray non-escaping auto-use pilot metadata.
Related:
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
  - docs/development/current/main/design/array-result-option-canonical-surface-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-324-PACKED-002-SOURCE-PACKED-ARRAY-AUTOUSE-PILOT.md
  - docs/reference/language/EBNF.md
---

# Source PackedArray Auto-Use Pilot SSOT

## Decision

`PACKED-002` connects explicit source declarations such as:

```hako
box Store {
    metas: PackedArray<Meta>
}
```

to the existing C209 non-escaping packed ArrayBox pilot metadata when `Meta` is
already an eligible integer-lane record layout.

The new MIR row is:

```text
source_packed_array_autouse_pilot_plans
```

It records the source declaration site and consumes the existing
`array_record_packed_autouse_pilot_plans` row. It does not create production
runtime auto-use by itself.

## Scope

The row records:

```text
owner_box
field_name
declared_type = PackedArray<Record>
record_name
layout_id
pilot_kind
source_boundary_kind
source_declared_packed = true
direct_indexed_field_reads_enabled = true
private_runtime_storage_enabled = true
public_array_get_materialization_enabled = false
backend_lowering_enabled = false
boxed_fallback_enabled = false
```

## Stage Split

Stage1/MIR owns:

```text
source PackedArray<T> declaration-site metadata
link to existing C209 non-escaping packed ArrayBox pilot rows
MIR JSON exposure for the metadata row
```

Stage1/MIR does not own here:

```text
public ArrayBox.get record materialization
backend lowering
hako_alloc live migration
implicit PackedArray -> Array fallback
local variable PackedArray planning
record element write-through
```

## Stop Lines

```text
no boxed fallback
no backend_lowering_enabled = true
no public_array_get_materialization_enabled = true
no hako_alloc migration
no source mention of private compiler/runtime seams
```

## Retire Condition

This metadata row can retire when source `PackedArray<T>` declarations are owned
by the final CorePlan storage planner and the planner can prove the same site,
layout, non-escape boundary, and backend fail-fast contract without a separate
pilot row.
