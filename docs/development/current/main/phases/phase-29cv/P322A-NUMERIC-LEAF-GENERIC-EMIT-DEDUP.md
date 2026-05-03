---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P322a, numeric leaf and generic definition dedup
Related:
  - docs/development/current/main/phases/phase-29cv/P321A-GENERIC-STRING-I64-BINOP-TYPE-FACT.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_leaf_function_emit.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P322a: Numeric Leaf Generic Emit Dedup

## Problem

P321a removes the invalid `StringScanBox.read_char/2` concat site and advances
the source-exe probe to an IR uniqueness error:

```text
opt: /tmp/hako_p321_probe.ll:32808:12: error: invalid redefinition of function 'LowerLoopMultiCarrierBox._reg_add2/2'
```

The MIR module contains one `LowerLoopMultiCarrierBox._reg_add2/2` function.
The generated IR defines it twice:

```text
define i64 @"LowerLoopMultiCarrierBox._reg_add2/2" ...  # numeric leaf emitter
define i64 @"LowerLoopMultiCarrierBox._reg_add2/2" ...  # generic planned emitter
```

This is an emitter ownership bug.  The numeric leaf emitter already owns the
tiny single-block scalar definition.  The generic planned-definition pass must
not emit that same symbol again just because it was also planned as a
`generic_i64_body` direct target.

## Decision

Keep the numeric leaf emitter as the owner for numeric leaf bodies and skip
those functions in `emit_generic_pure_string_function_definition`.

The call-side contract remains unchanged:

- `numeric_i64_leaf` calls require `module_leaf_symbol_was_emitted`
- `generic_i64_body` calls can still use the planned symbol registry
- if the target body is a numeric leaf, the single existing definition satisfies
  the direct call

## Non-Goals

- no new body shape
- no generic_i64 lowering expansion
- no source rewrite of `LowerLoopMultiCarrierBox`
- no declaration/externalization fallback

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p322.exe lang/src/runner/stage1_cli_env.hako
opt -S -passes=mem2reg /tmp/hako_p322_probe.ll -o /tmp/hako_p322_mem2reg.ll
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

```text
LowerLoopMultiCarrierBox._reg_add2/2 has exactly one `define`.
The previous invalid redefinition error is gone.
```

## Result

Accepted.  The generic planned-definition pass now skips functions already
recognized as numeric i64 leaves, leaving the leaf emitter as the only owner for
those definitions.

Validation:

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p322.exe lang/src/runner/stage1_cli_env.hako
opt -S -passes=mem2reg /tmp/hako_p322_probe.ll -o /tmp/hako_p322_mem2reg.ll
```

`LowerLoopMultiCarrierBox._reg_add2/2` now has one definition and call sites
remain direct:

```text
706:define i64 @"LowerLoopMultiCarrierBox._reg_add2/2"(i64 %r0, i64 %r1)
```

The probe advances to the next IR declaration blocker:

```text
opt: /tmp/hako_p322_probe.ll:38630:19: error: use of undefined value '@nyash.map.keys_h'
```
