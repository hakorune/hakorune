# 296x-950 EXPR-STMT-CONVERTER-HELPER-SPLIT-001

Status: Landed
Date: 2026-06-16
Scope: Rust MIRBuilder typed converter helper split.

## Purpose

Move statement-like typed AST wrapper handling out of the middle of the regular
expression match body.

This keeps the compatibility route explicit without changing lowering
semantics.

## Implementation Shape

```text
new_private_boundary=StatementSurfaceDispatch
new_private_helper=try_build_statement_surface_expression
assignment_wrapper_helper=build_assignment_statement_expression
return_wrapper_helper=build_return_statement_expression
```

## Contract

```text
output_contract=expr_stmt_converter_helper_split_v0
behavior_changed=0
accepted_language_shape_added=0
statement_surface_dispatch_boundary_named=1
typed_assignment_converter_boundary_named=1
typed_return_converter_boundary_named=1
```

## Stop Line

```text
do_not_change_AssignStmt_converter=1
do_not_change_ReturnStmt_converter=1
do_not_route_new_statement_kinds=1
do_not_change_cf_if_or_cf_loop_semantics=1
```

## Next

```text
next_card=EXPR-STMT-RUST-MIRBUILDER-CLEANUP-001
```

