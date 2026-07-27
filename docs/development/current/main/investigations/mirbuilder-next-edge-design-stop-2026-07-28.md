---
Status: accepted
Date: 2026-07-28
Decision: LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0
Scope: select the first exact production edge after the three fixed cutovers
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

The three fixed production cutovers are closed. The manifest has no remaining
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

## Resolution

Accepted after a four-worker bounded census:

```text
LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0
```

Execution authority:

```text
docs/development/current/main/investigations/local-statement-descent-cutover0-i0-r0-task-2026-07-28.md
```

The selected live raw/default Local selector is exactly one. The generic owner
also has one detached located caller with production root ingress zero; the
execution card guards these separately instead of making a false global
caller-count claim.

## Minimum implementation slice after decision

One manifest row, one named production caller switch, one selected old edge
deletion, one focused parity fixture, and the existing shared guard update.
No new guard, macro pack, language/backend behavior, special activation,
fallback, or unrelated cleanup may be added.

## Non-claims

- selection does not close the fourth production row
- no macro pack is closed
- no detached asset is promoted
- no Stage-B, Ownership, selfhost, language, runtime, or backend work resumes
