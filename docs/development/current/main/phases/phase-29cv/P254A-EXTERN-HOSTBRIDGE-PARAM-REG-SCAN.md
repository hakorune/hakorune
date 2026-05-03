---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P254a, Extern hostbridge param register scan
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P253A-COUNT-PARAM-TEXT-SENTINEL-WRAPPERS.md
  - lang/src/mir/builder/func_body/extern_call_box.hako
---

# P254a: Extern Hostbridge Param Register Scan

## Problem

After P253a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=ExternCallLowerBox.lower_hostbridge/4
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`ExternCallLowerBox.lower_hostbridge/4` only needs to map a single argument name
to its parameter register. The active route currently builds a `MapBox` for that
small lookup:

```text
param_map = new MapBox()
param_map.set(name, reg)
arg_reg = param_map.get(arg_name)
```

This is collection plumbing, not semantic ownership.

## Decision

Do not add generic MapBox acceptance and do not add a new body shape.

Replace the temporary `MapBox` with an owner-local scalar scan:

```text
_param_reg(params_json, arg_name) -> i64
```

The helper walks `ParamListBox` directly and returns `0` when no parameter
matches.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox method widening
- no change to hostbridge extern semantics
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p254a_extern_hostbridge_param_reg_scan.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the MapBox method
blocker inside `ExternCallLowerBox.lower_hostbridge/4`; a later blocker may
remain.

## Result

`ExternCallLowerBox.lower_hostbridge/4` now routes as
`generic_string_or_void_sentinel_body` with `DirectAbi`.

The owner-local scalar helper routes as:

```text
ExternCallLowerBox._param_reg/2  generic_i64_body  DirectAbi
```

The existing string helpers remain direct:

```text
ExternCallLowerBox._build_params_json/1  generic_pure_string_body          DirectAbi
ExternCallLowerBox._build_func_name/3    generic_string_or_void_sentinel_body DirectAbi
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox.resolve_call_target/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```
