# 293x-487 USERBOX-ROUTE-SPLIT-003 Materialization Owner

Status: landed
Date: 2026-05-16

## Decision

`USERBOX-ROUTE-SPLIT-003` is the BoxShape cleanup selected by
`USERBOX-ROUTE-SPLIT-002`.

After `USERBOX-ROUTE-SPLIT-001`, module convergence is owned by
`convergence.rs`. The next narrow cut is to move function-level route row
materialization out of `user_box_method_route_plan.rs` and into a dedicated
`materialization.rs` owner.

## Scope

- Add `src/mir/user_box_method_route_plan/materialization.rs`.
- Move only `refresh_function_user_box_method_routes_with_context()` and the
  helper state needed to build `UserBoxMethodRoute` rows.
- Keep `refresh_function_user_box_method_routes()` and
  `refresh_module_user_box_method_routes()` as public facades.
- Keep target collection, return-shape inference, origin inference, and publish
  passes unchanged.

## Stop Lines

- Do not add, remove, or rename accepted user-box method route shapes.
- Do not change `UserBoxMethodRoute` JSON-facing fields or reason vocabulary.
- Do not move target collection in this row.
- Do not change backend lowering, C shim behavior, pure-first preflight, or
  allocator behavior.
- Do not activate provider hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `UBMAT.1` | Document row and owner. | Current points to this card. | no code before docs |
| `UBMAT.2` | Extract function-level materialization into `materialization.rs`. | Public facades remain unchanged. | no route semantics change |
| `UBMAT.3` | Keep focused tests green. | user-box route tests pass. | no new route acceptance |
| `UBMAT.4` | Closeout docs and advance current. | Required evidence is green. | no allocator/provider activation |

## Required Evidence

```text
cargo test -q user_box_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row is landed.

Implementation:

- Added `src/mir/user_box_method_route_plan/materialization.rs`.
- Moved function-level route row materialization out of
  `src/mir/user_box_method_route_plan.rs`.
- Kept the public refresh facades and accepted route shapes unchanged.
- Kept target collection, return-shape inference, origin inference, publish
  passes, backend lowering, and allocator behavior unchanged.

Evidence:

```text
cargo test -q user_box_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Next:

```text
USERBOX-ROUTE-SPLIT-004:
  post-materialization row selection
```
