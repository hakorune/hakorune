---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P168, generic string ArrayBox length flow
Related:
  - docs/development/current/main/phases/phase-29cv/P167-GENERIC-STRING-NEWBOX-COLLECTION-BIRTH.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P168: Generic String Array Length Flow

## Problem

After P167, the source-execution probe advances to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The first method blocker is `RuntimeDataBox.size()` on an `ArrayBox` builder
carrier that flows through copies and all-array PHIs.

## Decision

Allow `generic_pure_string_body` to accept `ArrayBox` length observation
(`len` / `length` / `size`) when the receiver is proven from `new ArrayBox()`
through copy or all-array-PHI flow evidence.

The same evidence must publish a `generic_method.len` route with `ArrayLen`, and
the module generic string emitter must lower it through `nyash.array.slot_len_h`.

This card does not accept array `get`, `push`, `set`, map methods, mixed
collection PHIs, or collection-handle returns.

## Acceptance

```bash
cargo test -q records_runtime_data_array_len_from_phi_origin --lib
cargo test -q refresh_module_semantic_metadata_accepts_array_size_in_generic_pure_string_body --lib
cargo test -q global_call_routes --lib
cargo test -q generic_method_route_plan --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p168_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

The probe advances past `JsonFragNormalizerBox._normalize_instructions_array/1`.
The next blocker is:

```text
target_shape_blocker_symbol=BuilderFinalizeChainBox.coerce_promoted_mir_checked/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat that method-call surface as the next card. Do not fold it into the
ArrayBox length-flow card.
