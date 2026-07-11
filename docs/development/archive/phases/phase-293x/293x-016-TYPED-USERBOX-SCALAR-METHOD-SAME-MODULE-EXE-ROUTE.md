# 293x-016 Typed UserBox Scalar Method Same-Module EXE Route

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: expand the MIR-owned user-box method route from `birth` only to the
  first scalar-return instance method shape.

## Decision

- `user_box_method_routes[]` remains the truth for user-box method calls.
  Backends read route facts and do not classify `Box.method` names locally.
- The first instance-method shape is deliberately narrow: same-module,
  known-receiver, scalar `Integer` / `Bool` return, and single-block body.
- Unsupported user-box methods remain visible in MIR metadata but are not
  consumed by the C shim as direct routes. This keeps real-app boundary probes
  pinned at the existing `mir_call_no_route` seam instead of advancing into
  unproved method bodies.
- The body-shape checker is split into a dedicated small module so later
  method and `birth` shape expansions do not bloat the route model.

## Accepted Shape

Accepted in this card:

- known receiver user-box method calls other than `birth`
- typed-object plan exists for the receiver box
- same-module `Box.method/N` target exists
- target arity is receiver plus explicit args
- target return type is `Integer` or `Bool`
- target body is a single block ending in `Return`
- target body instructions limited to:
  `Const`, `Copy`, typed `NewBox`, `FieldSet`, `FieldGet`, and simple integer
  `BinOp`

Rejected / deferred:

- multi-block user-box methods
- user-box methods with nested unsupported calls
- handle/string/object return methods
- dynamic dispatch
- hot inline object layout
- app-specific C shim matching

## Route Contract

MIR metadata:

```text
user_box_method_routes[]
  route_id=user_box.method_call
  route_kind=user_box.method
  proof=typed_user_box_method_same_module
  return_shape=scalar_i64
```

Birth keeps the existing proof:

```text
route_kind=user_box.birth
proof=typed_user_box_birth_same_module
return_shape=void_sentinel_i64_zero
```

Unsupported method bodies remain non-direct:

```text
proof=typed_user_box_method_contract_missing
reason=user_box_method_body_unsupported
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

  sum() {
    return me.left + me.right
  }
}

static box Main {
  main() {
    local pair = new Pair(10, 20)
    return pair.sum()
  }
}
```

Path:

- `apps/typed-object-method-min/main.hako`
- `tools/smokes/v2/profiles/integration/apps/typed_object_method_min_exe.sh`

## Boundary Movement

The dedicated method fixture now builds and runs as a direct EXE through
`mir_call_user_box_method_same_module_emit`.

The real-app EXE boundary remains a blocker probe. Real apps still stop at
broader method / `birth` body shapes that require multi-block support or nested
call routes.

## Gates

```bash
cargo test --release user_box_method_route --lib
cargo build --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_method_min_exe.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_birth_min_exe.sh
bash tools/smokes/v2/profiles/integration/apps/real_apps_exe_boundary_probe.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Add the next real-app call shape only when the boundary probe identifies it:
multi-block methods, nested `birth` calls, or scalar handle returns should each
land as separate fixture-backed boxes.
