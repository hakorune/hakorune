# 3334 - MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001

## Token

```text
MIRBUILDER-POST-RHS-VALUEID-REQUEST-ABI-NEXT-SEAM-SELECTION-001
```

## Purpose

Consume the 3333 request ABI hard-authority pilot and select the next concrete
seam without treating Source Selfhost, route selection, or runtime authority as
opened.

The selected seam is the existing scoped `LiteralI64` constant emission bridge.
This card is selection-only: it does not itself perform actual RHS `ValueId`
resolution, allocation, or MIR mutation.

## Output Contract

```text
rust-lifecycle-mirbuilder-post-rhs-valueid-request-abi-next-seam-selection-v0
```

## Selected Next Seam

```text
candidate:
  CompareRhsLiteralI64ConstantEmissionBridgeBoundary

owner:
  CompareRhsConstantEmissionBridge

input_surface:
  CompareRhsValueIdResolutionRequestSnapshotV1

output_surface:
  CompareRhsValueIdResolutionResponseV1

mutation_scope:
  ConstInstructionOnly

selected_next_card:
  MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001
```

## Basis

The selected seam is backed by the already-green bridge gate:

```text
tools/checks/rust_lifecycle_mirbuilder_compare_rhs_literal_i64_constant_emission_bridge_gate.sh
```

This selector only records that the bridge is the next scoped seam after the
request ABI boundary.

## Claims

```text
post_rhs_valueid_request_abi_next_seam_selected = 1
request_abi_pilot_evidence_consumed = 1
literal_i64_const_emission_bridge_selected = 1
existing_literal_i64_bridge_guard_green = 1
```

## Non-Claims

```text
next_seam_implemented = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
native_seed_materialization = 0
actual_rhs_valueid_resolution_literal_i64 = 0
literal_constant_valueid_allocation = 0
constant_mir_emission = 0
symbol_lookup_execution = 0
local_ssa_finalize_compare_execution = 0
mir_cmp_emission = 0
branch_emission = 0
basic_block_control_flow_mutation = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

