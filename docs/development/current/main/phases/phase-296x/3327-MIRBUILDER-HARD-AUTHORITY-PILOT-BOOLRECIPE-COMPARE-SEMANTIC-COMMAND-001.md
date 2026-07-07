# 3327 - MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001

## Token

```text
MIRBUILDER-HARD-AUTHORITY-PILOT-BOOLRECIPE-COMPARE-SEMANTIC-COMMAND-001
```

## Purpose

Implement the first registry-selected hard-authority pilot by fixing the
BoolRecipe Compare semantic command seam as a `.hako` owner with AOT/EXE
evidence.

This card consumes the 3326 policy selection. It does not make a HakoAdopted
decision and does not move Source Selfhost or runtime route authority.

## Output Contract

```text
rust-lifecycle-mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command-v0
```

## Implemented Seam

```text
candidate:
  BoolRecipeCompareSemanticCommandBoundary

owner:
  BoolRecipeCompareLoweringIntentSnapshotBox

input_surface:
  BoolRecipeComparePublicationV1

semantic_command_surface:
  BoolRecipeCompareLoweringIntentSnapshotV1
  CompareLoweringSymbolicCommandSnapshotV1

downstream_boundary:
  Compare RHS materialization / ValueId resolution / Compare emission chain
```

## Evidence

The pilot uses existing `.hako` owners:

```text
lang/src/compiler/mirbuilder/bool_recipe_compare_lowering_intent_snapshot.hako
lang/src/compiler/mirbuilder/compare_lowering_symbolic_command_snapshot.hako
```

The guard validates the prior policy selection, the existing AOT/EXE intent
gate, the symbolic command field parity rows, and the downstream closeout
fixture without opening runtime route authority.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hard-authority-pilot-boolrecipe-compare-semantic-command-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_hard_authority_pilot_boolrecipe_compare_semantic_command_guard.sh
```

## Claims

```text
hard_authority_pilot_implemented = 1
boolrecipe_compare_semantic_command_owner = 1
hako_semantic_command_surface = 1
rust_oracle_parity = 1
aot_exe_guard = 1
downstream_boundary_present = 1
```

## Non-Claims

```text
hako_adopted_decision = 0
source_selfhost_claim = 0
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
