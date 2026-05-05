---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381CC, Stage0 generic_i64 numeric leaf owner reconciliation
Related:
  - docs/development/current/main/phases/phase-29cv/P322A-NUMERIC-LEAF-GENERIC-EMIT-DEDUP.md
  - docs/development/current/main/phases/phase-29cv/P381CB-BUILDBOX-PROGRAM-JSON-EXTERN-CANONICALIZE.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_leaf_function_emit.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381CC: Generic I64 Numeric Leaf Owner

## Problem

P381CB correctly moved `generic_i64_body` targets whose body is a pure numeric
leaf under the numeric leaf emitter owner. That avoided duplicate same-module
definitions after declarations were retired.

The call shell still treated every `generic_i64_body` route as owned only by the
module generic definition registry. When a selected multi-function emitter body
called a helper such as:

```text
LowerLoopMultiCarrierBox._reg_add2/2
LowerLoopMultiCarrierBox._reg_add3/3
LowerLoopMultiCarrierBox._reg_sub1/1
```

the leaf definitions were emitted, but the `generic_i64_body` call shell rejected
the call because those symbols were no longer in the generic registry.

## Decision

Keep P322A's owner split:

- numeric leaf bodies are defined by `module_leaf_function_emit`
- non-leaf generic i64 bodies remain defined by the module generic emitter

For `generic_i64_body` callsites, the call shell now accepts either owner:

```text
module_generic_symbol_will_be_defined(target)
  OR module_leaf_symbol_was_emitted(target)
```

`module_leaf_symbol_was_emitted` is intentionally stronger than
`will_be_defined`: the direct call is only accepted after the leaf owner has
actually produced the same-module LLVM definition.

## Rules

Allowed:

- route `generic_i64_body` targets with numeric leaf bodies to the leaf emitter
- let `generic_i64_body` callsites call an already emitted leaf definition
- fall back to generic emission only when the symbol is not owned by leaf

Forbidden:

- reintroducing same-name LLVM declarations for planned same-module functions
- duplicating numeric leaf definitions in the generic emitter
- silently externalizing missing same-module helper definitions

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_stage1_cli_env_parse_probe.ll \
  target/release/ny-llvmc \
  --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj --out /tmp/hakorune_stage1_cli_env_parse_probe.o
git diff --check
```

Expected: `_reg_add2/_reg_add3/_reg_sub1` are defined by the leaf emitter, and
the Stage1 CLI env MIR probe emits an object without undefined same-module
helper symbols.

## Result

Accepted. The Stage1 CLI env MIR probe now emits an object successfully, with
the leaf-owned helpers present in dumped LLVM IR:

```text
define i64 @"LowerLoopMultiCarrierBox._reg_add2/2"
define i64 @"LowerLoopMultiCarrierBox._reg_add3/3"
define i64 @"LowerLoopMultiCarrierBox._reg_sub1/1"
```

This keeps numeric helpers under their narrow owner while allowing the uniform
multi-function emitter path to continue through direct `generic_i64_body`
callsites.
