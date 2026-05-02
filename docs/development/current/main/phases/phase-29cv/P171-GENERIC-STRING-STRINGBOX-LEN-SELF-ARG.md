---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P171, generic string StringBox length self-arg
Related:
  - docs/development/current/main/phases/phase-29cv/P170-GENERIC-STRING-VOID-LOGGING-BODY.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P171: Generic String StringBox Len Self Arg

## Problem

After P170, the source-execution probe advances to:

```text
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The first unsupported method surface in `_decode_escapes/1` is a Known
`StringBox.length` call emitted with both a receiver and a single self argument:

```text
box=StringBox method=length receiver=<string> args=[<same string flow>]
```

The backend already knows how to lower string length through
`nyash.string.len_h`, but the MIR-owned generic method route planner only
accepted zero-arg length surfaces.

## Decision

Allow `generic_method.len` / `StringLen` and `generic_pure_string_body` to accept
Known `StringBox.length(self)` when the single argument is proven to be a string
flow value. The route records `arity=1` but keeps the same direct helper:

```text
source_route_id=generic_method.len
core_op=StringLen
symbol=nyash.string.len_h
route_proof=len_surface_policy
```

This card does not accept arbitrary one-arg `length` calls, RuntimeData
one-arg length calls, or any later `_decode_escapes/1` instruction blocker.

## Acceptance

```bash
cargo test -q records_stringbox_length_self_arg_route --lib
cargo test -q refresh_module_semantic_metadata_accepts_stringbox_length_self_arg_in_generic_pure_string_body --lib
cargo test -q generic_method_route_plan --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p171_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

`JsonFragBox._decode_escapes/1` now records the Known `StringBox.length(self)`
site as a direct `generic_method.len` route. The probe advances to the next
blocker in the same function:

```text
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

Treat the remaining unsupported instruction as the next card. Do not fold it
into the length self-arg route.
