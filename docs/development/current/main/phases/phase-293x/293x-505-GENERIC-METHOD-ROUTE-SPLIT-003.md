# 293x-505 GENERIC-METHOD-ROUTE-SPLIT-003

Status: selected current
Date: 2026-05-17

## Decision

`GENERIC-METHOD-ROUTE-SPLIT-003` is a BoxShape cleanup for MIR generic method
route planning. It moves string route matching out of the root
`generic_method_route_plan.rs` facade into a dedicated owner module.

## Scope

- Add `src/mir/generic_method_route_plan/string_routes.rs`.
- Move string route matchers for `substring`, `indexOf`, `lastIndexOf`, and
  `contains`.
- Move only the local helper policy needed by those string matchers.
- Keep `refresh_function_generic_method_routes_with_context` as the root
  orchestration facade.

## Stop Lines

- Do not change accepted generic method route behavior.
- Do not change route kind, route proof, route id, value demand, return shape,
  publication policy, or core method carrier spelling.
- Do not touch collection read routes, write routes, scalar map proof,
  typed-object origin inference, MIR JSON route emission, allocator behavior,
  provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GMR2.1` | Add the `string_routes` owner module and wire it from the root facade. | module compiles. | no route behavior change |
| `GMR2.2` | Move `substring` and index route matching. | route tests remain green. | no proof/value-demand changes |
| `GMR2.3` | Move `contains` route matching. | route tests remain green. | no accepted method changes |
| `GMR2.4` | Verify and close out. | required evidence is green. | no adjacent cleanup |

## Required Evidence

```text
cargo test -q generic_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row closes when string route matching has a dedicated `string_routes`
owner and existing generic method route behavior is unchanged.
