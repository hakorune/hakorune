---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P178, generic string one-argument substring
Related:
  - docs/development/current/main/phases/phase-29cv/P177-STRING-OR-VOID-CORRIDOR-RETURN-PROFILE.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P178: Generic String Substring Suffix

## Problem

After P177, the source-execution probe advanced to:

```text
target_shape_blocker_symbol=JsonNumberCanonicalBox.canonicalize_f64/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

`canonicalize_f64/1` uses suffix substring forms such as `s.substring(1)`.
`generic_method_routes` already planned those calls as `generic_method.substring`,
but the generic string body classifier only accepted the two-argument
`substring(start, end)` surface. The module generic string emitter also only
lowered two explicit substring bounds.

## Decision

Accept `RuntimeDataBox.substring(start)` / `StringBox.substring(start)` in the
generic string classifiers when the receiver is string-class and the explicit
argument is scalar. The backend must still consume the existing
`generic_method.substring` / `StringSubstring` LoweringPlan entry.

For module generic string emission, lower one-argument substring as:

```text
substring(start, length(receiver))
```

The implicit length call is emitted by the module generic string emitter and the
prescan declares `nyash.string.len_h` when such a one-argument substring appears.
No raw method-name fallback or function-name special case is added.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_runtime_data_string_substring_suffix_method --lib
cargo test -q runtime_methods --lib
cargo test -q global_call_routes --lib
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p178_substring1_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonNumberCanonicalBox.canonicalize_f64/1` no longer blocks on the suffix
substring method surface.

The probe advances to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._const_canonicalize/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat `_const_canonicalize/1` method-call acceptance as the next card. Do not
fold it into one-argument substring lowering.
