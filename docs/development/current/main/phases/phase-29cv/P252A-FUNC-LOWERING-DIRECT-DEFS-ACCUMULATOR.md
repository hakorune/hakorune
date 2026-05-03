---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P252a, FuncLowering direct defs accumulator
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P251A-MULTI-CARRIER-DIRECT-TEXT-EMIT.md
  - lang/src/mir/builder/func_lowering.hako
---

# P252a: FuncLowering Direct Defs Accumulator

## Problem

After P251a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`FuncLoweringBox.lower_func_defs/2` still collects lowered function JSON strings
through an active `ArrayBox`:

```text
func_jsons = new ArrayBox()
func_jsons.push(mir_func)
func_jsons.length()
func_jsons.get(fi)
```

This is collection plumbing only. The function already owns a string accumulator
(`func_defs_mir`) whose final shape is a comma-prefixed JSON fragment.

## Decision

Do not add ArrayBox method acceptance or a new body shape.

Accumulate lowered function JSON directly into `func_defs_mir` at the point each
`mir_func` is accepted:

```text
func_defs_mir = func_defs_mir + "," + mir_func
```

Keep the existing `func_map` MapBox for call-resolution ownership. This card
only removes the function-output ArrayBox from the active route.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic ArrayBox method widening
- no C body-specific emitter
- no rewrite of call resolution
- no change to emitted function JSON ordering

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p252a_func_lowering_direct_defs_accumulator.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the active ArrayBox
method blocker inside `FuncLoweringBox.lower_func_defs/2`; a later blocker may
remain.

## Result

`FuncLoweringBox.lower_func_defs/2` no longer uses the active `func_jsons`
ArrayBox for output assembly. Lowered function JSON fragments are appended
directly to `func_defs_mir`.

The source-exe probe moved past the previous local ArrayBox method blocker. The
next nested owner is now:

```text
target_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_loop/4
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The current top-level source-exe blocker is:

```text
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```
