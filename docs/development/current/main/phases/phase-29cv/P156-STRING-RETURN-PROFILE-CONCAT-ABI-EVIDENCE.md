---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P156, string return-profile raw-i64 concat evidence
Related:
  - docs/development/current/main/phases/phase-29cv/P155-STAGE1-RAW-PROGRAM-JSON-WRAPPER-RECIPE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/string_return_profile.rs
---

# P156: String Return Profile Concat ABI Evidence

## Problem

P155 moved the source-execution stop-line to:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

That reason was too early. `resolve_for_source/1` builds its return prefix with
string concatenation:

```hako
prefix = prefix + "\n" + code + "\n"
```

The MIR value metadata can still label those handle values as raw `i64`. The
return-profile scan treated that raw ABI label as semantic non-string evidence,
so the real first unsupported operation inside the resolver stayed hidden.

## Decision

Keep this as diagnostic/observational cleanup only:

- do not make `Stage1UsingResolverBox.resolve_for_source/1` lowerable
- do not add a backend by-name resolver rule
- do not call the compiled-stage1 compat stub
- do not add MapBox, ArrayBox, or FileBox support here

For string return-profile evidence:

- scalar `value_types` metadata is no longer semantic non-string proof
- `String + ...` is semantic proof that the result is a string handle
- loop-carried PHIs with observed string evidence and no observed non-string
  evidence may carry that string return-profile class through raw ABI metadata

This mirrors the P153 debug-string lesson, but only in the return-profile scan
used to expose the next owner boundary.

## Evidence

The MIR JSON route for the resolver now surfaces the real body blocker:

```text
Stage1SourceProgramAuthorityBox._merge_using_prefix/1
  -> Stage1UsingResolverBox.resolve_for_source/1
  tier=Unsupported
  target_shape_reason=generic_string_unsupported_extern_call
```

The top pure-first source-execution stop is still on the using resolver owner,
but no longer on the stale return ABI reason:

```text
target_shape_blocker_symbol=Stage1UsingResolverBox.resolve_for_source/1
target_shape_blocker_reason=generic_string_unsupported_extern_call
```

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_marks_string_concat_loop_before_unsupported_extern --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json /tmp/hakorune_p156_string_return_profile_concat.mir.json lang/src/runner/stage1_cli_env.hako
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p156_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The final `--emit-exe` command is accepted as an advance-to-next-blocker
probe, not a full green source-execution gate.
