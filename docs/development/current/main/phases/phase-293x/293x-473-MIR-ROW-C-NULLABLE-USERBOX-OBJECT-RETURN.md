# 293x-473 MIR-ROW-C Nullable User-Box Object Return

Status: landed
Date: 2026-05-16

## Decision

`MIR-ROW-C` is the compiler acceptance sidecar selected by `MIMAP-039B`.

Same-module user-box route metadata must accept a method that returns a
nullable selected object through a loop-carried local:

```text
local selected = null
loop(...) {
  local item = items.get(i)
  if ... {
    selected = item
    break
  }
}
return selected
```

The route must publish an object-handle contract so caller-side field reads and
method calls do not fall back to `RuntimeDataBox` or lose typed owner facts.

## Scope

- Extend `user_box_method_route_plan` return inference for `null | same-box`
  object values through `select` and loop-carried `phi`.
- Publish `return_shape=object_handle` and `target_result_box_name=<Box>` for
  the accepted route.
- Add an allocator-neutral proof app that uses the returned object for both a
  field read and a same-module method call.
- Add a focused guard that checks MIR JSON metadata and pure-first EXE parity.

## Stop Lines

- Do not rewrite hako_alloc page queue source in this row.
- Do not add allocator behavior, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts, app-name classifiers, or
  box-name special cases.
- Do not broaden ArrayBox semantics beyond the existing route metadata.
- Do not make `null | scalar` a scalar return contract.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `MIR-C.1` | Add SSOT / proof app / guard shell. | The accepted shape and forbidden shortcuts are documented. | no compiler behavior yet |
| `MIR-C.2` | Add route-plan unit fixture. | Nullable loop-carried object return is red before the implementation and green after. | no allocator fixture dependency |
| `MIR-C.3` | Extend return inference. | `null | same-box` through select/phi yields object handle and concrete box. | no mixed-box silent acceptance |
| `MIR-C.4` | Run proof bundle. | Guard, focused cargo test, current pointer, and quick gate pass. | no source workaround |

## Required Evidence

```text
bash tools/checks/k2_wide_userbox_nullable_loop_return_guard.sh
cargo test -q user_box_method_route_plan::tests::object_handles::refresh_module_user_box_method_routes_accepts_loop_carried_nullable_object_return
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

This row closes when nullable selected-object returns are represented in
same-module route metadata and pure-first EXE can consume the returned object
without backend name matching.

## Landed Implementation

```text
owner:
  src/mir/user_box_method_route_plan.rs
  src/mir/user_box_method_route_plan/return_shape.rs
proof app:
  apps/userbox-nullable-loop-return-proof/main.hako
guard:
  tools/checks/k2_wide_userbox_nullable_loop_return_guard.sh
unit fixture:
  user_box_method_route_plan::tests::object_handles::refresh_module_user_box_method_routes_accepts_loop_carried_nullable_object_return
```

Closeout:

```text
current blocker moves to MIMAP-039C post-nullable-object-return row selection.
```
