# 296x-1050 EXACT-STACK-OBJECT-RETIRE-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-17
Scope: ExactStackObject active vocabulary retirement

## Contract

```text
output_contract=hako-exact-stack-object-retire-implementation-v0
row_kind=implementation

source_evidence=296x-1049
exact_stack_object_retired=1
exact_stack_object_source_presence_count=0
active_exact_storage_forms=ExactNativeStruct,Scalarized,FlattenedNestedFields
stack_allocation_support_claimed=0

backend_behavior_changed=0
product_default_changed=0
mirbuilder_object_management_enabled=0
new_storage_variant_added=0

summary=ok
```

## Changes

Retired `ExactStackObject` from the active `ObjectStoragePlan` vocabulary.

The active exact storage forms are now:

```text
ExactNativeStruct
Scalarized
FlattenedNestedFields
```

The implementation removes the inactive enum variant, updates active SSOT docs,
and changes the vocabulary audit to report the retired invariant instead of a
future merge candidate.

## Non-Goals

```text
do not add stack allocation support
do not change backend lowering
do not change product runtime behavior
do not merge ExactNativeStruct and Scalarized
do not introduce a replacement storage variant
```

## Verification

```bash
cargo test -q object_storage_plan --lib
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/k2_wide_phase296x_object_storage_plan_ssot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
