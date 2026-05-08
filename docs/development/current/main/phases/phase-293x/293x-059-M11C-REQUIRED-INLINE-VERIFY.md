---
Status: done
Date: 2026-05-09
Scope: verifier-backed required inline acceptance
---

# 293x-059 M11c Required Inline Verify

## Decision

M11c-required-verify is live as a MIR verifier row.

`@rune Lowering(inline_required)` now fails verification unless the function has
both required contracts and the narrow leaf-inline shape:

```hako
@rune Lowering(inline_required)
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
```

Accepted plans are marked `verified=true` in MIR `inline_plans`. Backend use is
still disabled.

## Owned

- shared narrow leaf-inline shape SSOT
- required-inline verifier
- `InlinePlan.verified` refresh for accepted required plans
- `InlinePlanViolation` diagnostics
- K2-wide guard
- manual and SSOT updates

## Not Owned

- backend-required inline lowering
- `.inc` / ll_emit consumption of `inline_plans`
- cross-module inline
- nested verified inline calls
- intrinsic route replacement
- Profile expansion
- EffectPlan / CapabilityPlan boundary

## First Accepted Shape

```text
one block
Return terminator
instruction_count <= 8
Const / UnaryOp / BinOp / Compare / StaticDataLoad / Copy / Select / TypeOp
no nested Call
no dynamic dispatch
no recursive cycle
Contract(no_alloc)
Contract(no_safepoint)
```

## Diagnostics

```text
[inline-plan/missing-contract]
[inline-plan/body-too-large]
[inline-plan/recursive-cycle]
[inline-plan/dynamic-dispatch]
[inline-plan/unsupported-call]
[inline-plan/required-not-verified]
```

## Acceptance

```bash
bash tools/checks/k2_wide_inline_required_verify_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Files

```text
src/mir/inline_leaf.rs
src/mir/inline_plan.rs
src/mir/verification/inline_required.rs
src/mir/verification.rs
src/mir/verification_types.rs
tools/checks/k2_wide_inline_required_verify_guard.sh
tools/checks/dev_gate.sh
```
