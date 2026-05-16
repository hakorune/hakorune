# 293x-478 MIMAP-040C Post-Diagnostics Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-040C` is the planning-only row after `PURE-FIRST-DIAG-001`.

It must select exactly one next row.

It must not land code.

## Candidate Set

```text
candidate:
  continue with the next narrow allocator behavior row now that selectPage loop
  cleanup and route diagnostics are landed
candidate:
  select a focused probe for the next facade/object queue consumer if compiler
  acceptance is uncertain
candidate:
  select a compiler/language sidecar only if the next allocator row exposes a
  new independent acceptance blocker
candidate:
  select MIR-ROW-D only if dense queue field-read proof is the next actual
  blocker
```

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless explicitly reopened
- keep BoxShape cleanup separate from allocator behavior
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
