---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P209c, inline MirJsonEmitBox static schema field reads
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P209B-MIR-JSON-SOFT-MAP-SCAN-PRUNE.md
  - lang/src/shared/mir/json_emit_box.hako
---

# P209c: MIR JSON Static Field Read Inline

## Problem

P209b removes the recursive soft-map scan and advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._map_get_soft/2
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_map_get_soft/2` is now only:

```hako
if obj == null { return null }
return obj.get(key)
```

The remaining problem is the dynamic-key wrapper itself. Its callers pass
static MIR schema keys (`"op"`, `"operation"`, `"incoming"`, etc.), but the
wrapper erases that const-key fact from the callsite. Adding a new wrapper body
shape would make Stage0 understand another MirJsonEmitBox helper instead of
consuming existing MIR method-route facts.

## Decision

Inline the static schema field reads at each callsite:

```text
me._map_get_soft(inst, "op") -> inst.get("op")
```

Callers already own missing-key sentinel handling and schema fallback/default
policy. This keeps the const key at the MIR method-call site, so existing
generic method route facts remain the source of truth.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic dynamic-key map lookup body
- no generic `RuntimeDataBox.get` acceptance without route evidence
- no C body-specific emitter
- no change to sentinel/default policy at the caller

## Acceptance

Probe result should move past `_map_get_soft/2`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p209c_static_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

