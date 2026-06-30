# 1904 - MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001
```

## Purpose

Resolve the `ValueReturnAstScan` subcluster selected after the pure-method
catalog.

The selected source surface is:

```text
contains_value_return(nodes) -> bool
```

This surface is a recursive AST traversal helper. It observes whether a function
body contains a value-returning `Return` node inside selected AST containers. It
does not own call lowering semantics, route selection, or a standalone Hako
projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_value_return_ast_scan_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-value-return-ast-scan-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_value_return_ast_scan_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentAstScan
projection_surface_selected = 0
ast_traversal_projection_selected = 0

next_card =
  MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001
```

## Evidence

```text
source_count = 1
source_surface = contains_value_return
ast_variants =
  FunctionDeclaration, If, Loop, Program, Return, ScopeBox, TryCatch
recursion_markers =
  then_body, body, try_body, catch_clauses, finally_body, statements
```

## Acceptance

```text
policy = KeepParentAstScan
projection_surface_selected = 0
ast_traversal_projection_selected = 0
runtime_or_projection_policy_by_name = 0
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no standalone Hako AST traversal policy
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
