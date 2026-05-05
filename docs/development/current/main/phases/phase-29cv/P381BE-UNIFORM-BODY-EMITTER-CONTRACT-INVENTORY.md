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
| `GenericStringOrVoidSentinelBody` | `typed_global_call_generic_string_or_void_sentinel` / `generic_string_or_void_sentinel_body` | `string_handle_or_null` / `runtime_i64_or_handle` | `ORG_STRING` through the generic-string direct predicate | planned as a generic module symbol; shares the generic string body path | `global_call_route_plan::tests::void_sentinel`, `hostbridge`, `runtime_methods`, runner MIR JSON void-sentinel tests |
| `GenericStringVoidLoggingBody` | superseded by P381BJ: `typed_global_call_generic_string_void_logging` / `target_shape=null` | `void_sentinel_i64_zero` / `scalar_i64` | none; no result register is allowed for void logging sites | planned as a generic module symbol; body still goes through generic module body emission | `global_call_route_plan::tests::void_logging` |
| `ParserProgramJsonBody` | superseded by P381BN: `typed_global_call_parser_program_json` / `target_shape=null` | `string_handle` / `runtime_i64_or_handle` | `ORG_STRING` | planned as a generic module symbol and a parser Program(JSON) symbol; body still has a dedicated `emit_parser_program_json_function_definition` path | `global_call_route_plan::tests::shape_reasons`, runner MIR JSON parser Program(JSON) tests |
| `StaticStringArrayBody` | superseded by P381BL: `typed_global_call_static_string_array` / `target_shape=null` | `array_handle` / `runtime_i64_or_handle` | `ORG_ARRAY_STRING_BIRTH` through the static-array contract predicate | planned as a generic module symbol and a static-array symbol; body has static-array active-function checks and array-push handling | `global_call_route_plan::tests::static_string_array`, runner MIR JSON static array tests |
| `MirSchemaMapConstructorBody` | superseded by P381BM: `typed_global_call_mir_schema_map_constructor` / `target_shape=null` | `map_handle` / `runtime_i64_or_handle` | `ORG_MAP_BIRTH` through the MIR-schema map contract predicate | planned as a generic module symbol; body relies on MIR schema map constructor support in the generic module body path | `global_call_route_plan::tests::mir_schema_map_constructor` |
| `BoxTypeInspectorDescribeBody` | superseded by P381BO: `typed_global_call_box_type_inspector_describe` / `target_shape=null` | `map_handle` / `runtime_i64_or_handle` | `ORG_MAP_BIRTH` | planned as a generic module symbol; active source-owner callers already use scalar predicates; body remains a later uniform-emitter cleanup item | `global_call_route_plan::tests::box_type_inspector_describe` |
| `PatternUtilLocalValueProbeBody` | `typed_global_call_pattern_util_local_value_probe` / `pattern_util_local_value_probe_body` | `mixed_runtime_i64_or_handle` / `runtime_i64_or_handle` | none | planned as a generic module symbol; body remains mixed scalar/handle source-owner shaped | `global_call_route_plan::tests::pattern_util_local_value_probe` |

## Current C-Side Branch Sites

These branches must be checked before deleting any capsule:

- metadata predicate:
  `hako_llvmc_ffi_lowering_plan_metadata.inc`
- call emission / origin propagation:
  `hako_llvmc_ffi_mir_call_shell.inc`
- selected-set planning:
  `hako_llvmc_ffi_module_generic_string_plan.inc`
- module body prepass and body emission:
  `hako_llvmc_ffi_module_generic_string_function_emit.inc`

## Retirement Readiness

Completed focused probe:

- `GenericStringVoidLoggingBody`
  - retired as a target-shape variant in P381BJ
  - direct ABI truth now lives in stored proof and return-contract facts
- `StaticStringArrayBody`
  - retired as a target-shape variant in P381BL
  - direct ABI truth now lives in stored proof and `array_handle` return-contract facts
- `MirSchemaMapConstructorBody`
  - retired as a target-shape variant in P381BM
  - direct ABI truth now lives in stored proof and `map_handle` return-contract facts
- `ParserProgramJsonBody`
  - retired as a target-shape variant in P381BN
  - direct ABI truth now lives in stored proof and `string_handle` return-contract facts
  - the dedicated body emitter remains as a later uniform-emitter cleanup item
- `BoxTypeInspectorDescribeBody`
  - retired as a target-shape variant in P381BO after active source-owner callers
    moved to scalar predicates
  - direct ABI truth now lives in stored proof and `map_handle` return-contract facts
  - the dedicated body emitter remains as a later uniform-emitter cleanup item

Not ready for shape-delete-only:

- `GenericStringOrVoidSentinelBody`: sentinel plumbing remains source-owner
  shaped
- `PatternUtilLocalValueProbeBody`: mixed scalar/handle cleanup should happen
  first

## Boundary

Allowed:

- use this table to choose the next single-capsule task
- add a MIR-owned fact before deleting a shape branch
- require both Rust route tests and runner MIR JSON tests when the capsule has
  external JSON plan evidence

Not allowed:

- delete all matching `lowering_plan_global_call_view_is_direct_*` branches at
  once
- remove origin propagation without an equivalent MIR-owned return/origin fact
- mix source-owner cleanup capsules with the first backend capsule probe

## Result

Done:

- T1 inventory is complete
- the first safe implementation probe, `GenericStringVoidLoggingBody`, is now
  retired as a target-shape variant by P381BJ
- `StaticStringArrayBody`, `MirSchemaMapConstructorBody`, and
  `ParserProgramJsonBody`, and `BoxTypeInspectorDescribeBody` are also retired
  as target-shape variants after proof/return contracts became the SSOT
- remaining temporary capsules have explicit blockers before deletion

Next:

1. choose the next origin-carrying or source-owner cleanup capsule
2. keep `stage0_shape_inventory_guard.sh` green while doing it

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
