---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-LOOP-TRUE-EARLY-EXIT-ROUTE-SMOKE-001
Scope: Keep the loop_true_early_exit route smoke focused on behavior while tag checks stay in tag-specific wrappers.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_release_adopt_vm.sh
---

# COREPLAN-LOOP-TRUE-EARLY-EXIT-ROUTE-SMOKE-001

## Decision

`loop_true_early_exit_vm` is the route/behavior smoke. It should accept the
VM exit code `3` even when strict/dev defaults emit FlowBox observability tags
into the raw stream. The strict shadow and release adopt wrappers follow the
same output rule, then separately verify their own tag contracts.

FlowBox tag assertions for this route remain owned by the dedicated
`loop_true_early_exit_strict_shadow_vm` and
`loop_true_early_exit_release_adopt_vm` wrappers.

## Implementation

```text
loop_true_early_exit_route_smoke_accepts_exit_code=3
loop_true_early_exit_route_smoke_owns_flowbox_tag_check=0
loop_true_early_exit_strict_smoke_accepts_exit_code=3
loop_true_early_exit_release_smoke_accepts_exit_code=3
accepted_shape_added=0
fallback_route_added=0
```

## Acceptance

```text
loop_true_early_exit_vm=PASS
loop_true_early_exit_strict_shadow_vm_owns_tag_check=1
loop_true_early_exit_strict_shadow_vm_accepts_exit_code=3
loop_true_early_exit_release_adopt_vm_owns_release_silence_check=1
loop_true_early_exit_release_adopt_vm_accepts_exit_code=3
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_strict_shadow_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/loop_true_early_exit_release_adopt_vm.sh
```

## Stop Line

```text
do not make route smoke depend on raw FlowBox stream cleanliness
do not remove strict/release tag-specific wrappers
do not add planner fallback for this smoke
```
