---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P261a, basic return-method direct text emit
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P260A-COUNT-PARAM-TEXT-RESULT-GUARD.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P261a: Basic Return Method Direct Text Emit

## Problem

After P260a, the source-exe probe advances to:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._try_lower_return_method/4
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`_try_lower_return_method/4` lowers a narrow `Return(Method(...))` shape, but it
uses temporary `ArrayBox`/`MapBox` state plus `MirSchemaBox` map construction to
shuttle argument and instruction facts inside one helper.

This is collection plumbing in Stage0, not a reason to widen generic collection
method acceptance.

## Decision

Do not add generic ArrayBox/MapBox acceptance and do not add a new body shape.

Parse method arguments once and directly build:

```text
insts
arg_ids
call_arity
```

Emit the final function JSON as text, with parameter register lookup handled by
an owner-local scalar scan.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic ArrayBox/MapBox method widening
- no change to the supported method set
- no C body-specific emitter

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p261a_basic_return_method_direct_text_emit.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past the
`FuncBodyBasicLowerBox._try_lower_return_method/4` collection/method blocker; a
later blocker may remain.

## Result

The temporary `ArrayBox`/`MapBox` argument and instruction shuttles are removed
from `FuncBodyBasicLowerBox._try_lower_return_method/4`.

The owner-local parameter scan routes as:

```text
FuncBodyBasicLowerBox._param_reg/2  generic_i64_body  DirectAbi
```

The helper no longer reports the P261a collection/method blocker. Its current
non-direct classification is:

```text
FuncBodyBasicLowerBox._try_lower_return_method/4  generic_string_unsupported_instruction
```

The source-exe probe now advances to:

```text
target_shape_blocker_symbol=FuncLoweringBox._lower_return_call/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```
