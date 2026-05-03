---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P295a, builder-registry dispatch LoweringPlan contract reader
Related:
  - docs/development/current/main/phases/phase-29cv/P293A-BUILDER-REGISTRY-DISPATCH-PLAN-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P294A-JSONFRAG-CONST-CANONICALIZE-TEXT-SENTINEL.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P295a: Builder Registry Dispatch Contract Reader

## Problem

P293a wired `typed_global_call_builder_registry_dispatch` through the module
generic string planner, prepass, and call shell. P294a then exposed that the
LoweringPlan global-call view reader still rejects the same proof before the
direct-target check:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirBuilderBox._try_emit_registry_program_json/2
target_shape_blocker_reason=-
```

The exact failing site is:

```text
MirBuilderBox._try_emit_registry_program_json/2
  BuilderRegistryAuthorityBox.try_lower/1
  proof=typed_global_call_builder_registry_dispatch
  target_shape=builder_registry_dispatch_body
  return_shape=string_handle_or_null
```

The bug is not missing route metadata and not missing emitter wiring. The C shim
contract reader simply does not list `typed_global_call_builder_registry_dispatch`
as a typed global-call contract.

## Decision

Register `typed_global_call_builder_registry_dispatch` in the LoweringPlan
global-call contract reader.

This keeps the P293a design intact: the C shim consumes an existing MIR-owned
route fact and does not rediscover builder-registry semantics from the body.

## Non-Goals

- no new `GlobalCallTargetShape`
- no new body-specific emitter
- no generic method acceptance widening
- no `.hako` registry policy change
- no fallback or externalization

## Acceptance

- `read_lowering_plan_global_call_view` accepts
  `typed_global_call_builder_registry_dispatch`.
- `MirBuilderBox._try_emit_registry_program_json/2` no longer fails at the
  builder-registry dispatch callsite.
- source-execution probe advances to the next blocker or emits successfully.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

`read_lowering_plan_global_call_view` now accepts the existing
`typed_global_call_builder_registry_dispatch` proof. The source-execution probe
no longer stops at the `MirBuilderBox._try_emit_registry_program_json/2`
prepass callsite and advances to the next same-module emission blocker:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=BuilderRegistryAuthorityBox.try_lower/1
target_shape_blocker_reason=-
```

Keep the new body-emission issue separate; P295a only fixes the contract reader
hole exposed by P294a.
