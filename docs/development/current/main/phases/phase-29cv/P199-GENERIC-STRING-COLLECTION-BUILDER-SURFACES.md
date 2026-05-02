---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P199, collection builder method surfaces in DirectAbi string/map bodies
Related:
  - docs/development/current/main/phases/phase-29cv/P196-MIR-SCHEMA-MAP-CONSTRUCTOR-BODY.md
  - docs/development/current/main/phases/phase-29cv/P198-GENERIC-STRING-ARRAY-PUSH-WRITE-ANY.md
  - lang/src/mir/builder/internal/compat_mir_emit_box.hako
  - src/mir/global_call_route_plan/generic_string_surface.rs
  - src/mir/global_call_route_plan/mir_schema_map_constructor_body.rs
---

# P199: Generic String Collection Builder Surfaces

## Problem

P198 moved the active source-execution probe to:

```text
target_shape_blocker_symbol=CompatMirEmitBox.emit_array_push_sequence/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The source body builds Program(JSON v0) fragments with Hako list/map literals.
After MIR lowering, those literals are visible as collection builder calls:

```text
new ArrayBox()
ArrayBox.birth()
ArrayBox.push(...)
new MapBox()
MapBox.birth()
MapBox.set(key, value)
```

`ArrayPush` and `MapSet` already have generic method route facts. `NewBox`
already gives the classifier Array/Map value classes. The missing piece is that
the generic string/map constructor classifiers reject the post-newbox
`birth()` lifecycle call and do not consume `MapBox.set` in string-return
collection builders.

## Decision

Keep this structural. Do not add `CompatMirEmitBox` by-name matching.

Add two collection builder surfaces:

- `ArrayBox.birth()` / `MapBox.birth()` as no-op lifecycle calls after a proven
  collection receiver
- `MapBox.set(key, value)` as `write_any` when the receiver is MapBox and the
  key is a proven string

The surface must not:

- treat collection birth as string evidence
- accept unknown collection receivers
- accept arbitrary methods on MapBox/ArrayBox
- change object-return ABI rules outside the existing map constructor shape

## Implementation

- `generic_string_surface.rs` owns the collection-builder surface predicates.
- `generic_string_body.rs` consumes proven `ArrayBox.birth` / `MapBox.birth`
  and `MapBox.set` before the generic unknown-method rejection.
- `mir_schema_map_constructor_body.rs` accepts the same collection lifecycle
  birth calls for existing map-constructor bodies.
- The C generic-string emitter treats `ArrayBox.birth` / `MapBox.birth` as
  no-op scalar lifecycle calls and keeps method emission plan-driven.

## Probe Result

The prior blocker was removed:

```text
target_shape_blocker_symbol=CompatMirEmitBox.emit_array_push_sequence/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The source-execution probe now reaches the next blocker:

```text
target_shape_blocker_symbol=IfMirEmitBox._ret_block/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q collection_builder_surface --lib
cargo test -q mir_schema_map_constructor_birth --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p199_collection_builder_surface_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The `--emit-exe` command remains a next-blocker probe.
