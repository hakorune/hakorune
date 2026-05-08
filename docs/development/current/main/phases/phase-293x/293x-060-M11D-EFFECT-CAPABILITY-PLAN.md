---
Status: done
Date: 2026-05-09
Scope: M11d EffectPlan / CapabilityPlan metadata boundary
---

# 293x-060 M11d Effect Capability Plan

## Decision

M11d is live as a MIR-owned metadata boundary.

Existing verifier-backed contracts now flow through `EffectPlan`:

```text
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
-> MIR metadata.effect_plans
-> rune contract verifier consumes EffectPlan
```

`CapabilityPlan` is present as an empty metadata vocabulary. There is no
`Profile(...)` syntax, no `Capability(...)` syntax, and no backend use.

## Owned

- `EffectPlan` / `CapabilityPlan` MIR data types
- effect-plan refresh from declaration-local `Contract(...)` runes
- empty capability-plan boundary
- MIR JSON `metadata.effect_plans` / `metadata.capability_plans`
- rune contract verifier consumption of `EffectPlan`
- guard that keeps Profile/Capability parser surface disabled
- guard that keeps `.inc` / backend readers out of these metadata rows

## Not Owned

- `@rune Profile(...)` parser acceptance
- `@rune Capability(...)` parser acceptance
- Profile expansion
- capability-use verification
- backend or `.inc` consumption
- allocator raw-page proof
- pointer attrs / LLVM export widening

## First Live Shape

```text
Contract(no_alloc)      -> EffectRequirement::NoAlloc
Contract(no_safepoint) -> EffectRequirement::NoSafepoint

EffectPlan:
  function
  requires
  verified=false
  source=rune_contract

CapabilityPlan:
  []
```

`Contract(pure)` and `Contract(readonly)` remain metadata-only for this row.

## Acceptance

```bash
bash tools/checks/k2_wide_effect_capability_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Files

```text
src/mir/effect_capability_plan.rs
src/mir/function/types.rs
src/mir/verification/rune_contracts.rs
src/runner/mir_json_emit/root.rs
tools/checks/k2_wide_effect_capability_plan_guard.sh
```
