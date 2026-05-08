# Rune Profile Registry

Status: M12b live-docs registry.

`@rune Profile(...)` is not accepted parser surface yet. This file reserves the
profile names and their future primitive expansion targets so later parser work
cannot invent a second truth source.

Profile expansion order:

```text
Profile(name)
-> primitive rune metadata
-> MIR-owned InlinePlan / EffectPlan / CapabilityPlan / LayoutPlan facts
-> verifier acceptance
-> MIR transform or route selection
-> backend emits already-decided facts
```

Backends, `.inc`, and ll_emit must never branch on profile names.

## Registry

| Profile | Status | Future primitive expansion target | Notes |
| --- | --- | --- | --- |
| `allocator.fast` | reserved | `Hint(hot)`, `Lowering(inline_required)`, `Contract(no_alloc)`, `Contract(no_safepoint)`, future `Contract(no_panic)`, `CapabilityPlan allow=[hako.mem,hako.ptr,hako.tls]` | Strict allocator fast path. No parser acceptance until M12c or later owns expansion and verifier checks. |
| `allocator.slow` | reserved | `Hint(cold)`, `Hint(noinline)`, `CapabilityPlan allow=[hako.mem,hako.osvm,hako.gc]` | Slow path may allocate and may safepoint, so it must not expand to `no_alloc` or `no_safepoint`. |
| `substrate.leaf` | reserved | `Hint(inline)`, `Lowering(inline_required)`, `Contract(no_alloc)`, `Contract(no_safepoint)`, `CapabilityPlan allow=[hako.mem,hako.buf,hako.ptr]` | Small substrate helper leaf. Backend still sees only verified MIR facts. |
| `intrinsic.leaf` | reserved | `Hint(inline)`, `Contract(no_alloc)`, `Contract(no_safepoint)`, future `IntrinsicCandidate(...)` | Intrinsic selection remains registry/verifier owned, not symbol-name inferred. |
| `raw.layout` | reserved | `LayoutPlan repr_c_v0`, future `Contract(no_alloc)`, future `CapabilityPlan allow=[hako.ptr]` | Raw layout truth stays in MIR layout facts. No source `struct` / `repr(C)` is accepted by this registry. |

## Current Live Surface

The only live primitive facts in this registry's target vocabulary are:

- `Hint(inline/noinline/hot/cold)` to `InlinePlan`
- `Lowering(inline_required)` to `InlinePlan request=required`
- `Contract(no_alloc/no_safepoint)` to `EffectPlan`
- empty `CapabilityPlan` metadata emission
- MIR `repr_c_v0` raw-layout vocabulary

The following remain disabled:

- `@rune Profile(...)` parser acceptance
- `@rune Capability(...)` parser acceptance
- profile expansion into primitive rune metadata
- capability verifier acceptance
- backend or `.inc` use of profile names
- `no_panic`, `no_io`, `no_trace`, or other effect requirements
- restricted `unsafe(...)`
- native pointer strong attrs from profile names

## Admission Rule

Before any profile can become parser surface, the implementing card must:

- point to this registry
- expand to primitive facts only
- update MIR metadata tests for the expanded facts
- add verifier acceptance or fail-fast rejection for unsupported facts
- prove `.inc` and ll_emit do not read profile names
- keep profile names out of backend route selection
