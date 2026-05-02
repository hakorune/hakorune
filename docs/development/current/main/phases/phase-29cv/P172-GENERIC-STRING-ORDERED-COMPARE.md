---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P172, generic string ordered compare
Related:
  - docs/development/current/main/phases/phase-29cv/P171-GENERIC-STRING-STRINGBOX-LEN-SELF-ARG.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P172: Generic String Ordered Compare

## Problem

After P171, the source-execution probe advances to:

```text
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

`JsonFragBox._decode_escapes/1` contains string range checks such as:

```text
c >= "0"
c <= "9"
c >= "a"
c <= "f"
```

The C generic string function emitter already lowers ordered string compares
through `nyash.string.lt_hh` / `nyash.string.eq_hh`, but the MIR-owned
`generic_pure_string_body` classifier only accepted `Eq` / `Ne` on string
operands.

## Decision

Allow `generic_pure_string_body` to accept ordered string comparisons
(`Lt` / `Le` / `Gt` / `Ge`) when both operands are already proven string flow
values.

This is not a method route and does not create a `LoweringPlan` method entry.
The authority boundary remains the target body shape: Facts accepts only proven
string operands, and ny-llvmc lowers the compare inside the generic string
function emitter.

This card does not infer string-ness from unknown operands, does not accept
mixed string/scalar ordered compares, and does not widen arbitrary compare
handling.

## Acceptance

```bash
cargo test -q refresh_module_semantic_metadata_accepts_ordered_string_compare_in_generic_pure_string_body --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p172_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonFragBox._decode_escapes/1` now passes the ordered string compare sites.
The probe advances to the next blocker in the same function:

```text
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

Treat the remaining void/null sentinel flow as the next card. Do not fold it
into ordered string compare acceptance.
