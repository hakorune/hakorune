---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1156-BUILD-FRONTEND-AST-NODES-PASSIVE-PREFLIGHT-001.md
---

# BUILD-FRONTEND-AST-SIMPLE-DECLS-PASSIVE-SPLIT-001

## Decision

Move AST declaration metadata without `ASTNode` fields into
`hakorune-frontend-ast`.

```text
moved_types=ParamDecl,DelegateExposeDecl,DelegateDecl,TransitionDecl,ContractKind
new_owner=crates/hakorune_frontend_ast/src/decls.rs
compat_reexport=src/ast/decls.rs
historical_import_path_preserved=crate::ast::{ParamDecl,DelegateExposeDecl,DelegateDecl,TransitionDecl,ContractKind}
behavior_changed=0
```

## Boundary

Still deferred:

```text
deferred_types=CatchClause,FieldDecl,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
reason=contain_ASTNode_or_FieldDecl_with_ASTNode_default_value
```

## Checks

```text
cargo_check_default_green=1
ast_external_refs_outside_literal_box_bridge=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-FIELD-DECL-BOUNDARY-DESIGN-001
reason=FieldDecl is the next blocker because default_value carries Option<Box<ASTNode>>
implementation_allowed=design_only
```
