---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207f, BoxTypeInspectorBox.is_array direct scalar source flow
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207C-BOX-TYPE-INSPECTOR-IS-MAP-DIRECT-SCALAR.md
  - docs/development/current/main/phases/phase-29cv/P207E-JSONFRAG-READ-BOOL-DIRECT-SCALAR.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/shared/common/box_type_inspector_box.hako
---

# P207f: BoxTypeInspector is_array Direct Scalar

## Problem

P207e removed the `JsonFragBox.read_bool_from/2` scalar blocker. The remaining
transitive MIR/JSON emit path still contains the sibling wrapper:

```text
BoxHelpers.is_array/1 -> BoxTypeInspectorBox.is_array/1
```

`BoxTypeInspectorBox.is_array/1` currently calls `_describe/1`, then reads the
metadata map:

```text
_describe(value) -> MapBox
MapBox.get("is_array") -> flag
```

That is the same Stage0 growth risk that P207c removed for `is_map/1`.

## Decision

Do not add generic-i64 MapBox metadata reads or a new body shape.

Inline the existing array-detection rule inside `is_array/1` and return the
scalar flag directly:

```text
null                           -> 0
repr starts with "ArrayBox("   -> 1
repr starts with "["           -> 1
otherwise                      -> 0
```

Keep the existing `HAKO_BOX_INTROSPECT_TRACE` line shape so diagnostics remain
available, but avoid `_describe/1` and `MapBox.get` on this bool predicate
path.

## Boundary

This card may only change `BoxTypeInspectorBox.is_array/1`.

It must not:

- change `_describe/1` map construction
- change `kind/1`, `is_map/1`, or `describe/1`
- widen `generic_i64_body`
- add a `BoxTypeInspectorIsArrayBody` target shape
- add ny-llvmc map-read semantics for this wrapper

## Probe Contract

Before this card, route inventory shows:

```text
BoxHelpers.is_array/1 -> BoxTypeInspectorBox.is_array/1
tier=Unsupported
target_shape_reason=generic_string_unsupported_method_call
```

After this card, `BoxTypeInspectorBox.is_array/1` should classify through the
same generic-i64 scalar lane as `is_map/1`. The source-exe probe may still stop
on a deeper MIR/JSON emit target.

## Probe Result

After this card, the route inventory shows the sibling wrapper on the scalar
lane:

```text
BoxHelpers.is_array/1 -> BoxTypeInspectorBox.is_array/1
tier=DirectAbi
target_shape=generic_i64_body
target_return_type=i64
```

The source-exe probe still stops at:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox.is_map/1
target_shape_blocker_reason=generic_string_global_target_shape_unknown
backend_reason=missing_multi_function_emitter
```

This is a transitive MIR/JSON emit target-shape blocker, not the old
`BoxTypeInspectorBox.is_array/1` `_describe()->MapBox.get` wrapper failure.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207f_box_type_inspector_is_array_direct_scalar.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
