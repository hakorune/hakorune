# 293x-484 VMHAKO-EXTERN-SPEC-002 Post-Subset-Validator Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`VMHAKO-EXTERN-SPEC-002` is the planning-only row after the landed
`VMHAKO-EXTERN-SPEC-001` subset legacy externcall spec validator cleanup.

It must select exactly one next row.

It must not land code.

## Candidate Set

```text
candidate:
  split user_box_method_route_plan fixed-point orchestration into a narrow
  behavior-preserving owner
candidate:
  thin mir builder expression dispatcher without changing accepted AST shapes
candidate:
  introduce record_values common registration helper without adding record
  acceptance
candidate:
  continue the next narrow allocator behavior row if no compiler cleanup is
  blocking readability
candidate:
  clean OSVM export validation boilerplate only if it stays kernel-local
```

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened
- keep BoxShape cleanup separate from allocator behavior
- avoid broad planner/validator rewrites
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
