# 296x-939 PHI-HELPERS-LIFECYCLE-ROUTE-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI lifecycle migration.

## Purpose

Route the legacy `MirBuilder::insert_phi` helper family through the PHI
lifecycle owner without changing accepted source shapes.

The goal is to remove a convenient bypass seam before migrating higher-level
CorePlan PHI sites. This row does not change PHI placement policy, CFG shape,
or release behavior.

## Changes

- Add builder facade methods:
  - `define_current_block_phi_final`
  - `define_current_block_phi_final_with_type_hint`
- Route `src/mir/utils/phi_helpers.rs` helper entry points through the facade.
- Preserve `FreezeContract` diagnostics when no current block exists.
- Preserve existing helper metadata propagation for local values produced by
  the PHI.

## Contract

```text
output_contract=phi_helpers_lifecycle_route_v0
phi_helpers_insert_phi_uses_lifecycle_facade=1
phi_helpers_insert_phi_with_dst_uses_lifecycle_facade=1
phi_helpers_direct_cf_common_call_count=0
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
summary=ok
```

## Stop Line

```text
do_not_migrate_loop_header_phi_builder=1
do_not_touch_join_ir_vm_bridge=1
do_not_touch_json_v0_bridge=1
do_not_add_new_phi_semantics=1
```

## Proof

```bash
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
```

