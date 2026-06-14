---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-LOOP-SIMPLE-WHILE-SUBSET-REJECT-OVERACCEPT-001
Scope: Restore the loop_simple_while subset-reject FlowBox negative gate as a fixture-local observation.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1025-COREPLAN-ISINTEGER-STRICT-DRIFT-001.md
  - docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
  - tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh
---

# COREPLAN-LOOP-SIMPLE-WHILE-SUBSET-REJECT-OVERACCEPT-001

## Decision

Keep `loop_simple_while_subset_reject_extra_stmt_vm` as a strict FlowBox
negative gate, but make the raw tag check fixture-local.

The fixture still returns `3`, so the loop body is not dropped. The observed
failure came from `NYASH_JOINIR_DEV=1` smoke defaults: stage3/dev support
compilation can emit unrelated FlowBox `break` / `continue` tags before the
target fixture result. That noise must not be treated as over-acceptance of the
target fixture.

Therefore this smoke keeps `HAKO_JOINIR_STRICT=1` and pins
`NYASH_JOINIR_DEV=0` for the target run.

## Implementation

```text
loop_simple_while_subset_reject_fixture_local_negative_gate=1
loop_simple_while_subset_reject_nyash_joinir_dev_override=0
loop_simple_while_subset_reject_hako_joinir_strict=1
loop_simple_while_subset_reject_timeout_secs=30
accepted_shape_added=0
fallback_route_added=0
```

## Evidence

```text
loop_simple_while_subset_reject_extra_stmt_vm.sh -> PASS
```

## Acceptance

```text
loop_simple_while_subset_reject_exit_code=3
loop_simple_while_subset_reject_fixture_local_flowbox_noise=0
loop_simple_while_subset_reject_timeout_secs=30
accepted_shape_added=0
fallback_route_added=0
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/loop_simple_while_subset_reject_extra_stmt_vm.sh
```

## Stop Line

```text
do not weaken the fixture expected exit code
do not remove the negative FlowBox gate
do not treat unrelated stage3/dev support FlowBox tags as target-fixture evidence
do not add a planner fallback for this smoke
```
