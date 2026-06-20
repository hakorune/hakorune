# 296x-1446 CONDITION-BINDING-RESOLUTION-REWRITE-DESIGN-001

Status: closed
Date: 2026-06-20

## Purpose

Design how promoted-name resolution should consume the condition-binding
identity proof without reviving `CarrierVar.join_id`.

This row is docs-only. It does not rewrite resolution code.

## Selected By

```text
296x-1445-POST-CONDITION-BINDING-IDENTITY-PROOF-OWNER-SELECTION-001
```

## Output

```text
design_doc=docs/development/current/main/design/condition-binding-resolution-rewrite-design.md
guard=tools/checks/rust_lifecycle_condition_binding_resolution_design_guard.sh
```

## Decision

```text
rewrite_shape=additive_adapter
new_adapter=resolve_promoted_condition_binding_identity
legacy_resolve_promoted_join_id_kept=1
CarrierVar_join_id_producer_added=0
implementation_started=0
```

## Acceptance

```text
rewrite_design_documented=1
additive_adapter_selected=1
legacy_join_id_resolution_preserved=1
condition_binding_identity_input_named=1
implementation_started=0
backend_behavior_changed=0
generated_program_execution_claim=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_condition_binding_resolution_design_guard.sh
bash tools/checks/rust_lifecycle_condition_binding_promoted_identity_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
cargo check -q --lib
```

## Stop Line

```text
do_not_modify_resolution_code=1
do_not_remove_resolve_promoted_join_id=1
do_not_emit_trim_route_lowering=1
do_not_claim_generated_program_execution=1
```
