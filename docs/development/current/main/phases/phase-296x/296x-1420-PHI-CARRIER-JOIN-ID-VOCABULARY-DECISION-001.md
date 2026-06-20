# 296x-1420 PHI-CARRIER-JOIN-ID-VOCABULARY-DECISION-001

Status: closed
Date: 2026-06-20

## Purpose

Decide the lifecycle-lane status of `CarrierVar.join_id` after production code
shows no `Some(ValueId)` producer.

## Selected By

```text
296x-1419-POST-LIFECYCLE-EMITTER-PROBE-OWNER-SELECTION-001
```

## Scope

```text
design=docs/development/current/main/design/phi-carrier-join-id-vocabulary-decision.md
guard=tools/checks/rust_lifecycle_join_id_vocabulary_guard.sh
```

Decision:

```text
status=test_fixture_or_stale_vocabulary
keep_parked=1
retire_now=0
implement_now=0
resolver_dependency=0
emitter_dependency=0
```

## Non-Goals

```text
do_not_delete_join_id=1
do_not_add_join_id_producer=1
do_not_modify_Rust_code=1
do_not_change_scope_manager_tests=1
do_not_claim_PHI_carrier_lifecycle_complete=1
```

## Acceptance

```text
production_join_id_some_producer=0
production_join_id_mutation_assignment=0
production_join_id_none_initializers=present
resolver_denies_join_id=1
verifier_denies_join_id=1
emitter_denies_join_id=1
join_id_retired_now=0
join_id_implemented_now=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_join_id_vocabulary_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
production_join_id_some_producer=0
production_join_id_mutation_assignment=0
production_join_id_none_initializers=present
resolver_denies_join_id=1
verifier_denies_join_id=1
emitter_denies_join_id=1
join_id_retired_now=0
join_id_implemented_now=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_join_id_vocabulary_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-join-id-vocabulary-v0
production_join_id_some_producer=0
production_join_id_mutation_assignment=0
production_join_id_none_initializers=present
resolver_denies_join_id=green
verifier_denies_join_id=green
emitter_denies_join_id=green
join_id_retired_now=0
join_id_implemented_now=0
summary=ok
```

Next:

```text
296x-1421-POST-JOIN-ID-VOCABULARY-DECISION-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_treat_parked_join_id_as_proven_lifecycle=1
do_not_delete_join_id_without_separate_cleanup_row=1
do_not_add_dummy_join_id_producer=1
```
