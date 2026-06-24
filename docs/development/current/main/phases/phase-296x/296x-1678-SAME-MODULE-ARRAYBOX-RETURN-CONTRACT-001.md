# 296x-1678: Same-Module ArrayBox Return Contract

Status: Complete
Date: 2026-06-25
Token: SAME-MODULE-ARRAYBOX-RETURN-CONTRACT-001

## Decision

Select the focused same-module `ArrayBox` return slice before opening ID
generator scalarization.

```text
selected option:
  B

implementation owner:
  source-complete multi-carrier exit facts plus existing same-module
  ObjectHandle return contract
```

This is not arbitrary object-return support. The selected owner is the live red
edge exposed by the converter matrix:

```text
MultiCarrierExitPhiPilotApi.project_exit_carriers/1
  -> ArrayBox
```

## Problem

The generated multi-carrier helper declares an `ArrayBox` return, but the
current generated default branch returns scalar `7` after printing a fail
message. That creates a mixed return shape inside a helper that should have a
single owned `ArrayBox` transport.

The fix is not to widen backend ABI to `MixedRuntimeI64OrHandle`. The source
shape must be normalized first so every return path has the same transport.

## Selected Source Slice

```text
docs/.../multi-carrier-exit-phi-source.rs
  MultiCarrierExitPhiPilot::project_exit_carriers
```

Selected shape:

```text
input:
  one i64 selector

output:
  owned pair of i64
  current Hako transport = ArrayBox

branches:
  break        -> [1, 10]
  continue     -> [2, 20]
  early_return -> [3, 30]
  default      -> [0, 0]
```

All return paths must use the same owned `ArrayBox` transport.

## Authority

```text
Rust match arms including default
  -> MultiCarrierExitPhiFacts
       exits
       default_exit
       carrier types
       carrier arity
  -> VerifiedHako operation
       ExplicitMultiExitPhiI64Array
       cases
       default_values
  -> canonical MIR return sites
  -> GlobalCallTargetFacts body-wide return-contract agreement
  -> GlobalCallReturnContract::ObjectHandle
       target_result_box_name=ArrayBox
  -> SameModuleDefinitionPlan
  -> C uniform MIR consumer
```

## Non-Authority

The following must not select the route or prove the contract:

```text
callee symbol spelling
function/static-box name
Main only passing 0, 1, and 2
new ArrayBox appearing somewhere in the body
declared return type alone
raw i64 return lane
C-side ORG_ARRAY_BIRTH inference
scalar fail-code reinterpretation
```

## Implementation Boundary

1. Add `default_exit` to the multi-carrier exit facts.
2. Validate default arity and i64 carrier types.
3. Remove harness fail policy from the helper operation.
4. Render the default branch as owned ArrayBox values:

```hako
} else {
    carriers.push(0)
    carriers.push(0)
}
return carriers
```

5. Use the existing same-module object-handle contract:

```text
tier=DirectAbi
emit_kind=direct_function_call
return_shape=object_handle
value_demand=runtime_i64_or_handle
target_result_box_name=ArrayBox
definition_owner=uniform_mir
proof=typed_global_call_same_module_object_handle
reason=none
```

No new route kind, ABI, canonical MIR instruction, or runtime fallback is
selected.

## Acceptance

Generated Hako:

```text
project_exit_carriers return type = ArrayBox
helper return 7 = 0
helper fail-message print = 0
exit_kind=0  -> [1,10]
exit_kind=1  -> [2,20]
exit_kind=2  -> [3,30]
exit_kind=99 -> [0,0]
```

Route metadata:

```text
reason = none
tier = DirectAbi
emit_kind = direct_function_call
proof = typed_global_call_same_module_object_handle
return_shape = object_handle
value_demand = runtime_i64_or_handle
target_result_box_name = ArrayBox
definition_owner = uniform_mir
```

Gates:

```text
bash tools/checks/rust_lifecycle_multi_carrier_exit_phi_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh
cargo test -q global_call_route_plan:: --lib
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

Full converter matrix green is required because this slice is selected to close
the known matrix red edge.

## Closeout Evidence

```text
source_default_exit_facts=green
helper_scalar_fail_return_removed=green
generated_hako_exe_aot=green
same_module_arraybox_return_contract=green
same_module_definition_plan=green
backend_ready_semantic_refresh_before_ny_llvmc=green
full_converter_matrix=green
runtime_try_hako_then_rust_fallback=0
```

Validated with:

```text
python3 tools/rust_lifecycle/generate_multi_exit_phi_artifact.py --check
bash tools/checks/rust_lifecycle_multi_carrier_exit_phi_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh
cargo test -q global_call_route_plan:: --lib
bash tools/checks/rust_mirbuilder_negative_converter_fixtures_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```

## Negative Acceptance

```text
default arm missing
  -> Deny(UnsupportedDirectShape)
     detail=DefaultExitMissing

default carrier arity mismatch
  -> Deny(PhiJoinRequired)
     detail=DefaultCarrierArityMismatch

ScalarI64 and ObjectHandle return paths mixed
  -> fail closed
  -> no MixedRuntime promotion

target_result_box_name missing
  -> fail closed

SameModuleDefinitionPlan missing
  -> fail-fast
  -> no extern fallback

definition duplicated
  -> fail-fast
```

## Non-Claims

```text
arbitrary ObjectHandle return = 0
MapBox return = 0
StringBox return = 0
user-box return = 0
nullable handle return = 0
MixedRuntimeI64OrHandle expansion = 0
throw / exception lowering = 0
recursive same-module functions = 0
new canonical MIR instruction = 0
new backend route kind = 0
```

## Parked Follow-Ups

```text
NEWTYPE-ID-GENERATOR-SCALARIZATION-001
MIRBUILDER-DERIVED-CONTEXT-BUNDLE-V1-001
minimal MirBuilder execution path
DerivedMainline pilot
```
