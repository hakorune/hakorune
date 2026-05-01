---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P135, generic pure string debug print acceptance
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P134-GLOBAL-CALL-STRING-SUBSTRING-METHOD.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P135: Global Call String Print Side Effect

## Problem

After P134, the source-execution blocker moved to:

```text
main._run_emit_program_mode/0
  target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
  target_shape_blocker_reason=generic_string_unsupported_backend_global_call
```

The blocker is the existing backend global `print` in the debug branch of
`resolve_emit_program_source_text/0`.

## Decision

Allow the already-supported backend global `print` inside
`generic_pure_string_body` as a no-result debug side-effect. This does not make
`print` a same-module `global.user_call`, and it does not externalize the call.

The module generic string emitter consumes the same central MIR call global
surface policy used by the entry generic pure emitter:

```text
Global print -> HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT
```

## Rules

Allowed:

- `Global("print")` with one argument and no produced value inside a generic
  pure string body
- existing central print surface emission in ny-llvmc

Forbidden:

- treating `print` as a same-module user/global function
- adding a raw unresolved external function call for `print`
- widening this card to other backend globals
- changing debug branch semantics outside source-execution lowering

## Expected Evidence

After this card, `Stage1InputContractBox.resolve_emit_program_source_text/0`
should no longer block on `generic_string_unsupported_backend_global_call`.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `target/release/hakorune --emit-exe ... stage1_cli_env.hako` advances past
  the `resolve_emit_program_source_text/0` print blocker.
