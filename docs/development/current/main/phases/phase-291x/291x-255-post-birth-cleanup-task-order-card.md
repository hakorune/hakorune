---
Status: Active
Date: 2026-04-25
Scope: Fix the post-birth-prune cleanup order before taking the next `.inc` mirror task.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-254-birth-emit-kind-prune-card.md
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-255 Post-Birth Cleanup Task Order Card

## Goal

Make the remaining compiler-cleanup tasks explicit after the `birth`
generic-method emit-kind row was pruned.

This is a planning / task-order card. It does not change `.inc` behavior.

## Current Baseline

The `.inc` no-growth guard currently tracks:

```text
classifiers=14
rows=14
```

Remaining debt is split into two families:

- `mir_call_route_policy` receiver/method surface compat sentinels
- generic-method fallback rows for `set` / `has` / `get` / `len` / `push`

Do not mix those families in one implementation card.

## Task Order

1. `mir_call_receiver_surface ArrayBox` prune probe
   - Reason: constructor `birth` blocker is gone, but ArrayHas fallback may
     still require the row.
   - Outcome: prune if exact smokes pass; otherwise write a no-safe-prune card
     with the failing metadata-absent boundary.

2. `mir_call_receiver_surface MapBox` prune probe
   - Reason: direct MapBox get/has rows were already pruned, but route-surface
     fallback may still be a sentinel.
   - Outcome: prune or exact no-safe-prune evidence.

3. `mir_call_receiver_surface RuntimeDataBox` review
   - Reason: RuntimeData fallback rows are broader and should not be deleted
     by analogy.
   - Outcome: likely keep-review unless metadata-absent RuntimeData boundaries
     are covered.

4. `has` family cleanup
   - Includes generic `mname == "has"`, ArrayBox has-route row,
     RuntimeDataBox has-route row, and mir-call method-surface `has`.
   - Must be handled as one family, but with one prune/review decision per row.

5. `len` family cleanup
   - Includes ArrayBox / MapBox len-route fallback rows.
   - Requires metadata-absent len boundary coverage before deletion.

6. `push` family cleanup
   - Includes ArrayBox push-route row and RuntimeDataBox push-route row.
   - Requires mutating metadata-absent boundary coverage.

7. `set` family cleanup
   - Includes generic `mname == "set"` and RuntimeDataBox set-route fallback.
   - Keep separate from push/len because value-shape and storage-route
     decisions are coupled to Set.

8. RuntimeData `get` fallback cleanup
   - Keep after receiver-surface and has/set reviews because RuntimeData get
     participates in mixed i64-or-handle facade semantics.

## Per-Task Acceptance

Each implementation card must include:

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Run targeted smokes for the affected family first. Run `tools/checks/dev_gate.sh
quick` before closing an implementation card.

## Boundary

- Do not add new CoreMethod rows in these cleanup cards.
- Do not add hot lowering while pruning fallback rows.
- Do not delete RuntimeData fallback rows by analogy with ArrayBox / MapBox.
- If a prune fails, land a no-safe-prune review card with the exact boundary
  instead of adding a workaround.
