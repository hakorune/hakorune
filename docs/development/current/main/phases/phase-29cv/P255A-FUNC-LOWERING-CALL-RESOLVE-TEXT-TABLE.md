---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P255a, FuncLowering call resolve text table
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P254A-EXTERN-HOSTBRIDGE-PARAM-REG-SCAN.md
  - lang/src/mir/builder/func_lowering.hako
  - lang/src/mir/builder/func_body/cli_run_lower_box.hako
---

# P255a: FuncLowering Call Resolve Text Table

## Problem

After P254a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox.resolve_call_target/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`FuncLoweringBox.resolve_call_target/2` only needs an owner-local lookup from a
function name to the qualified `"Box.method"` target accumulated by
`lower_func_defs/2`.

The active route currently materializes that lookup through `MapBox`:

```text
func_map = new MapBox()
func_map.set(func_name, box_name + "." + func_name)
resolved = func_map.get(call_name)
```

This is a small function-definition table, not a reason to widen Stage0 with
generic `MapBox.get` semantics.

## Decision

Do not add generic MapBox acceptance and do not add a new body shape.

Replace the temporary `MapBox` with an owner-local text table:

```text
|name=Box.name;|other=Box.other;
```

`resolve_call_target(call_name, func_map_text)` scans the text table with
string/index operations. The existing `HAKO_MIR_BUILDER_CALL_RESOLVE` toggle
continues to decide whether resolution is enabled.

The table is built incrementally, matching the previous `MapBox` visibility:
the current def is registered before lowering its body, and earlier defs remain
visible.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox method widening
- no change to call target resolution policy
- no C body-specific emitter
- no two-pass function table behavior change

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p255a_func_lowering_call_resolve_text_table.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the MapBox method
blocker inside `FuncLoweringBox.resolve_call_target/2`; a later blocker may
remain.

## Result

`FuncLoweringBox.resolve_call_target/2` now routes as:

```text
FuncLoweringBox.resolve_call_target/2  generic_pure_string_body  DirectAbi
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_loop/4
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```
