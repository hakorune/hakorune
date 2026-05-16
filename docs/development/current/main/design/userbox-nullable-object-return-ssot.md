# UserBox Nullable Object Return

Status: SSOT
Date: 2026-05-16
Scope: same-module user-box route metadata for methods that return
`null | same-box-object` through loop-carried locals.
Related:
- docs/development/current/main/phases/phase-293x/293x-473-MIR-ROW-C-NULLABLE-USERBOX-OBJECT-RETURN.md
- docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
- src/mir/user_box_method_route_plan/README.md

## Decision

Same-module user-box route inference may treat a return value as a nullable
object handle when every concrete non-null return arm resolves to the same user
box and all remaining arms are `null`, `void` sentinel, or a self-cycle in the
same loop-carried value.

The produced route metadata is:

```text
return_shape: object_handle
value_demand: runtime_i64_or_handle
target_result_box_name: <the concrete user box>
proof: typed_user_box_method_same_module
```

## Accepted Shape

```text
local selected = null
loop(...) {
  local item = items.get(i)
  if condition {
    selected = item
    break
  }
}
return selected
```

The shape may lower through MIR `select` and `phi`, including a self-cycle from
the loop-carried selected local.

## Rejection Rules

- Mixed concrete boxes do not produce a route.
- Unknown non-null arms do not produce a route.
- `null | scalar` does not become a scalar route.
- Backend `.inc` code must not classify this shape by app, box, or method name.
- Allocator source must not be rewritten to hide this compiler acceptance gap.

## Consumer Contract

The caller may use the returned value for typed field reads and same-module
method calls only because MIR metadata carries the concrete
`target_result_box_name`. Backend consumers read the route row; they do not
re-infer the source pattern.
