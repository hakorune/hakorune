---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P177, string-or-void corridor return profile
Related:
  - docs/development/current/main/phases/phase-29cv/P176-STRING-OR-VOID-SCALAR-PHI-RETURN-PROFILE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/string_return_profile.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P177: String-Or-Void Corridor Return Profile

## Problem

After rebuilding and re-running the P176 probe from a clean tree, the active
blocker was still:

```text
target_shape_blocker_symbol=JsonNumberCanonicalBox.read_num_token/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

The final successful return in `read_num_token/2` is a `substring(pos, i)`.
MIR already had `string_corridor_facts` proving that call as `str.slice`, but
the string-or-void return-profile scanner did not seed receiver/start/end/result
classes from that SSOT. The body acceptance path also missed the same corridor
seed when checking void-sentinel bodies.

## Decision

Use existing `string_corridor_facts` as the seed for string-or-void return
profile and void-sentinel body acceptance:

- `str.slice` proves receiver/result as string and start/end as scalar
- `str.len` proves receiver as string and result as scalar
- no new AST rewrite, function-name special case, or backend fallback is added

This keeps the ownership with the MIR corridor SSOT and avoids duplicating the
substring legality proof in the return profile.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_corridor_fact_substring_void_sentinel_body --lib
cargo test -q void_sentinel --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p177_corridor_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonNumberCanonicalBox.read_num_token/2` no longer blocks on return ABI/profile
classification.

The probe advances to:

```text
target_shape_blocker_symbol=JsonNumberCanonicalBox.canonicalize_f64/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat `JsonNumberCanonicalBox.canonicalize_f64/1` method-call acceptance as the
next card. Do not fold it into corridor return-profile seeding.
