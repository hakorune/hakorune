---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1159-BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PREFLIGHT-001.md
---

# BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PASSIVE-SPLIT-001

## Decision

Move the remaining recursive AST graph and inherent `ASTNode` utility methods
into `hakorune-frontend-ast`.

```text
moved_types=FieldDecl,CatchClause,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
moved_wrappers=AssignStmt,ReturnStmt,IfStmt,BinaryExpr,CallExpr,MethodCallExpr
moved_inherent_utils=span,node_type,info,classification,traversal,analysis
new_owner=crates/hakorune_frontend_ast/src/ast_node.rs
new_owner=crates/hakorune_frontend_ast/src/node_wrappers.rs
new_owner=crates/hakorune_frontend_ast/src/utils/**
compat_facade=src/ast/mod.rs
runtime_bridge=src/ast/literal_box_bridge.rs
```

## Result

```text
src_ast_facade_file_count=2
src_ast_external_refs_outside_literal_box_bridge=0
frontend_ast_main_crate_refs=0
behavior_changed=0
cargo_check_default_green=1
```

`src/ast` now owns only:

```text
src/ast/mod.rs
src/ast/literal_box_bridge.rs
```

All passive AST data, wrapper structs, and inherent ASTNode utility methods are
owned by `hakorune-frontend-ast`.

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-SPLIT-CLOSEOUT-001
purpose=close the frontend AST passive split and select the next frontend/parser boundary
implementation_allowed=closeout_only
```
