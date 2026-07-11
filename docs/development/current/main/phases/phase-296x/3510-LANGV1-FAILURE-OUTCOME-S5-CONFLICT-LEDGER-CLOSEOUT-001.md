---
Status: Active implementation task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted inventory-only closeout continuation
---

# 3510 - LANGV1-FAILURE-OUTCOME-S5-CONFLICT-LEDGER-CLOSEOUT-001

## Objective

Make the known Failure/Outcome contradictions queryable and close the
inventory-only workstream without changing semantic carriers or runtime
behavior.

## Required Ledger Rows

```text
null_vs_void
local_default_null
weak_upgrade_to_void
env_missing_or_error_to_void
clock_failure_to_zero
missing_box_void_compatibility
canonical_literal_null
postfix_catch_vs_fault
```

Each row must name evidence references, conflict kind, current status, and a
non-activation next decision. A conflict row must not silently select a
semantic owner.

## Acceptance

```text
all eight conflicts are machine-readable
each row has evidence and a closed status vocabulary
S1-S4 checks remain green
semantic activation = 0
runtime/parser/MIR/backend behavior changed = 0
```

## Commands

```bash
python3 tools/docs/failure_outcome_conflict_ledger.py --check
python3 -m unittest tools/docs/test_failure_outcome_conflict_ledger.py
python3 tools/docs/failure_outcome_exhaustiveness.py --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Stop Boundary

After the ledger is complete, stop before selecting the first semantic
activation boundary. Any choice among Unit, absence, Err, Fault, CompatNull,
or ForeignNull requires a separate design consultation.
