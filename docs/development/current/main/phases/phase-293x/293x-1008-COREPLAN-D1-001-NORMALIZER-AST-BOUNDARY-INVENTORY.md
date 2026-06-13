---
Status: Landed
Date: 2026-06-14
Scope: normalizer AST-boundary inventory guard.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md
  - tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
  - src/mir/builder/control_flow/plan/normalizer/
  - src/mir/builder/control_flow/plan/recipe_tree/
---

# COREPLAN-D1-001: Normalizer AST Boundary Inventory

## Purpose

Make D1 measurable before moving code.

The normalizer still contains direct `ASTNode::` matching. That is the D1
cleanup surface, but removing it safely requires an inventory first. This card
adds a report-only guard that prints per-run AST-boundary counts without
failing solely because the current count is non-zero.

## Decision

```text
normalizer_ast_boundary_inventory=1
normalizer_ast_hit_counts_reported=1
synthetic_ast_composer_inventory=1
report_only=1
release_default_changed=0
accepted_shape_added=0
summary=ok
```

## Guard

```bash
bash tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
```

The guard reports:

```text
normalizer_ast_hit_count
normalizer_ast_file_count
recipe_tree_synthetic_ast_loop_count
```

## Stop Lines

```text
do not move AST matching to another folder without a named adapter boundary
do not merge D1 with C1 fail-fast work
do not add source shapes while reducing normalizer drift
do not make this guard fail on current non-zero counts until an allowlist is added
```
