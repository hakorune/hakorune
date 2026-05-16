# 293x-476 MIMAP-040B Post-SelectPage-Loop Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-040B` is the planning-only row after the landed `MIMAP-040A`
object-lifecycle `selectPage()` queue-length loop cleanup.

It must select exactly one next row.

It must not land code.

## Candidate Set

```text
candidate:
  continue with the next narrow allocator behavior row now that fixed
  object-lifecycle queue selection is gone
candidate:
  run a focused probe for the next facade/object queue consumer before selecting
  the behavior row if compiler acceptance is uncertain
candidate:
  select a compiler/language sidecar only if the next allocator row exposes a
  new independent acceptance blocker
candidate:
  select MIR-ROW-D only if dense queue field-read proof becomes the next actual
  blocker
```

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened
- keep BoxShape cleanup separate from allocator behavior
- avoid reintroducing fixed-slot queue selection or backend `.inc` matchers

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with clear owner/proof/guard names
and provider/host allocator replacement still inactive unless explicitly
reopened.
