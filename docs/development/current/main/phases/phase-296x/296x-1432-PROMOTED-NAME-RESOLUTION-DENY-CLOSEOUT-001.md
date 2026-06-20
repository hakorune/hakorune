# 296x-1432 PROMOTED-NAME-RESOLUTION-DENY-CLOSEOUT-001

Status: closed
Date: 2026-06-20

## Purpose

Close promoted-name resolution as an explicit deny boundary until a production
`CarrierVar.join_id` producer exists.

## Selected By

```text
296x-1431-POST-PROMOTED-BODY-LOCALS-PRODUCER-OWNER-SELECTION-001
```

## Scope

```text
design=docs/development/current/main/design/promoted-name-resolution-deny-closeout.md
guard=tools/checks/rust_lifecycle_promoted_name_resolution_deny_guard.sh
```

Decision:

```text
promoted_name_resolution_closed_as_deny=1
resolution_allowed=0
join_id_producer=0
resolver_selection_owner=0
converter_emission_added=0
```

## Acceptance

```text
resolve_promoted_join_id_requires_join_id=1
scope_manager_consumes_resolution=1
resolver_denies_join_id_production=1
join_id_dependent_paths_allowed=0
producer_fixtures_deny_promoted_name_resolution=1
resolution_allowed=0
join_id_producer=0
implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_promoted_name_resolution_deny_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
resolve_promoted_join_id_requires_join_id=1
scope_manager_consumes_resolution=1
resolver_denies_join_id_production=1
join_id_dependent_paths_allowed=0
producer_fixtures_deny_promoted_name_resolution=1
resolution_allowed=0
join_id_producer=0
implementation_started=0
backend_behavior_changed=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_promoted_name_resolution_deny_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-promoted-name-resolution-deny-v0
resolve_promoted_join_id_requires_join_id=1
scope_manager_consumes_resolution=1
resolver_denies_join_id_production=green
join_id_dependent_paths_allowed=0
producer_fixtures_deny_promoted_name_resolution=green
resolution_allowed=0
join_id_producer=0
implementation_started=0
summary=ok
```

Next:

```text
296x-1433-POST-PROMOTED-NAME-RESOLUTION-DENY-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_add_dummy_join_id_producer=1
do_not_allow_resolver_positive_resolution=1
do_not_expand_emitter_in_this_row=1
do_not_modify_Rust_behavior=1
```

