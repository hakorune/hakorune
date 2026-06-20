# 296x-1380 FILEBOX-DYNAMIC-PATH-LOOP-EXE-SHAPE-001

Status: closed
Date: 2026-06-20

## Purpose

Unblock manifest-order crate-bundle input without hand-unrolling seven module
reads.

296x-1379 closed the reusable-helper FileBox shape as a boundary decision:
FileBox stays owned by `Main` / input-route code for now. The remaining
blocker is that `Main` cannot yet read a dynamic list of paths in a loop and
then call `FileBox.open(path, "r")` on the EXE pure route.

## Selected By

```text
296x-1379-CRATE-BUNDLE-FILE-ROUTE-HELPER-EXE-SHAPE-001
```

## Problem

Current green wrappers keep FileBox reads in `Main`, but they use a fixed,
hand-written number of artifact reads. The 7-module `hakorune_mir_builder`
crate-bundle aggregation needs manifest-order iteration instead.

Focused failing shape:

```hako
static box Main {
    main() {
        local paths = new ArrayBox()
        paths.push("CURRENT_TASK.md")
        paths.push("hako.toml")

        local i = 0
        loop(i < paths.length()) {
            local path = paths.get(i)
            local file = new FileBox()
            local opened = file.open(path, "r")
            ...
            i = i + 1
        }
    }
}
```

Observed result:

```text
focused_filebox_main_dynamic_loop_probe_mir_emit=green
focused_filebox_main_dynamic_loop_probe_exe=red
trace_tag=[llvm-pure/unsupported-shape]
callee_symbol=open
receiver_origin_box_name=RuntimeDataBox
reason=mir_call_no_route
```

This indicates the backend sees the `open` receiver as `RuntimeDataBox` rather
than preserving the `FileBox` origin for the loop-local `new FileBox()` value.

## Scope

Close the narrow input-route shape:

```text
FileBox new/open/read/close inside Main-owned loop over dynamic path strings.
```

Allowed implementation targets:

```text
route-origin repair for loop-local FileBox values
or
documented fail-fast boundary if FileBox-in-loop is intentionally unsupported
```

The preferred implementation is route-origin repair because it preserves the
selected crate-bundle design without returning to a seven-module hand-unrolled
wrapper.

## Acceptance

Focused probe:

```text
focused_filebox_main_dynamic_loop_probe_mir_emit=green
focused_filebox_main_dynamic_loop_probe_exe=green
receiver_origin_box_name=FileBox_or_filebox_route_selected
```

Regression:

```text
existing_crate_wrapper_exe_smoke=green
converter_core_changed=0
hand_unrolled_7_module_wrapper_fallback_used=0
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
do_not_move_filebox_into_reusable_helper=1
do_not_enable_use_resolution=1
do_not_enable_name_resolution=1
generated_program_execution_claim=0
converter_core_changed=0
```

Do not implement crate aggregation in this row. This row only establishes the
dynamic FileBox input-route shape needed by the later aggregation row.

## Result

```text
output_contract=rust-subset-filebox-dynamic-path-loop-exe-shape-v0
focused_filebox_main_dynamic_loop_probe_mir_emit=green
focused_filebox_main_dynamic_loop_probe_exe=green
summary=ok
```

Implemented boundary:

```text
FileBox newbox in pure lowering publishes ORG_FILEBOX origin.
FileBox handle metadata preserves ORG_FILEBOX through scan/origin maps.
MIR call dispatch accepts RuntimeDataBox receiver surface only when the
receiver/copy-base origin is ORG_FILEBOX.
```

Stable guard:

```bash
bash tools/checks/rust_subset_filebox_dynamic_path_loop_exe_shape_guard.sh
```

Common checks:

```bash
cargo check -q --lib
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Next:

```text
296x-1382-HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001
```
