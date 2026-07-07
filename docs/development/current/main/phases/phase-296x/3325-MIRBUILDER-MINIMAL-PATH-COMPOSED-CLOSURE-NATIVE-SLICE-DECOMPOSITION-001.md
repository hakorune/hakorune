# 3325 - MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001

## Token

```text
MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
```

## Purpose

Decompose the minimal path composed closure into selector-ready hard authority
candidate rows using the consultation-approved `HardAuthoritySeamProofAxis`.

This card does not select the hard authority pilot and does not implement a
new owner. It creates the input for the next policy card:

```text
MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001
```

## Output Contract

```text
rust-lifecycle-mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v2
```

## Decomposition Output

```text
authority_seams = present
owner_dependency_graph = present
minimal_path_required_owner_set = present
first_hard_authority_candidate_selector_input = present
rejection_reasons_for_each_candidate = present

eligible_hard_authority_candidate_count = 1
selected_candidate_for_policy = BoolRecipeCompareSemanticCommandBoundary
```

## Decision

```text
decision:
  SelectHardAuthorityPilotPolicy

reason_token:
  ExactlyOneHardAuthoritySeamCandidateFromRegistryDecomposition

selected_next_card:
  MIRBUILDER-AUTHORITY-FACADE-HARD-AUTHORITY-PILOT-POLICY-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v2.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_minimal_path_composed_closure_native_slice_decomposition_v2_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
hard_authority_pilot_selected = 0
hard_authority_pilot_implemented = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
mir_mutation = 0
id_allocation = 0
new_backend_route = 0
new_abi = 0
```
