Status: Done
Date: 2026-06-18
Scope: close the vm-reference gate scaffold and select the next build-split row
Related:
  - docs/development/current/main/phases/phase-296x/296x-1126-BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001.md
  - docs/development/current/main/phases/phase-296x/296x-1137-BUILD-VM-COMMON-HELPERS-REFERENCE-GATE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-REFERENCE-GATE-CLOSEOUT-001

## Decision

```text
output_contract=build-vm-reference-gate-closeout-v0

vm_reference_feature_scaffold_closed=1
vm_reference_stays_default_on=1
vm_reference_default_off_claim=0

vm_direct_import_error_count_cli_plugins_without_vm_reference=0
mir_json_emit_route_preserved_without_vm_reference=1
exe_emit_route_preserved_without_vm_reference=1
hidden_aot_fallback_added=0

cargo_check_default_green=1
cargo_check_no_default_cli_plugins_green=1
cargo_check_no_default_cli_plugins_warning_count=0
cargo_check_no_default_features_green=0
remaining_no_default_failure=plugins_disabled_stub_surface
remaining_no_default_failure_is_vm=0

selected_next_task=BUILD-VM-REFERENCE-BUILD-MEASURE-001
summary=ok
```

The VM reference gate scaffold is closed. `vm-reference` now owns the
default-on Rust VM reference surface, while `VMValue` / `VMError` remain
available as shared value/error vocabulary. The no-VM `cli,plugins` check is
green, so direct VM imports no longer block a future no-VM product build
profile.

`cargo check --no-default-features` is still not a VM signal because disabling
all default features also disables plugin surfaces and exposes unrelated stub
API gaps. That must be handled by a plugin-stub row if full no-default support
is needed.

## Next Row

```text
next_task=BUILD-VM-REFERENCE-BUILD-MEASURE-001
purpose=measure the build impact of a no-vm cli/plugins profile before changing defaults
implementation_allowed=measurement_only
default_feature_change_allowed=0
```

The next row measures the useful candidate profile:

```bash
cargo check -q --no-default-features --features "cli,plugins"
```

and, if appropriate, a cold release build for the same feature set. This keeps
the VM retirement path evidence-based and avoids changing the default feature
set before the build-time benefit is visible.

## Stop Lines

```text
do_not_remove_vm_reference_from_default_in_this_row=1
do_not_fix_plugin_stub_errors_in_vm_closeout=1
do_not_claim_full_no_default_support=1
do_not_add_hidden_aot_fallback=1
```
