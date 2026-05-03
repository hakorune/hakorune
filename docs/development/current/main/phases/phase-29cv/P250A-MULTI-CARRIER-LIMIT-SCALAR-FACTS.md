---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P250a, LowerLoopMultiCarrier limit facts
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P249A-LOWER-LOOP-COUNT-PARAM-TEXT-SENTINEL.md
  - lang/src/mir/builder/internal/lower_loop_multi_carrier_box.hako
---

# P250a: Multi-Carrier Limit Scalar Facts

## Problem

P249a narrows the next nested blocker to:

```text
target_symbol=LowerLoopMultiCarrierBox.try_lower/2
target_shape_blocker_symbol=LowerLoopMultiCarrierBox._extract_limit_info/4
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```

`_extract_limit_info/4` returns a `MapBox` with:

```text
kind = "const" | "param"
value
param_reg
```

The active caller only needs those facts as scalar/text values. Accepting the
MapBox return would put object-return semantics back into Stage0.

## Decision

Do not add MapBox DirectAbi support and do not add a new body shape.

Add active scalar/text helpers:

```text
_extract_limit_kind(body_json, cmp_idx, loop_var, params_json) -> string
_extract_limit_value_text(body_json, cmp_idx, loop_var) -> string
_extract_limit_param_reg_text(body_json, cmp_idx, loop_var, params_json) -> string
```

`_extract_limit_kind` returns `""` on no-match. The active `try_lower/2` path
uses these helpers directly and only converts the param register text to i64 at
the final `LoopOptsBox` boundary. Keep `_extract_limit_info/4` as a legacy
wrapper that materializes the old MapBox shape from the scalar/text facts.

## Non-Goals

- no new `GlobalCallTargetShape`
- no MapBox DirectAbi support
- no generic MapBox method acceptance
- no C body-specific emitter
- no change to multi-carrier loop recognition

## Acceptance

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p250a_multi_carrier_limit_scalar_facts.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected source-exe result: the route should move past
`LowerLoopMultiCarrierBox._extract_limit_info/4`; a later blocker may remain.

## Result

Observed route metadata:

```text
target_symbol=LowerLoopMultiCarrierBox._extract_limit_kind/4
target_shape=generic_pure_string_body
return_shape=string_handle
tier=DirectAbi

target_symbol=LowerLoopMultiCarrierBox._extract_limit_value_text/3
target_shape=generic_pure_string_body
return_shape=string_handle
tier=DirectAbi

target_symbol=LowerLoopMultiCarrierBox._extract_limit_param_reg_text/4
target_shape=generic_pure_string_body
return_shape=string_handle
tier=DirectAbi

target_symbol=LowerLoopMultiCarrierBox._limit_target_type_idx/3
target_shape=generic_i64_body
return_shape=ScalarI64
tier=DirectAbi
```

The active route moved past `_extract_limit_info/4`. The next nested blocker is:

```text
target_symbol=LowerLoopMultiCarrierBox.try_lower/2
target_shape_blocker_symbol=LowerLoopMultiCarrierBox._collect_carrier_initials/3
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
```
