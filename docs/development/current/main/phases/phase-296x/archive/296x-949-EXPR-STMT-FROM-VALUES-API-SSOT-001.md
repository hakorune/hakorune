# 296x-949 EXPR-STMT-FROM-VALUES-API-SSOT-001

Status: Landed
Date: 2026-06-16
Scope: expr/stmt boundary and `_from_values` API SSOT.

## Purpose

Fix the narrow API contract for expression helpers that consume already-lowered
values.

The purpose is to prevent accidental re-lowering and keep converter cleanup
small.

## Boundary

```text
expr_owner=value_producing_lowering
stmt_owner=control_flow_and_side_effect_placement
converter_owner=typed_ast_wrapper_or_dispatch_boundary
from_values_owner=already_lowered_operand_consumer
```

## Contract

```text
output_contract=expr_stmt_from_values_api_ssot_v0

expr_dispatch_may_accept_statement_surface_for_compat=1
statement_surface_must_route_to_stmt_or_cf_helper=1
pure_expression_lowering_should_not_place_statement_control_flow=1

from_values_api_means_operands_already_lowered=1
from_values_api_must_not_rebuild_ast_operands=1
from_values_api_must_not_evaluate_operand_twice=1

build_binary_op_from_values_exists=1
build_index_access_from_values_exists=1
new_from_values_api_requires_callsite_proof=1
```

## Allowed Cleanup

```text
extract_statement_surface_dispatch_helper=1
extract_typed_statement_converter_helpers=1
keep_behavior_unchanged=1
```

## Forbidden

```text
new_language_shape=0
ast_rewrite=0
mirbuilder_object_management=0
control_flow_directory_move=0
```

## Next

```text
next_card=EXPR-STMT-CONVERTER-HELPER-SPLIT-001
```

