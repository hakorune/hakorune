---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv LoweringPlan tier/emit-kind audit closeout by reifying typed MIR vocabulary for backend-facing route plans
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381AR-INC-BOUNDARY-TRUTH-AND-RUNTIME-DECL-ATTR-AUDIT.md
  - src/mir/core_method_op.rs
  - src/mir/extern_call_route_plan.rs
  - src/mir/global_call_route_plan/model.rs
  - src/mir/generic_method_route_plan/model.rs
  - src/runner/mir_json_emit/route_json.rs
---

# P381AS: LoweringPlan Tier Vocabulary Reify

## Problem

The boundary audit in P381AR fixed the next keeper bucket as thin backend
boundary truth, but the MIR side still reconstructed LoweringPlan JSON tier and
emit-kind strings ad hoc at several emit sites.

That left two cleanup gaps:

1. `HotInline` existed in SSOT only as a future tier, not as a typed MIR
   vocabulary token.
2. route families (`generic_method`, `extern_call`, `global_call`) exposed plan
   tier / emit-kind mostly as raw strings, so JSON emission still had to rebuild
   plan semantics locally.

That shape was still better than `.inc` rediscovery, but it left stringly-typed
owner drift inside the MIR-to-JSON boundary.

## Decision

Reify the backend-facing LoweringPlan vocabulary in MIR and make JSON emission
consume typed route accessors instead of reconstructing plan strings inline.

Implemented:

- `LoweringPlanTier`
  - `HotInline`
  - `DirectAbi`
  - `ColdRuntime`
  - `Unsupported`
- `LoweringPlanEmitKind`
  - `inline_ir`
  - `direct_abi_call`
  - `direct_function_call`
  - `runtime_call`
  - `unsupported`

`CoreMethodLoweringTier` now maps explicitly into backend-facing plan tier /
emit-kind pairs:

- `hot_inline` -> `HotInline` + `inline_ir`
- `warm_direct_abi` -> `DirectAbi` + `direct_abi_call`
- `cold_fallback` -> `ColdRuntime` + `runtime_call`

The generic-method / extern-call / global-call route surfaces now expose typed
LoweringPlan accessors, and `route_json.rs` serializes from those typed enums.

## Boundary

Allowed:

- preserve current JSON tokens while reducing MIR-side string reconstruction
- keep `HotInline` reserved in the MIR vocabulary before any manifest row uses it
- keep global same-module calls distinct as `direct_function_call`

Not allowed:

- widening `.inc` responsibility
- changing runtime-decl attrs as part of tier cleanup
- solving `missing_multi_function_emitter` by adding new body-specific shim logic

## Acceptance

```bash
cargo fmt --all
cargo test --release core_method_op -- --nocapture
cargo test --release extern_call_route_plan -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
cargo test --release mir_json_emit -- --nocapture
```

## Result

Done:

- MIR now owns typed LoweringPlan tier / emit-kind vocabulary instead of
  reconstructing backend tokens only at JSON emission sites
- `HotInline` is represented explicitly in MIR carrier vocabulary even though no
  current manifest row promotes a keeper to that tier yet
- route JSON output stays stable while route families share typed accessors for
  tier/emit-kind ownership

Next:

1. plan the uniform multi-function emitter gap closeout
2. keep rejecting new body-specific `.inc` growth as the default answer
