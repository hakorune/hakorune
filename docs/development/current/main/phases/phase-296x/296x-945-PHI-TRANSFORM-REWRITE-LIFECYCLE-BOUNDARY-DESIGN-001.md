# 296x-945 PHI-TRANSFORM-REWRITE-LIFECYCLE-BOUNDARY-DESIGN-001

Status: Landed
Date: 2026-06-16
Scope: BoxShape-only PHI transform boundary design.

## Purpose

Classify remaining JoinIR merge PHI sites that transform an existing PHI
instruction rather than defining a new PHI.

These sites should not call `define_phi_final` or `define_phi_batch_prepend`.
They preserve an existing PHI `dst` and `type_hint`, and only rewrite block IDs
or remapped values as part of an instruction transform.

## Target Sites

```text
src/mir/builder/control_flow/joinir/merge/phi_block_remapper.rs
src/mir/builder/control_flow/joinir/merge/rewriter/stages/plan/instruction_rewrite.rs
```

## Decision

Create a thin PHI transform owner for existing-PHI rewrites.

```text
new_phi_definition_owner=phi_lifecycle
existing_phi_transform_owner=phi_block_remapper
json_import_owner=json_import_boundary
test_fixture_phi_owner=test_only
```

`phi_block_remapper` remains the transform owner, but its API should make the
boundary explicit:

```text
remap_existing_phi_block_ids(...)
```

`instruction_rewrite` should call that named transform boundary instead of a
generic `remap_phi_instruction` constructor.

## Contract

```text
output_contract=phi_transform_rewrite_boundary_design_v0
transform_sites_define_new_phi=0
transform_sites_preserve_dst=1
transform_sites_preserve_type_hint=1
transform_sites_rewrite_block_ids_only=1
builder_lifecycle_define_api_required=0
summary=ok
```

## Stop Line

```text
do_not_route_existing_phi_transform_through_define_phi_final=1
do_not_mix_json_import_boundary=1
do_not_touch_test_fixture_phi_builders=1
```

## Next

`PHI-TRANSFORM-REWRITE-BOUNDARY-RENAME-001`

