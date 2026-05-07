# 293x-014 Typed Object Untyped Handle Field EXE Route

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: expand the MIR-owned typed-object EXE route from declared i64 fields
  to real-app field surfaces: `init { ... }` names, inferred i64/handle
  storage, and observed empty user boxes.

## Decision

- `typed_object_plans[]` remains the layout truth. The `.inc` backend still
  reads plans only and does not rediscover slots from raw declarations.
- Instance box registration copies all source-owned field surfaces into MIR
  metadata: explicit field declarations, plain field names, and `init` field
  names.
- Untyped field storage is inferred from MIR observations. A field is accepted
  only when observations prove one consistent runtime storage kind.
- `i64` and `handle` storage use the same opaque runtime slot object ABI.
- Empty user boxes produce zero-field plans only when a `newbox` is observed.
  This avoids creating unused object layout rows for every declaration.

## Accepted Shape

Accepted in this card:

- non-weak user box fields from declared fields or `init { ... }`
- declared primitive storage: `IntegerBox`, `Integer`, `i64`, `BoolBox`,
  `Bool`, `bool`, `i1`
- declared handle storage: `StringBox`, `String`, `ArrayBox`, `MapBox`, and
  user-box handles
- untyped fields whose `FieldSet` observations infer `i64` or `handle`
- null/void writes as non-constraining observations
- `FieldGet`-to-`FieldSet` storage propagation for already inferred fields
- observed zero-field user-box allocation

Rejected / deferred:

- weak fields
- conflicting mixed storage for the same field
- unknown storage with no concrete observation
- float field storage
- dynamic field addition outside the declared source surface
- `birth` route ownership
- user-box instance method calls
- inline object layout

## Runtime ABI

The route continues to use the opaque typed slot helpers introduced by
293x-013:

```text
i64  nyash.object.new_typed_hi(i64 type_id, i64 field_count)
i64  nyash.object.new_typed_h(i64 type_id)
i64  nyash.object.field_get_hii(i64 object, i64 slot)
void nyash.object.field_set_hii(i64 object, i64 slot, i64 value)
```

Handle fields are stored as runtime handles in `i64` slots. The backend does
not inline handle layout or copy VM `InstanceBox` semantics.

## Fixture

```hako
box Holder {
  init { count, items }
}

static box Main {
  main() {
    local holder = new Holder()
    holder.count = 7

    local items = new ArrayBox()
    holder.items = items

    return holder.count
  }
}
```

Path:

- `apps/typed-object-untyped-field-min/main.hako`
- `tools/smokes/v2/profiles/integration/apps/typed_object_untyped_field_min_exe.sh`

## Boundary Movement

The real-app EXE boundary probe no longer pins `first_op=newbox`. General
user-box allocation and field slots now lower through `TypedObjectPlan`.

The remaining direct EXE blocker is the call seam:

```text
first_op=mir_call
reason=mir_call_no_route
```

This is still not EXE parity. `birth` and user-box instance method calls remain
separate route work.

## Gates

```bash
cargo test --release typed_object_plan --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_newbox_min_exe.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_untyped_field_min_exe.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Expand the pure-first call route for `birth` as a normal same-module call, then
handle user-box instance method calls. Keep app code idiomatic and keep compiler
blockers visible at the boundary probe.
