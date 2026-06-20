# 296x-1379 CRATE-BUNDLE-FILE-ROUTE-HELPER-EXE-SHAPE-001

Status: open
Date: 2026-06-20

## Purpose

Unblock the manifest-driven crate-bundle aggregation route by making the
FileBox-based reusable helper shape EXE-compatible, or by defining a narrower
input-route boundary that preserves the A2-lite aggregation design without
hand-unrolling the 7-module wrapper.

This row exists because 296x-1377 proved:

```text
helper_mir_emit=green
helper_exe_pure_route=red
```

## Selected By

```text
296x-1377-HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-001
```

## Problem

The selected crate-bundle helper needs to read a manifest and per-module
artifacts through `FileBox`.

Minimal failing shape:

```hako
box ReaderBox {
    read_text(path) {
        local file = new FileBox()
        if !file.open(path) {
            return null
        }
        local text = file.read_all()
        file.close()
        return text
    }
}
```

Focused row 1377 then hit the same family through the real helper:

```text
target_shape_blocker_symbol=RustSubsetCrateBundleFileRouteBox.convert_bundle/4
callee_symbol=RustSubsetCrateBundleFileRouteBox.convert_bundle/4
reason=module_generic_prepass_failed
trace_tag=[llvm-pure/unsupported-shape]
```

The earlier focused probe also exposed:

```text
target_shape_blocker_symbol=RustSubsetCrateBundleFileRouteBox.read_text/1
first_op=newbox
```

## Scope

Investigate and close one focused shape boundary:

```text
FileBox construction/use inside a reusable user-box helper target under EXE
pure-route lowering.
```

Allowed outcomes:

```text
A. compiler/backend fix:
   FileBox new/use inside this helper shape becomes EXE-compatible.

B. boundary decision:
   FileBox remains Main/input-wrapper owned, and the reusable helper accepts
   already-loaded manifest/module texts through a documented interface.
```

The row must pick one outcome based on evidence. Do not silently switch the
aggregation row to a hand-unrolled wrapper.

## Acceptance

```text
focused_filebox_helper_probe_mir_emit=green
focused_filebox_helper_probe_exe=green
```

or, if the correct result is a boundary decision:

```text
focused_filebox_helper_probe_mir_emit=green
focused_filebox_helper_probe_exe=red
boundary_decision_documented=1
next_implementation_row_named=1
```

Common checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_hand_unroll_7_module_wrapper=1
do_not_enable_use_resolution=1
do_not_enable_name_resolution=1
generated_program_execution_claim=0
crate_bundle_aggregation_implementation_committed=0
```

Do not change `RustSubsetConverter` core semantics in this row. This is an
input-route/backend-shape boundary, not a converter-core row.
