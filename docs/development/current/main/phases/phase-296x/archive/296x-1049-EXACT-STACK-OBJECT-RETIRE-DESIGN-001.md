# 296x-1049 EXACT-STACK-OBJECT-RETIRE-DESIGN-001

Status: Landed
Date: 2026-06-17
Scope: ExactStackObject passive vocabulary retire design

## Contract

```text
output_contract=hako-exact-stack-object-retire-design-v0
row_kind=design

selected_option=B_retire_exact_stack_object
exact_stack_object_external_producer_count=0
exact_stack_object_backend_consumer_count=0
exact_stack_object_retire_selected=1

active_exact_storage_forms=ExactNativeStruct,Scalarized,FlattenedNestedFields
stack_allocation_support_claimed=0
backend_behavior_changed=0
implementation_started=0

next_task=EXACT-STACK-OBJECT-RETIRE-IMPLEMENTATION-001
summary=ok
```

## Decision

Retire `ExactStackObject` from the active vocabulary.

The active exact-object representation vocabulary should be:

```text
ExactNativeStruct
Scalarized
FlattenedNestedFields
```

`ExactStackObject` currently has no external code producer:

```text
exact_stack_object_external_producer_count=0
```

It also risks implying stack allocation support that is not implemented by the
backend. Until a real stack placement owner appears, the name is more confusing
than useful.

## Why Not Keep Reserved

Keeping a reserved variant is cheap in code, but it preserves the confusing
claim:

```text
ObjectStoragePlan can choose ExactStackObject
```

while no active planner/backend row can produce or consume that choice.

The project already has `ExactNativeStruct`, `Scalarized`, and
`FlattenedNestedFields` for the exact local representation cases that are
actually modeled.

## Why Not Rename

Replacing it with a broader `LocalNativeObject` would add vocabulary instead of
shrinking it. That contradicts the current residue cleanup goal.

If a future row needs stack placement, it should introduce a new name with:

```text
producer proof
backend consumer proof
stack lifetime/fini/drop proof
publication/materialization proof
```

## Required Implementation Row

`EXACT-STACK-OBJECT-RETIRE-IMPLEMENTATION-001` should update:

```text
src/object_storage_plan/storage.rs
src/object_storage_plan/tests.rs
docs/development/current/main/design/object-storage-plan-boundary-ssot.md
docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
tools/checks/k2_wide_phase296x_object_storage_plan_ssot_guard.sh
tools/hako_check/object_storage_plan_vocab_audit.py
tools/hako_check/tests/test_object_storage_plan_vocab_audit.py
```

Historical cards may keep old evidence text. The implementation row should not
rewrite landed history unless a guard still reads it.

## Stop Line

```text
do not delete ExactStackObject in this design row
do not claim stack allocation support
do not change backend lowering
do not merge ExactNativeStruct and Scalarized
do not change product runtime behavior
```

## Verification

```bash
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
