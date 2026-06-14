# COREPLAN-TIMEOUT-001: stageb bundle mod if timeout metadata

Status: Landed
Date: 2026-06-14
Scope: keep the compile-heavy StageB bundle-mod fixture in the fast gate with an
explicit per-row timeout.

## Problem

After `COREPLAN-PLANNER-TAG-001`, the full phase29bq fast gate passed the
previous FlowBox evidence blocker and stopped at:

```text
case=phase29bq_selfhost_blocker_stageb_bundle_mod_if_min.hako
failure=timeout
default_timeout=10s
```

The focused row passes when run with the existing list-gate per-row timeout
mechanism:

```bash
RUN_TIMEOUT_SECS=60 \
  bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh \
    --only selfhost_stageb_bundle_mod_if_min
```

## Decision

Add explicit `timeout=60` metadata to the TSV row.

This fixture imports the StageB module and is compile-heavy by design. The gate
already supports per-row timeout metadata, and another compile-heavy row uses
the same mechanism.

## Non-goals

```text
fixture_expected_output_changed=0
source_fixture_changed=0
loop_v0_route_added=0
fallback_route_added=0
accepted_shape_added=0
```

## Proof

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_stageb_bundle_mod_if_min
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh
```

Result:

```text
selfhost_stageb_bundle_mod_if_min=PASS
phase29bq_fast_gate_cases=PASS
```

The full fast gate now passes the BQ list and stops at the next independent
blocker:

```text
case=phase29bq_joinir_port04_phi_exit_invariant_lock_vm
failure=timeout
side=hako
rust=0
hako=124
```

## Next

```text
COREPLAN-PORT04-TIMEOUT-001:
  investigate the Hako-side timeout in phase29bq_joinir_port04_phi_exit_invariant_lock_vm.
```
