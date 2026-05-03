---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P282a, fallback authority operator guard and phi-record capacity
Related:
  - docs/development/current/main/phases/phase-29cv/P281A-DELEGATE-FINALIZE-SEEN-INDEXOF-START.md
  - lang/src/mir/builder/internal/fallback_authority_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P282a: Fallback Authority Operator Guard and PHI Record Capacity

## Problem

After P281a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1
```

The function routes as DirectAbi, but the first prepass issue is `unop not`
from negative compound guards:

```text
if !(op2 == "<" || op2 == ">" || ...)
if !(op == "+" || op == "-" || ...)
```

The module generic prepass intentionally does not grow broad `unop not`
semantics for this owner-local validation guard.

After removing those `unop` instructions, the same body reaches the prepass
PHI-record capacity. Some blocks in this owner-local fallback body now carry
17-19 PHI records, while the generic string prepass stores only 16 PHI records
per block.

## Decision

Rewrite the operator validation as explicit integer flags:

```text
local supported = 0
if op == "..." { supported = 1 }
if supported != 1 { ... }
```

This keeps the validation behavior in `.hako` source and avoids widening the
Stage0 generic string prepass.

Raise only the module generic PHI record capacity per block from 16 to 24.
This is a storage-capacity fix for already accepted PHI records, not a new
MIR operation or body-shape semantic.

## Non-Goals

- no generic `unop not` support in the C prepass
- no new `GlobalCallTargetShape`
- no C shim/body-specific emitter change
- no new PHI semantics
- no fallback behavior change

## Acceptance

- `BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1` no longer contains
  `unop not` from these operator guards.
- The module generic prepass can record the body blocks with 17-19 PHI records.
- The source-execution probe advances to the next blocker.
- `cargo build -q --release --bin hakorune`
- `bash tools/build_hako_llvmc_ffi.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

Fresh metadata for `BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1`
has no `unop` instructions after the operator guard rewrite, and the module
generic prepass can record the high-PHI blocks with the 24-record per-block
capacity.

The 32-record trial was rejected because it made the stack-local lowering state
too large and caused a stack overflow. The committed capacity is 24 because the
observed maximum is 19 PHI records in a block.

Fresh source-execution advanced past
`BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1` and now stops at:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderProgramJsonInputContractBox._program_json_header_present/1
```
