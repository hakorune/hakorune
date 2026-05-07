# 293x-012 Typed Object EXE Plan For General UserBox NewBox

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: define the owner boundary for lowering general user-box `newbox` in
  direct EXE without adding broad user-box semantics to the C shim.

## Current Blocker

Real-app VM coverage is green, but direct EXE still reaches `ny-llvmc`
pure-first and stops at:

```text
first_op=newbox
unsupported pure shape for current backend recipe
```

`NewBox` already exists as MIR and JSON vocabulary. The missing piece is the
EXE-facing typed object representation:

```text
NewBox opcode: present
user_box_decls / user_box_field_decls: present
typed object layout / lowering contract: missing
```

## Decision

- Do not add broad general user-box `newbox` switching to the C shim.
- MIR owns `typed_object_plans[]` as the layout truth.
- BackendRecipeBox / lowering metadata owns route facts for `NewBox`,
  `field_set`, `field_get`, and later user-box method calls.
- Stage0 / `.inc` reads `typed_object_plans[]` and route facts only. It must not
  rediscover field slots from raw declarations or app names.
- Runtime owns opaque typed object allocation and slot field helpers.
- VM remains the reference execution path and proof oracle. EXE lowering must
  not clone `InstanceBox` semantics.
- The existing Point-only `userbox_local_scalar_seed_route` remains an exact
  seed proof. It must not be broadened by adding app-name or field-name
  switches; the general path replaces it with typed object plans.

## MIR-Owned Plan

The MIR module already copies `user_box_decls` and `user_box_field_decls` during
finalize. The next implementation slice turns those declarations into a
backend-readable plan:

```json
{
  "typed_object_plans": [
    {
      "box_name": "Point",
      "type_id": 1,
      "layout_kind": "runtime_slot_object_v0",
      "fields": [
        {
          "name": "x",
          "slot": 0,
          "declared_type": "IntegerBox",
          "storage": "i64",
          "weak": false
        },
        {
          "name": "y",
          "slot": 1,
          "declared_type": "IntegerBox",
          "storage": "i64",
          "weak": false
        }
      ]
    }
  ]
}
```

Contract:

- `typed_object_plans[]` is the layout truth.
- Field order and slot assignment are MIR-owned.
- Unsupported storage must fail fast with a stable tag:
  `[typed-object-plan] unsupported field storage: Box.field`.
- Reference metadata docs move from provisional to concrete when 293x-013 adds
  the first emitted plan.

## Lowering Contract

Initial lowering uses runtime helpers, not inline LLVM struct layout:

```text
%p = newbox Point()
  -> call i64 @nyash.object.new_typed_h(i64 type_id)

field_set %p.x = %v
  -> call void @nyash.object.field_set_hii(i64 %p, i64 slot, i64 %v)

%v = field_get %p.x
  -> call i64 @nyash.object.field_get_hii(i64 %p, i64 slot)
```

Initial route facts:

```text
NewBox Point:
  route_id = object.new.typed
  tier = ColdRuntime
  emit_kind = RuntimeCall

FieldSet Point.x:
  route_id = object.field.set
  tier = ColdRuntime

FieldGet Point.x:
  route_id = object.field.get
  tier = ColdRuntime
```

`HotInline` layout is explicitly deferred until escape/publication/lifecycle
proofs exist.

## Birth Contract

`NewBox` allocates only. `birth` remains a normal same-module call.

```text
new Point(1, 2)
  MIR:
    %p = newbox Point
    call Point.birth/2(%p, 1, 2)

  EXE:
    %p = object.new.typed(Point)
    call same-module Point.birth/2(%p, 1, 2)
```

Do not give `NewBox.args` constructor semantics in Stage0. Constructor inlining
is deferred.

## First Accepted Shape

Accepted first:

- non-weak user box
- fixed declared fields
- fields proven as `i64` storage
- allocation plus `field_set` / `field_get`
- no dynamic field add
- no method call requirement
- no inline layout

Unsupported first:

- weak fields
- mixed or unknown field storage
- generic field storage
- closure fields
- plugin object fields without an explicit handle route
- object publication optimization
- constructor inline

## Fixture Order

- `293x-013`: i64 field-only user-box `newbox` + `field_set` / `field_get`
  EXE fixture.
- `293x-014`: `birth` as a normal same-module function call after allocation.
- `293x-015`: user-box instance method call, e.g. `p.sum()`.
- `293x-016`: handle fields for String / Array / Map when the value type is
  explicit.
- `293x-017`: allocator-backed object allocation route.
- `293x-018`: BoxTorrent allocator-backed store EXE parity.
- `293x-019`: JSON stream aggregator EXE parity.

Minimal 293x-013 fixture:

```hako
box Point {
  x
  y
}

static box Main {
  main() {
    local p = new Point()
    p.x = 10
    p.y = 20
    return p.x + p.y
  }
}
```

## Verification

Docs-only card:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
