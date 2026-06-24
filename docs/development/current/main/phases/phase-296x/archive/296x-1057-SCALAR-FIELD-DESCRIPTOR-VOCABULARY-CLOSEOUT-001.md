# 296x-1057 SCALAR-FIELD-DESCRIPTOR-VOCABULARY-CLOSEOUT-001

Status: Landed
Date: 2026-06-17
Scope: close scalar field descriptor merge candidate

## Contract

```text
output_contract=hako-object-storage-plan-vocab-audit-v0
source_evidence=296x-1055,object-storage-plan-storage-rs
row_kind=inventory

keep_separate_count=6
merge_candidate_count=0
immediate_merge_allowed=0
vocabulary_merge_count=0
fact_fallback_separation_preserved=1
public_api_reexport_preserved=1
guard_path_compat_landed=1

scalar_field_descriptor_merge_enabled=0
field_scalar_plan_kept=1
flattened_nested_field_plan_kept=1
scalar_field_descriptor_candidate_closed=1
first_safe_followup=none
summary=ok
```

## Decision

Keep `FieldScalarPlan` and `FlattenedNestedFieldPlan` separate.

They share `scalar_type`, but they do not carry the same semantic payload:

```text
FieldScalarPlan:
  one direct field in one layout

FlattenedNestedFieldPlan:
  owner field + nested field + flattened field mapping across nested layout
```

Merging them would reduce type count at the cost of making flattened nested
field ownership harder to read.

## Result

The current ObjectStoragePlan vocabulary audit has no remaining merge
candidates:

```text
merge_candidate_count=0
first_safe_followup=none
```

## Stop Line

```text
do not merge FieldScalarPlan and FlattenedNestedFieldPlan
do not change flattened nested backend consumer behavior
do not change MIR JSON flattened nested plan shape
do not change product runtime behavior
```

## Verification

```bash
cargo test -q object_storage_plan --lib
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/k2_wide_phase296x_object_storage_plan_vocab_audit_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
