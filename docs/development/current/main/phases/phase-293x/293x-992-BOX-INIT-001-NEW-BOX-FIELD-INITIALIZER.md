# 293x-992 BOX-INIT-001 New-Box Field Initializer

Status: landed
Date: 2026-05-21

## Purpose

Accept a narrow construction-site field initializer surface:

```hako
local result = new Report {
    accepted: fields.accepted
    reason: fields.reason
}
```

This removes repetitive report-copy boilerplate without introducing named
constructor arguments, wildcard copy, record materialization, or a backend
route.

## Decision

`new Box { field: expr }` and `new Box(args...) { field: expr }` are accepted
as explicit sugar for:

```text
NewBox(Box, args)
FieldSet(result, field, expr)...
```

## Scope

- Extend `ASTNode::New` with `field_initializers`.
- Parse explicit `field: expr` entries after `new Box` or `new Box(args...)`.
- Transport the field initializer list through AST JSON / Program JSON v0.
- Lower the initializer block in the ordinary MIR builder as NewBox followed by
  FieldSet instructions.
- Reject duplicate initializer fields.
- Reject unknown initializer fields on known user-defined boxes.

## Stop Lines

- No shorthand copy such as `fields.accepted`.
- No wildcard copy such as `fields.*` or `...fields`.
- No named constructor argument binding.
- No constructor overload.
- No runtime record object or record materialization.
- No backend route, `.inc` matcher, or owner-name classification.
- CorePlan/JoinIR normalization may fail-fast until a separate row accepts this
  shape there.

## Evidence

```bash
cargo check -q --bin hakorune
cargo test -q parser_box_new_field_initializer_surface
cargo test -q mir_box_new_field_initializer
bash tools/checks/k2_wide_box_new_field_initializer_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

BOX-INIT-002 selects whether to add explicit same-name shorthand
(`new Report { fields.accepted }`) or return to the mimalloc provider lane.
