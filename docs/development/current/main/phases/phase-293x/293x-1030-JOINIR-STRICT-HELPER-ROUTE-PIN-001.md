---
Status: Landed
Date: 2026-06-14
Task: JOINIR-STRICT-HELPER-ROUTE-PIN-001
Scope: Make JoinIR strict/release smoke helpers hermetic against VM route-pin env leakage.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - tools/smokes/v2/lib/test_runner.sh
  - tools/smokes/v2/lib/vm_route_pin.sh
  - tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh
---

# JOINIR-STRICT-HELPER-ROUTE-PIN-001

## Decision

`run_joinir_vm_strict` and `run_joinir_vm_release` are route-owning helper
boundaries. They must not inherit `NYASH_VM_HAKO_PREFER_STRICT_DEV` from a
previous planner-first gate or caller environment.

The planner-first master list deliberately pins the compat VM route with
`NYASH_VM_HAKO_PREFER_STRICT_DEV=0`. If that value leaks into the later JoinIR
purity gate, the strict `StringUtils.is_integer` fixture runs as standard VM
and returns `0` instead of the intended VM-Hako subset fail-fast reject. The
strict helper now pins strict/dev preference explicitly, and the release helper
pins compat preference explicitly.

This is a smoke-helper hermeticity fix only. It does not change CorePlan route
selection, accepted shapes, or fixture semantics.

## Implementation

```text
run_joinir_vm_strict_route_pin=NYASH_VM_HAKO_PREFER_STRICT_DEV=1
run_joinir_vm_release_route_pin=NYASH_VM_HAKO_PREFER_STRICT_DEV=0
planner_first_route_pin_leak_blocked=1
accepted_shape_added=0
fallback_route_added=0
```

## Acceptance

```text
joinir_purity_gate_vm=PASS
joinir_purity_gate_vm_with_parent_route_pin_0=PASS
phase29bp_full_gate_advances_past_purity_gate=1
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
env NYASH_VM_HAKO_PREFER_STRICT_DEV=0 bash tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --full
```

## Stop Line

```text
do not make planner-first compat route pins global test state
do not weaken the is_integer strict subset fail-fast expectation
do not use output filtering to hide route-owner drift
```
