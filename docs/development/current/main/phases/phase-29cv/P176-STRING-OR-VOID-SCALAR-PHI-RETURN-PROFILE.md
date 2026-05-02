---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P176, string-or-void scalar PHI return profile
Related:
  - docs/development/current/main/phases/phase-29cv/P175-GENERIC-STRING-ARRAY-PUSH-FLOW.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/string_return_profile.rs
---

# P176: String-Or-Void Scalar PHI Return Profile

## Problem

After P175, the source-execution probe advanced to:

```text
target_shape_blocker_symbol=JsonNumberCanonicalBox.read_num_token/2
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`JsonNumberCanonicalBox.read_num_token/2` returns either a numeric token string
or `null`. Its final string return is a `substring(text, start, i)` where the
bound values flow through loop-carried scalar PHIs. The body scanner can prove
those bounds as scalar, but the weaker return-profile scan left them unknown
and therefore failed to prove the function as a string-or-void sentinel body.

## Decision

Allow the return-profile scan to carry scalar/other evidence through
loop-carried PHIs when either:

- the PHI has a scalar MIR type hint, or
- observed scalar evidence exists and there is no string-like or void evidence

This evidence is return-profile only. It does not bypass the normal body scan,
does not infer string handles from scalar metadata, and does not make arbitrary
helpers lowerable without matching body acceptance and LoweringPlan validation.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_loop_scalar_phi_substring_void_sentinel_body --lib
cargo test -q void_sentinel --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p176_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonNumberCanonicalBox.read_num_token/2` now passes the string-or-void return
profile instead of stopping on void/null sentinel evidence.

The probe advances to:

```text
target_shape_blocker_symbol=JsonFragBox.get_str/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

Treat `JsonFragBox.get_str/2` return ABI/profile handling as the next card. Do
not fold it into scalar PHI return-profile evidence.
