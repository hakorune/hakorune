---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P276a, Stage1 MIR debug print return contract
Related:
  - docs/development/current/main/phases/phase-29cv/P275A-BUILDBOX-SOURCE-ONLY-BUNDLE-FACADE.md
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/stage1_cli_env/emit_pipeline.hako
---

# P276a: Stage1 MIR Debug Print Return Contract

## Problem

After P275a, the source-execution probe advances to:

```text
target_shape_blocker_symbol=Stage1MirResultValidationBox._debug_print_mir_state/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

`_debug_print_mir_state/2` is a debug-only logging helper. It prints stable
diagnostic lines and has no semantic return value, but the method body does not
end with an explicit scalar sentinel. That leaves the DirectAbi return contract
implicit.

## Decision

Keep the helper as logging-only and make its return contract explicit:

```text
_debug_print_mir_state(...) -> print diagnostics -> return 0
```

Apply the same change to the same-file source and the split
`stage1_cli_env/emit_pipeline.hako` mirror.

## Non-Goals

- no generic classifier widening
- no new `GlobalCallTargetShape`
- no C shim/body-specific emitter change
- no debug output behavior change

## Acceptance

- `Stage1MirResultValidationBox._debug_print_mir_state/2` no longer reports
  `generic_string_return_abi_not_handle_compatible`.
- The active source-execution probe advances past the Stage1 MIR debug-print
  return ABI boundary.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

The Stage1 MIR validation/debug helpers now classify as scalar DirectAbi:

```text
Stage1MirResultValidationBox._debug_print_mir_state/2       generic_i64_body  ScalarI64  DirectAbi
Stage1MirResultValidationBox._debug_print_materialized_mir/2 generic_i64_body  ScalarI64  DirectAbi
Stage1MirResultValidationBox.finalize_emit_result/3          generic_i64_body  ScalarI64  DirectAbi
Stage1EmitMirDispatchBox.run_emit_mir_mode/1                 generic_i64_body  ScalarI64  DirectAbi
```

The source-execution probe now reaches the backend recipe boundary:

```text
reason=no_lowering_variant
```

Route metadata still records later unsupported object-return owners such as
`MirBuilderSourceCompatBox.emit_root_from_source_v0/2 ->
MirRootHydratorBox._parse_object/1`, but the immediate probe failure is no
longer the Stage1 MIR debug-print return ABI.
