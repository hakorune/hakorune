---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P312a, MIR JSON PHI incoming array fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P214A-MIR-JSON-PHI-INCOMING-ARRAY-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P311A-MIR-JSON-PARAMS-ARRAY-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P312a: MIR JSON PHI Incoming Array Consume

## Problem

P311a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_phi_incoming_rec/3
target_shape_blocker_reason=-
```

`MirJsonEmitBox._emit_phi_incoming_rec/3` already has MIR-owned exact
`generic_method.get` rows from P214a:

```text
proof = mir_json_phi_incoming_array_item
  values.get(idx) -> mixed runtime i64 or handle

proof = mir_json_phi_incoming_pair_scalar
  item.get(0|1) -> scalar i64 or missing zero
```

The C module generic string emitter does not yet consume these exact ArrayGet
rows, so the prepass rejects the body even though the route facts are already
owned by MIR.

## Decision

Consume only the exact `mir_json_phi_incoming_array_item` and
`mir_json_phi_incoming_pair_scalar` ArrayGet rows in the module generic string
emitter.

## Non-Goals

- no generic `ArrayBox.get` widening
- no PHI incoming body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR PHI incoming schema change

## Acceptance

```bash
cargo test -q proves_mir_json_phi_incoming_array_get_routes --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p312a_phi_incoming.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox._emit_phi_incoming_rec/3 no longer fails module_generic_prepass_failed
on values.get(idx) or item.get(0|1).
```

## Result

Accepted. The C module generic string emitter now consumes exact
`mir_json_phi_incoming_array_item` and `mir_json_phi_incoming_pair_scalar`
ArrayGet rows through the planned `nyash.array.slot_load_hi` route.

Validation:

```bash
cargo test -q proves_mir_json_phi_incoming_array_get_routes --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p312.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `MirJsonEmitBox._emit_phi_incoming_rec/3` to:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox._norm_cmp_op_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```
