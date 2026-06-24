---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1155-BUILD-FRONTEND-AST-LITERAL-VALUE-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-AST-NODES-PASSIVE-PREFLIGHT-001

## Inventory

```text
src_ast_mod_rs_lines=627
src_ast_nodes_rs_lines=263
ast_external_refs_outside_literal_box_bridge=0
astnode_direct_extraction_allowed=0
```

`ASTNode` remains the large recursive owner:

```text
recursive_astnode_fields=many
hashmap_method_fields=1
body_vec_astnode_fields=many
```

Moving `ASTNode` directly would drag the whole declaration/expression graph at
once. That is too large for the current crate split mode.

## Safe Next Bundle

The next passive bundle should move only declaration metadata that does not
contain `ASTNode`:

```text
selected_type_family=ast_simple_decls
selected_types=ParamDecl,DelegateExposeDecl,DelegateDecl,TransitionDecl,ContractKind
astnode_containing_types_deferred=CatchClause,FieldDecl,ContractClause,EnumVariantDecl,EnumMatchArm,CheckItem,ASTNode
```

Rationale:

```text
ParamDecl=names_and_optional_type_only
DelegateExposeDecl=string_metadata_only
DelegateDecl=string_metadata_plus_delegate_exposes
TransitionDecl=string_metadata_only
ContractKind=small_enum_only
```

## Boundary

Allowed:

```text
passive_type_split_only=1
compat_reexport=1
behavior_changed=0
```

Forbidden:

```text
astnode_move=0
fielddecl_move=0
contractclause_move=0
parser_behavior_change=0
mir_lowering_change=0
```

## Next

```text
selected_next_task=BUILD-FRONTEND-AST-SIMPLE-DECLS-PASSIVE-SPLIT-001
implementation_allowed=passive_type_split_only
```
