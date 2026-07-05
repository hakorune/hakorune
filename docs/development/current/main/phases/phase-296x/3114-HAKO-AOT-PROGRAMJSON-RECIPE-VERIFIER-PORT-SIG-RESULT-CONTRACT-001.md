# 3114 - HAKO-AOT-PROGRAMJSON-RECIPE-VERIFIER-PORT-SIG-RESULT-CONTRACT-001

Status: green

## Scope

Close the RecipeVerifier / RecipePortSig AOT route-contract blocker exposed by
the PhaseState consumer probe.

This card claims only RecipeVerifierBox and RecipePortSigBox route metadata are
AOT-callable as DirectAbi scalar or map-handle routes. Full PhaseState parse
remains unclaimed.

## Implementation

- Annotated RecipeVerifier result-map helpers as `MapBox`.
- Annotated RecipePortSig helpers as `MapBox` / `i64`.
- Replaced dynamic-name map publication in PortSig with `CountOnlyPortSigV1`.
- Removed RecipeVerifier `BoxHelpers.array_len/1` routes in favor of local
  array length calls.
- Kept MIR mutation, route selection, lowering, ID allocation, and new ABI out
  of scope.

## Evidence

```bash
bash tools/checks/hako_aot_programjson_recipe_verifier_port_sig_result_contract_guard.sh
```

## Remaining Blocker

```text
ProgramJsonV0PhaseStateBox scan-body recursion
ProgramJsonV0RuneAttrsBox function-name normalization
```

Next selected card:

```text
HAKO-AOT-PROGRAMJSON-PHASE-STATE-SCAN-BODY-REC-RESULT-CONTRACT-001
```

## Non-Claims

```text
phase_state_parse_aot_call_fixed = 0
layer4_recipe_dto_parity_green = 0
source_selfhost_claim = 0
mir_mutation = 0
id_allocation = 0
backend_lowering_claim = 0
route_selection_migration = 0
runtime_route_switch = 0
new_backend_route = 0
new_abi = 0
```
