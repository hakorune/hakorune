---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381BP, PatternUtil local-value probe target-shape retirement
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P202-PATTERN-UTIL-LOCAL-VALUE-PROBE-BODY.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/pattern_util_local_value_probe_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P381BP: PatternUtil Local-Value Probe Target Shape Retire

## Problem

`PatternUtilLocalValueProbeBody` still existed as a public
`GlobalCallTargetShape` variant even though the ABI contract is fully described
by MIR-owned proof and return facts:

```text
proof=typed_global_call_pattern_util_local_value_probe
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
```

The only Rust-side dependency on the shape was recursive child-probe detection
inside the PatternUtil classifier. That is not a reason to keep a Stage0 JSON
target-shape string; the child route can read the stored proof/return contract
directly.

## Decision

Retire `PatternUtilLocalValueProbeBody` as a `GlobalCallTargetShape`.

The route still accepts the existing local-value probe body, but stores the ABI
truth as direct contract facts:

```text
target_shape=null
proof=typed_global_call_pattern_util_local_value_probe
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
```

Recursive child-probe recognition must consume those same proof/return facts.
C lowering predicates must not require the legacy target-shape string.

This card does not delete PatternUtil-specific body handling. That belongs to
the later uniform multi-function emitter cleanup.

## Boundary

Allowed:

- remove the `GlobalCallTargetShape::PatternUtilLocalValueProbeBody` variant
- keep the existing body recognizer as the proof producer
- keep mixed scalar/handle ABI behavior under the stored return contract

Not allowed:

- widen the PatternUtil classifier beyond the existing exact probe body
- replace child-probe recognition with source-name matching
- delete body handling before uniform multi-function MIR emission owns the path

## Evidence

The focused route tests cover both the int probe and the bool probe that depends
on a local-value child probe. They now assert a null target shape while checking
the PatternUtil proof and mixed return contract.

## Acceptance

```bash
cargo test --release pattern_util_local_value_probe -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
