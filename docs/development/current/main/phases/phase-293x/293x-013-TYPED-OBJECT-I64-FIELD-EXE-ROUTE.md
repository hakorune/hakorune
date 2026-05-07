# 293x-013 Typed Object I64 Field EXE Route

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: implement the first general user-box typed-object EXE route for
  declared i64 fields: `newbox` allocation plus slot `field_set` / `field_get`.

## Decision

- MIR emits `typed_object_plans[]` as module-level layout truth.
- The pure-first `.inc` backend reads `typed_object_plans[]` only. It does not
  scan raw user-box declarations for slots and does not add app-specific
  `newbox` switches.
- Runtime/kernel owns opaque typed slot objects through C ABI helpers.
- The first accepted route is `ColdRuntime` helper lowering, not inline LLVM
  struct layout.

## Accepted Shape

Accepted in this card:

- non-weak user box
- declared field storage `IntegerBox`, `Integer`, or `i64`
- `newbox BoxName()`
- `field_set` for planned i64 slots
- `field_get` for planned i64 slots
- local/copy object binding in the generic pure-first walk

Rejected / deferred:

- weak fields
- untyped or mixed storage fields
- handle fields such as String / Array / Map
- `birth` ownership beyond normal existing MIR calls
- user-box instance method calls
- constructor inline
- inline object layout

## Runtime ABI

The initial helper ABI is intentionally slot based and opaque:

```text
i64  nyash.object.new_typed_hi(i64 type_id, i64 field_count)
i64  nyash.object.new_typed_h(i64 type_id)
i64  nyash.object.field_get_hii(i64 object, i64 slot)
void nyash.object.field_set_hii(i64 object, i64 slot, i64 value)
```

`new_typed_hi` is the route used by pure-first lowering because the runtime
needs the MIR-owned field count to allocate the slot vector. `new_typed_h`
remains available as a zero-field compatibility wrapper.

## Fixture

```hako
box Pair {
  left: IntegerBox
  right: IntegerBox
}

static box Main {
  main() {
    local pair = new Pair()
    pair.left = 10
    pair.right = 20
    return pair.left + pair.right
  }
}
```

Path:

- `apps/typed-object-newbox-min/main.hako`
- `tools/smokes/v2/profiles/integration/apps/typed_object_newbox_min_exe.sh`

## Gates

```bash
cargo test -q -p nyash-rust typed_object_plan --lib
cargo test -q -p nyash_kernel typed_object_helpers_store_and_load_i64_slots --lib
cargo build --release -q -p nyash-rust --bin hakorune -p nyash-llvm-compiler --bin ny-llvmc
cargo build --release -q -p nyash_kernel
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/typed_object_newbox_min_exe.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
```

## Next

Direct real-app EXE parity is still not claimed. The current real apps mostly
use untyped `init { ... }` fields, handle fields, `birth`, and instance methods;
their top-level boundary probe still intentionally pins the remaining
general-user-box EXE gap.
