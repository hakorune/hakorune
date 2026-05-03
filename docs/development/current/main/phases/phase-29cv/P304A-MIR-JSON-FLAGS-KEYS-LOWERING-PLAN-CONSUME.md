---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P304a, MIR JSON flags keys LoweringPlan and module-generic consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P226A-MIR-JSON-FLAGS-KEYS-ROUTE.md
  - docs/development/current/main/phases/phase-29cv/P227A-MIR-JSON-FLAGS-KEYS-VOID-GUARD.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/runner/mir_json_emit/route_json.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P304a: MIR JSON Flags Keys LoweringPlan Consume

## Problem

P303a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags/1
target_shape_blocker_reason=-
```

`_emit_flags/1` already has a MIR-owned exact route for the key projection:

```text
block=5374 instruction_index=3
route_id=generic_method.keys
route_kind=map_keys_array
proof=mir_json_flags_keys
helper=nyash.map.keys_h
```

The route is present in `generic_method_routes`, but it is not present in
`lowering_plan` because it is intentionally not a CoreMethod manifest row.
The module generic string emitter only reads `lowering_plan`, so its prepass
cannot type the result of `flags.keys()`.

## Decision

Materialize the existing exact `mir_json_flags_keys` route into
`lowering_plan` without widening generic map semantics:

```text
source=generic_method_routes
source_route_id=generic_method.keys
core_op=MapKeys
proof=mir_json_flags_keys
route_proof=mir_json_flags_keys
route_kind=map_keys_array
tier=DirectAbi
emit_kind=direct_abi_call
symbol=nyash.map.keys_h
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
publication_policy=no_publication
```

The C module generic string emitter consumes only this exact plan row and emits
`nyash.map.keys_h(receiver)`.

The MIR generic-method route fact also treats this exact `keys()` result as an
Array origin so the following `keys.length()` remains an ArrayLen route rather
than drifting into a StringLen route.

## Non-Goals

- no generic `MapBox.keys` acceptance
- no generic map iteration semantics in `generic_string_body`
- no new `GlobalCallTargetShape`
- no ny-llvmc body-specific emitter
- no fallback to raw method names in the C module emitter

## Acceptance

```bash
cargo test -q mir_json_flags_keys --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p304a_flags_keys.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
_emit_flags/1 no longer fails module_generic_prepass_failed at flags.keys()
```

Observed next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_flags_rec/4
target_shape_blocker_reason=-
```
