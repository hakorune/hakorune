---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P293a, consume existing builder-registry dispatch route fact
Related:
  - docs/development/current/main/phases/phase-29cv/P194-BUILDER-REGISTRY-DISPATCH-BODY-SHAPE.md
  - docs/development/current/main/phases/phase-29cv/P292A-TYPEOP-CHECK-ACCEPT-SHAPE-INDEXOF.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P293a: Builder Registry Dispatch Plan Consume

## Problem

After P292a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirBuilderBox._try_emit_registry_program_json/2
target_shape_blocker_reason=-
```

`MirBuilderBox._try_emit_registry_program_json/2` has no missing `mir_call`
sites. The blocker is a plan-consumption mismatch: MIR metadata already records
the child call as:

```text
symbol=BuilderRegistryAuthorityBox.try_lower/1
target_shape=builder_registry_dispatch_body
proof=typed_global_call_builder_registry_dispatch
return_shape=string_handle_or_null
```

but the ny-llvmc generic string module planner/prepass/call shell only consumes
generic string/string-or-void/parser/program-json/static-array/map/probe shapes.

## Decision

Consume the existing `builder_registry_dispatch_body` LoweringPlan fact as a
direct module generic string-or-null call.

This is not a new body shape and not a body-specific emitter. It wires an
already MIR-owned route fact into the same generic module-function definition
lane used by other DirectAbi string-or-null helpers.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new ny-llvmc body-specific emitter
- no generic method acceptance widening
- no `.hako` registry/fallback semantic change
- no fallback or externalization

## Acceptance

- `BuilderRegistryAuthorityBox.try_lower/1` is planned as a same-module generic
  definition when referenced by LoweringPlan.
- generic string prepass accepts `builder_registry_dispatch_body` result values
  as runtime string-or-null handles.
- `MirBuilderBox._try_emit_registry_program_json/2` advances to the next blocker
  or emits successfully.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done.

`bash tools/build_hako_llvmc_ffi.sh` succeeds after wiring the existing
`typed_global_call_builder_registry_dispatch` fact into the generic module
string lane.

The source-execution probe advances past
`MirBuilderBox._try_emit_registry_program_json/2` and stops at the next module
generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=-
```
