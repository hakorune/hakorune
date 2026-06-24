Status: Done
Date: 2026-06-17
Scope: audit reported fastpath/object-storage/callsite residue after 1072

# FASTPATH-RESIDUE-AUDIT-001

## Purpose

Check the reported medium/low residue list before selecting the next lane.

This row is audit-only. It does not reopen fastpath optimization and does not
start implementation work.

## Findings

### callsite-canonicalize 4 entries

The four production entries are real, but they are no longer ownerless
duplicates. They all call the schedule facade:

```text
output_contract=hako-callsite-canonicalize-entry-inventory-v0
production_entry_count=4
known_entry_count=4
unknown_entry_count=0
single_transform_owner=1
centralized_schedule_owner=1
entry_0_call_kind=schedule_facade
entry_1_call_kind=schedule_facade
entry_2_call_kind=schedule_facade
entry_3_call_kind=schedule_facade
summary=ok
```

Decision:

```text
callsite_canonicalize_4_entries_classification=intentional_schedule_entries
callsite_canonicalize_ownerless_duplication=0
callsite_canonicalize_refactor_required_now=0
```

### speculative fastpath vocabulary

The previously risky Rust-side speculative vocabulary is already retired or
active-owned:

```text
fastpath_reachability_rust_vocab_retired=1
fastpath_reachability_non_test_consumer_count=0
fastpath_deny_owner_code_retired=1
fastpath_deny_owner_source_presence_count=0
fastpath_decision_non_test_consumer_count=7
```

`LocalFastPathFact` and `FastPathDecision` are still live vocabulary and are
not zero-consumer residue. Reachability remains hako_check-owned post-hoc
tooling.

Decision:

```text
speculative_fastpath_rust_residue=0
fastpath_reachability_rust_code_residue=0
fastpath_decision_retire_allowed=0
```

### ExactStackObject

The active `ObjectStoragePlan` code no longer defines `ExactStackObject`:

```text
exact_stack_object_retired=1
exact_stack_object_source_presence_count=0
active_exact_storage_forms=ExactNativeStruct,Scalarized,FlattenedNestedFields
stack_allocation_support_claimed=0
```

Decision:

```text
exact_stack_object_code_residue=0
exact_stack_object_followup_required=0
```

### duplicate report key

`src/object_storage_plan/report.rs` has no duplicate report key at this audit:

```text
total_report_keys=87
duplicate_key_count=0
unknown_publication_forces_generic_fallback_count=1
```

Decision:

```text
duplicate_report_key_residue=0
report_key_cleanup_required=0
```

## Commands

```bash
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
python3 tools/hako_check/fastpath_consumer_inventory.py --format kv
python3 - <<'PY'
import re
from pathlib import Path
p = Path("src/object_storage_plan/report.rs")
keys = re.findall(r'\("([^"]+)",\s*"[^"]*"\)', p.read_text())
seen = {}
for key in keys:
    seen[key] = seen.get(key, 0) + 1
print(f"total_keys={len(keys)}")
print(f"duplicate_key_count={sum(1 for value in seen.values() if value > 1)}")
PY
rg -n "FastPathReachability|FastPathDenyOwner|ExactStackObject" \
  src/object_storage_plan src/mir src/runner -S
```

## Contract

```text
output_contract=fastpath-residue-audit-v0

callsite_canonicalize_4_entries_classification=intentional_schedule_entries
callsite_canonicalize_ownerless_duplication=0
callsite_canonicalize_refactor_required_now=0

speculative_fastpath_rust_residue=0
fastpath_reachability_rust_code_residue=0
fastpath_decision_retire_allowed=0

exact_stack_object_code_residue=0
duplicate_report_key_residue=0

implementation_started=0
backend_lowering_changed=0
route_priority_changed=0
current_blocker_unchanged=NEXT-LANE-SELECTION-AFTER-FASTPATH-SWEEP-001
summary=ok
```

## Stop Lines

```text
do not remove callsite canonicalize entries from this audit
do not retire LocalFastPathFact or FastPathDecision while non-test consumers exist
do not reopen ExactStackObject; stack allocation support is not claimed
do not change backend lowering from residue audit evidence
```
