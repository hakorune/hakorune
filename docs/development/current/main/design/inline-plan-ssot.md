---
Status: SSOT
Decision: accepted boundary, implementation rows reserved
Date: 2026-05-08
Scope: inline metadata, MIR InlinePlan ownership, verifier gates, and backend responsibility boundaries.
Related:
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
  - docs/reference/mir/hints.md
  - docs/reference/runtime/substrate-capabilities.md
---

# InlinePlan SSOT

## Decision

Inline support is required for allocator-grade fast paths, but it must not be
implemented as an ad-hoc `inline` keyword or as backend-local symbol matching.

The accepted flow is:

```text
source rune metadata
-> MIR-owned InlinePlan / CallsiteInlinePlan
-> verifier acceptance
-> MIR transform or intrinsic route selection
-> backend emits the already-decided MIR/route
```

The backend is a consumer of already-decided shapes. It is not the inline
planner.

## Vocabulary Split

Inline-related annotations have three different meanings:

```text
Hint:
  advisory; may be ignored without changing program meaning

Contract:
  verifier-backed obligation; may be used only after proof

Lowering:
  backend/lane acceptance requirement; failure is a compile-time reject in the
  lanes that opt into it
```

Current public surface:

```hako
@rune Hint(inline)
@rune Hint(noinline)
@rune Hint(hot)
@rune Hint(cold)
```

Reserved substrate-only surface:

```hako
@rune Lowering(inline_required)
```

M11c-required-vocab live surface:

```text
@rune Lowering(inline_required)
-> MIR InlinePlan request=required
-> requires=["no_alloc", "no_safepoint"]
-> verified=false
-> fallback=fail_fast
-> source=rune_lowering
```

This row is vocabulary and preservation only. It does not make required inline
backend-active and does not fail a program for missing verifier proof yet.

`Hint(inline)` is not `inline_required`.

Future `@rune Profile(...)` expansion may produce `Hint(...)`,
`Lowering(inline_required)`, and `Contract(...)` facts, but Profile is not an
InlinePlan truth source. The MIR facts produced by expansion are the only facts
the optimizer and backend may consume.

`Hint(always_inline)` is also not `inline_required`; do not introduce it as a
public guarantee. If an always-inline spelling is ever accepted, it remains an
optimizer hint. Required inline belongs to the `Lowering(...)` family.

## MIR Ownership

Backends must not read source rune strings and decide to inline. Source metadata
is normalized into MIR-owned facts first.

Proposed function-level shape:

```json
{
  "inline_plans": [
    {
      "function": "Main.align_up/2",
      "request": "prefer",
      "hotness": "hot",
      "max_ir": null,
      "requires": [],
      "verified": false,
      "fallback": "keep_call"
    }
  ]
}
```

Proposed callsite-level shape:

```json
{
  "callsite_inline_plans": [
    {
      "callsite": 17,
      "callee": "Main.align_up/2",
      "mode": "prefer",
      "proof": null,
      "fallback": "keep_call"
    }
  ]
}
```

Required inline rows may use:

```json
{
  "function": "MiHeap.alloc_small/1",
  "request": "required",
  "reason": "allocator_fast_path",
  "max_ir": 48,
  "requires": ["no_alloc", "no_safepoint"],
  "verified": true,
  "fallback": "fail_fast"
}
```

The exact serialized schema may be narrowed by implementation cards, but the
truth remains MIR-owned. `.inc`, ll_emit, and C shims do not infer inline policy
from function names.

M11c-preserve live schema:

```json
{
  "inline_plans": [
    {
      "function": "Main.align_up/2",
      "request": "prefer",
      "hotness": null,
      "max_ir": null,
      "requires": [],
      "verified": false,
      "fallback": "keep_call",
      "source": "rune_hint"
    }
  ]
}
```

Current hint mapping:

```text
Hint(inline)   -> request=prefer
Hint(noinline) -> request=avoid
Hint(hot)      -> request=none, hotness=hot
Hint(cold)     -> request=none, hotness=cold
```

`verified=false` and `fallback=keep_call` are part of the preservation
contract. Backends must not treat this row as a mandate.

M11c-soft-leaf live transform:

```text
Hint(inline) / request=prefer
-> same-module Callee::Global(name)
-> callee has one entry block, no PHI/control, no nested Call
-> callee body size <= 8 supported pure instructions
-> MIR optimizer expands the body at the callsite
-> unsupported shapes keep the original call
```

The supported first-row body vocabulary is intentionally narrow:

```text
Const
UnaryOp
BinOp
Compare
StaticDataLoad
Copy
Select
TypeOp
Return
```

`Hint(noinline)` / `request=avoid` wins over any soft inline attempt. This row
does not add required-inline semantics and does not make backends read
`inline_plans`.

M11c-required-vocab live schema:

```json
{
  "inline_plans": [
    {
      "function": "MiHeap.alloc_small/1",
      "request": "required",
      "hotness": null,
      "max_ir": null,
      "requires": ["no_alloc", "no_safepoint"],
      "verified": false,
      "fallback": "fail_fast",
      "source": "rune_lowering"
    }
  ]
}
```

