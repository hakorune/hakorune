# 296x-941 PHI-HEADER-BATCH-PREPEND-DESIGN-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI lifecycle design.

## Purpose

Define the lifecycle boundary for loop-header PHIs that must be inserted as an
ordered batch at the beginning of a header block.

This is not the same shape as the simple single-PHI lifecycle path. The loop
header builder validates all carriers, materializes all predecessor-specific
inputs, then prepends the entire PHI batch and matching spans before the
existing header body.

## Decision

Add a lifecycle-owned batch/prepend API.

```text
PhiBatchItem:
  dst
  inputs
  type_hint
  span
  item_tag

define_phi_batch_prepend:
  materialize every item first
  sort each item input list by predecessor block id
  record caller metadata per dst
  prepend instructions and spans atomically
```

Repeated calls to `define_phi_final` are rejected for this row because they
would allow partial insertion if a later PHI input fails to materialize.

## Invariants

```text
output_contract=phi_header_batch_prepend_design_v0
phi_batch_api_owner=phi_lifecycle
low_level_batch_prepend_owner=cf_common
loop_header_phi_order=current_carrier_phis_iteration_order
batch_insertion_atomic=1
instruction_span_lockstep_required=1
input_materialization_owner=phi_lifecycle
caller_metadata_owner=phi_lifecycle
carrier_order_semantics_changed=0
accepted_shape_added=0
summary=ok
```

## Stop Line

```text
do_not_switch_to_carrier_order=1
do_not_migrate_join_ir_vm_bridge=1
do_not_migrate_json_v0_bridge=1
do_not_touch_test_fixture_phi_builders=1
```

