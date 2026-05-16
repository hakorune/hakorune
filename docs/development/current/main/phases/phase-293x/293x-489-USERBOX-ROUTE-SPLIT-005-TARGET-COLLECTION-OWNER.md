# 293x-489 USERBOX-ROUTE-SPLIT-005 Target Collection Owner

Status: landed
Date: 2026-05-16

## Decision

`USERBOX-ROUTE-SPLIT-005` is the BoxShape cleanup selected by
`USERBOX-ROUTE-SPLIT-004`.

After `USERBOX-ROUTE-SPLIT-003`, function-level route materialization is owned
by `materialization.rs`. The next narrow cut is to move same-module method
target fact collection out of `user_box_method_route_plan.rs` and into a
dedicated `target_collection.rs` owner.

## Scope

- Add `src/mir/user_box_method_route_plan/target_collection.rs`.
- Move `UserBoxMethodTargetFacts`, `collect_method_targets()`,
  method-symbol helpers, and result-box-name inference helpers needed by target
  facts.
- Keep public refresh facades unchanged.
- Keep convergence, materialization, origin inference, return-shape inference,
  and publish passes behavior-preserving.

## Stop Lines

- Do not add, remove, or rename accepted user-box method route shapes.
- Do not change `UserBoxMethodRoute` JSON-facing fields or reason vocabulary.
- Do not change route materialization, backend lowering, C shim behavior,
  pure-first preflight, or allocator behavior.
- Do not activate provider hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `UBTGT.1` | Document row and owner. | Current points to this card. | no code before docs |
| `UBTGT.2` | Extract target fact collection into `target_collection.rs`. | Existing route tests pass. | no route semantics change |
| `UBTGT.3` | Keep focused tests green. | user-box route tests pass. | no new route acceptance |
| `UBTGT.4` | Closeout docs and advance current. | Required evidence is green. | no allocator/provider activation |

## Required Evidence

```text
cargo test -q user_box_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row is landed.

Implementation:

- Added `src/mir/user_box_method_route_plan/target_collection.rs`.
- Moved target fact collection, method-symbol helpers, and result-box-name
  inference helpers out of `src/mir/user_box_method_route_plan.rs`.
- Updated convergence, materialization, origin inference, and value-type publish
  modules to import the moved helpers from their owning modules.
- Kept public refresh facades, accepted route shapes, route reason vocabulary,
  backend lowering, and allocator behavior unchanged.

Evidence:

```text
cargo test -q user_box_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Next:

```text
USERBOX-ROUTE-SPLIT-006:
  post-target-collection row selection
```
