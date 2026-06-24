# 296x-951 EXPR-STMT-RUST-MIRBUILDER-CLEANUP-001

Status: Landed
Date: 2026-06-16
Scope: Rust MIRBuilder expression dispatcher cleanup.

## Purpose

Make the Rust expression dispatcher reflect the existing expr/stmt split:

```text
statement/control-flow surface:
  routed through a named compatibility boundary

regular expression surface:
  remains in the main expression match
```

This is cleanup only. It prepares converter work and selfhost parity work; it
does not add a new source shape.

## Acceptance

```text
output_contract=expr_stmt_rust_mirbuilder_cleanup_v0
statement_surface_dispatch_boundary_named=1
regular_expression_match_has_statement_surface_removed=1
assignment_statement_expression_helper=1
return_statement_expression_helper=1
behavior_changed=0
accepted_language_shape_added=0
summary=ok
```

## Result

```text
rust_expression_dispatcher=src/mir/builder/exprs.rs::build_expression_impl
statement_surface_dispatch=try_build_statement_surface_expression
statement_surface_dispatch_result=StatementSurfaceDispatch
assignment_statement_helper=build_assignment_statement_expression
return_statement_helper=build_return_statement_expression

program_if_loop_try_throw_print_task_scope_local_outbox_nowait_using_routed_before_regular_expression_match=1
box_declaration_remains_declaration_context=1
```

## Proof

```bash
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Out-of-Scope Gate Note

`tools/checks/dev_gate.sh quick` was attempted. It passed the current-state,
metadata, Program(JSON), layer, route, PHI, variable_map, facade/import, and
`cargo check` stages, then failed in the K2-core RawArray acceptance guard while
compiling `crates/nyash_kernel` test code.

The failure is outside this row's touched files and reported missing/ambiguous
test helpers and kernel test imports, not an expr/stmt dispatcher regression.

## Stop Line

```text
do_not_change_source_semantics=1
do_not_change_hako_mirbuilder_handlers=1
do_not_move_control_flow_modules=1
do_not_add_fallback=1
```
