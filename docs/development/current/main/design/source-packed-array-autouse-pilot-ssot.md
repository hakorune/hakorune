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

## PACKED-003 Direct-read Consumption

PACKED-003 consumes `source_packed_array_autouse_pilot_plans` into:

```text
source_packed_array_direct_read_consumption_plans
```

The row is per record field and records:

```text
owner_box
source_field
declared_type
record_name
layout_id
record_field
record_field_slot
storage
read_kind = source_packed_record_field_direct_read_v0
source_declared_packed = true
direct_indexed_field_reads_consumed = true
private_runtime_storage_consumed = true
public_array_get_materialization_enabled = false
backend_lowering_enabled = false
boxed_fallback_enabled = false
```

This is still metadata-only. Runtime/backend lowering, public record
materialization, hako_alloc migration, and boxed fallback remain disabled.

## PACKED-004 Backend Fail-fast Hardening

PACKED-004 connects source direct-read consumption plans to the existing packed
record backend capability gate.

Current rows keep:

```text
backend_lowering_enabled = false
```

so no backend is required today. If a future row enables backend lowering on a
source PackedArray direct-read route, unsupported backends must fail with the
existing packed record backend tag instead of falling back:

```text
[freeze:backend][array-record/packed-route-unsupported]
```

## Stage Split

Stage1/MIR owns:

```text
source PackedArray<T> declaration-site metadata
link to existing C209 non-escaping packed ArrayBox pilot rows
MIR JSON exposure for the metadata row
PACKED-003 direct-read consumption metadata
PACKED-004 backend fail-fast hardening for future required source routes
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
