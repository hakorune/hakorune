# allocator-fast-path-exe-proof

Purpose: M13 scalar EXE proof that `@rune Profile(allocator.fast)` reaches
direct EXE through MIR-owned facts, not backend-local profile handling.

## Accepted Shape

- `Profile(allocator.fast)` expands to `InlinePlan`, `EffectPlan`, and
  `CapabilityPlan` metadata.
- The verified `InlinePlan request=required` is consumed by the MIR optimizer
  for a narrow same-module scalar leaf helper.
- The pure-first EXE backend receives already-expanded scalar MIR and does not
  branch on `Profile(allocator.fast)`.

## Non-Goals

- No RawBuf/RawArray EXE lowering.
- No native pointer, TLS, atomic, or `hako.mem` lowering.
- No `.inc` or backend-specific profile-name matcher.
- No allocator runtime ownership change.
