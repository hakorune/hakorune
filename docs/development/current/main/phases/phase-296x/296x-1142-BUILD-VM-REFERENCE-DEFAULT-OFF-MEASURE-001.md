Status: Done
Date: 2026-06-18
Scope: cold release build measurement after removing vm-reference from defaults
Related:
  - docs/development/current/main/phases/phase-296x/296x-1141-BUILD-VM-REFERENCE-DEFAULT-OFF-IMPLEMENTATION-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-VM-REFERENCE-DEFAULT-OFF-MEASURE-001

## Command

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune
```

## Result

```text
output_contract=build-vm-reference-default-off-measure-v0

release_build_status=green
release_build_target=hakorune
default_features=["cli","plugins"]
vm_reference_enabled_by_default=0

cold_build_real_sec=149.82
cold_build_user_sec=206.56
cold_build_sys_sec=9.71
cargo_reported_release_time=2m29s

baseline_card=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
baseline_default_cold_build_real_sec=161.28
default_off_cold_build_real_sec=149.82
default_off_real_delta_sec=-11.46

candidate_card=BUILD-VM-REFERENCE-BUILD-MEASURE-001
candidate_no_vm_cold_build_real_sec=151.21
default_off_vs_candidate_real_delta_sec=-1.39

build_time_winner_claim=1
vm_reference_default_off_build_time_win=1
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-CLOSEOUT-001
summary=ok
```

Removing `vm-reference` from Cargo defaults produced a visible cold release
build-time win for the default product profile. The Rust VM remains available
through `--features vm-reference`.

## Stop Lines

```text
do_not_delete_vm_reference_feature=1
do_not_claim_full_no_default_support=1
do_not_fix_plugin_stub_errors_in_measure_row=1
do_not_reopen_vm_product_route=1
```
