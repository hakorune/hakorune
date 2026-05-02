---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P163, module generic function failure diagnostic
Related:
  - docs/development/current/main/phases/phase-29cv/P162-GENERIC-I64-STRING-INDEXOF-METHOD.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P163: Module Generic Failure Diagnostic

## Problem

P162 moved the active source-execution probe past
`Stage1ProgramResultValidationBox.finalize_emit_result/1`, but the next backend
stop was too broad:

```text
[llvm-pure/unsupported-shape] ... first_op=unknown reason=no_lowering_variant
```

The failing path was inside module generic function definition emission, which
returned `-1` before recording a boundary inventory detail. That made the next
real blocker harder to assign without hand-editing the backend.

## Decision

Keep the existing dev-only `[llvm-pure/unsupported-shape]` tag and add no new
route policy. When module generic function emission fails, record:

- `first_op=module_function`
- `reason=module_generic_prepass_failed` or `module_generic_body_emit_failed`
- `target_shape_blocker_symbol=<function name>`

This is diagnostic-only. It does not accept any new MIR shape and does not
change lowering decisions.

## Acceptance

```bash
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p163_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is expected to remain red. After rebuilding the
C FFI boundary, P162's `indexOf` consumer is exercised and the current next
blocker becomes:

```text
first_block=14533
first_inst=2
first_op=mir_call
reason=missing_multi_function_emitter
target_shape_blocker_symbol=StringHelpers.to_i64/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

That is a separate entry-dispatch / emit-mir branch blocker, not part of this
diagnostic-only card.
