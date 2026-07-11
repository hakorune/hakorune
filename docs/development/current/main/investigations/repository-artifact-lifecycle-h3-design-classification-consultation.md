---
Status: Active design consultation stop (C2 owner-family review)
Date: 2026-07-11
Owner: repository-artifact-lifecycle-current.md
Decision: C1 accepted; C2 pending
---

# H3 Design Registry Classification Consultation

## Current Evidence

```text
design direct files = 849
registered rows = 5
unregistered files = 844
warning baseline = 844
registry violations = 0

accepted roles:
  authority
  navigation
  supporting
  status-ledger
  superseded
```

The registry schema, no-growth rule, precedence-cycle guard, sidecar ownership,
and README navigation boundary are implemented. What remains is semantic role
classification; filename/status/reference popularity cannot decide it safely.

## C1 Closeout (Accepted)

C1 used only explicit README section evidence. The reviewed rows are now in
`design/INDEX.md` with `classification_basis` recorded per row. No file move
was performed, and the remaining backlog stays warning-unregistered.

```text
c1_review_basis = explicit README section evidence
c1_review_rows = 112
c1_role_counts = authority:107, supporting:2, status-ledger:3
registered_rows = 117
unregistered_current = 732
unregistered_baseline = 732
registry_violations = 0
design_file_move_started = 0
```

C2 is now the active stop: remaining files are grouped by explicit owner
family, and ambiguous families require a focused consultation before a role
or physical move is assigned.

## Proposed Classification Order

```text
C1 root authority review:
  DOCS_LAYOUT / AGENTS / CURRENT_STATE / INDEX seed union

C2 owner-family review:
  group remaining files by explicit owner/prefix family
  select one authority spine per family
  classify explanations as supporting
  classify mutable ledgers as status-ledger

C3 supersession review:
  require superseded_by
  require root-reachability/reference closure
  then move to design/superseded

C4 strict closeout:
  every direct file is a row or owned sidecar
  unregistered = 0
  README projection checked/generated from INDEX
```

## Questions

```text
1. May C1 classify only documents explicitly named by current root surfaces,
   leaving the rest warning-unregistered until owner-family review?

2. Is one authority spine per owner family the required default, with multiple
   authority rows allowed only when precedence_parent makes the split explicit?

3. Should `*-closeout*`, `*-inventory*`, `*-report*`, and mutable TOML proof
   artifacts default to review candidates for status-ledger, never automatic
   classification?

4. Should an owner-family with no clear authority spine stop for a focused
   consultation instead of assigning supporting by heuristic?

5. Is physical movement forbidden until the entire superseded row's incoming
   reachable-reference set is zero, even when its owner family is otherwise
   classified?
```

## Recommended Answer

Accept all five. Classification should advance in reviewed owner-family
batches; suffixes and reference counts generate queues only and never assign
roles.

## Minimum Next Slice After Acceptance

```text
1. Generate deterministic C1 review queue.
2. Add reviewed C1 rows to INDEX.
3. Lower unregistered baseline to the new exact count.
4. Verify no precedence cycle, orphan sidecar, or README authority drift.
5. Do not move design files in C1.
```

## Non-Claims

```text
design_registry_complete = 0
design_registry_decided = 1
unregistered_design_files = 732
heuristic_role_assignment = 0
design_file_move_started = 0
strict_design_registry_guard = 0
failure_outcome_design_accepted = 0
selfhost_claim = 0
```
