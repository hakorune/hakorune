---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: per-capsule contract inventory before uniform body-emitter deletion work
Related:
  - docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md
  - docs/development/current/main/phases/phase-29cv/P381BD-STAGE0-MEASUREMENT-SCOPE-LOCK.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - src/mir/global_call_route_plan/model.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BE: Uniform Body-Emitter Contract Inventory

## Problem

T1 from P381BC requires a per-capsule contract table before any
`GlobalCallTargetShape` branch is deleted.

The same-module call instruction is already mostly uniform:

```llvm
%rN = call i64 @"target.symbol"(...)
```

But the surrounding contracts are still not uniform. A capsule can only be
retired after these contracts are either proven unnecessary or represented by
MIR-owned facts instead of shape-specific Stage0 branches.

## Contract Table

| Capsule | Proof / target shape | Return / demand | Result origin side effect | Selected-set / body requirement | Current proof surface |
| --- | --- | --- | --- | --- | --- |
| `GenericStringOrVoidSentinelBody` | superseded by P381BQ: `typed_global_call_generic_string_or_void_sentinel` / `target_shape=null` | `string_handle_or_null` / `runtime_i64_or_handle` | `result_origin=string` from P381CG LoweringPlan metadata | planned as a generic module symbol through the shared P381CF module-generic helper | `global_call_route_plan::tests::void_sentinel`, `hostbridge`, `runtime_methods`, runner MIR JSON void-sentinel tests |
| `GenericStringVoidLoggingBody` | superseded by P381BJ: `typed_global_call_generic_string_void_logging` / `target_shape=null` | `void_sentinel_i64_zero` / `scalar_i64` | none; no result register is allowed for void logging sites | planned as a generic module symbol through the shared P381CF module-generic helper | `global_call_route_plan::tests::void_logging` |
| `ParserProgramJsonBody` | superseded by P381BN: `typed_global_call_parser_program_json` / `target_shape=null` | `string_handle` / `runtime_i64_or_handle` | `result_origin=string` from P381CG LoweringPlan metadata | parser-only body emitter removed by P381CD; planned through the shared P381CF module-generic helper | `global_call_route_plan::tests::shape_reasons`, runner MIR JSON parser Program(JSON) tests |
| `StaticStringArrayBody` | superseded by P381BL: `typed_global_call_static_string_array` / `target_shape=null` | `array_handle` / `runtime_i64_or_handle` | `result_origin=array_string_birth` from P381CG LoweringPlan metadata | selected-kind registry and push fallback removed by P381CE; planned through the shared P381CF module-generic helper | `global_call_route_plan::tests::static_string_array`, runner MIR JSON static array tests |
| `MirSchemaMapConstructorBody` | superseded by P381BM: `typed_global_call_mir_schema_map_constructor` / `target_shape=null` | `map_handle` / `runtime_i64_or_handle` | `result_origin=map_birth` from P381CG LoweringPlan metadata | planned as a generic module symbol through the shared P381CF module-generic helper | `global_call_route_plan::tests::mir_schema_map_constructor` |
| `BoxTypeInspectorDescribeBody` | superseded by P381BO: `typed_global_call_box_type_inspector_describe` / `target_shape=null` | `map_handle` / `runtime_i64_or_handle` | `result_origin=map_birth` from P381CG LoweringPlan metadata | active source-owner callers already use scalar predicates; backend body is planned through the shared P381CF module-generic helper | `global_call_route_plan::tests::box_type_inspector_describe` |
| `PatternUtilLocalValueProbeBody` | superseded by P381BP: `typed_global_call_pattern_util_local_value_probe` / `target_shape=null` | `mixed_runtime_i64_or_handle` / `runtime_i64_or_handle` | none | child-probe recognition uses proof/return facts; backend body is planned through the shared P381CF module-generic helper | `global_call_route_plan::tests::pattern_util_local_value_probe` |

## Current C-Side Branch Sites

These sites are now centralized by P381CF:

- metadata predicate:
  `hako_llvmc_ffi_lowering_plan_metadata.inc`
- call emission / origin propagation:
  shared module-generic helpers are consumed by
  `hako_llvmc_ffi_mir_call_shell.inc`; result-origin propagation consumes the
  P381CG `result_origin` metadata field
- selected-set planning:
  `hako_llvmc_ffi_module_generic_string_plan.inc` calls the shared
  module-generic helper, which now reads P381CH `definition_owner` metadata
- module body prepass and body emission:
  `hako_llvmc_ffi_module_generic_string_function_emit.inc` calls the shared
  module-generic and result-origin helpers

## Retirement Readiness

Completed focused probe:

- `GenericStringVoidLoggingBody`
  - retired as a target-shape variant in P381BJ
  - direct ABI truth now lives in stored proof and return-contract facts
- `StaticStringArrayBody`
  - retired as a target-shape variant in P381BL
  - direct ABI truth now lives in stored proof and `array_handle` return-contract facts
  - selected-kind registry and active static-array push fallback removed by P381CE
