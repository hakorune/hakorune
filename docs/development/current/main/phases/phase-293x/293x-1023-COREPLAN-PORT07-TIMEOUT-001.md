---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-PORT07-TIMEOUT-001
Scope: Timeout budget correction for PORT07 expression parity selfhost gate.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port07_expr_parity_seed_vm.sh
  - docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md
---

# COREPLAN-PORT07-TIMEOUT-001

## Decision

The PORT07 failure was a gate budget problem, not a new CorePlan accepted-shape
gap.

`phase29bq_joinir_port07_expr_parity_seed_vm.sh` previously used a 30 second
default timeout. Direct Stage-B Program(JSON) emission for even a minimal
`return 0` source can consume nearly that whole budget in the current
selfhost-first configuration, leaving insufficient time for the
Program(JSON)->MIR builder child.

The gate default is now 180 seconds. The fixture, expected output, route
contract, and accepted shape are unchanged.

## Evidence

```text
RUN_TIMEOUT_SECS=10  -> timeout
RUN_TIMEOUT_SECS=180 -> PASS (elapsed=1:18.55 on 2026-06-14)
focused_fixture=apps/tests/phase29bq_joinir_port07_expr_unary_compare_logic_seed_min.hako
```

## Acceptance

```text
port07_hako_timeout=0
phase29bq_joinir_port07_expr_parity_seed_vm=PASS
port07_timeout_budget_secs=180
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_joinir_port07_expr_parity_seed_vm.sh
```

## Stop Line

```text
do not treat timeout-budget fixes as CorePlan expressivity wins
do not add a loop route for this row
do not change fixture output or expression acceptance policy in this row
```
