---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P167, generic string collection births
Related:
  - docs/development/current/main/phases/phase-29cv/P166-GENERIC-I64-UNKNOWN-RETURN-WRAPPER.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P167: Generic String Collection Births

## Problem

After P166, the source-execution probe advances to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_instruction
```

The first unsupported canonical instruction in that body is a no-argument
`newbox ArrayBox`, followed by additional `ArrayBox` and `MapBox` births used as
local builder carriers.

## Decision

Allow `generic_pure_string_body` to classify no-argument `ArrayBox` and `MapBox`
birth instructions, and lower those births in the module generic string
function emitter through the existing C-ABI helpers:

- `ArrayBox` -> `nyash.array.birth_h`
- `MapBox` -> `nyash.map.birth_h`

This card does not accept collection methods, constructor arguments, object
returns, or collection-handle returns. Arrays and maps are only value classes
that keep the classifier from rejecting inert local births before the next real
collection-method blocker is surfaced.

## Acceptance

```bash
cargo test -q refresh_module_global_call_routes_accepts_collection_births_in_generic_pure_string_body --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p167_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The final `--emit-exe` command is an advance-to-next-blocker probe.

## Result

The probe advances past the collection birth instructions. The next blocker is:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

Treat collection methods in the normalizer body as the next card. Do not fold
that method-call surface into this collection-birth card.
