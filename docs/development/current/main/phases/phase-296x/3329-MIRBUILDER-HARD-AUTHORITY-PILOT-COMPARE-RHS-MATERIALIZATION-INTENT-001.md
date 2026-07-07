# 3329 - MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001

## Token

```text
MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001
```

## Purpose

Implement the selected next hard-authority seam by fixing
`CompareRhsMaterializationIntentSnapshotBox` as a `.hako` read-only semantic
intent owner.

This consumes the 3328 selection and stays below actual ValueId resolution,
constant/helper emission, MIR mutation, route selection, and Source Selfhost.

## Output Contract

```text
rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-materialization-intent-v0
```

## Implemented Seam

```text
candidate:
  CompareRhsMaterializationIntentBoundary

owner:
  CompareRhsMaterializationIntentSnapshotBox

input_surface:
  CompareLoweringSymbolicCommandSnapshotV1

output_surface:
  CompareRhsMaterializationIntentSnapshotV1

downstream_boundary:
  CompareRhsValueIdResolutionPlanSnapshotBox
```

## Evidence

The pilot uses the existing `.hako` owner:

```text
lang/src/compiler/mirbuilder/compare_rhs_materialization_intent_snapshot.hako
```

The guard verifies both supported read-only materialization intent rows:

```text
command_literal_i64
command_symbol_ref
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hard-authority-pilot-compare-rhs-materialization-intent-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_hard_authority_pilot_compare_rhs_materialization_intent_guard.sh
```

## Claims

```text
hard_authority_pilot_implemented = 1
compare_rhs_materialization_intent_owner = 1
hako_semantic_intent_surface = 1
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
rhs_runtime_materialization = 0
constant_mir_emission = 0
runtime_helper_emission = 0
mir_mutation = 0
id_allocation = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```
