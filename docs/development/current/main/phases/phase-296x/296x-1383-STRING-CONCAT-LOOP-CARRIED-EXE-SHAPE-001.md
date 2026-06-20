# 296x-1383 STRING-CONCAT-LOOP-CARRIED-EXE-SHAPE-001

Status: open
Date: 2026-06-20

## Purpose

Unblock crate-bundle aggregation by accepting the narrow EXE pure-route shape
needed to accumulate generated module text in a loop.

296x-1382 proved that the Main-owned dynamic FileBox route is no longer the
blocker. The next blocker is loop-carried string accumulation:

```hako
local output = ""
loop(i < modules.length()) {
    ...
    output = output + chunk
    i = i + 1
}
print(output)
```

## Selected By

```text
296x-1382-HAKORUNE-MIR-BUILDER-CRATE-BUNDLE-AGGREGATION-RESUME-001
```

## Problem

The first crate-bundle wrapper reaches MIR emit but fails EXE pure-route
lowering with an undefined LLVM value in string concat lowering:

```text
llvm_error=use_of_undefined_value
callee=nyash.string.concat3_hhh
undefined_value=%r667
failure_owner=loop_carried_string_concat_pure_route
```

This is not a RustSubset converter semantic issue and not a FileBox route
issue.

## Scope

Create a focused minimal probe for loop-carried string accumulation and either:

```text
repair the pure-route lowering shape
or
document a narrower compiler stop line if the shape is intentionally unsupported
```

The implementation must be compiler/backend-owned. Do not change converter
semantics to work around the missing shape.

## Acceptance

Focused probe:

```text
loop_carried_string_concat_mir_emit=green
loop_carried_string_concat_exe=green
output_matches_expected=green
```

Regression:

```text
filebox_dynamic_path_loop_guard=green
crate_bundle_wrapper_can_resume=1
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
do_not_move_bundle_output_to_host_tool=1
do_not_change_RustSubset_conversion_semantics=1
do_not_enable_use_resolution=1
do_not_enable_name_resolution=1
generated_program_execution_claim=0
```

If fixing this requires a broad string-corridor redesign, stop and split the
owner before editing the crate-bundle wrapper.
