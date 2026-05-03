---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P321a, generic string emitter scalar binop type fact
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P320A-STRINGOPS-INDEX-OF-FROM-TEXT-CONTRACT.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P321a: Generic String I64 Binop Type Fact

## Problem

P320a advances the source-exe probe past body-shape blockers, but LLVM
optimization fails during `mem2reg`:

```text
stage=mem2reg result=fail reason=opt
reason=no_lowering_variant
target_shape_blocker_symbol=-
```

The generated IR for `StringScanBox.read_char/2` contains an invalid use:

```llvm
%r70 = call i64 @"nyash.string.concat_hh"(i64 %r58, i64 %r72)
%r68 = call i64 @"nyash.string.substring_hii"(i64 %r60, i64 %r58, i64 %r70)
```

`%r72` is the MIR-side constant `i64 1`, but it is not emitted as an LLVM
register because i64 constants are tracked as immediates.  The real issue is
not `StringScanBox`: direct MIR already records:

```json
{"dst":72,"op":"const","value":{"type":"i64","value":1}}
{"dst":70,"dst_type":"i64","lhs":71,"op":"binop","operation":"+","rhs":72}
```

The generic string emitter seeds broad string origins for string-body params,
then treats `i + 1` as string concat because one copied operand inherits a
string origin.  That ignores the MIR-owned `dst_type: "i64"` fact.

## Decision

Make `module_generic_string_emit_binop` and its prepass honor
`dst_type == "i64"` as a scalar result contract.  For `+`:

- if `dst_type` is `i64`, emit scalar `add i64`
- otherwise, concat only when the existing string-origin proof says an operand
  is a string

This keeps the fix in the LoweringPlan consumer, does not widen generic method
acceptance, and does not add a new body shape.

## Non-Goals

- no source workaround in `StringScanBox`
- no new `GlobalCallTargetShape`
- no generic collection or method acceptance widening
- no body-specific emitter for `StringScanBox`

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p321_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p321.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

```text
StringScanBox.read_char/2 emits `add i64 ..., 1` for the substring end index.
The previous undefined `%r72` concat site is gone.
```

## Result

Accepted.  `module_generic_string_emit_binop` and its prepass now treat
`dst_type == "i64"` as a scalar result fact before considering string concat.

Validation:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --backend mir --emit-mir-json /tmp/hako_p321_direct.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p321.exe lang/src/runner/stage1_cli_env.hako
opt -S -passes=mem2reg /tmp/hako_p321_probe.ll -o /tmp/hako_p321_mem2reg.ll
```

`StringScanBox.read_char/2` now emits:

```llvm
%r70 = add i64 %r58, 1
%r68 = call i64 @"nyash.string.substring_hii"(i64 %r60, i64 %r58, i64 %r70)
```

The probe advances to the next IR-level blocker:

```text
opt: /tmp/hako_p321_probe.ll:32808:12: error: invalid redefinition of function 'LowerLoopMultiCarrierBox._reg_add2/2'
```
