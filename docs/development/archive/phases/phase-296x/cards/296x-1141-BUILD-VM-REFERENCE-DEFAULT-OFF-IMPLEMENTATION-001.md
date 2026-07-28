Status: Done
Date: 2026-06-18
Scope: remove vm-reference from Cargo default features
Related:
  - docs/development/current/main/phases/phase-296x/296x-1140-BUILD-VM-REFERENCE-DEFAULT-OFF-PREFLIGHT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-VM-REFERENCE-DEFAULT-OFF-IMPLEMENTATION-001

## Change

```toml
default = ["cli", "plugins"]
```

`vm-reference` remains available as an explicit feature. This makes the product
default profile no-VM while preserving the Rust VM as a semantic reference
route.

## Proof

```text
output_contract=build-vm-reference-default-off-implementation-v0

default_features=["cli","plugins"]
vm_reference_removed_from_default=1
vm_reference_feature_removed=0
vm_reference_explicit_profile_green=1

cargo_check_default_green=1
cargo_check_default_warning_count=0
cargo_check_features_vm_reference_green=1
cargo_check_features_vm_reference_warning_count=0
cargo_check_no_default_cli_plugins_green=1
cargo_check_no_default_cli_plugins_warning_count=0

emit_mir_json_default_no_vm_green=1
vm_terminal_without_feature_failfast=1
hidden_aot_fallback_added=0
full_no_default_support_claim=0
plugin_stub_fix_in_scope=0

selected_next_task=BUILD-VM-REFERENCE-DEFAULT-OFF-MEASURE-001
summary=ok
```

Commands:

```bash
cargo check -q
cargo check -q --features vm-reference
cargo check -q --no-default-features --features "cli,plugins"
cargo run -q --bin hakorune -- --backend vm basic_test.hako
rm -f /tmp/hakorune-default-off-basic.mir.json && \
  cargo run -q --bin hakorune -- --emit-mir-json /tmp/hakorune-default-off-basic.mir.json basic_test.hako && \
  test -s /tmp/hakorune-default-off-basic.mir.json
```

The explicit VM command exits with fail-fast:

```text
VM keep/reference execution is not available in this build.
Rebuild with --features vm-reference or use an explicit EXE/AOT emit route.
```

## Deferred

```text
emit_exe_default_no_vm_checked=0
emit_exe_deferred_reason=libnyash_kernel_archive_not_present_in_workspace
```

EXE emit preservation is structurally unchanged by this row. A product-route
gate may verify it once the native runtime archive is available.

## Stop Lines

```text
do_not_remove_vm_reference_feature=1
do_not_fix_plugin_stub_errors_in_default_off_row=1
do_not_claim_cargo_check_no_default_features_green=1
do_not_add_hidden_aot_fallback=1
```