- `MirSchemaMapConstructorBody`
  - retired as a target-shape variant in P381BM
  - direct ABI truth now lives in stored proof and `map_handle` return-contract facts
- `ParserProgramJsonBody`
  - retired as a target-shape variant in P381BN
  - direct ABI truth now lives in stored proof and `string_handle` return-contract facts
  - the dedicated body emitter was removed by P381CD
- `BoxTypeInspectorDescribeBody`
  - retired as a target-shape variant in P381BO after active source-owner callers
    moved to scalar predicates
  - direct ABI truth now lives in stored proof and `map_handle` return-contract facts
  - the dedicated body emitter remains as a later uniform-emitter cleanup item
- `PatternUtilLocalValueProbeBody`
  - retired as a target-shape variant in P381BP
  - direct ABI truth now lives in stored proof and
    `mixed_runtime_i64_or_handle` return-contract facts
  - recursive child-probe recognition now reads proof/return facts instead of
    the legacy shape string
- `GenericStringOrVoidSentinelBody`
  - retired as a target-shape variant in P381BQ
  - direct ABI truth now lives in stored proof and
    `string_handle_or_null` return-contract facts
  - generic-method string-origin consumers now read proof/return facts instead
    of the legacy shape string
- C-side call-site consolidation
  - P381CF moved selected-set planning, result-origin propagation, and global
    call emission trace selection behind shared LoweringPlan view helpers
  - P381CG moved result-origin truth into Rust LoweringPlan metadata, removing
    the C proof-name origin branch
  - P381CH moved definition ownership and emit trace consumer truth into Rust
    LoweringPlan metadata, removing the C proof-name selected-set list
  - P381CI deleted the now-unused retired-capsule C direct-view predicates
  - P381CJ synced the durable LoweringPlan JSON and Stage0 inventory SSOT docs
    with the MIR-owned global-call metadata fields
  - P381CK moved active primitive definition ownership checks to
    `definition_owner`, deleting the leaf/generic-i64 C proof/shape predicates
  - P381CL centralized the repeated MIR JSON array-item generic-method view
    predicates behind a shared module-generic helper
  - P381CM centralized the repeated MIR JSON map-field generic-method view
    predicates behind a shared module-generic helper
  - P381CN centralized the repeated StringBox scalar generic-method view
    predicates behind a shared module-generic helper
  - P381CO centralized receiver-origin generic-method view predicates behind a
    shared module-generic exact-match helper
  - P381CP centralized extern-call LoweringPlan validators in the MIR call shell
    behind a shared exact-match helper
  - P381CQ moved the common extern-call need-policy contract checks into the
    matcher and removed repeated row fields
  - P381CR centralized generic-method route tuple matching behind one
    LoweringPlan helper shared by route, emit-kind, need, and set-route users
  - P381CS centralized ownerless MIR JSON generic-method view matching in the
    module-generic method view helpers
  - P381CT made the module-generic prepass reuse cached LoweringPlan views
    instead of re-reading the same entry through each predicate branch
  - P381CU made the module-generic method body dispatch reuse one cached
    generic-method LoweringPlan view across method-specific emitters
  - P381CV centralized paired origin/scan-origin publication in the
    module-generic emitter while leaving origin-only updates explicit
  - P381CW centralized paired i64 type plus origin publication in the
    module-generic emitter while leaving scalar-only cases explicit
  - P381CX centralized MIR JSON field-key origin publication in the
    module-generic emitter while keeping route acceptance unchanged
  - P381CY centralized module-generic `get` route flags behind one local view
    consumed by helper selection and result-origin follow-up
  - P381CZ made the module-generic prepass reuse the same `get` route view
    while keeping its numeric-field origin exception explicit
  - capsule-specific proof readers remain only for route-contract parsing and
    Rust-owned proof serialization

No target-shape-only capsule remains. Remaining work is body/source-owner
cleanup and uniform emitter consolidation.

## Boundary

Allowed:

- use this table to choose the next single-capsule task
- add a MIR-owned fact before deleting a shape branch
- require both Rust route tests and runner MIR JSON tests when the capsule has
  external JSON plan evidence

Not allowed:

- reintroduce capsule-specific direct-call lists outside the shared P381CF
  LoweringPlan view helpers
- remove origin propagation without the MIR-owned P381CG `result_origin` fact
- mix source-owner cleanup capsules with the first backend capsule probe

## Result

Done:

- T1 inventory is complete
- the first safe implementation probe, `GenericStringVoidLoggingBody`, is now
  retired as a target-shape variant by P381BJ
- `StaticStringArrayBody`, `MirSchemaMapConstructorBody`, and
  `ParserProgramJsonBody`, `BoxTypeInspectorDescribeBody`, and
  `PatternUtilLocalValueProbeBody`, and `GenericStringOrVoidSentinelBody` are
  also retired as target-shape variants after proof/return contracts became the
  SSOT
- no temporary target-shape capsule remains in the Stage0 inventory

Next:

1. continue with uniform multi-function emitter and `.inc` consolidation
2. keep `stage0_shape_inventory_guard.sh` green while doing it

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
