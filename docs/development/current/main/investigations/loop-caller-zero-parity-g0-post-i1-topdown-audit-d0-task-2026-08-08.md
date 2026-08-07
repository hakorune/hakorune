# LOOP-CALLER-ZERO-PARITY-G0-POST-I1-AUDIT-D0

Status: `next design-only row`
Date: `2026-08-08`
Parent: `docs/development/current/main/investigations/loop-caller-zero-parity-g0-i1-r0-task-2026-08-08.md`
North star: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`

## Objective

After the caller-zero Generic G0 I1 canary is closed, choose the smallest
remaining prerequisite before any production selection or legacy retirement.
This is a top-down design audit, not an implementation row. It must decide
whether the next bounded work is an M8 Recipe cohort, M9 selfhost parity, or a
production-selection admission/deletion design. Do not open a physical route
while this choice is unresolved.

## Audit inputs

```text
G0 I1 implementation receipt
M8 all-19 portable Recipe SSOT and task ladder
M9 selfhost parity SSOT and task ladder
M10b production cutover/deletion manifest contract
current caller census and exact legacy edges
```

## Required output

Produce one accepted decision containing:

```text
selected next row and sole owner
exact source/facts/Recipe or admission input
Builder-effect boundary
positive and negative fixtures
same-commit legacy/fallback deletion boundary, if applicable
exact docs/reference and README update set
non-claims for the rows not selected
```

The audit must preserve the existing pipeline:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify
        -> Lower -> Seal -> Collect -> Atomic Publish
```

It must not introduce a second selector, a G0-specific physicalizer, a new
CFG/SSA/PHI owner, runtime retry/fallback, or a second pipeline SSOT.

## Stop lines

```text
implementation/code edits = 0
production caller switch = 0
M10b activation = 0
M11/M12 deletion = 0
same-session retry/fallback = 0
collector/module publication change = 0
backend/performance claim = 0
```

## Acceptance

- the next row is selected from current repository evidence, not task-name
  history;
- the chosen row has one exact owner, one input contract, one rejection seam,
  and one bounded fixture/gate;
- M8, M9, and M10b/M11/M12 remain explicitly classified as selected,
  deferred, or blocked;
- the current pointer, workstream, and relevant reference are updated in the
  same design commit;
- any later implementation row must update its exact `docs/reference/**` and
  affected README in the same commit, with a final reference update again at
  production cutover.
