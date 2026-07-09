# 3453 - DOCS-MIRBUILDER-TASK-ORDER-POINTER-SLIM-001

## Purpose

Restore the MirBuilder task-order document as a current-only restart entry.
This is a BoxShape cleanup. It changes no compiler, route, runtime, backend,
mutation, publication, Delete, wide, or Source Selfhost authority.

## Result

The task-order now contains only the current blocker, current evidence,
consultation frontier, invariants, and durable history pointers. The obsolete
landed-history ledger and concatenated `Active Next 3` chain were removed.

History remains available through phase cards, the Source Selfhost history
JSONL, the frozen compatibility snapshot, policy SSOTs, and git history.

## Guard

`current_state_pointer_guard.sh` now enforces both a 1,000-line active-doc
budget and a 500-character maximum task-order line. This rejects future
history packing into a single long line.

## Non-Claims

```text
compiler_behavior_change = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0
```
