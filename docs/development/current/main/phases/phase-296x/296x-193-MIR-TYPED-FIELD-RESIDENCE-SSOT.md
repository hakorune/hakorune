---
Status: Landed
Date: 2026-05-28
Scope: define the MIR typed-field residence contract after the runtime storage fast lane keeper.
Blocker: MIR-TYPED-FIELD-RESIDENCE-SSOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-192-TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT.md
---

# 296x-193 MIR Typed Field Residence SSOT

## Purpose

Define the next C-parity seam after row192 proved that removing the
typed-object storage lock/global-slab boundary is a keeper. Runtime fast lane
is a large improvement, but hot field operations still cross exported helper
symbols. The next long-term owner is a MIR field residence plan that can keep
selected scalar fields resident inside a method and write back only at safe
boundaries.

## Decision

```text
Decision: provisional

MIR typed-field residence is the next design owner.
Runtime SingleThreadExactStore remains a diagnostic exact-EXE fast lane.
Field helper ABI stays as fallback.
No transform is implemented in this SSOT row.
```

## Planned FieldResidencePlan Shape

```text
FieldResidencePlan:
  function
  receiver_value
  receiver_box_type
  field_name
  slot
  storage_class
  residence_kind:
    - method_receiver_cache_writeback
    - local_newbox_residence
  init_policy:
    - helper_load_on_first_use
    - default_zero
  writeback_policy:
    - writeback_before_escape
    - writeback_on_return
    - no_writeback_readonly
  barriers:
    - unknown_call
    - receiver_escape
    - phi_merge
    - ret_receiver
    - dynamic_slot
    - weak_field
```

## Non-Goals

```text
- Do not transform MIR in this row.
- Do not remove typed-object helper ABI.
- Do not optimize ArrayBox here.
- Do not specialize by hako_alloc box or field names.
- Do not open provider activation, allocator replacement, hooks, globals, or
  winner claims.
```

## Acceptance

```text
mir_typed_field_residence_ssot=accepted
transform_open=0
helper_abi_fallback=1
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
