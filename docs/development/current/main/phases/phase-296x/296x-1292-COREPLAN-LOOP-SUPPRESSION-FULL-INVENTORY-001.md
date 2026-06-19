---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Add a fail-closed full inventory entry for legacy loop route
  suppression and record the first existing blocker.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1291-COREPLAN-LOOP-ACTUAL-SELECTION-TRACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1290-COREPLAN-LOOP-LEGACY-OBSERVER-001.md
---

# COREPLAN-LOOP-SUPPRESSION-FULL-INVENTORY

## Decision

Full suppression inventory must be fail-closed:

```text
sampling_limit=none
failure_masking=0
head_or_tail_sampling=0
ignore_failures_with_or_true=0
```

The inventory reads two diagnostic seams:

```text
[plan/trace:loop_legacy_observer]
[plan/trace:loop_legacy_selected]
```

It does not select routes, delete suppressions, or feed observations back into
the resolver. Its only job is to show which legacy routes match, which route
actually succeeds, and where suppression remains.

## Implementation

Tool:

```text
tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_route_suppression_inventory.py
```

Contract:

```text
output_contract=coreplan-loop-suppression-full-inventory-v0
failure_masking=0
sampling_limit=none
summary=ok|failed
```

The tool reports failures as key-value output instead of a Python traceback.
This keeps the next owner visible while still returning non-zero.

## First Full-Inventory Blocker

The first full run stops at an existing fast-gate debt that is also present on
pushed HEAD `e5ed037b25`:

```text
case_id=loop_continue_only_multidelta_min
expected=31
actual=52
```

In strict/planner-required debug inventory mode, the same case reaches
`generic_loop_v1` and fails earlier with a PHI input availability contract:

```text
[plan/trace:loop_legacy_observer] decision=allow:generic_loop_v1 legacy_matched=generic_loop_v1 legacy_effective=generic_loop_v1 legacy_suppressed=none
[flowbox/adopt box_kind=Loop features=break,continue via=shadow]
[freeze:contract][loop_lowering/phi_input_not_available_in_pred]
```

This means suppression deletion is not allowed yet. The next row must first
separate normal fast-gate output debt from strict/planner-required inventory
debt, then choose the actual owner.

## Evidence

Current worktree:

```bash
cargo test -q registry::tests::
```

Result:

```text
31 passed
```

Existing HEAD comparison:

```bash
git worktree add --detach /tmp/hakorune-head-check e5ed037b25
cd /tmp/hakorune-head-check
cargo build --features vm-reference --bin hakorune
NYASH_BIN=target/debug/hakorune \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
  --only loop_continue_only_multidelta_min
```

Result:

```text
Expected: 31
Actual:   52
```

## Next Row

```text
next_task=COREPLAN-LOOP-MULTIDELTA-OWNER-SELECTION-001
```

The next row should decide whether `loop_continue_only_multidelta_min` is:

```text
normal fast-gate expected-output debt
strict/planner-required route-owner debt
generic_loop_v1 carrier/PHI contract debt
or fixture expectation drift
```

## Stop Lines

```text
do not delete registry suppression while full inventory is blocked
do not claim legacy observer parity with an independent resolver
do not use strict/planner-required inventory failures as release semantics
do not update expected output without owner selection
```

## Report

```text
output_contract=coreplan-loop-suppression-full-inventory-v0
implementation_changed=1
route_selection_changed=0
suppression_deleted=0
inventory_tool_added=1
failure_masking=0
sampling_limit=none
first_blocker=loop_continue_only_multidelta_min
first_blocker_present_on_head=1
next_task=COREPLAN-LOOP-MULTIDELTA-OWNER-SELECTION-001
summary=blocked_by_existing_gate_debt
```
