---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1158-BUILD-FRONTEND-AST-FIELD-DECL-BOUNDARY-DESIGN-001.md
---

# BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PREFLIGHT-001

## Inventory

```text
src_ast_mod_rs_lines=550
src_ast_nodes_rs_lines=263
src_ast_utils_impl_lines=956
ast_external_refs_outside_literal_box_bridge=0
```

The remaining recursive graph is dependency-clean for frontend ownership:

```text
remaining_graph_uses_main_crate_runtime=0
remaining_graph_uses_main_crate_parser=0
remaining_graph_uses_main_crate_mir=0
remaining_graph_uses_main_crate_backend=0
```

## Constraint

Moving only `ASTNode` is not sufficient. The `src/ast/utils/**` helpers are
inherent `impl ASTNode` methods. If `ASTNode` moves to `hakorune-frontend-ast`,
those impls must move with it because Rust does not allow inherent impls for an
external type.

```text
orphan_rule_requires_impl_move=1
main_crate_inherent_astnode_impl_allowed_after_move=0
```

## Decision

Move the recursive AST graph as one compatibility-facade bundle:

```text
selected_bundle=ast_recursive_graph_with_methods
move_types=FieldDecl,CatchClause,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
move_wrappers=AssignStmt,ReturnStmt,IfStmt,BinaryExpr,CallExpr,MethodCallExpr
move_inherent_utils=span,node_type,info,classification,traversal,analysis
compat_reexport=src/ast/mod.rs
```

Keep the main crate bridge-only:

```text
main_crate_ast_runtime_bridge=src/ast/literal_box_bridge.rs
main_crate_ast_compat_facade=src/ast/mod.rs
behavior_changed=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PASSIVE-SPLIT-001
implementation_allowed=recursive_graph_plus_inherent_utils_split_only
```
