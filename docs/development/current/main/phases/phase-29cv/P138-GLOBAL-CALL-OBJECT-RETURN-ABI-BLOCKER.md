---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P138, object-return ABI blocker reason
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P137-GLOBAL-CALL-VOID-SENTINEL-RETURN-BLOCKER.md
  - src/mir/global_call_route_plan.rs
  - src/runner/mir_json_emit/tests/global_call_routes.rs
---

# P138: Global Call Object Return ABI Blocker

## Problem

After P137, source-execution diagnostics point into the BuildBox path. The next
concrete blocker is an object-return helper:

```text
BuildBox._new_prepare_scan_src_result/1
target_return_type=box<MapBox>
target_shape_reason=generic_string_return_abi_not_handle_compatible
```

The broad ABI reason hides that this is an object boundary, not a string or
sentinel body variant.

## Decision

Split non-string object returns into a dedicated diagnostic reason:

```text
generic_string_return_object_abi_not_handle_compatible
```

This is classifier evidence only. It does not add a lowerable target shape and
does not authorize MapBox/object lowering in the generic string emitter.

## Rules

Allowed:

- report `box<...>` returns other than `StringBox` with the object ABI reason
- mirror the reason into MIR JSON and LoweringPlan
- keep the route unsupported

Forbidden:

- treating object returns as `string_handle`
- adding MapBox method/object lowering in this card
- externalizing same-module object-return helpers

## Expected Evidence

`BuildBox._new_prepare_scan_src_result/1` should report:

```text
target_return_type=box<MapBox>
target_shape_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q build_mir_json_root_emits_object_return_abi_shape_reason`
  succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` reports the
  object ABI reason for `BuildBox._new_prepare_scan_src_result/1`.
