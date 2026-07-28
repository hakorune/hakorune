Status: Done
Date: 2026-06-18
Scope: move pure syntax operator/predicate AST data into hakorune-frontend-ast
Related:
  - docs/development/current/main/phases/phase-296x/296x-1150-BUILD-FRONTEND-AST-NEXT-PASSIVE-TYPE-SELECTION-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-AST-SYNTAX-PASSIVE-SPLIT-001

## Change

```text
output_contract=build-frontend-ast-syntax-passive-split-v0

moved_types=UnaryOperator,BinaryOperator,BuildPredicate
new_owner=crates/hakorune_frontend_ast/src/operators.rs
new_owner=crates/hakorune_frontend_ast/src/build_predicate.rs
compat_reexport=src/ast/syntax.rs
historical_import_path_preserved=crate::ast::{UnaryOperator,BinaryOperator,BuildPredicate}

literal_value_moved=0
literal_value_deferred_reason=main_crate_runtime_box_conversion_inherent_impl
behavior_changed=0
```

`LiteralValue` remains in the main crate. Operator and build predicate passive
data now live in `hakorune-frontend-ast`, with the historical `crate::ast::*`
imports preserved through the existing syntax facade.

## Proof

```bash
cargo check -q
rg -n "pub enum (UnaryOperator|BinaryOperator|BuildPredicate)|pub use .*UnaryOperator|pub use .*BuildPredicate" \
  src/ast crates/hakorune_frontend_ast/src
```

## Result

```text
cargo_check_default_green=1
frontend_ast_active_type_count=4
selected_next_task=BUILD-FRONTEND-AST-ATTRS-PROFILE-SEAM-001
summary=ok
```

## Stop Lines

```text
do_not_move_literal_value=1
do_not_change_operator_display_semantics=1
do_not_change_build_predicate_semantics=1
do_not_remove_main_crate_compat_reexport=1
```
