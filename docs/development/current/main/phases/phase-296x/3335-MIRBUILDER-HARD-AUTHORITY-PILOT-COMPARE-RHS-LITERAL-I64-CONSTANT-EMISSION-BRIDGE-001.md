# 3335 - MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001

## Token

```text
MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-LITERAL-I64-CONSTANT-EMISSION-BRIDGE-001
```

## Purpose

Connect the selected post-3333 seam to the existing scoped `LiteralI64`
constant emission bridge.

This is the first current hard-authority pilot in this chain that opens a
mutation-bearing RHS `ValueId` resolution slice. The mutation scope is limited
to one integer `Const` instruction and its `ValueId` / type publication.

## Output Contract

```text
rust-lifecycle-mirbuilder-hard-authority-pilot-compare-rhs-literal-i64-constant-emission-bridge-v0
```

## Implemented Seam

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
```

## Evidence

The pilot uses the existing Rust owner:

```text
src/mir/builder/compare_rhs_valueid_resolution_bridge.rs
```

It reuses the existing guard:

```text
tools/checks/rust_lifecycle_mirbuilder_compare_rhs_literal_i64_constant_emission_bridge_gate.sh
```

## Claims

```text
hard_authority_pilot_implemented = 1
compare_rhs_literal_i64_const_emission_bridge_owner = 1
actual_rhs_valueid_resolution_literal_i64 = 1
literal_constant_valueid_allocation = 1
constant_mir_emission = 1
integer_type_publication = 1
mutation_performed_const_only = 1
```

## Non-Claims

```text
hako_adopted_decision = 0
source_selfhost_claim = 0
native_seed_materialization = 0
actual_rhs_valueid_resolution_general = 0
symbol_ref_valueid_resolution = 0
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

