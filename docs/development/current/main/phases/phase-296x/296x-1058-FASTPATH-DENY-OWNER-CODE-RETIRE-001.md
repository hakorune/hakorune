Status: Done
Date: 2026-06-17
Scope: retire speculative Rust deny-owner mapping vocabulary
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1036-FASTPATH-DENY-OWNER-MAPPING-001.md
  - docs/development/current/main/phases/phase-296x/296x-1057-SCALAR-FIELD-DESCRIPTOR-VOCABULARY-CLOSEOUT-001.md

# FASTPATH-DENY-OWNER-CODE-RETIRE-001

## Purpose

Close the remaining low-risk speculative vocabulary residue in
`ObjectStoragePlan`.

The fresh residue review found that `FastPathDenyOwner` /
`LocalFastPathFallbackReason::owner_mapping()` existed only as Rust passive
vocabulary. That made the code carry task-owner routing before the resolver has
an active execution path.

## Decision

Retire deny-owner mapping from Rust code.

Keep the owner mapping as docs/report guidance only:

```text
FastPathDecision:
  Allow(LocalFastPathFact)
  Deny(LocalFastPathFallbackReason)

Deny owner mapping:
  code_enabled=0
  owner=docs_report
```

This keeps the fact/fallback split intact:

```text
LocalFastPathFact:
  positive backend-consumable proof

LocalFastPathFallbackReason:
  deny reason only

Owner mapping:
  docs/report owner-selection guide
```

## Residue Triage

```text
callsite_canonicalize_4_entries:
  status=closed
  reason=296x-1046..1048 moved production entries through schedule facade
  single_transform_owner=1
  centralized_schedule_owner=1
  entry_removal_enabled=0

ExactStackObject:
  status=closed
  reason=296x-1049..1050 retired active source vocabulary
  exact_stack_object_source_presence_count=0

unknown_publication_forces_generic_fallback_duplicate_key:
  status=not_reproduced
  reason=current src/object_storage_plan/report.rs emits one key

FastPathDenyOwner:
  status=retired_from_rust_code
  reason=zero active non-test consumer; docs/report are the right owner
```

## Contract

```text
output_contract=hako-fastpath-deny-owner-code-retire-v0
fastpath_deny_owner_code_enabled=0
fastpath_deny_owner_mapping_owner=docs_report
fastpath_deny_owner_source_presence_count=0
fastpath_decision_shape=AllowFact_or_DenyReason
fallback_fact_enabled=0
backend_behavior_changed=0
route_priority_changed=0
summary=ok
```

## Stop Lines

```text
do not remove LocalFastPathFallbackReason
do not make deny reasons backend-consumable
do not turn fallback evidence into facts
do not change route priority
do not change optimizer/backend behavior
```

## Validation

```text
cargo test -q object_storage_plan --lib
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/k2_wide_phase296x_object_storage_plan_vocab_audit_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
