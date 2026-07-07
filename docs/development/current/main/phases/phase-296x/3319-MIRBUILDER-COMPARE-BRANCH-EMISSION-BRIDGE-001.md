# 3319 - MIRBUILDER-COMPARE-BRANCH-EMISSION-BRIDGE-001

## Purpose
Open scoped Branch emission after Compare result creation.

## Boundary
This card consumes an already-emitted Compare result `ValueId` as a branch
condition, then calls the existing Rust owners:

```text
ssa::local::finalize_branch_cond(condition)
emission::branch::emit_conditional(condition, then_bb, else_bb)
```

The bridge writes a conditional Branch terminator only. It does not select a
route, switch runtime authority, or claim ProgramJSON authority.

## Positive Claims
- `compare_branch_emission_bridge = 1`
- `branch_condition_consumption = 1`
- `localssa_finalize_branch_cond_execution = 1`
- `branch_emission_execution = 1`

## Explicit Non-Claims
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_branch_emission_bridge_gate.sh
```
