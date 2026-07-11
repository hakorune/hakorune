# 293x-490 USERBOX-ROUTE-SPLIT-006 Post-Target-Collection Row Selection

Status: landed
Date: 2026-05-16

## Decision

`USERBOX-ROUTE-SPLIT-006` is the planning-only row after the landed
`USERBOX-ROUTE-SPLIT-005` target collection owner cleanup.

It selects exactly one next row:

```text
RECORD-VALUES-REG-001:
  introduce a common builder-local record field/register helper without adding
  record acceptance
```

It does not land code.

## Candidate Set

```text
candidate:
  thin mir builder expression dispatcher without changing accepted AST shapes
candidate:
  introduce record_values common registration helper without adding record
  acceptance
candidate:
  continue the next narrow allocator behavior row if compiler route cleanup is
  no longer blocking readability
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
  RECORD-VALUES-REG-001
owner:
  src/mir/builder/record_values.rs
scope:
  builder-local record field construction and placeholder registration helper
stop_line:
  no record acceptance changes
  no record escape/materialization
  no backend/provider behavior
```
