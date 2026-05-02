---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P207c, BoxTypeInspectorBox.is_map direct scalar source flow
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P207B-LOOPSCAN-FIND-VAR-NAME-EARLY-RETURN.md
  - docs/development/current/main/phases/phase-29cv/P201-BOX-TYPE-INSPECTOR-DESCRIBE-BODY.md
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/src/shared/common/box_type_inspector_box.hako
---

# P207c: BoxTypeInspector is_map Direct Scalar

## Problem

P207b moved the source-execution probe to:

```text
target_shape_blocker_symbol=BoxTypeInspectorBox.is_map/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`is_map/1` currently calls `_describe/1`, then reads the returned metadata map:

```text
_describe(value) -> MapBox
MapBox.get("is_map") -> flag
flag == 1 || flag == true -> 1
```

That shape is semantically a scalar predicate, but the MIR contains a
`MapBox.get` method on the metadata object. Adding that method to
`generic_i64_body` would make Stage0 understand another BoxTypeInspector
wrapper shape.

## Decision

Do not add generic-i64 MapBox metadata reads or a new body shape for this
blocker.

Inline the existing map-detection rule inside `is_map/1` and return the scalar
flag directly:

```text
null                         -> 0
repr starts with "MapBox("   -> 1
repr starts with "{"         -> 1
otherwise                    -> 0
```

Keep the existing `HAKO_BOX_INTROSPECT_TRACE` line shape so diagnostics remain
available, but avoid `_describe/1` and `MapBox.get` on this bool predicate
path.

## Boundary

This card may only change `BoxTypeInspectorBox.is_map/1`.

It must not:

- change `_describe/1` map construction
- change `kind/1`, `is_array/1`, or `describe/1`
- widen `generic_i64_body`
- add a `BoxTypeInspectorIsMapBody` target shape
- add ny-llvmc map-read semantics for this wrapper

If `is_array/1` becomes the next blocker, it must be handled as a separate
card.

## Probe Contract

Before this card, the stage probe stopped at:

```text
BoxTypeInspectorBox.is_map/1
generic_string_unsupported_method_call
```

After this card, that blocker should disappear. A later stop is the next
blocker, not a regression.

## Probe Result

The `--emit-exe` probe no longer stops at `BoxTypeInspectorBox.is_map/1`.
The next observed stop is:

```text
target_shape_blocker_symbol=LowerLoopLocalReturnVarBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

That later stop is outside this card's boundary.

## Acceptance

```bash
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p207c_box_type_inspector_is_map_direct_scalar.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
