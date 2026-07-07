# 3318 - MIRBUILDER-COMPARE-BRANCH-EMISSION-DESIGN-STOP-001

## Purpose
Stop before consuming a Compare result as a branch condition.

## Boundary
Previous cards can now produce:

```text
Compare result ValueId with Bool type publication
```

This card selects a separate Branch emission bridge for the next slice. That
bridge may finalize the branch condition and emit a conditional Branch
terminator, but it must not select a route, switch runtime authority, or claim
ProgramJSON authority.

## Selected Next
```text
MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001
```

## Explicit Non-Claims
- Branch emission execution: `0`
- Branch condition consumption: `0`
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_branch_emission_design_stop_guard.sh
```

## Closeout

Guard result: green.

```text
output_contract=rust-lifecycle-mirbuilder-compare-branch-emission-design-stop-v0
decision=SelectBranchEmissionBridgeFirst
compare_branch_emission_design_stop=1
branch_emission_bridge_selected=1
branch_emission_execution=0
branch_condition_consumption=0
route_selection=0
runtime_route_switch=0
programjson_runtime_authority=0
runtime_fallback=0
source_selfhost_claim=0
selected_next_card=MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001
```
