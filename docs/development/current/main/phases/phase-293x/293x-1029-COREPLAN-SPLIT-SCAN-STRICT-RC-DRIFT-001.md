---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-SPLIT-SCAN-STRICT-RC-DRIFT-001
Scope: Align split_scan strict shadow smoke with the OK fixture runtime result.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/pattern6-7-contracts.md
  - tools/smokes/v2/profiles/integration/joinir/split_scan_strict_shadow_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/split_scan_release_adopt_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/flowbox_tag_coverage_gate_vm.sh
---

# COREPLAN-SPLIT-SCAN-STRICT-RC-DRIFT-001

## Decision

`apps/tests/split_scan_ok_min.hako` is the accepted split-scan fixture and
returns the split result length `3`. The release adopt wrapper already treats
exit code `3` as the correct result. The strict shadow wrapper uses the same
runtime result while separately checking the strict FlowBox tag.

`flowbox_tag_coverage_gate_vm` uses the shared `run_joinir_vm_strict` helper,
which routes this fixture through the VM-Hako subset check. That path emits the
FlowBox tag, then fail-fast rejects the unsupported global static call. The
coverage gate therefore owns only tag coverage plus the subset fail-fast marker,
not the behavior result.

This is a smoke-contract synchronization only. It does not add a new accepted
shape, fallback route, or planner behavior.

## Implementation

```text
split_scan_ok_fixture_expected_exit_code=3
split_scan_strict_shadow_vm_expected_exit_code=3
split_scan_release_adopt_vm_expected_exit_code=3
split_scan_coverage_gate_strict_expected_exit_code=1
split_scan_coverage_gate_strict_marker=mir_call(global:StringUtils.split_ok/2)
flowbox_tag_coverage_gate_default_timeout_secs=30
split_scan_strict_shadow_owns_flowbox_tag_check=1
accepted_shape_added=0
fallback_route_added=0
```

## Acceptance

```text
split_scan_strict_shadow_vm=PASS
split_scan_release_adopt_vm=PASS
flowbox_tag_coverage_gate_vm=PASS
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/split_scan_strict_shadow_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/split_scan_release_adopt_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/flowbox_tag_coverage_gate_vm.sh
```

## Stop Line

```text
do not change split_scan route acceptance from this card
do not make release wrappers depend on FlowBox tag streams
do not treat strict tag checks as behavior-result owners
```
