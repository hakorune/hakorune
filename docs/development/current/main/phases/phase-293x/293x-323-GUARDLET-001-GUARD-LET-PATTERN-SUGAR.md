# 293x-323 GUARDLET-001 guard-let pattern sugar

Status: Complete
Date: 2026-05-14

## Decision

Add the narrow MVP form:

```hako
guard let Result::Ok(value) = result else {
  return 0
}
```

This is sugar for a known single-payload enum variant check plus a local binding.
It stays in the `guard` family and does not add `try`, `throw`, or `?`.

## Scope

- Parse contextual `let` only after `guard`.
- Accept explicit `Type::Variant(binding)` patterns for known single-payload enum
  variants.
- Rewrite to existing `ScopeBox`, hidden temp `Local`, failure `If`, and binding
  `Local` using `EnumMatchExpr`.
- Keep record, tuple, unit, unqualified, and else-side binding patterns deferred.

## Guard

- `tools/checks/k2_wide_guard_let_pattern_sugar_guard.sh`

## Validation

- `bash tools/checks/k2_wide_guard_let_pattern_sugar_guard.sh`
