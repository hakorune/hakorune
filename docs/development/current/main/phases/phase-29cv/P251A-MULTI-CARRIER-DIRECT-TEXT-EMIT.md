---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P251a, LowerLoopMultiCarrier direct text emit
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P250A-MULTI-CARRIER-LIMIT-SCALAR-FACTS.md
  - lang/src/mir/builder/internal/lower_loop_multi_carrier_box.hako
---

# P251a: Multi-Carrier Direct Text Emit

## Problem

P250a moves the active route past `_extract_limit_info/4`. The next nested
blocker is:

```text
target_symbol=LowerLoopMultiCarrierBox.try_lower/2
target_shape_blocker_symbol=LowerLoopMultiCarrierBox._collect_carrier_initials/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`_collect_carrier_initials/3` returns an `ArrayBox` of carrier initial values.
The active caller then passes that ArrayBox into `LoopOptsBox.build2(opts)` via
a MapBox. This keeps object-return and option-map semantics in the Stage0 route.

## Decision

Do not add ArrayBox/MapBox DirectAbi support.

Add active carrier facts:

```text
_carrier_count(body_json, loop_idx, loop_var) -> i64
_carrier_value_text(body_json, loop_idx, loop_var, index) -> string
```

Emit the multi-carrier MIR JSON directly from `LowerLoopMultiCarrierBox`, using
the same register layout as `LoopFormBox.build_loop_multi_carrier`. Keep
`_collect_carrier_initials/3` as a legacy wrapper that materializes the old
ArrayBox shape from the scalar/text facts.

## Non-Goals

- no new `GlobalCallTargetShape`
- no ArrayBox/MapBox DirectAbi support
- no generic collection method acceptance
- no C body-specific emitter
- no change to multi-carrier loop semantics

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p251a_multi_carrier_direct_text_emit.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past
`LowerLoopMultiCarrierBox._collect_carrier_initials/3`; a later blocker may
remain.

## Result

`LowerLoopMultiCarrierBox.try_lower/2` now routes as
`generic_string_or_void_sentinel_body` with `DirectAbi`.

The active multi-carrier path no longer materializes the old carrier ArrayBox or
LoopOpts MapBox. The new scalar/text facts route as:

```text
LowerLoopMultiCarrierBox._carrier_count/3          generic_i64_body          DirectAbi
LowerLoopMultiCarrierBox._carrier_value_text/4     generic_pure_string_body  DirectAbi
LowerLoopMultiCarrierBox._emit_multi_count_json/7  generic_pure_string_body  DirectAbi
```

Register arithmetic used by the text emitter is isolated behind tiny i64
helpers so generic string classification does not reinterpret loop-carried
scalar values as string operands:

```text
LowerLoopMultiCarrierBox._reg_add2/2  generic_i64_body  DirectAbi
LowerLoopMultiCarrierBox._reg_add3/3  generic_i64_body  DirectAbi
LowerLoopMultiCarrierBox._reg_sub1/1  generic_i64_body  DirectAbi
```

The source-exe probe now advances to the next owner:

```text
target_shape_blocker_symbol=FuncLoweringBox.lower_func_defs/2
target_shape_blocker_reason=generic_string_unsupported_method_call
```
