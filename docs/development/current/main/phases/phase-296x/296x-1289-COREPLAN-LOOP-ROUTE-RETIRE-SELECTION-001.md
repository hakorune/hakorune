---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Select the first loop route debt to retire after B-lite resolver shadow
  evidence.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1288-COREPLAN-LOOP-RESOLVER-SHADOW-001.md
  - docs/development/current/main/phases/phase-296x/296x-1287-COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1284-COREPLAN-LOOP-RESOLVER-REAGGREGATION-TASKBOARD-001.md
---

# COREPLAN-LOOP-ROUTE-RETIRE-SELECTION

## Decision

Select the registry suppression seam as the first retire target, not a named
loop lowering route.

```text
selected_retire_candidate=registry_candidate_suppression
first_branch=loop_cond_continue_only_redundant_suppression
implementation_started=0
```

Rationale:

```text
1. The active continue fixture now has a single legacy effective candidate:
   generic_loop_v1 is both legacy matched and legacy effective.

2. A debug sweep over existing phase29bq fast-gate fixtures found no
   `legacy_suppressed != none` B-lite observer lines.

3. The remaining visible debt is therefore the registry suppression mechanism
   itself, not a proven wrong named-route lowering implementation.

4. The safest first branch is the statically redundant
   `loop_cond_continue_only` suppression. `pred_loop_cond_continue_only`
   already rejects `pred_loop_continue_only`, so suppressing
   `loop_cond_continue_only` again at registry level is duplicate policy.
```

This selection intentionally avoids changing loop behavior in this row. The
next implementation row must either delete a dead suppression branch with
green gates or prove a still-live branch and move that condition into the
owning route predicate.

## Evidence

Target fixture shadow:

```text
[plan/trace:loop_legacy_observer] decision=allow:generic_loop_v1 legacy_matched=generic_loop_v1 legacy_effective=generic_loop_v1 legacy_suppressed=none
```

Fast-gate fixture sweep command shape:

```bash
awk -F '\t' 'NF>=5 && $1 !~ /^#/ {print $1 "\t" $5}' \
  tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv \
| head -80 \
| while IFS=$'\t' read -r file case_id; do
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_CLI_VERBOSE=0 \
    NYASH_JOINIR_DEV=1 \
    HAKO_JOINIR_STRICT=1 \
    HAKO_JOINIR_PLANNER_REQUIRED=1 \
    HAKO_JOINIR_DEBUG=1 \
    HAKO_DEBUG=0 \
    HAKO_SHOW_CALL_LOGS=0 \
    HAKO_SILENT_TAGS=0 \
    timeout 10 ./target/release/hakorune --backend vm "$file" 2>&1 \
      | rg "loop_legacy_observer" || true
  done
```

Observed selection signal:

```text
suppressed_non_none_count=0
active_fixture_legacy_effective_singleton=generic_loop_v1
wrong_named_route_owner_selected=0
```

## Selected Next Row

```text
COREPLAN-LOOP-ROUTE-RETIRE-001
```

Scope:

```text
target=registry_candidate_suppression
first_branch=loop_cond_continue_only_redundant_suppression
```

Allowed implementation:

```text
remove dead suppression branch if the full fast gate stays green
or
move a live negative condition into the owning route predicate
```

Not allowed:

```text
do not delete LoopSimpleWhile / loop_cond_break_continue lowering
do not add another named route
do not add a new suppression condition
do not use B-lite shadow as a lowering selector
```

## Stop Lines

```text
do not implement route retirement in this selection row
do not claim every suppression branch is dead from the 80-case observation
do not change app-front/json_native behavior
do not modify VM product route policy
```

## Report

```text
output_contract=coreplan-loop-route-retire-selection-v0
implementation_changed=0
selected_retire_candidate=registry_candidate_suppression
first_branch=loop_cond_continue_only_redundant_suppression
active_fixture_legacy_effective_singleton=generic_loop_v1
suppressed_non_none_count=0
next_task=COREPLAN-LOOP-ROUTE-RETIRE-001
summary=ok
```
