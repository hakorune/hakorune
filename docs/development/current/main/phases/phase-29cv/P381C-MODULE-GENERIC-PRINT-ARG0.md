---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381C, module generic print argument zero
Related:
  - docs/development/current/main/phases/phase-29cv/P381B-STAGE1-EMIT-MIR-SOURCE-EXTERN-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P380Y-GENERIC-I64-PRINT-DEAD-DST.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P381C: Module Generic Print Arg0

## Problem

P381B advanced the phase29cg replay to a module generic body emitter failure:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=Stage1MirPayloadContractBox._fail_invalid_mir_text/1
```

The function is already classified as `generic_i64_body`. Its body is only:

```text
print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
print(mir_text)
return 96
```

The partial generated LLVM shows the first print emitted successfully, then the
second print stopped:

```llvm
define i64 @"Stage1MirPayloadContractBox._fail_invalid_mir_text/1"(i64 %r0) {
bb0:
  %r1 = call i64 @"nyash.box.from_i8_string_const"(...)
  %print_call_1 = call i64 @"nyash.console.log_handle"(i64 %r1)
  %r4 = call i64 @"nyash.box.from_i8_string_const"(...)
  unreachable
}
```

The second print uses the function parameter register `%r0`. The shared
`emit_global_mir_call(...)` helper currently treats `a0 == 0` as missing, so it
rejects a valid register-zero argument.

## Decision

Allow register `0` as a valid print argument when the caller has already parsed
the argument list. This aligns C emission with the existing MIR route classifier
from P380Y, which accepts dead-dst `print` in generic i64 bodies.

This is an emitter bug fix, not a new body shape:

```text
Global print arg register 0 -> emit nyash.console.log_handle(%r0)
```

## Non-Goals

- no new `GlobalCallTargetShape`
- no wider print acceptance in the classifier
- no source rewrite
- no VM fallback
- no acceptance of malformed print calls without an argument-list owner

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh

rm -rf /tmp/p381c_phase29cg
KEEP_OUT_DIR=1 \
OUT_DIR=/tmp/p381c_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the blocker moves beyond
`Stage1MirPayloadContractBox._fail_invalid_mir_text/1`.

## Result

Accepted. The phase29cg replay moved beyond
`Stage1MirPayloadContractBox._fail_invalid_mir_text/1`.

The next blocker is:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=Stage1ProgramJsonMirCallerBox._emit_mir_from_program_json_text_checked/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

This confirms the module generic emitter now treats register `%r0` as a valid
print argument when the callsite already owns a parsed argument list.
