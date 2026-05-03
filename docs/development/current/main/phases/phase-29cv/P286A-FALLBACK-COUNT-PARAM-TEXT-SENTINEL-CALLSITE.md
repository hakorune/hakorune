---
Status: Done
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P286a, fallback count-param text sentinel callsite
Related:
  - docs/development/current/main/phases/phase-29cv/P249A-LOWER-LOOP-COUNT-PARAM-TEXT-SENTINEL.md
  - docs/development/current/main/phases/phase-29cv/P278A-COUNT-PARAM-CALLSITE-TEXT-MATERIALIZATION.md
  - lang/src/mir/builder/internal/fallback_authority_box.hako
  - lang/src/mir/builder/internal/lower_loop_count_param_box.hako
---

# P286a: Fallback Count-Param Text Sentinel Callsite

## Problem

After P285a, the source-execution probe advances to:

```text
reason=missing_multi_function_emitter
target_shape_blocker_symbol=LowerLoopCountParamBox.try_lower_text/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

The blocker is not inside `LowerLoopCountParamBox.try_lower_text/1` itself. The
remaining source caller in `BuilderFallbackAuthorityBox._try_boxed_lowerers/1`
still checks the text-sentinel API with a nullable contract:

```hako
if out_loopp != null && out_loopp != "" { return out_loopp }
```

`try_lower_text/1` is already contracted as:

```text
"" means no-match
non-empty string means lowered MIR JSON
```

So the `null` check reintroduces a void sentinel comparison at the callsite.

## Decision

Keep the owner-local text sentinel contract and remove the stale nullable check
at this callsite:

```hako
if out_loopp != "" { return out_loopp }
```

This mirrors the already-clean `FuncBodyBasicLowerBox._try_lower_loop/4`
callsite and does not widen Stage0.

Also keep the final successful emit path as a direct child string return:

```hako
return me._emit_count_param_json(start_text, limit, step, cmp)
```

The child route already publishes `string_handle`. Re-wrapping it through
`"" + child` drops the useful direct child return fact and leaves the parent
classified through the stale void-sentinel path.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic void-sentinel classifier widening
- no C shim/body-specific emitter change
- no accepted count-loop shape change
- no fallback/compat route behavior change

## Acceptance

- `BuilderFallbackAuthorityBox._try_boxed_lowerers/1` no longer classifies the
  count-param text lowerer through a void-sentinel comparison.
- The source-execution probe advances to the next blocker.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done. Probe advanced past `LowerLoopCountParamBox.try_lower_text/1`; the next
blocker is:

```text
reason=generic_type_inventory_full
target_shape_blocker_symbol=BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1
target_shape_blocker_reason=generic_type_inventory_full
```
