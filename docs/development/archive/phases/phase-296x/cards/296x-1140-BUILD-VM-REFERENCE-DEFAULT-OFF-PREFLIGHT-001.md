Status: Done
Date: 2026-06-18
Scope: preflight removing vm-reference from the Cargo default feature set
Related:
  - docs/development/current/main/phases/phase-296x/296x-1138-BUILD-VM-REFERENCE-GATE-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1139-BUILD-VM-REFERENCE-BUILD-MEASURE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-VM-REFERENCE-DEFAULT-OFF-PREFLIGHT-001

## Decision

```text
output_contract=build-vm-reference-default-off-preflight-v0

candidate_default_features=["cli","plugins"]
removed_default_feature=vm-reference
vm_reference_feature_remains_available=1
full_no_default_support_claim=0
plugin_stub_fix_in_scope=0

input_measure_card=BUILD-VM-REFERENCE-BUILD-MEASURE-001
no_vm_cli_plugins_release_green=1
no_vm_cli_plugins_cold_build_real_sec=151.21
latest_default_baseline_cold_build_real_sec=161.28
candidate_real_delta_sec=-10.07

implementation_allowed_next=1
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-IMPLEMENTATION-001
summary=ok
```

The useful product default candidate is:

```toml
default = ["cli", "plugins"]
```

This makes VM execution an explicit reference feature while preserving the
shared `VMValue` / `VMError` vocabulary. The implementation row must verify
that the default build still supports MIR JSON / EXE product routes and
fail-fasts for VM-only terminals without a hidden AOT fallback.

## Acceptance For Implementation

```text
cargo_check_default_green=1
cargo_check_default_warning_count=0
cargo_check_vm_reference_green=1
cargo_check_no_default_cli_plugins_green=1
emit_mir_json_early_exit_preserved=1
emit_exe_early_exit_preserved=1
vm_terminal_without_feature_failfast=1
hidden_aot_fallback_added=0
```

## Stop Lines

```text
do_not_remove_vm_reference_feature=1
do_not_fix_plugin_stub_errors_in_default_off_row=1
do_not_claim_cargo_check_no_default_features_green=1
do_not_add_hidden_aot_fallback=1
```
