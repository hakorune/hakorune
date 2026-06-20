# 296x-1481 RUST-TO-HAKO-LIFECYCLE-PARITY-GATE-001

Status: closed
Date: 2026-06-20

## Purpose

Compare one emitted lifecycle surface against the Rust oracle for its selected
family only.

This row must not claim crate-wide or MirBuilder-wide lifecycle parity.

## Selected By

```text
296x-1480-RUST-TO-HAKO-LIFECYCLE-EMITTER-SURFACE-001
```

## Scope

```text
subject=CarrierInfo::merge_from
plan_kind=OwnedCarrierInfoMerge
surface=carrier-info-merge-from-emitter-surface-v0.hako
oracle=carrier-info-merge-from-oracle-vectors-v0.json
```

Allowed:

```text
fixture-only parity checker
bounded oracle comparison for selected family
diagnostic-only report
```

Forbidden:

```text
crate_wide_lifecycle_parity=1
mirbuilder_wide_lifecycle=1
backend_behavior_changed=1
rustc_adapter_integration=1
```

## Acceptance

```text
selected_family_parity_checked=1
surface_matches_oracle_contract=1
crate_wide_lifecycle_parity=0
mirbuilder_wide_lifecycle=0
backend_behavior_changed=0
rustc_integration_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
selected_family_parity_checked=1
surface_matches_oracle_contract=1
crate_wide_lifecycle_parity=0
mirbuilder_wide_lifecycle=0
backend_behavior_changed=0
rustc_integration_started=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_emitter_oracle_parity_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-emitter-oracle-parity-v0
selected_family_parity_checked=1
surface_matches_oracle_contract=1
crate_wide_lifecycle_parity=0
mirbuilder_wide_lifecycle=0
backend_behavior_changed=0
rustc_integration_started=0
summary=ok
```

Next:

```text
296x-1482-RUSTC-SEMIR-LIFECYCLE-FACTS-ADAPTER-PROBE-001
```

## Stop Line

```text
do_not_claim_broader_parity=1
do_not_integrate_rustc_adapter=1
do_not_change_emitter_policy=1
do_not_change_backend_behavior=1
```
