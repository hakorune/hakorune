---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Run the B-lite loop resolver as a behavior-preserving observer beside
  existing named route selection.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1287-COREPLAN-LOOP-RESOLVER-B-LITE-SSOT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1284-COREPLAN-LOOP-RESOLVER-REAGGREGATION-TASKBOARD-001.md
---

# COREPLAN-LOOP-RESOLVER-SHADOW

## Decision

The B-lite resolver now runs as a shadow observer when JoinIR debug logging is
enabled. It does not select or alter the lowering route.

The shadow trace reports:

```text
raw_candidates
effective_candidates
suppressed_candidates
resolver_decision
route_disagreement
```

## Evidence

Target fixture:

```text
apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
```

Trace command:

```bash
NYASH_DISABLE_PLUGINS=1 \
NYASH_CLI_VERBOSE=0 \
NYASH_JOINIR_DEV=1 \
HAKO_JOINIR_STRICT=1 \
HAKO_JOINIR_PLANNER_REQUIRED=1 \
HAKO_JOINIR_DEBUG=1 \
HAKO_DEBUG=0 \
HAKO_SHOW_CALL_LOGS=0 \
HAKO_SILENT_TAGS=0 \
./target/release/hakorune --backend vm \
  apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako \
  2>&1 | rg "loop_resolver_b_lite|entry_candidates"
```

Observed output:

```text
[plan/trace:loop_resolver_b_lite] decision=allow:generic_loop_v1 raw=generic_loop_v1 effective=generic_loop_v1 suppressed=none disagreement=false
```

Interpretation:

```text
selected_named_route=generic_loop_v1
resolver_decision=allow
resolver_would_select=generic_loop_v1
route_disagreement=0
suppression_visible_for_target_fixture=0
```

## Gate

Verified behavior-preserving gate:

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
do not change route selection from shadow output
do not add a new named loop route
do not add registry suppression based on this row
do not claim broad resolver coverage from one fixture
```

## Report

```text
output_contract=coreplan-loop-resolver-shadow-v0
behavior_changed=0
shadow_loop_resolver_enabled=1
selected_named_route=generic_loop_v1
resolver_decision=allow
resolver_would_select=generic_loop_v1
route_disagreement=0
raw_candidates=generic_loop_v1
effective_candidates=generic_loop_v1
suppressed_candidates=none
summary=ok
```
