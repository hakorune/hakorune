# 296x-946 PHI-TRANSFORM-REWRITE-BOUNDARY-RENAME-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI transform boundary.

## Purpose

Make the existing-PHI transform boundary explicit.

This row does not change PHI semantics. It renames the transform entry point so
callers do not read it as a new PHI definition lifecycle path.

## Changes

- Rename `remap_phi_instruction` to `remap_existing_phi_block_ids`.
- Keep `dst`, `type_hint`, and incoming `ValueId`s preserved.
- Keep JSON import and test fixture PHI construction out of scope.

## Contract

```text
output_contract=phi_transform_rewrite_boundary_rename_v0
existing_phi_transform_boundary_named=1
remap_existing_phi_block_ids_present=1
remap_phi_instruction_present=0
transform_sites_define_new_phi=0
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

