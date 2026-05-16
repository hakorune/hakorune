# 293x-480 MIMAP-041B Post-Record-Report Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-041B` is the planning-only row after the landed `MIMAP-041A` record
report boundary cleanup.

It must select exactly one next row.

It must not land code.

## Candidate Set

```text
candidate:
  continue with the next narrow allocator behavior row now that the bounded
  scheduler report boundary is record-shaped
candidate:
  select one more report cleanup only if it is the next concrete readability
  blocker and does not broaden into a report sweep
candidate:
  select a focused compiler/language sidecar only if the next allocator row
  exposes a new independent acceptance blocker
candidate:
  select a usize field-group migration only if the owner and sentinel policy
  are explicitly named first
candidate:
  select a rune/verifier promotion only if it is independent from allocator
  behavior and has a verifier owner
```

## Selection Criteria

The selected row must:

- name one owner, proof/guard, and stop lines before implementation
- keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive unless an explicit provider ladder is reopened
- keep BoxShape cleanup separate from allocator behavior
- avoid broad report sweeps, broad usize migration, or broad rune promotion
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

`MIMAP-041B` selects `MIR-EXTERN-SPEC-001`.

Rationale:

- `src/mir/extern_call_route_plan.rs` owns route facts for accepted extern
  calls, but each route kind currently repeats route id, core op, symbol,
  return shape, demand, effect tags, arity, and value-argument policy across
  multiple `match self` blocks.
- This creates drift risk whenever hako.mem / hako.osvm / hako.tls /
  hako.atomic rows are extended.
- The first cleanup should create one MIR-owned route spec table. Later rows
  can reuse that table from subset validation without mixing two cleanups.

Selected row:

```text
row:
  MIR-EXTERN-SPEC-001 extern-call route spec table
owner:
  src/mir/extern_call_route_plan.rs
primary tests:
  src/mir/extern_call_route_plan/tests.rs
guard evidence:
  tools/checks/dev_gate.sh quick
stop lines:
  no route semantics change
  no new extern routes
  no backend lowering change
  no subset validator rewrite in this row
  no allocator behavior/provider activation
```

Closeout:

```text
current blocker moves to MIR-EXTERN-SPEC-001 extern-call route spec table.
```
