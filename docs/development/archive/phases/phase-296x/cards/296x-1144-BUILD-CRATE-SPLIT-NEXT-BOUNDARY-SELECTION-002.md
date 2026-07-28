Status: Done
Date: 2026-06-18
Scope: select the next build-time reduction boundary after vm-reference default-off closeout
Related:
  - docs/development/current/main/phases/phase-296x/296x-1143-BUILD-VM-REFERENCE-DEFAULT-OFF-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002

## Inventory

```text
parser_ast_frontend_total_lines=16308
parser_ast_file_count=92
parser_ast_cross_layer_reference_count=356

backend_runner_runtime_boxes_total_lines=114867
backend_file_count=92
runner_file_count=282
```

Largest frontend files:

```text
src/parser/mod.rs=680
src/parser/expr/match_expr_impl.rs=665
src/parser/statements/control_flow.rs=635
src/ast/mod.rs=626
src/parser/statements/helpers.rs=573
```

## Decision

```text
output_contract=build-crate-split-next-boundary-selection-v2

selected_next_boundary=hakorune_frontend
selected_next_task=BUILD-FRONTEND-CRATE-PREFLIGHT-001
reason=next_ranked_medium_size_boundary_after_backend_aot_and_vm_reference_default_off

direct_extraction_allowed=0
preflight_required=1
reason_preflight_required=parser_ast_cross_layer_reference_count_nonzero

rejected_boundary=runtime_boxes
rejected_reason=too_large_and_high_risk_for_next_split
rejected_boundary=runner
rejected_reason=larger_than_frontend_and_mixed_product_orchestration
rejected_boundary=backend_vm
rejected_reason=vm_reference_is_now_explicit_feature_not_product_default_owner

summary=ok
```

The next boundary should audit `src/parser` / `src/ast` as a candidate
`hakorune-frontend` crate. The preflight must classify cross-layer references
before moving code. Direct extraction is not allowed from this selection row.

## Stop Lines

```text
do_not_move_parser_or_ast_in_selection_row=1
do_not_change_language_acceptance=1
do_not_move_runtime_or_boxes=1
do_not_reopen_vm_product_route=1
```
