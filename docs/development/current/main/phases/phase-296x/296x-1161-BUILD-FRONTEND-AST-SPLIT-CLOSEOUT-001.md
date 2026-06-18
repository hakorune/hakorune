---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Build split / frontend AST passive crate.
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1160-BUILD-FRONTEND-AST-RECURSIVE-GRAPH-PASSIVE-SPLIT-001.md
---

# BUILD-FRONTEND-AST-SPLIT-CLOSEOUT-001

## Result

The frontend AST passive split is closed.

```text
new_crate=hakorune-frontend-ast
src_ast_mod_rs_lines=11
src_ast_literal_box_bridge_rs_lines=50
src_ast_facade_file_count=2
frontend_ast_crate_rust_file_count=17
frontend_ast_crate_main_crate_refs=0
behavior_changed=0
cargo_check_default_green=1
```

`src/ast` now owns only:

```text
compat_facade=src/ast/mod.rs
runtime_bridge=src/ast/literal_box_bridge.rs
```

Everything else is owned by `hakorune-frontend-ast`:

```text
ASTNode
ASTNode recursive metadata
ASTNode wrapper structs
ASTNode inherent utility methods
Span
LiteralValue
RuneAttr / DeclarationAttrs
syntax operators / BuildPredicate
simple declaration metadata
rune Profile vocabulary
```

## Boundary

The split is still behavior-preserving:

```text
parser_behavior_changed=0
mir_lowering_changed=0
runtime_box_conversion_owner=main_crate_bridge
historical_crate_ast_import_path_preserved=1
```

## Next

```text
selected_next_task=BUILD-FRONTEND-PARSER-CRATE-PREFLIGHT-001
purpose=audit parser/tokenizer extraction now that AST data is externalized
implementation_allowed=preflight_only
```
