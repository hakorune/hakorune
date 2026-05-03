---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P264a, basic local-if text/scalar contract
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P263A-BASIC-RETURN-METHOD-TRACE-SIDE-EFFECT-REMOVAL.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P264a: Basic Local-If Text/Scalar Contract

## Problem

After P263a, the source-exe probe first stops at:

```text
target_shape_blocker_symbol=FuncBodyBasicLowerBox._emit_local_if/7
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_local_if/7` still receives temporary `MapBox` fact objects from the
local-if extractors and reads them with `.get(...)`. It also delegates final MIR
construction to `IfMirEmitBox`, which builds through ArrayBox/MapBox schema
objects.

This is owner-local compiler plumbing, not a reason to widen generic MapBox or
collection method acceptance in Stage0.

## Decision

Do not add generic MapBox `.get` acceptance and do not add a new body shape.

Replace the local-if path with a text/scalar contract:

```text
local name/value      -> scalar/text values
compare op/rhs        -> scalar/text values
then/tail returns     -> scalar/text values
emit local-if MIR     -> direct function JSON text
```

`FuncBodyBasicLowerBox` may keep small extraction helpers, but they must not
return temporary MapBox fact objects for this path.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox/ArrayBox method widening
- no C shim/body-specific emitter change
- no change to the supported local-if shape

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p264a_basic_local_if_text_scalar_contract.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: route metadata no longer classifies
`FuncBodyBasicLowerBox._emit_local_if/*` as
`generic_string_unsupported_method_call`. A later explicit blocker may remain.

## Result

`cargo build -q --release --bin hakorune` passes.

The local-if path now routes through string-or-void DirectAbi helpers:

```text
FuncBodyBasicLowerBox._try_lower_local_if_return/4  generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
FuncBodyBasicLowerBox._emit_local_if/8              generic_string_or_void_sentinel_body  string_handle_or_null  DirectAbi
```

The old local-if temporary object helpers were removed from the active path:

```text
_extract_first_local
_extract_compare
_extract_operand
_extract_return_expr
_coerce_return_value_or_local
_inst_mir_call_method_box
```

The source-exe probe still fails explicitly. The first backend blocker is again
the isolated Return(Call) lowerer:

```text
target_shape_blocker_symbol=ReturnCallLowerBox.lower/6
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

No Stage0 classifier, `GlobalCallTargetShape`, or C shim emitter was widened.
