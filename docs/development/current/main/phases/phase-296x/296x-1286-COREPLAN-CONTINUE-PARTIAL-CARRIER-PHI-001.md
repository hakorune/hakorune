---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Fix the staged-loop continue fixture by routing loop-var updates through
  edge-local carrier PHI inputs instead of a shared step expression.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1285-COREPLAN-LOOP-ROUTE-DEBT-INVENTORY-001.md
  - docs/development/current/main/design/loop-cond-break-continue-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
---

# COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI

## Decision

`generic_loop_v1` must not evaluate the loop-var increment as one shared
`step_bb` expression when explicit continue edges exist. Continue and
fallthrough paths can have different edge-local next values:

```text
continue branch:
  may update loop_var before continue
  sends that branch-local value to the step join

fallthrough branch:
  executes the skipped tail loop increment in cleanup
  sends that fallthrough-local value to the step join
```

Therefore `generic_loop_v1` now uses edge-local `ContinueWithPhiArgs` for the
loop variable as well as other carriers. The header PHI reads the loop-var step
PHI, not a shared expression that is evaluated from one sibling path.

## Implementation

Changed behavior:

```text
generic_loop_v1 fallthrough cleanup:
  lowers the tail loop increment before appending ContinueWithPhiArgs

generic_loop_v1 carrier finalization:
  creates an empty loop-var step PHI when the body has continue edges
  creates the header PHI from preheader init + loop-var step PHI

generic_loop_v1 condition/step handoff:
  no longer emits shared loop-var step effects for v1
```

Unchanged:

```text
generic_loop_v0 shared step behavior
LoopSimpleWhile acceptance predicates
registry route suppression
json_native / RustSubset app-front behavior
```

## Acceptance

Promoted gate case:

```text
case_id=selfhost_read_number_continue_staged_min
fixture=apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
expected_output=3
expected_rc=0
required_tag=[flowbox/adopt box_kind=Loop features=break,continue via=shadow]
```

Verified command:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_read_number_continue_staged_min
```

Result:

```text
[PASS] phase29bq_fast_gate_cases:selfhost_read_number_continue_staged_min
[PASS] phase29bq_fast_gate_vm: PASS (mode=selfhost_read_number_continue_staged_min)
```

## Stop Lines

```text
do not add method-name / fixture-name branches
do not add a new named loop route
do not use registry suppression as the correctness owner
do not change app-front/json_native behavior
do not introduce a new PHI SSOT
```

## Report

```text
output_contract=coreplan-continue-partial-carrier-phi-v0
implementation_changed=1
gate_case=selfhost_read_number_continue_staged_min
planner_required_green=1
dominance_violation=0
shared_step_loop_var_eval_for_v1_continue=0
edge_local_continue_phi_args=1
registry_suppression_as_primary_fix=0
summary=ok
```
