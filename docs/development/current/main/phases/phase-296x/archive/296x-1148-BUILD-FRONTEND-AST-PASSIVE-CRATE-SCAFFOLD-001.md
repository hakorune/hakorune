Status: Done
Date: 2026-06-18
Scope: create passive frontend AST crate scaffold
Related:
  - docs/development/current/main/phases/phase-296x/296x-1147-BUILD-FRONTEND-PARSER-ENV-LOGGING-SEAM-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-AST-PASSIVE-CRATE-SCAFFOLD-001

## Change

```text
output_contract=build-frontend-ast-passive-crate-scaffold-v0

new_crate=hakorune-frontend-ast
new_crate_path=crates/hakorune_frontend_ast
new_crate_scope=passive_frontend_ast_data
root_dependency_added=1
active_ast_moved=0
behavior_changed=0
```

The new crate is scaffold-only. It owns no active AST implementation yet and
does not depend on parser, runtime, backend, MIR, or Box implementations.

## Proof

```bash
cargo check -q
rg -n "hakorune_frontend_ast|hakorune-frontend-ast" Cargo.toml crates/hakorune_frontend_ast
```

## Result

```text
cargo_check_default_green=1
frontend_ast_crate_created=1
frontend_ast_crate_dependency_count=0
selected_next_task=BUILD-FRONTEND-AST-SPAN-PASSIVE-SPLIT-001
summary=ok
```

## Stop Lines

```text
do_not_move_parser=1
do_not_move_full_ast=1
do_not_change_language_acceptance=1
do_not_add_runtime_dependency_to_frontend_ast_crate=1
```
