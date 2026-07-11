# 293x-476 MIMAP-040B Post-SelectPage-Loop Row Selection

Status: landed
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

## Selection Result

`MIMAP-040B` selects `PURE-FIRST-DIAG-001`.

Rationale:

- MIMAP-040A succeeded, but the row exposed that unsupported compiler shapes
  are still too easy to read as either "slow build" or "late backend failure".
- The next narrow step is a compiler diagnostics sidecar, not allocator
  behavior.
- The sidecar must make the pure-first acceptance layers explicit and classify
  missing semantic route contracts before LLVM/backend emission.

Selected row:

```text
row:
  PURE-FIRST-DIAG-001 pure-first acceptance layer diagnostics
owners:
  tools/checks/pure_first_route_preflight.py
  docs/development/current/main/design/pure-first-acceptance-layer-flow-ssot.md
guard:
  tools/checks/pure_first_route_preflight_guard.sh
primary proof:
  object_handle routes without target_result_box_name fail in semantic-route
  preflight with layer/contract diagnostics
stop lines:
  no backend lowering behavior change
  no allocator behavior change
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
```

Closeout:

```text
current blocker moves to PURE-FIRST-DIAG-001.
```
