---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-FULL-GATE-DRIFT-001
Scope: Full phase29bq gate drift after PORT07 timeout closeout.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1023-COREPLAN-PORT07-TIMEOUT-001.md
  - tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
---

# COREPLAN-FULL-GATE-DRIFT-001

## Decision

After the PORT07 timeout budget correction, the default `phase29bq` gate passes
through BQ, Hako MIRBuilder pin rows, Program JSON contract pin, PORT04, and
PORT07.

The next `--full` blocker is not PORT07. It is a 29ae regression-pack drift:
`joinir_purity_gate_vm` expects the `StringUtils.is_integer` strict lane to
fail-fast reject, but the current route executes successfully and emits
FlowBox adoption tags.

The 29bp master list also had metadata drift: the scan-methods rows were
documented as carrying `timeout=60`, but the TSV rows had lost that metadata.
This row restores the documented per-row timeout and records the new first
full-gate blocker.

## Evidence

```text
phase29bq_fast_gate_vm.sh                         -> PASS (mode=bq, elapsed=4:44.34)
phase29bq_fast_gate_vm.sh --only selfhost_blocker_scan_methods_loop_min
                                                  -> PASS (elapsed=0:25.52)
phase29bq_fast_gate_vm.sh --full                  -> FAIL in 29ae purity_gate_vm
first_full_failure=joinir_purity_gate_vm:is_integer_strict_reject_exit_code_0
```

## Acceptance

```text
scan_methods_timeout_metadata_restored=1
scan_methods_timeout_budget_secs=60
phase29bq_fast_gate_vm_bq=PASS
phase29bq_fast_gate_vm_full_reaches_29ae=1
next_full_blocker=joinir_purity_gate_is_integer_strict_drift
accepted_shape_added=0
fallback_route_added=0
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_blocker_scan_methods_loop_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --full
```

## Stop Line

```text
do not update the is_integer strict gate expectation without a decision card
do not treat strict reject drift as a timeout issue
do not add a route fallback to satisfy 29ae
```
