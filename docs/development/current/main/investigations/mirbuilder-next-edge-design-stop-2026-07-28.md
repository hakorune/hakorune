---
Status: active design stop
Date: 2026-07-28
Decision: pending sixth production edge
Scope: select the next exact production edge after five closed cutovers
---

# MirBuilder next production edge design stop

## Authority

- pack order and replacement law:
  `design/mirbuilder-inplace-replacement-policy-ssot.md`
- active workstream counters:
  `workstreams/mirbuilder-inplace-replacement-current.md`
- closed cells and detached assets:
  `design/fixtures/mirbuilder-inplace-replacement-v1.tsv`
- finite scope and non-claims:
  `investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md`

Historical phase cards, parked Stage-B activation, and disconnected assets are
not selection authority.

## Current boundary

Five production cutovers are closed. The manifest has no remaining
scheduled production cell, while all eight macro packs remain open. No source
edit may begin until one exact existing caller, replacement owner, delete
target, parity gate, and LOC repayment boundary are selected together.

Fail fast if a proposed row is inventory-only, has no selected old edge, needs
fallback/retry, revives special Stage-B activation, or mixes BoxCount with
BoxShape.

## Candidates

1. `REPLACEMENT-LEDGER0` bounded census, then select one historical live
   replacement whose new caller and old selected branch can both be guarded.
2. `DESCENT-SPINE0` select the next direct recursion/facade edge after the
   callable body cutover.
3. `FUNCTION-STATE0` select one duplicate function-state writer only if the
   ledger proves an existing production caller can switch in the same cell.

Recommended next decision: candidate 1. It follows the fixed pack order and
prevents naming an implementation row before the finite ledger has an exact
production edge.

## Previous resolution

The previous design stop accepted after a four-worker bounded census:

```text
LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0
```

Execution authority:

```text
docs/development/current/main/investigations/local-statement-descent-cutover0-i0-r0-task-2026-07-28.md
```

That cell is now closed. Its live raw/default Local selector is exactly one,
both old facades are physically absent, and the detached located caller remains
root-inactive. The shared guard owns the production-edge proof.

No fifth production edge is selected by that closeout. A fresh bounded census
must select the next exact caller/owner/delete-target/parity boundary.

Consultation packet:

```text
docs/development/current/main/investigations/
mirbuilder-fifth-production-edge-consultation-2026-07-28.md
```

## Fifth resolution

The consultation selected and the implementation closed:

```text
VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0-I0-R0
```

The execution card is:

```text
docs/development/current/main/investigations/
variable-assignment-descent-cutover0-i0-r0-task-2026-07-28.md
```

The cell owns the complete bounded raw/default variable-name reassignment
caller set: exact Variable target plus GroupedAssignmentExpr. One detached
located caller remains separately counted with production root ingress zero.
Both obsolete facades are physically absent and the shared guard owns the
production-edge proof.

No sixth production edge is selected. A fresh bounded census must select the
next exact caller/owner/delete-target/parity/LOC boundary.

## Minimum implementation slice after decision

One manifest row, one named production caller switch, one selected old edge
deletion, one focused parity fixture, and the existing shared guard update.
No new guard, macro pack, language/backend behavior, special activation,
fallback, or unrelated cleanup may be added.

## Non-claims

- no sixth production row is selected
- no macro pack is closed
- no detached asset is promoted
- no Stage-B, Ownership, selfhost, language, runtime, or backend work resumes
