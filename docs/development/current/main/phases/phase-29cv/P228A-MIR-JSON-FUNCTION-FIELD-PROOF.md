---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P228a, MirJsonEmitBox function schema field proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P227A-MIR-JSON-FLAGS-KEYS-VOID-GUARD.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P228a: MIR JSON Function Field Proof

## Problem

P227a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_function/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The first unsupported sites in `_emit_function/1` are static schema field reads:

```hako
local name = func.get("name")
local params = func.get("params")
local flags  = func.get("flags")
local blocks = func.get("blocks")
```

These are exact MIR JSON schema projections. They should not become general
`RuntimeDataBox.get` support.

## Decision

Add an exact `mir_json_function_field` proof for `_emit_function/1`:

```text
name   -> StringOrVoid
params -> Array
flags  -> Map
blocks -> Array
```

The route remains the existing `generic_method.get` / `runtime_data_load_any`
route with the existing runtime helper:

```text
helper = nyash.runtime_data.get_hh
tier = ColdRuntime
```

## Non-Goals

- no generic function-object schema inference
- no generic MapBox/RuntimeDataBox get acceptance
- no new `GlobalCallTargetShape`
- no C body-specific emitter

## Acceptance

```bash
cargo test -q mir_json_function_field --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p228a_function_fields.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed result:

```text
mir_json_function_field routes are present for name/params/flags/blocks.
The outer probe still reports MirJsonEmitBox._emit_function/1
generic_string_unsupported_method_call because the next required fact is
collection-or-void PHI refinement before blocks.length().
```
