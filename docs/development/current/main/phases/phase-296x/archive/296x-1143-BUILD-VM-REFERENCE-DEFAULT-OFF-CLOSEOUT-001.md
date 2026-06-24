Status: Done
Date: 2026-06-18
Scope: close vm-reference default-off build-time row
Related:
  - docs/development/current/main/phases/phase-296x/296x-1141-BUILD-VM-REFERENCE-DEFAULT-OFF-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1142-BUILD-VM-REFERENCE-DEFAULT-OFF-MEASURE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md

# BUILD-VM-REFERENCE-DEFAULT-OFF-CLOSEOUT-001

## Closeout

```text
output_contract=build-vm-reference-default-off-closeout-v0

vm_reference_default_off_closed=1
default_features=["cli","plugins"]
vm_reference_feature_remains_available=1
rust_vm_product_route_reopened=0
full_no_default_support_claim=0
plugin_stub_fix_in_scope=0

default_off_cold_build_real_sec=149.82
latest_default_baseline_cold_build_real_sec=161.28
default_off_real_delta_sec=-11.46
build_time_winner_claim=1

selected_next_task=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002
summary=ok
```

The Rust VM is now an explicit semantic-reference feature instead of part of
the product default build. This is a build-time win and a structural cleanup:
product app/selfhost validation should use EXE/AOT routes, while VM execution
is requested explicitly with `--features vm-reference`.

## Remaining Non-Goals

```text
full_no_default_features_green=0
remaining_full_no_default_owner=plugins_disabled_stub_surface
emit_exe_runtime_archive_gate_deferred=1
```

Full `cargo check --no-default-features` remains a separate plugin-stub surface.
The VM default-off row did not and should not absorb that work.

## Next

```text
next_task=BUILD-CRATE-SPLIT-NEXT-BOUNDARY-SELECTION-002
purpose=select the next build-time reduction boundary after the vm-reference default-off win
implementation_allowed=selection_only
```

The next selection should resume the crate-split ranking with current evidence
instead of continuing to patch VM reference surfaces.
