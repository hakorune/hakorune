# 1809 - DOCS-CHECK-INDEX-FAMILY-VIEW-001

## Token

```text
DOCS-CHECK-INDEX-FAMILY-VIEW-001
```

## Purpose

Move Source Selfhost check-index usage toward a family-view model.

The stable public entry is now:

```text
tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```

Historical row guards remain callable, but new Source Selfhost rows should not
require one check-index entry per row unless the row owns nontrivial build,
code, or perf validation.

## Boundary

```text
does:
  make row guards depend on the family guard index entry
  document row-specific entries as legacy traceability

does not:
  delete historical row guards
  remove traceability
  change semantic route state
```

## Acceptance

```text
source_selfhost_family_guard = green
current_state_pointer_guard = green
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
