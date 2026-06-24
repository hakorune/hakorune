Status: Done
Date: 2026-06-18
Scope: move the first passive AST data type into hakorune-frontend-ast
Related:
  - docs/development/current/main/phases/phase-296x/296x-1148-BUILD-FRONTEND-AST-PASSIVE-CRATE-SCAFFOLD-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-AST-SPAN-PASSIVE-SPLIT-001

## Change

```text
output_contract=build-frontend-ast-span-passive-split-v0

moved_type=Span
new_owner=crates/hakorune_frontend_ast/src/span.rs
compat_reexport=src/ast/span.rs
historical_import_path_preserved=crate::ast::Span
behavior_changed=0
```

`Span` is now owned by `hakorune-frontend-ast`. The main crate keeps
`crate::ast::Span` as a compatibility re-export, so existing parser/AST users
do not change.

## Proof

```bash
cargo check -q
rg -n "pub struct Span|pub use .*Span" src/ast crates/hakorune_frontend_ast/src
```

## Result

```text
cargo_check_default_green=1
frontend_ast_active_type_count=1
selected_next_task=BUILD-FRONTEND-AST-NEXT-PASSIVE-TYPE-SELECTION-001
summary=ok
```

## Stop Lines

```text
do_not_move_parser=1
do_not_move_full_ast=1
do_not_change_span_semantics=1
do_not_remove_main_crate_compat_reexport=1
```
