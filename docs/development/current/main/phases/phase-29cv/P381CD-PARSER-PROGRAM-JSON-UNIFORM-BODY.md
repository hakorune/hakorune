---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381CD, retire the Stage0 parser Program(JSON) dedicated body emitter
Related:
  - docs/development/current/main/phases/phase-29cv/P381CA-UNIFORM-MIR-FUNCTION-CANDIDATE-EMIT.md
  - docs/development/current/main/phases/phase-29cv/P381BN-PARSER-PROGRAM-JSON-TARGET-SHAPE-RETIRE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381CD: Parser Program(JSON) Uniform Body

## Problem

`ParserProgramJsonBody` had already been retired as a
`GlobalCallTargetShape`, but Stage0 still kept a dedicated C body emitter for
the parser Program(JSON) direct proof. That emitter bypassed the selected MIR
function body and duplicated source-owner meaning in the backend.

## Decision

Keep the MIR-owned direct proof:

```text
proof=typed_global_call_parser_program_json
return_shape=string_handle
emit_kind=direct_function_call
```

but plan the target as an ordinary module generic function. The existing
uniform body emitter now lowers the function body and its Stage1 extern call
through the same MIR/LoweringPlan path as other selected functions.

## Result

Implemented:

- removed the parser Program(JSON) planned-kind override
- removed `emit_parser_program_json_function_definition(...)`
- kept callsite validation on the existing MIR proof / return contract
- updated the Stage0 line-shape inventory to mark the dedicated emitter gone

Observed IR proof:

```llvm
define i64 @"Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"(i64 %r0) {
bb14753:
  %r3 = add i64 %r0, 0
  %r2 = call i64 @nyash.stage1.emit_program_json_v0_h(i64 %r3)
  ret i64 %r2
}
```

This body is emitted by the generic MIR function emitter, not by a parser-only
body clone.

## Verification

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_p381cd_stage1_cli_env.ll \
  target/release/ny-llvmc --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj --out /tmp/hakorune_p381cd_stage1_cli_env.o
NYASH_LLVM_SKIP_BUILD=1 NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_BACKEND=crate \
  NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc \
  NYASH_EMIT_EXE_NYRT=target/release HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/ny_mir_builder.sh --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
    --emit exe -o /tmp/hakorune_p381cd_stage1_cli_env.exe
```

Runtime sanity:

- `emit-program` on `apps/tests/mir-branch-ret/main.hako`: rc=0
- `emit-mir` on `apps/tests/mir-branch-ret/main.hako`: rc=0
