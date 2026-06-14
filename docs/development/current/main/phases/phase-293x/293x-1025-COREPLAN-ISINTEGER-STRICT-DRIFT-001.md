---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-ISINTEGER-STRICT-DRIFT-001
Scope: Restore `StringUtils.is_integer` strict fail-fast as a VM-Hako subset capability check.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1024-COREPLAN-FULL-GATE-DRIFT-001.md
  - src/runner/reference/vm_hako/subset_check/boxcalls.rs
  - tools/smokes/v2/profiles/integration/joinir/string_is_integer_strict_reject_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
---

# COREPLAN-ISINTEGER-STRICT-DRIFT-001

## Decision

Keep `StringUtils.is_integer` strict/dev as a fail-fast route, but move the
reject owner to the VM-Hako subset capability boundary.

CorePlan / FlowBox may observe and lower the loop structure before VM-Hako
subset validation runs. The unsupported part is not the loop shape; it is the
VM-Hako driver capability for non-`print` global `mir_call` targets such as
`StringUtils.is_integer/1`.

Therefore the accepted strict marker is now either the historical
`newbox(StringUtils)` subset reject or the current
`mir_call(global:StringUtils.is_integer/1)` subset reject. FlowBox tags are not
used as negative evidence for this row.

## Implementation

```text
vm_hako_subset_rejects_unsupported_global_mir_call=1
is_integer_strict_marker_accepts_global_mir_call=1
is_integer_release_adopt_unchanged=1
```

## Evidence

```text
cargo test -p nyash-rust --lib subset_rejects_unsupported_global_mir_call -> PASS
string_is_integer_strict_reject_vm.sh                                   -> PASS
string_is_integer_release_adopt_vm.sh                                    -> PASS
joinir_purity_gate_vm.sh with RUN_TIMEOUT_SECS=30                        -> PASS
phase29ae_regression_pack_vm                                             -> reaches next blocker
next_blocker=loop_simple_while_subset_reject_extra_stmt_over_accept
```

## Acceptance

```text
vm_hako_global_mir_call_capability_guard=1
strict_is_integer_fail_fast=1
strict_is_integer_exit_code=1
release_is_integer_exit_code=0
accepted_shape_added=0
fallback_route_added=0
```

## Proof

```bash
cargo test -p nyash-rust --lib subset_rejects_unsupported_global_mir_call
bash tools/smokes/v2/profiles/integration/joinir/string_is_integer_strict_reject_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/string_is_integer_release_adopt_vm.sh
RUN_TIMEOUT_SECS=30 bash tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh
```

## Stop Line

```text
do not treat FlowBox tags as negative evidence for is_integer strict
do not special-case StringUtils in the planner/router
do not allow VM-Hako unsupported global mir_call targets through subset check
```
