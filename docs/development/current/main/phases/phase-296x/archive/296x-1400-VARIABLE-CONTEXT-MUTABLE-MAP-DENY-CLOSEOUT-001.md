# 296x-1400 VARIABLE-CONTEXT-MUTABLE-MAP-DENY-CLOSEOUT-001

Status: closed
Date: 2026-06-20

## Purpose

Close `VariableContext::variable_map_mut()` as an explicit
`Deny(ReturnedMutableBorrow)` lifecycle boundary for the current migration
slice.

## Selected By

```text
296x-1399-VARIABLE-CONTEXT-POST-SNAPSHOT-RESTORE-OWNER-SELECTION-001
```

## Scope

```text
method=VariableContext::variable_map_mut()
policy=Deny(ReturnedMutableBorrow)
expected_external_rust_callsite_count=0
implementation_started=0
```

Allowed:

```text
guard that verifies no external Rust callsites exist
guard that verifies existing lifecycle fixtures deny variable_map_mut()
docs/current pointer updates
```

## Non-Goals

```text
do_not_change_Rust_API=1
do_not_add_with_map_operation=1
do_not_add_explicit_mutation_methods=1
do_not_model_carrier_PHI=1
do_not_add_general_resolver=1
do_not_claim_full_VariableContext_parity=1
```

## Acceptance

```text
external_variable_map_mut_callsite_count=0
deny_reason=ReturnedMutableBorrow
existing_fixtures_keep_variable_map_mut_denied=1
rust_api_changed=0
carrier_PHI_claim=0
general_resolver_implemented=0
full_VariableContext_parity_claim=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_mutable_map_deny_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
external_variable_map_mut_callsite_count=0
deny_reason=ReturnedMutableBorrow
existing_fixtures_keep_variable_map_mut_denied=1
rust_api_changed=0
carrier_PHI_claim=0
general_resolver_implemented=0
full_VariableContext_parity_claim=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_variable_context_mutable_map_deny_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-variable-context-mutable-map-deny-v0
external_variable_map_mut_callsite_count=0
deny_reason=ReturnedMutableBorrow
existing_fixtures_keep_variable_map_mut_denied=green
rust_api_changed=0
carrier_PHI_claim=0
general_resolver_implemented=0
full_VariableContext_parity_claim=0
summary=ok
```

Next:

```text
296x-1401-VARIABLE-CONTEXT-POST-MUTABLE-DENY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_model_returned_mut_BTreeMap_as_direct_mutation=1
do_not_change_Rust_API_in_closeout=1
do_not_start_carrier_PHI_from_this_row=1
```
