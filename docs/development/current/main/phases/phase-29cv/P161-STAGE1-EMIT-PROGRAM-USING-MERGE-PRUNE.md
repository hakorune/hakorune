---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P161, Stage1 emit-program using-merge prune
Related:
  - docs/development/current/main/phases/phase-29cv/P160-VOID-SIGNATURE-OBJECT-RETURN-BLOCKER.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - src/stage1/program_json_v0/authority.rs
  - lang/src/runner/stage1_cli_env.hako
---

# P161: Stage1 Emit-Program Using-Merge Prune

## Problem

P160 made the next boundary explicit:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox._collect_using_entries/1
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

That boundary was caused by the `emit-program` authority doing a same-file
using-prefix merge before calling `BuildBox.emit_program_json_v0(...)`.

The Rust Stage1 Program(JSON v0) authority already collects source `using`
lines into the Program(JSON) `imports` map. Keeping an extra `.hako` prefix
merge in front of it duplicated ownership and pulled `ArrayBox` resolver state
into the DirectABI source-execution path.

## Decision

Remove the `Stage1SourceProgramAuthorityBox._merge_using_prefix(...)` call from
the reduced Stage1 emit-program authority.

The exact source-only handoff is now:

```text
source_text -> _coerce_text_compat -> BuildBox.emit_program_json_v0(source, null)
```

This does not make `Stage1UsingResolverBox.resolve_for_source/1` lowerable and
does not add ArrayBox/MapBox lowering. Import ownership stays in the Stage1
Program(JSON v0) authority, where `collect_using_imports(...)` already records
aliases in the payload.

## Evidence

After this prune, the source-execution probe no longer stops at
`Stage1UsingResolverBox._collect_using_entries/1`. It advances to output
validation:

```text
target_shape_blocker_symbol=Stage1ProgramResultValidationBox.finalize_emit_result/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`Stage1SourceProgramAuthorityBox.emit_program_from_source/2` is now classified
as:

```text
target_shape=generic_string_or_void_sentinel_body
return_shape=string_handle_or_null
```

## Acceptance

```bash
cargo test -q source_to_program_json_v0_strict_accepts_stage1_cli_env_source --lib
cargo test -q emit_program_json_v0_for_current_stage1_build_box_mode_emits_stage1_cli_env_program_json --lib
target/release/hakorune --emit-mir-json /tmp/hakorune_p161_emit_program_no_using_merge.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p161_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker probe,
not a green source-execution gate.
