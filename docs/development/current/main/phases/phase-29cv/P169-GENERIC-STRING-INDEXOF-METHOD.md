---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P169, generic string indexOf observation
Related:
  - docs/development/current/main/phases/phase-29cv/P168-GENERIC-STRING-ARRAY-LEN-FLOW.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P169: Generic String IndexOf Method

## Problem

After P168, the source-execution probe advances to:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox.coerce_promoted_mir_checked/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The first unsupported method surface in that body is `RuntimeDataBox.indexOf`
on proven string data with a string needle.

## Decision

Allow `generic_pure_string_body` to accept string `indexOf` observation when:

- the receiver is proven string
- the needle is proven string
- the call has exactly one argument
- the matching `generic_method.indexOf` / `StringIndexOf` LoweringPlan entry is present for lowering

The method result is scalar i64. This card does not accept non-string needles,
general RuntimeData methods, or the void logging helper in
`BuilderFinalizeChainBox.log_fail/1`.

## Acceptance

```bash
cargo test -q refresh_module_semantic_metadata_accepts_string_indexof_in_generic_pure_string_body --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p169_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

The probe advances past `BuilderFinalizeChainBox.coerce_promoted_mir_checked/1`
string `indexOf` surfaces. The next blocker is:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox.log_fail/1
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

Treat the void-return logging helper as the next card. Do not fold it into the
string `indexOf` method card.
