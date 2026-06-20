# 296x-1379 CRATE-BUNDLE-FILE-ROUTE-HELPER-EXE-SHAPE-001

Status: closed
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

## Result

Closed by selecting the boundary decision outcome.

The focused probes show that pushing FileBox ownership into a reusable helper
currently opens larger backend/function-emission work than this aggregation row
should own.

### Probe A: user-box helper

Shape:

```hako
box ReaderBox {
    read_text(path) {
        local file = new FileBox()
        ...
        return text
    }
}
```

Result:

```text
focused_filebox_helper_probe_mir_emit=green
focused_filebox_helper_probe_exe=red
callee_symbol=read_text
receiver_origin_box_name=ReaderBox
reason=mir_call_no_route
user_box_method_route_reason=user_box_method_body_unsupported
target_body_supported=false
```

Even after forcing the helper to return a non-null string, the route stays:

```text
definition_owner=none
emit_kind=unsupported
proof=typed_user_box_method_contract_missing
```

### Probe B: static/global helper

Shape:

```hako
static box ReaderUtils {
    read_text(path) {
        local file = new FileBox()
        ...
        return text
    }
}
```

Result:

```text
focused_filebox_static_helper_probe_mir_emit=green
focused_filebox_static_helper_probe_exe=red
callee_symbol=ReaderUtils.read_text/1
reason=module_generic_prepass_failed
global_route_reason=missing_multi_function_emitter
target_shape_reason=generic_string_return_abi_not_handle_compatible
```

### Probe C: Main/input route with dynamic loop

Shape:

```hako
static box Main {
    main() {
        local paths = new ArrayBox()
        paths.push("CURRENT_TASK.md")
        paths.push("hako.toml")
        loop(i < paths.length()) {
            local path = paths.get(i)
            local file = new FileBox()
            local opened = file.open(path, "r")
            ...
        }
    }
}
```

Result:

```text
focused_filebox_main_dynamic_loop_probe_mir_emit=green
focused_filebox_main_dynamic_loop_probe_exe=red
callee_symbol=open
receiver_origin_box_name=RuntimeDataBox
reason=mir_call_no_route
```

Decision:

```text
selected_outcome=B_boundary_decision
filebox_helper_exe_fix_started=0
converter_core_changed=0
hand_unrolled_7_module_wrapper_fallback_used=0
```

The crate-bundle helper must not own FileBox directly until the backend can
support that shape. The next implementation blocker is narrower:

```text
296x-1380-FILEBOX-DYNAMIC-PATH-LOOP-EXE-SHAPE-001
```

That row keeps FileBox in the Main/input-route owner and focuses on preserving
the FileBox receiver route when paths are read dynamically in manifest order.
