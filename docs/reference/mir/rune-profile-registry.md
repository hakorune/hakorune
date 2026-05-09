# Rune Profile Registry

Status: M13 live-narrow allocator fast-path EXE proof.

`@rune Profile(...)` is accepted for the reserved profile names in this file.
The profile name is authoring sugar only: parsers validate it here, MIR plan
builders expand it into primitive facts, and backends never consume profile
names.

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
| `allocator.fast` | live-narrow | `Hint(hot)`, `Lowering(inline_required)`, `Contract(no_alloc)`, `Contract(no_safepoint)`, future `Contract(no_panic)`, `CapabilityPlan allow=[hako.mem,hako.ptr,hako.tls]` | Strict allocator fast path. Verified required inline is consumed by the MIR optimizer for narrow scalar same-module leaf helpers; capability allowance remains metadata only. |
| `allocator.slow` | live-narrow | `Hint(cold)`, `Hint(noinline)`, `CapabilityPlan allow=[hako.mem,hako.osvm,hako.gc]` | Slow path may allocate and may safepoint, so it does not expand to `no_alloc` or `no_safepoint`. |
| `substrate.leaf` | live-narrow | `Hint(inline)`, `Lowering(inline_required)`, `Contract(no_alloc)`, `Contract(no_safepoint)`, `CapabilityPlan allow=[hako.mem,hako.buf,hako.ptr]` | Small substrate helper leaf. Backend still sees only verified MIR facts. |
| `intrinsic.leaf` | live-narrow | `Hint(inline)`, `Contract(no_alloc)`, `Contract(no_safepoint)`, future `IntrinsicCandidate(...)` | Intrinsic selection remains registry/verifier owned, not symbol-name inferred. |
| `raw.layout` | live-narrow | future `LayoutPlan repr_c_v0`, future `Contract(no_alloc)`, `CapabilityPlan allow=[hako.ptr]` | Raw layout truth stays in MIR layout facts. No source `struct` / `repr(C)` is accepted by this registry. |

## Current Live Surface

The live primitive facts in this registry's target vocabulary are:

- `Hint(inline/noinline/hot/cold)` to `InlinePlan`
- `Lowering(inline_required)` to `InlinePlan request=required`
- `Contract(no_alloc/no_safepoint)` to `EffectPlan`
- Profile-derived `CapabilityPlan allow=[...]` metadata emission
- MIR `repr_c_v0` raw-layout vocabulary

The following remain disabled:

- `@rune Capability(...)` parser acceptance
- capability verifier acceptance
- backend or `.inc` use of profile names
- `no_panic`, `no_io`, `no_trace`, or other effect requirements
- restricted `unsafe(...)`
- native pointer strong attrs from profile names

## Admission Rule

Any later widening of a profile must:

- point to this registry
- expand to primitive facts only
- update MIR metadata tests for the expanded facts
- add verifier acceptance or fail-fast rejection for unsupported facts
- prove `.inc` and ll_emit do not read profile names
- keep profile names out of backend route selection
