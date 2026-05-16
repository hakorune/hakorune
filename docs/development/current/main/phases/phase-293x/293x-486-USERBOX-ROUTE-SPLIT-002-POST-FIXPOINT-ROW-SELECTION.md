# 293x-486 USERBOX-ROUTE-SPLIT-002 Post-Fixpoint Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`USERBOX-ROUTE-SPLIT-002` is the planning-only row after the landed
`USERBOX-ROUTE-SPLIT-001` fixed-point orchestration cleanup.

It must select exactly one next row.

It must not land code.

## Candidate Set

```text
candidate:
  split user_box_method_route_plan target collection / materialization if it
  can stay behavior-preserving
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