`verified=false` is intentional for this row. `M11c-required-verify` owns the
future transition from preserved vocabulary to accepted required inline.

## Inline Kinds

### MIR Function Inline

Use this for small `.hako` functions whose body can be expanded in MIR:

```text
align_up
size_to_bin
page_free_is_empty
block_next
```

First rows must be same-module and non-recursive. Cross-module, virtual method,
generic specialization, and dynamic dispatch inline are future rows.

### Intrinsic Route Lowering

Some calls should not be expanded as function bodies. They should lower to a
primitive route:

```text
hako.ptr.load_u64
hako.ptr.store_u64
hako.atomic.cas
hako.intrin.ctz_i64
hako.intrin.popcnt_i64
hako.intrin.prefetch
```

This remains separate from `@rune Hint(inline)`. Intrinsic route selection must
flow through a registry/route fact, not through source-name matching in a
backend.

### Native Bitcode / LTO Inline

Native helper body import is reserved. It must not be the first allocator fast
path strategy because it risks moving semantic ownership back into C/native
helpers.

Preferred first strategy:

```text
hako.ptr / hako.atomic / hako.intrin route
-> direct MIR/LLVM primitive
```

Only after the substrate route is truthful should native bitcode/LTO be
considered.

## Required Inline Verifier Conditions

`Lowering(inline_required)` vocabulary is accepted by M11c-required-vocab, but
required inline lowering is accepted only after verifier proof.

Minimum required checks:

- callee exists and resolves to a single same-module body
- body size is within the row budget
- recursive inline cycle is absent
- unsupported dynamic dispatch is absent
- unsupported call is absent, unless it is intrinsic-routed or itself verified
  inline
- `Contract(no_alloc)` is present and verified when required by the row
- `Contract(no_safepoint)` is present and verified when required by the row
- capability access stays within the row's allowed modules

## Backend Boundary

Allowed backend behavior:

- emit already-inlined MIR
- emit already-selected intrinsic/capability routes
- emit fail-fast diagnostics carried by MIR/lowering facts

Forbidden backend behavior:

- searching callee bodies and inlining them
- checking `Mi*`, `Allocator*`, or other app-specific names
- treating `Hint(inline)` as a semantic guarantee
- deriving `no_alloc` / `no_safepoint` from a method name
- adding `.inc` branches such as "if symbol is size_to_bin, inline it"

## Implementation Rows

Use `M11c` for InlinePlan work. `M11b` is already reserved for static const
tables.

Recommended order relative to existing allocator substrate rows:

```text
M11c-docs:
  InlinePlan boundary lock. No behavior change.

M11b-eval:
  const expression / const fn table generation.
  Keeps table generation complete before inline transforms start.

M11c-preserve:
  preserve existing Hint(inline/noinline/hot/cold) into MIR InlinePlan metadata.
  Live-narrow. No backend use.

M11c-soft-leaf:
  best-effort same-module leaf MIR inline.
  Live-narrow. Failed inline keeps the call.

M10c-pre:
  pointer/handle return proof vocabulary.

M10c:
  strong LLVM attrs widening after pointer proof.

M11c-required-vocab:
  substrate-only Lowering(inline_required) vocabulary.
  Parser parity is required because this adds rune vocabulary.
  Live-narrow. Preserves request=required metadata only; no verifier/backend use.

M11c-contract-repeat:
  allow distinct Contract(...) runes on the same declaration.
  Live-narrow. This is parser metadata shape only and exists so
  Contract(no_alloc) + Contract(no_safepoint) can both be present before
  required-inline verification.

M11c-required-verify:
  required inline verifier connection to no_alloc/no_safepoint and call graph
  checks.

M11d:
  EffectPlan / CapabilityPlan boundary.
  Defines the MIR-owned effect and capability facts required before Profile can
  safely expand allocator.fast or substrate.leaf into strict lowering facts.

M12:
  mimalloc raw-page proof.

M12b:
  Profile registry docs.
  Reserves profile names and expansion targets only.

M12c:
  Profile expansion to primitive rune/Plan facts.
  Backend still reads InlinePlan / EffectPlan / CapabilityPlan, not Profile
  strings.

M13:
  allocator fast-path EXE proof.
```

This order keeps static table data, inline planning, pointer proof, and
allocator proof separated.

## Diagnostics

Stable diagnostics for future rows:

```text
[inline-plan/required-not-verified]
[inline-plan/body-too-large]
[inline-plan/recursive-cycle]
[inline-plan/dynamic-dispatch]
[inline-plan/unsupported-call]
[inline-plan/missing-contract]
[inline-plan/backend-boundary]
```

## Non-Goals

- no `inline` keyword
- no public `always_inline` guarantee
- no `.inc` / ll_emit inliner
- no app-specific inline switch
- no backend-active use of `Hint(inline)` before MIR InlinePlan exists
- no required inline before verifier proof exists
