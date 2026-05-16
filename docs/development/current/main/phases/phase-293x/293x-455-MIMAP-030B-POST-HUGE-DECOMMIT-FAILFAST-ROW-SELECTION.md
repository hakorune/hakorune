# 293x-455 MIMAP-030B Post-Huge-Decommit-Failfast Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-030B` is a planning-only row. It selects exactly one next allocator
behavior row after the landed MIMAP-030A duplicate/stale huge decommit
fail-fast diagnostics route.

It must not land code.

The next row should be chosen from the smallest behavior that advances the
allocator lifecycle without opening provider activation or host allocator
replacement:

```text
candidate:
  MIMAP-031A OSVM unreserve capability inventory / planning row
candidate:
  facade huge decommit state-marker reuse / verifier-contract row
candidate:
  post-decommit stats/diagnostic observer row
candidate:
  explicit park row if unreserve/recommit still needs more substrate inventory
```

## Selection Criteria

The selected row must:

- build directly on MIMAP-029A and MIMAP-030A evidence
- keep allocator-provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive
- keep backend `.inc` matcher shortcuts forbidden
- avoid adding unreserve/recommit behavior unless the row is explicitly scoped
  as an inventory / planning row
- keep duplicate/stale decommit fail-fast allocator-side, not page-source-owned

## Candidate Template

The closeout for this card should fill in:

```text
row:
owner:
proof app:
guard:
reused owners:
primary proof:
stop lines:
```

If the next row needs a compiler/language sidecar, this card must name the
sidecar and keep allocator implementation parked until the sidecar is green.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `030B.1` | Review MIMAP-029A / MIMAP-030A closeout evidence. | Next row does not repeat already-landed decommit success/fail-fast proof. | no code |
| `030B.2` | Pick exactly one next allocator row. | Owner/proof/guard/stop lines are named. | no broad provider work |
| `030B.3` | Update current pointers. | Current state moves to the selected next row. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next allocator behavior row is selected with clear
owner/proof/guard names and provider/host allocator replacement still inactive.
