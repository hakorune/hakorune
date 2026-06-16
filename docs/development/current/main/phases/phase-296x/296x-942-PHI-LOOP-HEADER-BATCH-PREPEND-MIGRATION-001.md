# 296x-942 PHI-LOOP-HEADER-BATCH-PREPEND-MIGRATION-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI lifecycle migration.

## Purpose

Migrate only `loop_header_phi_builder::finalize()` from direct PHI construction
to the lifecycle-owned batch/prepend API.

## Changes

- Add `PhiBatchItem` and `define_phi_batch_prepend` in `phi_lifecycle`.
- Add `insert_phi_batch_prepend_spanned_with_type_hint` in `cf_common`.
- Change `loop_header_phi_builder::finalize()` to build lifecycle batch items
  instead of constructing `MirInstruction::Phi` directly.
- Keep carrier iteration order unchanged.
- Keep bridge/import/test PHI builders out of scope.

## Contract

```text
output_contract=phi_loop_header_batch_prepend_migration_v0
loop_header_finalize_direct_phi_construction=0
loop_header_finalize_uses_phi_lifecycle_batch=1
batch_insertion_atomic=1
instruction_span_lockstep_preserved=1
type_hint_preserved=1
carrier_order_semantics_changed=0
guard_updated=1
summary=ok
```

## Proof

```bash
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

