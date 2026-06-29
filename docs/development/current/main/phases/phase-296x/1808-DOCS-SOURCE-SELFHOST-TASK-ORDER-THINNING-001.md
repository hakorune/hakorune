# 1808 - DOCS-SOURCE-SELFHOST-TASK-ORDER-THINNING-001

## Token

```text
DOCS-SOURCE-SELFHOST-TASK-ORDER-THINNING-001
```

## Purpose

Keep the MirBuilder task-order SSOT as a current queue pointer, not a landed
history ledger.

Detailed row history now lives in phase cards, git history, and the reusable
Source Selfhost family guard manifest.

## Boundary

```text
does:
  shorten selected decision / evidence chains
  keep active blocker and Active Next 3 visible
  preserve guard-readable tokens

does not:
  drop evidence
  select a family
  open route repair
  claim Source Selfhost
```

## Acceptance

```text
task_order_lines < 800
current_state_pointer_guard = green
source_selfhost_family_guard = green
current_blocker_token =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Non-Claims

```text
no route repair
no family adoption decision
no wider route selection
no Source Selfhost claim
```
