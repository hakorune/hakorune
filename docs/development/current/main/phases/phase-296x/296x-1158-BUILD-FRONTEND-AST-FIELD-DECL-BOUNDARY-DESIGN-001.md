---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1157-BUILD-FRONTEND-AST-SIMPLE-DECLS-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-AST-FIELD-DECL-BOUNDARY-DESIGN-001

## Problem

`FieldDecl` is not a simple passive declaration row:

```rust
pub struct FieldDecl {
    pub name: String,
    pub declared_type_name: Option<String>,
    pub is_weak: bool,
    pub default_value: Option<Box<ASTNode>>,
}
```

Moving it alone would either keep a dependency on the main-crate `ASTNode` or
force a generic/partial field declaration type.

## Decision

Do not split `FieldDecl` alone.

```text
fielddecl_standalone_split_selected=0
generic_field_signature_split_selected=0
reason=would_create_parallel_FieldDecl_truth
```

`FieldDecl` should move with the recursive AST graph that owns its default
value type:

```text
recursive_graph_bundle=FieldDecl,CatchClause,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
```

This avoids creating a second `FieldSignatureDecl` / `FieldDecl` naming layer
that later must be reconciled.

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PREFLIGHT-001
purpose=check whether the remaining recursive AST graph is now dependency-clean enough for one compatibility-facade move
implementation_allowed=preflight_only
```
