Status: Done
Date: 2026-06-18
Scope: separate passive AST literal data from runtime Box conversion helpers
Related:
  - docs/development/current/main/phases/phase-296x/296x-1145-BUILD-FRONTEND-CRATE-PREFLIGHT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-AST-PASSIVE-SEAM-001

## Change

```text
output_contract=build-frontend-ast-passive-seam-v0

new_module=src/ast/literal_box_bridge.rs
passive_literal_data_owner=src/ast/syntax.rs
runtime_box_conversion_owner=src/ast/literal_box_bridge.rs

syntax_rs_runtime_box_ref_count=0
literal_box_bridge_runtime_box_ref_count=7
public_api_changed=0
literal_semantics_changed=0
```

`LiteralValue` stays in `syntax.rs` as passive frontend data. The runtime
conversion methods remain available on `LiteralValue`, but their implementation
now lives in `literal_box_bridge.rs`.

This creates a clear seam for future frontend crate extraction: passive AST
data can move before runtime Box conversion helpers.

## Proof

```bash
cargo check -q
rg -n "crate::box_trait|crate::boxes|crate::runtime|crate::config" \
  src/ast/syntax.rs src/ast/literal_box_bridge.rs src/ast/mod.rs
```

## Result

```text
cargo_check_default_green=1
syntax_rs_runtime_ref_count=0
behavior_changed=0
selected_next_task=BUILD-FRONTEND-PARSER-ENV-LOGGING-SEAM-001
summary=ok
```

## Stop Lines

```text
do_not_move_parser=1
do_not_move_ast_to_new_crate=1
do_not_change_literal_semantics=1
do_not_change_parser_env_policy=1
do_not_change_logging_behavior=1
```
