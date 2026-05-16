# 293x-485 USERBOX-ROUTE-SPLIT-001 Fixpoint Orchestration

Status: landed
Date: 2026-05-16

## Decision

`USERBOX-ROUTE-SPLIT-001` is the BoxShape cleanup selected by
`VMHAKO-EXTERN-SPEC-002`.

`src/mir/user_box_method_route_plan.rs` already has separate owners for origin
inference, return-shape inference, and value-type publish. The remaining
module-level `refresh_module_user_box_method_routes()` function still owns the
fixed-point convergence loop, target collection timing, route materialization,
and publish pass order in one large block.

This row extracts that convergence orchestration into a dedicated submodule
without changing route semantics.

## Scope

- Add a `convergence` owner under `src/mir/user_box_method_route_plan/`.
- Keep the public facade `refresh_module_user_box_method_routes()` in
  `user_box_method_route_plan.rs`.
- Move only the module-level fixed-point loop and its changed-state accounting.
- Preserve the current inference/publish order exactly.
- Keep route materialization, target fact shape, return inference, and publish
  helpers behavior-preserving.

## Stop Lines

- Do not add, remove, or rename accepted user-box method route shapes.
- Do not change `UserBoxMethodRoute` JSON-facing fields or reason vocabulary.
- Do not change backend lowering, C shim behavior, pure-first preflight, or
  allocator behavior.
- Do not combine with expr dispatcher or record_values cleanup.
- Do not activate provider hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `UBSPLIT.1` | Document row and owner. | Current points to this card. | no code before docs |
| `UBSPLIT.2` | Extract convergence loop into `convergence.rs`. | Public facade remains unchanged. | no semantics change |
| `UBSPLIT.3` | Keep focused tests green. | user-box route tests pass. | no new route acceptance |
| `UBSPLIT.4` | Closeout docs and advance current. | Required evidence is green. | no allocator/provider activation |

## Required Evidence

```text
cargo test -q user_box_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when convergence orchestration has one submodule owner, existing
route tests remain green, and current moves to the next row selection card.

## Landed Implementation

```text
owners:
  src/mir/user_box_method_route_plan.rs
  src/mir/user_box_method_route_plan/convergence.rs
```

The public `refresh_module_user_box_method_routes()` facade remains in
`user_box_method_route_plan.rs`. The fixed-point loop, iteration budget,
route snapshotting, route materialization order, and convergence check now live
in `convergence.rs`.

The inference/publish order is unchanged:

```text
collect targets without field-return hints
infer param origins
infer field origins
refine param/field origins
publish param and field-get value types
collect targets with field-return hints
materialize routes
publish route/generic/propagated value types
check route metadata convergence
```

Evidence:

```text
cargo test -q user_box_method_route_plan
```

Closeout:

```text
current blocker moves to USERBOX-ROUTE-SPLIT-002 post-fixpoint row selection.
```
