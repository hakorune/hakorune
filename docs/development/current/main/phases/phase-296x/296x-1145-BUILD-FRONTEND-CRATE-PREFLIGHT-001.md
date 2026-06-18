Status: Done
Date: 2026-06-18
Scope: preflight parser/ast extraction into a frontend crate
Related:
  - docs/development/current/main/phases/phase-296x/296x-1144-BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-FRONTEND-CRATE-PREFLIGHT-001

## Inventory

```text
output_contract=build-frontend-crate-preflight-v0

parser_ast_total_lines=16308
parser_ast_file_count=92

parser_ast_mir_ref_count=0
parser_ast_backend_ref_count=0
parser_ast_runtime_ref_count=28
parser_ast_config_box_runner_ref_count=37
parser_ast_internal_parser_ast_ref_count=328
```

High-value blockers:

```text
ast_literal_runtime_box_conversion=1
parser_runtime_logging_refs=28
parser_config_env_refs=29
direct_full_frontend_extraction_allowed=0
```

The frontend candidate is cleaner than runner/runtime boundaries because it has
no MIR or backend dependency. Direct extraction is still blocked by runtime Box
conversion in AST literal helpers and parser-side runtime logging/config reads.

## Decision

```text
selected_first_slice=ast_passive_data_boundary
selected_next_task=BUILD-FRONTEND-AST-PASSIVE-SEAM-001
reason=LiteralValue_and_AST_data_are_frontend_pure_except_runtime_box_conversion_helpers

full_frontend_crate_split_selected=0
parser_logging_seam_deferred=1
parser_config_env_seam_deferred=1
behavior_changed=0
summary=ok
```

The first slice should separate passive AST data from runtime Box conversion.
This keeps the crate split structural and avoids moving parser config/logging
concerns in the same row.

## Stop Lines

```text
do_not_move_parser_in_preflight_row=1
do_not_move_ast_in_preflight_row=1
do_not_change_literal_semantics=1
do_not_change_parser_env_policy=1
do_not_change_logging_behavior=1
```
