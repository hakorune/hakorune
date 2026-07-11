# 293x-482 MIR-EXTERN-SPEC-002 Post-Extern-Spec Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIR-EXTERN-SPEC-002` is the planning-only row after the landed
`MIR-EXTERN-SPEC-001` extern-call route spec table cleanup.

It selects exactly one next row:

```text
VMHAKO-EXTERN-SPEC-001:
  reuse ExternCallRouteSpec from vm-hako subset legacy externcall validation
```

It does not land code.

## Candidate Set

```text
candidate:
  reuse ExternCallRouteSpec from vm-hako subset externcall validation so
  hako_osvm / hako_intrin / substrate externcall knowledge stops drifting
candidate:
  continue with the next narrow allocator behavior row if no compiler cleanup
  is blocking readability
candidate:
  select a user-box method route planner split only if it can be kept
  behavior-preserving and one owner at a time
candidate:
  select record_values common helper cleanup only if it stays local and does
  not add new record acceptance
```

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened
- keep BoxShape cleanup separate from allocator behavior
- avoid broad validator rewrites unless the spec table is the only policy owner
- preserve pure-first diagnostics layer/contract output

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with clear owner/proof/guard names
and provider/host allocator replacement still inactive unless explicitly
reopened.

## Selection Result

```text
selected:
  VMHAKO-EXTERN-SPEC-001
owner:
  src/runner/reference/vm_hako/subset_check/externcalls.rs
scope:
  legacy externcall validation only
  route-backed rows already present in ExternCallRouteSpec only
stop_line:
  do not broaden accepted externcall symbols
  do not add hako_intrin/page_size rows to ExternCallRouteSpec in this row
  do not change mir_call validation
```
