# 293x-488 USERBOX-ROUTE-SPLIT-004 Post-Materialization Row Selection

Status: landed
Date: 2026-05-16

## Decision

`USERBOX-ROUTE-SPLIT-004` is the planning-only row after the landed
`USERBOX-ROUTE-SPLIT-003` materialization owner cleanup.

It selects exactly one next row:

```text
USERBOX-ROUTE-SPLIT-005:
  split user_box_method_route_plan target collection into a narrow
  behavior-preserving owner
```

It does not land code.

## Candidate Set

```text
candidate:
  split user_box_method_route_plan target collection into a narrow
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
- keep BoxShape cleanup separate from allocator behavior
- avoid adding, removing, or renaming accepted language/compiler shapes
- avoid broad planner/validator rewrites
- preserve pure-first diagnostics layer/contract output
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with a clear owner, stop lines,
and evidence plan.

## Selection Result

```text
selected:
  USERBOX-ROUTE-SPLIT-005
owner:
  src/mir/user_box_method_route_plan/target_collection.rs
scope:
  target fact collection and method-symbol helpers only
stop_line:
  no materialization changes
  no accepted route shape changes
  no route reason vocabulary changes
```
