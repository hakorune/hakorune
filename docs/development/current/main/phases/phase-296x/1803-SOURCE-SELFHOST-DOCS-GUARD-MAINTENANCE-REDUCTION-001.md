# 1803 - SOURCE-SELFHOST-DOCS-GUARD-MAINTENANCE-REDUCTION-001

## Token

```text
SOURCE-SELFHOST-DOCS-GUARD-MAINTENANCE-REDUCTION-001
```

## Purpose

Introduce a short maintenance phase before the next semantic Source Selfhost
slice. The goal is to reduce docs / guard update cost that now blocks progress
more often than the compiler semantics do.

This card does not reopen family selection. The Source Selfhost lane remains
stopped at:

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Worker Inventory

The worker inventory found two concrete maintenance problems:

```text
1. Source Selfhost row guards still pin live CURRENT_STATE latest-card values.
2. CURRENT_STATE / task-order / check index carry too much row-level history.
```

It also found one stale guard drift:

```text
rust_lifecycle_mirbuilder_minimal_path_mainline_readiness_resolver_guard.sh
  -> task-order missing: same-state composed prefix evidence
```

This is guard vocabulary drift, not a semantic route failure.

## Selected Maintenance Sequence

```text
P0:
  MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-GUARD-REALIGNMENT-001

P1:
  GUARD-SOURCE-SELFHOST-CURRENT-POINTER-DECOUPLE-001
  GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001

P2:
  DOCS-SOURCE-SELFHOST-COMPACT-CURRENT-STATE-001
  DOCS-SOURCE-SELFHOST-TASK-ORDER-THINNING-001
  DOCS-CHECK-INDEX-FAMILY-VIEW-001
```

After this maintenance sequence, resume the semantic lane at:

```text
MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
```

## Output Contract

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-docs-guard-maintenance-reduction-v0.json

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_docs_guard_maintenance_reduction_guard.sh
```

## Acceptance

```text
docs_only_closeout = 0
code_or_guard_delta_required = 1
current_blocker_token =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no family adoption decision
no route repair
no wider route selection
no native slice decomposition
no Source Selfhost claim
no Rust deletion
```
