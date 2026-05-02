---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P166, generic i64 unknown-return wrapper
Related:
  - docs/development/current/main/phases/phase-29cv/P165-GENERIC-STRING-CORRIDOR-METHOD-RECEIVER.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_i64_body.rs
---

# P166: Generic I64 Unknown-Return Wrapper

## Problem

After P165, the source-execution probe advances to:

```text
target_shape_blocker_symbol=StringScanBox.find_quote/2
target_shape_blocker_reason=generic_string_return_not_string
```

`StringScanBox.find_quote/2` is a thin scalar wrapper:

```text
return StringScanBox.find_unescaped(text, "\"", pos)
```

The child target is already classified as `generic_i64_body`, but the wrapper's
signature return type is `?`, so `generic_i64_body` rejected it before checking
that the canonical return value was proven i64.

## Decision

Allow `generic_i64_body` to enter the body scan when the return signature is
`i64` or `?`. The existing return validation remains authoritative:

- every explicit return must be present
- every returned value must be proven i64 by the fixpoint
- params must stay within the existing handle-compatible scalar/string set

This does not classify unknown returns as scalar by default and does not add a
by-name route for `StringScanBox.find_quote/2`.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_unknown_return_generic_i64_wrapper --lib
cargo test -q generic_i64 --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p166_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

The probe advances past `StringScanBox.find_quote/2`. The next blocker is:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

Treat that as the next card.
