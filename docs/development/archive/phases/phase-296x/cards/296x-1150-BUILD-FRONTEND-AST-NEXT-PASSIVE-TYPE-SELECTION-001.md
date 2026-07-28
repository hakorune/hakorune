Status: Done
Date: 2026-06-18
Scope: select the next passive AST type family after Span
Related:
  - docs/development/current/main/phases/phase-296x/296x-1149-BUILD-FRONTEND-AST-SPAN-PASSIVE-SPLIT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-AST-NEXT-PASSIVE-TYPE-SELECTION-001

## Inventory

```text
ast_total_lines=2139
ast_file_count=13

candidate_file=src/ast/syntax.rs
candidate_file_lines=118
candidate_passive_types=UnaryOperator,BinaryOperator,BuildPredicate

deferred_type=LiteralValue
deferred_reason=main_crate_runtime_box_conversion_inherent_impl

deferred_file=src/ast/attrs.rs
deferred_reason=rune_profile_registry_dependency
```

`LiteralValue` is passive data, but the main crate still owns
`LiteralValue::to_nyash_box` / `from_nyash_box` as inherent methods. Moving
`LiteralValue` now would either break that API or require a trait bridge. Keep
it in the main crate until that bridge is designed.

## Decision

```text
output_contract=build-frontend-ast-next-passive-type-selection-v0

selected_type_family=syntax_operator_predicate
selected_types=UnaryOperator,BinaryOperator,BuildPredicate
selected_next_task=BUILD-FRONTEND-AST-SYNTAX-PASSIVE-SPLIT-001
reason=pure_data_no_runtime_box_bridge_no_parser_dependency

literal_value_moved=0
attrs_moved=0
summary=ok
```

## Stop Lines

```text
do_not_move_literal_value_until_box_conversion_bridge_is_redesigned=1
do_not_move_attrs_until_rune_profile_registry_dependency_is_isolated=1
do_not_change_operator_or_predicate_semantics=1
```
