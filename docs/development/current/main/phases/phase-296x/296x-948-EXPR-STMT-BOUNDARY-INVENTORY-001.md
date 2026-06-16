# 296x-948 EXPR-STMT-BOUNDARY-INVENTORY-001

Status: Landed
Date: 2026-06-16
Scope: Rust MIRBuilder expr/stmt boundary inventory.

## Purpose

Inventory the remaining statement/control-flow surfaces that still enter the
Rust expression dispatcher before cleanup.

This is a BoxShape row. It does not add a new accepted language shape.

## Findings

```text
rust_expression_dispatcher=src/mir/builder/exprs.rs::build_expression_impl
rust_statement_orchestrator=src/mir/builder/stmts/mod.rs
hako_stmt_handler_owner=lang/src/compiler/mirbuilder/stmt_handlers
hako_consumer_dispatch_owner=lang/src/compiler/mirbuilder/program_json_v0_phase_state_consumer_box.hako
```

`exprs.rs` still lowers statement/control-flow surfaces:

```text
Program
ScopeBox
TaskScope
ContextScope
Print
If
Loop
TryCatch
Throw
Assignment
Return
Local
Outbox
UsingStatement
Nowait
```

Existing split surfaces:

```text
src/mir/builder/stmts/block_stmt.rs
src/mir/builder/stmts/print_stmt.rs
src/mir/builder/stmts/return_stmt.rs
src/mir/builder/stmts/variable_stmt.rs
src/mir/builder/stmts/task_scope_stmt.rs
src/mir/builder/stmts/async_stmt.rs
```

Existing `_from_values` API surfaces:

```text
src/mir/builder/ops/mod.rs::build_binary_op_from_values
src/mir/builder/indexing.rs::build_index_access_from_values
```

## Decision

```text
expr_stmt_split_is_worth_doing=1
reason=cheap_existing_visitors_and_from_values_api
boxshape_only=1
accepted_language_shape_added=0
```

## Stop Line

```text
do_not_duplicate_hako_mirbuilder_design_in_rust=1
do_not_move_control_flow_directories=1
do_not_change_source_language_semantics=1
do_not_add_new_ast_rewrite=1
```

## Next

```text
next_card=EXPR-STMT-FROM-VALUES-API-SSOT-001
```

