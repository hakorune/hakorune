---
Status: Landed
Date: 2026-06-15
Task: COREPLAN-NORMALIZER-COMPOSITION-001
Scope: Move one normalizer AST-owned decision behind an adapter boundary.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-1038-COREPLAN-JOINIR-MERGE-PHI-001.md
  - src/mir/builder/control_flow/plan/normalizer/stmt_only_prelude_view.rs
  - src/mir/builder/control_flow/plan/normalizer/cond_lowering_prelude.rs
  - tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
---

# COREPLAN-NORMALIZER-COMPOSITION-001

## Decision

This is a BoxShape-only normalizer composition row. It moves one AST-owned
decision family behind a named adapter without adding accepted source shapes.

```text
selected_row=stmt_only_prelude_view_adapter
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

## Implementation

```text
stmt_only_prelude_view:
  owns ASTNode statement-shape extraction for statement-only condition /
  block-expression preludes

cond_lowering_prelude:
  consumes StmtOnlyPreludeView instead of matching each accepted AST statement
  shape inline

classification source:
  classify_cond_prelude_stmt remains the acceptance vocabulary owner
```

This keeps the normalizer moving toward composition-only behavior:

```text
AST shape extraction -> adapter
prelude effect composition -> cond_lowering_prelude
accepted shape vocabulary -> cond_prelude_vocab
```

## Acceptance

```text
coreplan_normalizer_composition_stmt_only_prelude=1
stmt_only_prelude_view_adapter=1
one_normalizer_ast_decision_moved_to_adapter=1
normalizer_composition_only_progress=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
summary=ok
```

## Proof Commands

```bash
bash tools/checks/coreplan_normalizer_ast_boundary_inventory_guard.sh
cargo check --bin hakorune
```

## Stop Line

```text
do not add accepted shapes
do not move AST matching sideways without adapter ownership
do not combine this with variable_map or PHI lifecycle rows
do not make normalizer route acceptance truth
```

## Next

```text
compiler_foundation_checkpoint:
  run the CorePlan guards and decide whether to continue compiler-first or
  pause back to MIMALLOC-AOT-KERNEL-FRONT-SELECT-002.
```
