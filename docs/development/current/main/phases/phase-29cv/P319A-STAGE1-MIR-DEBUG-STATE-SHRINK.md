---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P319a, Stage1 MIR debug state output shrink
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P318A-STAGE1-MIR-PAYLOAD-TEXT-CONTRACT.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/stage1_cli_env/emit_pipeline.hako
---

# P319a: Stage1 MIR Debug State Shrink

## Problem

P318a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1MirResultValidationBox._debug_print_mir_state/2
target_shape_blocker_reason=-
```

`_debug_print_mir_state/2` is called only behind `STAGE1_CLI_DEBUG`, but the
Stage0 source-exe line still has to classify the function body.  The current
body contains many debug-only branches and string predicates, so Stage0 is being
asked to carry diagnostic complexity that is not part of the correctness path.

## Decision

Shrink `_debug_print_mir_state/2` to one stable debug line:

```text
[stage1-cli/debug] emit-mir materialized.len=<...> preview=<...>
```

This keeps the existing debug gate and useful payload visibility while removing
debug-only structural validation from the compiled Stage0 route.  Structural MIR
payload validation remains owned by `Stage1MirPayloadContractBox`.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C body-specific emitter for debug helpers
- no generic branch/control-flow widening for debug-only diagnostics
- no change to the non-debug correctness path

## Acceptance

```bash
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p319_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p319.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
Stage1MirResultValidationBox._debug_print_mir_state/2 no longer blocks
module_generic_prepass_failed.
```

## Result

Accepted. `_debug_print_mir_state/2` now emits a single gated debug line using
the existing `_debug_len_inline` and `_debug_preview_inline` helpers.  The
structural payload checks remain in `Stage1MirPayloadContractBox`, and Stage0 no
longer has to classify the old debug-only branch tree.

Validation:

```bash
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p319_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p319.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `Stage1MirResultValidationBox._debug_print_mir_state/2`
to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=StringOps.index_of_from/3
target_shape_blocker_reason=-
```
