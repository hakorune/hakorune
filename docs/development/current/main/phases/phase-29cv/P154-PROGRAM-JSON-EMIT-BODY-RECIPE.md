---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P154, exact BuildBox Program(JSON v0) emit wrapper recipe
Related:
  - docs/development/current/main/phases/phase-29cv/P153-GENERIC-I64-DEBUG-STRING-SURFACE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/compiler/build/build_box.hako
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
---

# P154: Program JSON Emit Body Recipe

## Problem

P153 moved the source-execution stop-line to the Program(JSON v0) enrichment
wrapper:

```text
main._run_emit_program_mode/0
  target_shape_blocker_symbol=FuncScannerBox.scan_all_boxes/1
  target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The immediate blocker was not a general ArrayBox-return lowering problem. It
was reached through the exact `BuildBox._emit_program_json_from_scan_src/1`
wrapper:

```hako
local ast_json = me._parse_program_json_from_scan_src(scan_src)
if me._is_freeze_tag(ast_json) == 1 { return ast_json }
return BuildProgramFragmentBox.enrich(ast_json, scan_src)
```

Lowering `FuncScannerBox.scan_all_boxes/1` as a generic ArrayBox route would be
too broad for this stop-line.

## Decision

Add one MIR-owned direct target shape:

```text
program_json_emit_body
proof=typed_global_call_program_json_emit
return_shape=string_handle
value_demand=runtime_i64_or_handle
```

The shape accepts only the exact wrapper contract:

- one source parameter
- one call to `BuildBox._parse_program_json_from_scan_src/1` with that source
- one freeze check through `BuildBox._is_freeze_tag/1`
- a freeze branch that returns the parse result unchanged
- one call to `BuildProgramFragmentBox.enrich/2` with `(ast_json, scan_src)`
- a normal branch that returns the enrich result
- copies and single-input PHIs are allowed only when they preserve those values

ny-llvmc consumes only the MIR proof and emits a same-module function that calls:

```text
nyash.stage1.emit_program_json_v0_h(i64 source_text_handle) -> i64 program_json_handle
```

This is not a `FuncScannerBox` by-name lowering and not a general object-return
route.

## Evidence

The MIR JSON route now records both callsites from `BuildBox.emit_program_json_v0/2`
as direct:

```text
BuildBox.emit_program_json_v0/2 -> BuildBox._emit_program_json_from_scan_src/1
  tier=DirectAbi
  target_shape=program_json_emit_body
  proof=typed_global_call_program_json_emit
```

The top pure-first source-execution stop moved to the next owner boundary:

```text
target_shape_blocker_symbol=BuildBox._new_prepare_scan_src_result/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_program_json_emit_body_direct_target --lib
cargo test -q build_mir_json_root_emits_direct_plan_for_program_json_emit_body --lib
cargo test -q global_call_routes --lib
cargo test -q parser_program_json --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p154_program_json_emit_body.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p154_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
