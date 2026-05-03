---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P318a, Stage1 MIR payload text predicate owner-local contract
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P317A-MIR-SCHEMA-MAP-SET-REDUNDANT-RECEIVER.md
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/stage1_cli_env/emit_pipeline.hako
---

# P318a: Stage1 MIR Payload Text Contract

## Problem

P317a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1MirPayloadContractBox._mir_text_has_functions/1
target_shape_blocker_reason=-
```

The blocker body is a small MIR payload predicate:

```hako
method _mir_text_has_functions(mir_text) {
  local has_functions = mir_text.indexOf("functions")
  if has_functions >= 0 { return 1 }
  return 0
}
```

`mir_text` is materially text before this call, but this local helper receives
it as an untyped parameter.  The generic route planner therefore has no
owner-local text fact for the `RuntimeDataBox.indexOf` receiver.

## Decision

Make the text contract explicit inside the owner-local predicate:

```hako
local text = "" + mir_text
local has_functions = text.indexOf("functions")
```

This keeps the contract in `.hako` source, uses existing string-concat and
`indexOf` facts, and avoids widening generic method acceptance for arbitrary
unknown `RuntimeDataBox.indexOf` receivers.

## Non-Goals

- no generic `RuntimeDataBox.indexOf` widening for unknown receivers
- no new `GlobalCallTargetShape`
- no C body-specific emitter for `Stage1MirPayloadContractBox`
- no fallback or by-name backend rediscovery

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p318.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
Stage1MirPayloadContractBox._mir_text_has_functions/1 no longer fails
module_generic_prepass_failed on RuntimeDataBox.indexOf.
```

## Result

Accepted. `_mir_text_has_functions/1` now performs an owner-local
`"" + mir_text` coercion before `indexOf`, so the generic route planner emits
the existing exact `generic_method.indexOf` / `StringIndexOf` fact without
accepting arbitrary unknown `RuntimeDataBox.indexOf` receivers.

Validation:

```bash
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p318_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p318.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `Stage1MirPayloadContractBox._mir_text_has_functions/1`
to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1MirResultValidationBox._debug_print_mir_state/2
target_shape_blocker_reason=-
```
