# 293x-478 MIMAP-040C Post-Diagnostics Row Selection

Status: landed
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

## Selection Result

`MIMAP-040C` selects `MIMAP-041A`.

Rationale:

- `PURE-FIRST-DIAG-001` makes compiler-route failures easier to distinguish,
  so the next step can return to narrow source cleanup without hiding
  acceptance blockers.
- The bounded purge/decommit scheduler report still carries a 16-argument
  `report(...)` helper even though record literal construction/read is now an
  accepted Stage1 surface.
- This is a BoxShape cleanup, not allocator behavior. It should prove whether
  local record literals can document report field groups without record escape
  or backend materialization.

Selected row:

```text
row:
  MIMAP-041A record report boundary cleanup
owner:
  lang/src/hako_alloc/memory/purge_bounded_scheduler_box.hako
primary guard:
  tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
primary proof:
  apps/hako-alloc-bounded-purge-decommit-scheduler-proof/main.hako
stop lines:
  no allocator behavior change
  no broad report sweep
  no record pass/return/store escape
  no packed ArrayBox / record materialization / backend matcher
  no usize or rune promotion in this row
  no provider activation / host allocator replacement / hook / #[global_allocator]
```

Closeout:

```text
current blocker moves to MIMAP-041A record report boundary cleanup.
```
