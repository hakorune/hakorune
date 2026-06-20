# 296x-1450 SCOPE-MANAGER-CONDITION-BINDING-ADAPTER-WIRING-DESIGN-001

Status: closed
Date: 2026-06-20

## Purpose

Document how `LoopBreakScopeManager` should consume the condition-binding
identity adapter without removing legacy promoted-join lookup.

This row is docs-only.

## Selected By

```text
296x-1449-POST-CONDITION-BINDING-RESOLUTION-ADAPTER-OWNER-SELECTION-001
```

## Output

```text
design_doc=docs/development/current/main/design/scope-manager-condition-binding-adapter-wiring-design.md
guard=tools/checks/rust_lifecycle_scope_manager_condition_binding_wiring_design_guard.sh
```

## Decision

```text
wiring_shape=explicit_scope_manager_condition_bindings_input
lookup_order=condition_env,loop_body_local_env,captured_env,condition_binding_adapter,legacy_join_id
legacy_resolve_promoted_join_id_kept=1
implementation_started=0
```

## Acceptance

```text
wiring_design_documented=1
condition_bindings_input_named=1
lookup_order_documented=1
legacy_path_preserved=1
implementation_started=0
backend_behavior_changed=0
generated_program_execution_claim=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_scope_manager_condition_binding_wiring_design_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_change_scope_manager_code=1
do_not_emit_trim_route_lowering=1
do_not_remove_resolve_promoted_join_id=1
do_not_claim_generated_program_execution=1
```
