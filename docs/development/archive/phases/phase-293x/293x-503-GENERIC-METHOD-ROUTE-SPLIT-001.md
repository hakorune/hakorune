# 293x-503 GENERIC-METHOD-ROUTE-SPLIT-001

Status: landed
Date: 2026-05-17

## Decision

`GENERIC-METHOD-ROUTE-SPLIT-001` is a BoxShape cleanup for MIR generic method
route planning. It moves collection read route matching out of the root
`generic_method_route_plan.rs` facade into a dedicated owner module.

## Scope

- Add `src/mir/generic_method_route_plan/collection_read_routes.rs`.
- Move collection read route matchers for `has`, `get`, `length`/`len`, and
  `keys`.
- Move only the local helper policy needed by those read matchers.
- Keep `refresh_function_generic_method_routes_with_context` as the root
  orchestration facade.

## Stop Lines

- Do not change accepted generic method route behavior.
- Do not change route kind, route proof, route id, value demand, return shape,
  publication policy, or core method carrier spelling.
- Do not touch write routes, scalar map proof, typed-object origin inference,
  MIR JSON route emission, allocator behavior, provider activation, hooks, host
  allocator replacement, or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GMR.1` | Add the `collection_read_routes` owner module and wire it from the root facade. | module compiles. | no route behavior change |
| `GMR.2` | Move `has`/`get` route matching and their helper policy. | route tests remain green. | no proof/value-demand changes |
| `GMR.3` | Move `length`/`len` and `keys` route matching. | route tests remain green. | no accepted method changes |
| `GMR.4` | Verify and close out. | required evidence is green. | no adjacent cleanup |

## Required Evidence

```text
cargo test -q generic_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when collection read route matching has a dedicated
`collection_read_routes` owner and existing generic method route behavior is
unchanged.

## Result

Landed:

- Added `src/mir/generic_method_route_plan/collection_read_routes.rs`.
- Moved collection read route matchers for `has`, `get`, `length`/`len`, and
  `keys`.
- Moved only the local helper policy used by those read route matchers.
- Updated the generic method route plan README with the new owner.

No accepted generic method route behavior, route kind, route proof, route id,
value demand, return shape, publication policy, core method carrier spelling,
write route behavior, scalar map proof, typed-object origin inference, MIR JSON
route emission, allocator behavior, provider activation, hooks, host allocator
replacement, or `#[global_allocator]` behavior changed.

## Evidence

```text
cargo test -q generic_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
