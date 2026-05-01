---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P132, null-guarded generic string body blocker exposure
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P131-GLOBAL-CALL-GENERIC-I64-BODY.md
  - src/mir/global_call_route_plan.rs
---

# P132: Global Call Null Guard String Blocker

## Problem

After P131, debug flag helpers are lowerable, so the next blockers are debug
string helpers:

```text
Stage1InputContractBox._debug_len_inline/1
Stage1InputContractBox._debug_preview_inline/1
```

Both begin with a null guard. Before this card, MIR stopped at the guard's
`void` constant:

```text
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

That reason is too early; the real next unsupported surface is the method call
behind the guard (`length` / `substring`).

## Decision

Allow `null`/`void` constants in the generic pure string classifier only as
comparison sentinels for `==` / `!=` against string values. The sentinel cannot
flow through PHI, arithmetic, or return positions.

This is not a method-call acceptance card. The null guard becomes transparent
enough to expose the next blocker:

```text
target_shape_reason=generic_string_unsupported_method_call
```

## Rules

Allowed:

- classify `null`/`void` const as a comparison-only sentinel
- accept only `Eq` / `Ne` comparisons involving string/sentinel values
- keep returning or flowing the sentinel unsupported

Forbidden:

- accepting `length` or `substring`
- treating sentinel values as string returns
- backend-local null-guard detection
- changing `.hako` debug helper bodies

## Expected Evidence

After this card, `stage1_cli_env.hako` still fails fast, but the blocker moves
from null guard to method call:

```text
callee=Stage1InputContractBox._debug_len_inline/1
target_shape_reason=generic_string_unsupported_method_call
```

The next BoxCount can add a narrow method shape for `length`, or split debug
branch handling before accepting `substring`.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `generic_string_unsupported_method_call` for the active debug helper blocker.
