Status: Done
Date: 2026-06-18
Scope: cold release build measurement for the no-vm cli/plugins profile
Related:
  - docs/development/current/main/phases/phase-296x/296x-1138-BUILD-VM-REFERENCE-GATE-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-VM-REFERENCE-BUILD-MEASURE-001

## Command

```bash
cargo clean && /usr/bin/time -p cargo build --release --bin hakorune --no-default-features --features "cli,plugins"
```

## Result

```text
output_contract=build-vm-reference-build-measure-v0

release_build_status=green
release_build_target=hakorune
feature_profile=cli,plugins
vm_reference_enabled=0

cold_build_real_sec=151.21
cold_build_user_sec=207.32
cold_build_sys_sec=10.76
cargo_reported_release_time=2m31s

baseline_card=BUILD-MIR-JSON-EMIT-POST-SPLIT-MEASURE-001
baseline_default_cold_build_real_sec=161.28
candidate_no_vm_cold_build_real_sec=151.21
candidate_real_delta_sec=-10.07

post_stage1_card=BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001
post_stage1_default_cold_build_real_sec=158.95
candidate_vs_post_stage1_real_delta_sec=-7.74

build_time_candidate_visible=1
default_feature_changed=0
winner_claim=profile_candidate_only
summary=ok
```

The no-VM `cli,plugins` profile is a visible build-time candidate. This is not
yet a default-route claim because the default feature set has not changed in
this row.

## Decision

```text
selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-PREFLIGHT-001
reason=no_vm_cli_plugins_profile_is_green_warning_free_and_build_time_candidate_is_visible
implementation_allowed=preflight_only
```

The next row should preflight removing `vm-reference` from the default feature
set:

```toml
default = ["cli", "plugins"]
```

That row must prove the product default still preserves MIR JSON / EXE routes,
fails fast for VM-only terminals, and does not reopen full `--no-default-features`
plugin stub work.

## Stop Lines

```text
do_not_change_default_features_in_measure_row=1
do_not_claim_full_no_default_support=1
do_not_fix_plugin_stub_errors_in_vm_measure_row=1
do_not_add_hidden_aot_fallback=1
```
