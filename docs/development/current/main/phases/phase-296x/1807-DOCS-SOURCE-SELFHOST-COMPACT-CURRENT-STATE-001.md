# 1807 - DOCS-SOURCE-SELFHOST-COMPACT-CURRENT-STATE-001

## Token

```text
DOCS-SOURCE-SELFHOST-COMPACT-CURRENT-STATE-001
```

## Purpose

Compact `CURRENT_STATE.toml` back into a live pointer file.

The previous file had grown into a landed-history ledger. That made every
current pointer update expensive and increased conflict risk. Detailed history
belongs in phase cards and git history, not in the current-state pointer.

## Boundary

```text
does:
  keep live current pointers
  keep a short landed tail
  keep the Source Selfhost design stop active

does not:
  change semantic route state
  select a family
  open route repair
  claim Source Selfhost
```

## Acceptance

```text
current_state_pointer_guard = green
CURRENT_STATE.toml stays compact
task_order_lines < 800
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
