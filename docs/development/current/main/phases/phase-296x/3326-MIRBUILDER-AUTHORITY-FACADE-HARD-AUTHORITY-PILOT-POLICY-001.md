# 3326 - MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001

## Token

```text
MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001
```

## Purpose

Consume the selector-ready minimal path decomposition and select the first
hard-authority pilot policy target.

The selected pilot is:

```text
BoolRecipeCompareSemanticCommandBoundary
```

This card does not implement the pilot. It only authorizes the next
implementation card and fixes the claim ceiling.

## Output Contract

```text
rust-lifecycle-mirbuilder-authority-facade-hard-authority-pilot-policy-v0
```

## Policy Selection

```text
selected_candidate:
  BoolRecipeCompareSemanticCommandBoundary

selected_next_card:
  MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001

claim_ceiling:
  scoped_hard_authority_pilot
```

## Required Pilot Boundary

```text
input_surface:
  BoolRecipeComparePublicationV1

output_surface:
  BoolRecipeCompareLoweringIntentSnapshotV1

downstream_consumer:
  CompareLoweringSymbolicCommandSnapshotV1

forbidden:
  route selection
  runtime route switch
  MIR mutation
  ID allocation
  Source Selfhost claim
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-authority-facade-hard-authority-pilot-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_authority_facade_hard_authority_pilot_policy_guard.sh
```

## Non-Claims

```text
hard_authority_pilot_implemented = 0
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
mir_mutation = 0
id_allocation = 0
new_backend_route = 0
new_abi = 0
```
