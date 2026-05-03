---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P258a, return-call direct arg builder
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P257A-RETURN-BINARY-PARAM-REG-SCAN.md
  - lang/src/mir/builder/func_lowering.hako
---

# P258a: Return Call Direct Arg Builder

## Problem

After P257a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_call/6
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`FuncLoweringBox._lower_return_call/6` lowers a small `Return(Call(...))` shape,
but its active route builds several temporary collections:

```text
args_arr = new ArrayBox()
arg_info = new MapBox()
param_map = new MapBox()
arg_regs = new ArrayBox()
```

These collections only shuttle argument kind/value/register facts inside one
owner-local lowering helper.

## Decision

Do not add generic ArrayBox/MapBox acceptance and do not add a new body shape.

Parse the call arguments once and directly build:

```text
insts      # const instructions for literal args
args_list  # comma-separated register ids
call_arity # accepted argument count
```

Parameter register lookup uses the existing owner-local `_param_reg/2` helper
from P257a.

Also replace the methodize dot check with `JsonFragBox.index_of_from` so this
helper does not depend on an additional string method call.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic ArrayBox/MapBox method widening
- no change to supported argument forms (`Int` and `Var` only)
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p258a_return_call_direct_arg_builder.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the
`FuncLoweringBox._lower_return_call/6` collection/method blocker; a later
blocker may remain.

## Result

The temporary `ArrayBox`/`MapBox` argument shuttles are removed from
`FuncLoweringBox._lower_return_call/6`.

The helper no longer reports the P258a collection/method blocker. Its current
non-direct classification is:

```text
FuncLoweringBox._lower_return_call/6  generic_string_unsupported_instruction
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_binary/5
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```
