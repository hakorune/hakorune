---
Status: done
Date: 2026-05-09
Scope: M11d behavior-preserving cleanup
---

# 293x-061 M11d Rune Plan Refresh SSOT

## Decision

Rune-derived plan refresh has one SSOT entry.

```text
metadata.runes changes
-> refresh_function_rune_plans(function)
   -> EffectPlan refresh
   -> InlinePlan refresh
```

This is a BoxShape cleanup only. It does not add a new accepted source shape,
parser surface, verifier rule, or backend route.

## Owned

- `src/mir/rune_plan_refresh.rs` as the single refresh entry for rune-derived
  MIR plans.
- Builder and JSON v0 bridge callers use `refresh_function_rune_plans`.
- MIR semantic refresh uses the same entry before other metadata refreshes.
- MIR JSON plan metadata emission moves out of `root.rs` into
  `src/runner/mir_json_emit/plan_metadata.rs`.
- The M11d guard checks that builder/bridge callers do not call individual plan
  refresh helpers directly.

## Not Owned

- Profile parser acceptance.
- Capability parser acceptance.
- New verifier facts.
- Backend or `.inc` consumption.
- M12 raw-page allocator proof.

## Acceptance

```bash
bash tools/checks/k2_wide_effect_capability_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
