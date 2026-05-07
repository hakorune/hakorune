# 293x-015 Typed UserBox Birth Same-Module EXE Route

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: lower the first typed user-box `birth` surface through a MIR-owned
  call route and same-module uniform ABI, without moving user-box semantics
  into the C shim.

## Decision

- `NewBox` remains allocation-only. Constructor semantics stay in the normal
  `Box.birth/N` method body.
- MIR owns the route proof through `user_box_method_routes[]` and
  `lowering_plan[]`; the `.inc` backend reads the plan and emits only direct
  same-module calls that the plan proves.
- The first accepted shape is intentionally conservative: `birth` body support
  is part of the MIR route contract. If the target body is outside the current
  same-module emitter surface, the route is recorded as `Unsupported`.
- The backend does not infer method symbols from raw names and does not clone
  VM `InstanceBox` semantics.

## Accepted Shape

Accepted in this card:

- known receiver user-box `birth` calls
- typed-object plan exists for the receiver box
- same-module `Box.birth/N` target exists
- target arity is receiver plus explicit args
- target body is a single block ending in `Return`
- target body instructions limited to:
  `Const`, `Copy`, typed `NewBox`, `FieldSet`, `FieldGet`, and simple integer
  `BinOp`
- return shape is the existing void sentinel `i64 0`

Rejected / deferred:

- general user-box instance method calls
- multi-block `birth` bodies
- nested unsupported calls inside `birth`
- dynamic dispatch
- inline object layout
- app-specific or by-name C shim matching

## Call Route Contract

MIR metadata:

```text
user_box_method_routes[]
  route_id=user_box.method_call
  proof=typed_user_box_birth_same_module
  target_symbol=Box.birth/N
  target_body_supported=true
```

Lowering plan:

```text
source=user_box_method_routes
tier=DirectAbi
emit_kind=DirectFunctionCall
return_shape=void_sentinel_i64_zero
definition_owner=typed_object_method
```

Unsupported targets remain visible:

```text
proof=typed_user_box_method_contract_missing
reason=user_box_birth_body_unsupported
```

## Fixture

```hako
box Pair {
  left: IntegerBox
  right: IntegerBox

  birth(left, right) {
    me.left = left
    me.right = right
  }
}

static box Main {
  main() {
    local pair = new Pair(10, 20)
    return pair.left + pair.right
  }
}
```

Path:

- `apps/typed-object-birth-min/main.hako`
- `tools/smokes/v2/profiles/integration/apps/typed_object_birth_min_exe.sh`

## Boundary Movement

The dedicated fixture now builds and runs as a direct EXE through
`mir_call_user_box_birth_same_module_emit`.

The real-app EXE boundary remains a blocker probe, not parity. Real apps still
stop at the broader birth/method call route seam when they need unsupported
`birth` bodies or user-box instance method calls.

## Gates

```bash
cargo test --release user_box_method_route --lib
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_birth_min_exe.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_newbox_min_exe.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_untyped_field_min_exe.sh
bash tools/smokes/v2/profiles/integration/apps/real_apps_exe_boundary_probe.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Expand route coverage for user-box instance method calls and the next real-app
`birth` body shapes. Keep every accepted shape MIR-owned and fixture-backed.
