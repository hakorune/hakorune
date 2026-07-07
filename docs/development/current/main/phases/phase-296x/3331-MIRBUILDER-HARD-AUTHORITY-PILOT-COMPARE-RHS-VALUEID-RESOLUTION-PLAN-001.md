# 3331 - MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001

## Token

```text
MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-RESOLUTION-PLAN-001
```

## Purpose

Implement the selected next hard-authority seam by fixing
`CompareRhsValueIdResolutionPlanSnapshotBox` as a `.hako` read-only semantic
plan owner.

This consumes the 3330 selection and stays below actual RHS `ValueId`
resolution, literal constant allocation, symbol lookup execution, constant or
helper emission, LocalSSA, MIR Compare/Branch emission, mutation, route
selection, and Source Selfhost.

## Output Contract

```text
rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-v0
```

## Implemented Seam

```text
candidate:
  CompareRhsValueIdResolutionPlanBoundary

owner:
  CompareRhsValueIdResolutionPlanSnapshotBox

input_surface:
  CompareRhsMaterializationIntentSnapshotV1

output_surface:
  CompareRhsValueIdResolutionPlanSnapshotV1

downstream_boundary:
  CompareRhsValueIdResolutionRequestSnapshotBox
```

## Evidence

The pilot uses the existing `.hako` owner:

```text
lang/src/compiler/mirbuilder/compare_rhs_valueid_resolution_plan_snapshot.hako
```

The guard verifies both supported read-only plan rows:

```text
intent_literal_i64
intent_symbol_ref
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hard-authority-pilot-compare-rhs-valueid-resolution-plan-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_valueid_resolution_plan_guard.sh
```

## Claims

```text
hard_authority_pilot_implemented = 1
compare_rhs_valueid_resolution_plan_owner = 1
hako_semantic_plan_surface = 1
rust_oracle_parity = 1
aot_exe_guard = 1
downstream_boundary_present = 1
```

## Non-Claims

```text
hako_adopted_decision = 0
source_selfhost_claim = 0
native_seed_materialization = 0
rhs_value_id_resolution = 0
literal_constant_value_id_allocation = 0
constant_mir_emission = 0
runtime_helper_emission = 0
symbol_lookup_execution = 0
local_ssa_finalize_compare_execution = 0
mir_cmp_emission = 0
branch_emission = 0
basic_block_mutation = 0
value_id_allocation = 0
mir_mutation = 0
id_allocation = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```
