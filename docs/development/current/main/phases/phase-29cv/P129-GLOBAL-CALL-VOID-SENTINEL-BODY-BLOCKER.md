---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P129, MIR global-call void sentinel body blocker evidence
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P128-GLOBAL-CALL-VOID-SENTINEL-RETURN-REASON.md
  - src/mir/global_call_route_plan.rs
---

# P129: Global Call Void Sentinel Body Blocker

## Problem

P128 split `void`-signature string-or-void sentinel functions from the broad
return ABI blocker, but the stop was still too coarse:

```text
Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_reason=generic_string_return_void_sentinel_candidate
```

That proves the return profile is a sentinel candidate, but it hides the next
actual blocker inside the function body.

## Decision

For `MirType::Void` functions that are string-or-void sentinel candidates, MIR
runs a body blocker scan with the same ownership as the generic pure string
classifier. The scan allows `null`/`void` constants only as sentinel evidence and
reports the first unsupported body dependency through:

```text
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=<child target>
target_shape_blocker_reason=<child reason>
```

The target remains `tier=Unsupported` with `target_shape=null`.

## Rules

Allowed:

- reuse the MIR-owned global-call target classifier to expose body blockers
- allow `null`/`void` constants only in the void sentinel body scan
- keep child blocker evidence in `target_shape_blocker_*`

Forbidden:

- marking string-or-void sentinel functions as `generic_pure_string_body`
- adding a backend-local sentinel body scanner
- treating `generic_string_return_void_sentinel_candidate` as a permission to
  emit a call
- changing `vm-hako` or compat fallback behavior

## Expected Evidence

After this card, `stage1_cli_env.hako` still fails fast, but the blocker moves
inside the sentinel candidate:

```text
callee=Stage1InputContractBox.resolve_emit_program_source_text/0
target_return_type=void
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1InputContractBox._stage1_debug_on/0
target_shape_blocker_reason=generic_string_global_target_shape_unknown
```

The next slice can now inspect `_stage1_debug_on/0` and decide whether to split
debug-only branches or add a smaller accepted shape.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits the
  `_stage1_debug_on/0` child blocker for
  `Stage1InputContractBox.resolve_emit_program_source_text/0`.
