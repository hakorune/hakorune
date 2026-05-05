---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381CE, retire static string array selected-kind and push fallback from Stage0 module generic emission
Related:
  - docs/development/current/main/phases/phase-29cv/P381BL-STATIC-ARRAY-TARGET-SHAPE-RETIRE.md
  - docs/development/current/main/phases/phase-29cv/P381BR-MODULE-GENERIC-SELECTED-KIND-REGISTRY.md
  - docs/development/current/main/phases/phase-29cv/P381BT-MODULE-GENERIC-ARRAY-PUSH-HELPER-CLEANUP.md
  - docs/development/current/main/phases/phase-29cv/P381BU-MODULE-GENERIC-ARRAY-APPEND-EMIT-HELPER-CLEANUP.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381CE: Static Array Uniform Push

## Problem

`StaticStringArrayBody` was already retired as a target-shape variant, but
Stage0 still remembered selected static-array functions with a special planned
kind. The module body emitter also kept a fallback path that accepted `push`
inside those functions by checking the active function kind instead of relying
on the `generic_method.push` LoweringPlan row.

That left source-owner identity in the selected-set registry after MIR already
published the needed generic method contract.

## Decision

Remove the static-array selected-kind registry and the active-function
`push` fallback. Static string array functions are now ordinary selected module
generic functions. Their array append sites must be lowered through the same
MIR-owned route as other array pushes:

```text
source=generic_method_routes
source_route_id=generic_method.push
route_kind=array_append_any
symbol=nyash.array.slot_append_hh
```

The global-call proof for the function result remains unchanged:

```text
proof=typed_global_call_static_string_array
return_shape=array_handle
```

## Result

Implemented:

- removed `planned_module_generic_string_kinds[]`
- removed static-array selected-kind lookup helpers
- removed `module_generic_string_active_function_is_static_string_array()`
- removed the static-array-only push prepass and emit fallback
- kept static array result origin propagation at the global callsite

Observed IR proof for `PatternRegistryBox.candidates/0`:

```llvm
define i64 @"PatternRegistryBox.candidates/0"() {
bb11387:
  %r1 = call i64 @nyash.array.birth_h()
  %r7 = call i64 @nyash.box.from_i8_string_const(ptr @.fnstr_855_7)
  %r4 = call i64 @nyash.array.slot_append_hh(i64 %r1, i64 %r7)
  ...
  ret i64 %r1
}
```

The body is emitted by the generic MIR body path, and the `push` sites are
owned by `generic_method.push` route facts.

## Verification

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_DUMP_IR=/tmp/hakorune_p381ce_stage1_cli_env.ll \
  target/release/ny-llvmc --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj --out /tmp/hakorune_p381ce_stage1_cli_env.o
NYASH_LLVM_SKIP_BUILD=1 NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_BACKEND=crate \
  NYASH_NY_LLVM_COMPILER=target/release/ny-llvmc \
  NYASH_EMIT_EXE_NYRT=target/release HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/ny_mir_builder.sh --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
    --emit exe -o /tmp/hakorune_p381ce_stage1_cli_env.exe
```

Runtime sanity:

- `emit-program` on `apps/tests/mir-branch-ret/main.hako`: rc=0
- `emit-mir` on `apps/tests/mir-branch-ret/main.hako`: rc=0
