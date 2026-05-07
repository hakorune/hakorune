# 293x-017 Typed Object Birth-Param Storage EXE Route

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: infer untyped typed-object field storage from observed `newbox`
  constructor arguments through same-module `birth` parameters.

## Decision

- `TypedObjectPlan` remains the layout truth. Storage inference for untyped
  fields may use observed `new Box(args...) -> Box.birth/N` parameter flow.
- The C shim does not classify user-box names or rediscover constructor
  semantics. It still consumes route facts and typed-object plans only.
- This card accepts the real-app-derived `HakoAllocPage.birth/3` shape without
  adding an app workaround or broad dynamic constructor inference.
- Conflicting observations still reject the field plan instead of silently
  choosing one storage.

## Accepted Shape

Accepted in this card:

- user box has declared fields without explicit field types
- a same-module `Box.birth/N` function exists
- `new Box(arg0, arg1, ...)` observes constructor argument storage
- explicit constructor arg `i` maps to `birth` parameter `i + 1`
- parameter 0 remains the receiver
- `birth` assigns those parameters into typed-object fields
- observed scalar and known handle values reuse the existing storage kinds

Rejected / deferred:

- generic parameter storage inference not anchored by `new Box(...)`
- mixed/conflicting writes to the same field
- dynamic field addition
- weak fields
- multi-block method support
- nested unsupported calls such as broader allocator seeding loops
- app-specific C shim matching

## Fixture

```hako
box Page {
  init { page_id, capacity }

  birth(page_id, capacity) {
    me.page_id = page_id
    me.capacity = capacity
  }

  sum() {
    return me.page_id + me.capacity
  }
}

static box Main {
  main() {
    local page = new Page(7, 23)
    return page.sum()
  }
}
```

Path:

- `apps/typed-object-birth-param-min/main.hako`
- `tools/smokes/v2/profiles/integration/apps/typed_object_birth_param_min_exe.sh`

## Boundary Movement

Before this card, BoxTorrent allocator-backed MIR exposed:

```text
HakoAllocPage.birth/3 reason=typed_object_plan_missing
```

After this card, MIR emits a `HakoAllocPage` typed-object plan and
`HakoAllocPage.birth/3` direct routes with:

```text
proof=typed_user_box_birth_same_module
definition_owner=typed_object_method
target_body_supported=true
```

The real-app EXE boundary is not parity yet. Broader `birth` / method bodies
such as allocator seeding remain separate route-shape work.

## Gates

```bash
cargo test --release typed_object_plan --lib
cargo build --release --bin hakorune
bash tools/smokes/v2/profiles/integration/apps/typed_object_birth_param_min_exe.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Keep expanding pure-first route coverage one real-app-derived shape at a time.
The next expected seam is a broader allocator `birth` / method body shape rather
than object allocation itself.
