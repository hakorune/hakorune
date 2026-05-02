---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P165, generic string corridor method receiver proof
Related:
  - docs/development/current/main/phases/phase-29cv/P164-GENERIC-I64-STRING-ORDERED-COMPARE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/generic_method_route_plan.rs
---

# P165: Generic String Corridor Method Receiver

## Problem

After P164, the source-execution probe advances to:

```text
target_shape_blocker_symbol=StringScanBox.read_char/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`StringScanBox.read_char/2` is not blocked by its name. Its body uses the
existing supported string methods:

```text
text.length()
text.substring(i, i + 1)
```

The receiver is an unknown parameter, so `generic_pure_string_body` could not
prove it was a string before it reached the method call. However, the existing
string corridor pass already records those exact calls as `str.len` and
`str.slice` facts.

## Decision

Use existing string corridor method facts as the receiver proof for generic
pure string body classification and generic method route publication:

- `str.len` on a `RuntimeDataBox` / `StringBox` call proves that exact receiver
  is `StringBox` and the result is i64.
- `str.slice` on a `RuntimeDataBox` / `StringBox` call proves that exact
  receiver is `StringBox`, the bounds are i64, and the result is string.
- The C backend still consumes `generic_method.len` / `generic_method.substring`
  LoweringPlan rows; it does not regain raw method-name authority.

This does not classify arbitrary unknown parameters as string and does not add
a by-name route for `StringScanBox.read_char/2`.

## Acceptance

```bash
cargo test -q refresh_module_semantic_metadata_accepts_read_char_unknown_receiver_from_string_corridor --lib
cargo test -q runtime_methods --lib
cargo test -q generic_method_route_plan --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p165_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

The probe advances past `StringScanBox.read_char/2`. The next blocker is a
separate return-shape issue:

```text
target_shape_blocker_symbol=StringScanBox.find_quote/2
target_shape_blocker_reason=generic_string_return_not_string
```

Treat that as the next card.
