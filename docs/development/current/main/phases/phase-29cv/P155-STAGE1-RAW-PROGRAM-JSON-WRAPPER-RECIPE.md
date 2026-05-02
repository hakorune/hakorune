---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P155, exact Stage1 raw Program(JSON v0) wrapper recipe
Related:
  - docs/development/current/main/phases/phase-29cv/P154-PROGRAM-JSON-EMIT-BODY-RECIPE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/src/runner/stage1_source_program_authority_box.hako
  - src/mir/global_call_route_plan/program_json_emit_body.rs
---

# P155: Stage1 Raw Program JSON Wrapper Recipe

## Problem

P154 moved the source-execution stop-line to the object-return preparation
boundary:

```text
target_shape_blocker_symbol=BuildBox._new_prepare_scan_src_result/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

The next observed entry path reaches that boundary through a narrower Stage1
raw wrapper:

```hako
return BuildBox.emit_program_json_v0(source_text, null)
```

Lowering `BuildBox.emit_program_json_v0/2` generally would pull in the
MapBox/options bundle path and hide the real next owner. The source-execution
path currently passes `null` options, so the accepted shape can stay exact.

## Decision

Extend the existing `program_json_emit_body` target shape to also accept the
exact `Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1`
wrapper:

```text
program_json_emit_body
proof=typed_global_call_program_json_emit
return_shape=string_handle
value_demand=runtime_i64_or_handle
```

The Stage1 raw wrapper shape accepts only:

- one source parameter
- one `null` or `void` options sentinel
- one call to `BuildBox.emit_program_json_v0/2` with `(source, sentinel)`
- a return of that call result
- exact copies that preserve those values

This is not a general `BuildBox.emit_program_json_v0/2` lowering, not a MapBox
support slice, and not a bypass for the using-merge owner. The backend still
consumes only the MIR-owned proof.

## Evidence

The MIR JSON route now records the Stage1 raw wrapper callsite as direct:

```text
Stage1SourceProgramAuthorityBox._emit_program_json_from_source_checked/2
  -> Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1
  tier=DirectAbi
  target_shape=program_json_emit_body
  proof=typed_global_call_program_json_emit
```

The top pure-first source-execution stop moved to the next owner boundary:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_stage1_raw_program_json_wrapper_direct_target --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p155_stage1_raw_program_json_wrapper.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p155_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
