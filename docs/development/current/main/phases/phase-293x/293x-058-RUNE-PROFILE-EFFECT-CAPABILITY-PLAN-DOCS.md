---
Status: done
Date: 2026-05-09
Scope: Profile / EffectPlan / CapabilityPlan task-order documentation
---

# 293x-058 Rune Profile Effect Capability Plan Docs

## Decision

`@rune Profile(...)` is reserved as future authoring sugar only.

The truth order is:

```text
source rune metadata / capability calls
-> MIR InlinePlan / EffectPlan / CapabilityPlan / LayoutPlan / AttrPlan
-> verifier acceptance
-> MIR transform or route selection
-> backend emits facts
```

Profile names must not be consumed by `.inc`, ll_emit, or backend-local
planners.

## Task Order

The next implementation order is now fixed as:

1. `M11c-required-verify`
2. `M11d EffectPlan/CapabilityPlan boundary`
3. `M12 mimalloc raw-page proof`
4. `M12b Profile registry docs`
5. `M12c Profile expansion to facts`
6. `M13 allocator fast-path EXE proof`

Deferred feature rows include raw layout source syntax, logical shift and
wrapping arithmetic, TLS/atomic useful operations, native pointer strong attrs,
restricted unsafe blocks, `static_assert`, final/sealed/private dispatch proof,
generic specialization, layout-aware `Option` / `Result`, and `PerfContract`.

## Owned

- new Profile / Effect / Capability Plan SSOT
- mimalloc taskboard ordering update
- InlinePlan relationship to future Profile expansion
- runtime substrate manual update
- design README pointer update

## Not Owned

- parser acceptance of `@rune Profile(...)`
- Profile expansion implementation
- EffectPlan / CapabilityPlan serialization
- required-inline verifier implementation
- backend behavior changes

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Files

```text
docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md
docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
docs/development/current/main/design/inline-plan-ssot.md
docs/development/current/main/design/README.md
docs/reference/runtime/substrate-capabilities.md
```
