# 296x-1045 RESIDUE-RETIRE-DESIGN-QUEUE-001

Status: Landed
Date: 2026-06-17
Scope: residue retire queue / design decision stop-line

## Contract

```text
output_contract=hako-residue-retire-design-queue-v0
row_kind=design_queue

callsite_canonicalize_entry_design_required=1
exact_stack_object_retire_design_required=1
fastpath_reachability_retire_or_enable_design_required=1

behavior_changed=0
code_deleted=0
schedule_changed=0
backend_behavior_changed=0

selected_next_design=CALLSITE-CANONICALIZE-ENTRY-OWNER-DESIGN-001
implementation_allowed=0
summary=ok
```

## Evidence Inputs

### Callsite Canonicalize Entries

Source:

```bash
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
```

Current report:

```text
production_entry_count=4
known_entry_count=4
unknown_entry_count=0
single_transform_owner=1
centralized_schedule_owner=0
canonicalize_entry_refactor_allowed=0
```

Reading:

```text
The transform owner is already single:
  src/mir/passes/callsite_canonicalize/

The schedule entries are not single:
  src/mir/compiler/mod.rs
  src/mir/optimizer/core.rs
  src/runner/json_v0_bridge/core.rs
  src/runner/mir_json_v0.rs
```

This is the highest-value design candidate because it is a real schedule-owner
question, not just passive vocabulary cleanup.

### Object Storage Passive Vocabulary

Source:

```bash
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
```

Current report:

```text
exact_stack_object_external_producer_count=0
fastpath_decision_non_test_consumer_count=7
fastpath_reachability_non_test_consumer_count=0
passive_vocab_execution_enabled=0
vocab_retire_allowed=0
```

Reading:

```text
ExactStackObject:
  no external producer, but SSOT/docs/guards still name it

FastPathReachability:
  post-hoc vocabulary exists, but no non-test execution consumer currently
  reads it

FastPathDecision:
  not dead; inventory/shadow code consumes it to preserve Fact/fallback
  separation
```

## Design Queue

### 1. CALLSITE-CANONICALIZE-ENTRY-OWNER-DESIGN-001

Recommended first.

Question:

```text
Should canonicalize callsite scheduling stay as four explicit pipeline entries,
or should those entries delegate through one named schedule owner?
```

Allowed outcomes:

```text
A. Keep four entries, document each timing seam.
B. Add a thin schedule facade used by all four entries.
C. Move some entries after proving timing equivalence.
```

Stop line:

```text
do not remove an entry without proving the affected pipeline still canonicalizes
do not reorder semantic refresh around canonicalization by accident
do not merge Program(JSON v0) and MIR JSON loader policy silently
```

### 2. EXACT-STACK-OBJECT-RETIRE-DESIGN-001

Question:

```text
Should ExactStackObject remain reserved vocabulary, or be merged into
ExactNativeStruct / Scalarized until a real producer appears?
```

Allowed outcomes:

```text
A. Keep reserved variant, add clearer "reserved, no producer" docs.
B. Retire ExactStackObject and update SSOT/guard/tests.
C. Replace it with a more general LocalNativeObject variant.
```

Stop line:

```text
do not delete the variant until object-storage-plan SSOT and guards agree
do not change backend object lowering behavior
do not claim stack allocation support from vocabulary alone
```

### 3. FASTPATH-REACHABILITY-RETIRE-OR-ENABLE-DESIGN-001

Question:

```text
Should FastPathReachability remain post-hoc vocabulary, be wired into an active
report consumer, or be retired until resolver execution exists?
```

Allowed outcomes:

```text
A. Keep post-hoc vocabulary and add a real report consumer.
B. Retire FastPathReachability until a front needs it.
C. Fold it into existing fastpath gap inventory output.
```

Stop line:

```text
do not make reachability feed resolver eligibility
do not convert preemption into a Deny reason
do not let backend infer fastpath eligibility from reachability rows
```

## Stop Line For This Row

```text
do not implement any of the design outcomes here
do not delete code here
do not change pass schedule here
do not enable backend lowering here
```

## Verification

```bash
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
