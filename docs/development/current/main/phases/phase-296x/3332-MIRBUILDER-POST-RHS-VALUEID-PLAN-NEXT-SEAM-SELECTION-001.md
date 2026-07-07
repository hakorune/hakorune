# 3332 - MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001

## Token

```text
MIRBUILDER-POST-RHS-VALUEID-PLAN-NEXT-SEAM-SELECTION-001
```

## Purpose

Consume the `CompareRhsValueIdResolutionPlanBoundary` hard-authority pilot
evidence and select the next read-only hard-authority seam.

This card selects the request ABI boundary only. It does not open actual RHS
`ValueId` resolution, constant emission, SymbolRef lookup, LocalSSA, MIR
emission, route selection, or Source Selfhost.

## Output Contract

```text
rust-lifecycle-mirbuilder-post-rhs-valueid-plan-next-seam-selection-v0
```

## Selected Seam

```text
candidate:
  CompareRhsValueIdResolutionRequestAbiBoundary

owner:
  CompareRhsValueIdResolutionRequestSnapshotBox

input_surface:
  CompareRhsValueIdResolutionPlanSnapshotV1

output_surface:
  CompareRhsValueIdResolutionRequestSnapshotV1

downstream_boundary:
  CompareRhsValueIdResolutionResponseV1
```

## Evidence

```text
owner:
  lang/src/compiler/mirbuilder/compare_rhs_valueid_resolution_request_snapshot.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-compare-rhs-valueid-resolution-request-response-abi-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_compare_rhs_valueid_resolution_request_response_abi_gate.sh
```

## Claims

```text
post_rhs_valueid_plan_next_seam_selected = 1
compare_rhs_valueid_resolution_request_abi_selected = 1
rhs_valueid_plan_pilot_evidence_consumed = 1
```

## Non-Claims

```text
next_seam_implemented = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
native_seed_materialization = 0
actual_rhs_valueid_resolution = 0
literal_constant_valueid_allocation = 0
constant_mir_emission = 0
symbol_lookup_execution = 0
local_ssa_finalize_compare_execution = 0
mir_cmp_emission = 0
branch_emission = 0
basic_block_mutation = 0
value_id_allocation = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Next

```text
MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-VALUEID-REQUEST-ABI-001
```
