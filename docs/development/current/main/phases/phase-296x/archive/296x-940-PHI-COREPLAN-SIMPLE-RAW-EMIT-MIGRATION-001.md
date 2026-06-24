# 296x-940 PHI-COREPLAN-SIMPLE-RAW-EMIT-MIGRATION-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI lifecycle migration.

## Purpose

Move simple CorePlan raw PHI emission sites onto the PHI lifecycle facade.

This row targets straightforward single-PHI emission sites only. It
intentionally does not migrate ordered/batched loop-header PHI insertion or
bridge/import code.

## Changes

- Add a typed PHI insert helper in `cf_common` so lifecycle callers can
  preserve `type_hint`.
- Add `define_phi_final_with_type_hint` in `phi_lifecycle`.
- Migrate CorePlan simple raw emit sites:
  - `control_flow/plan/features/if_join.rs`
  - `control_flow/plan/lowerer/effect_emission.rs`

## Contract

```text
output_contract=phi_coreplan_simple_raw_emit_migration_v0
coreplan_simple_raw_emit_migrated=1
if_join_raw_phi_emit=0
effect_emission_select_phi_raw_emit=0
type_hint_preserved=1
phi_lifecycle_owner_preserved=1
accepted_shape_added=0
fallback_route_added=0
summary=ok
```

## Stop Line

```text
loop_header_batch_prepend_migration_started=0
join_ir_vm_bridge_migration_started=0
json_v0_bridge_migration_started=0
test_fixture_phi_rewrite_started=0
```

## Next

`PHI-HEADER-BATCH-PREPEND-DESIGN-001`

The next row should design how ordered/batched PHI insertion is represented
before touching `loop_header_phi_builder.rs`.

## Proof

```bash
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

