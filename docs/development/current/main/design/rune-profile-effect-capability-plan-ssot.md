---
Status: SSOT
Decision: accepted design order, implementation rows reserved
Date: 2026-05-09
Scope: @rune Profile, EffectPlan, CapabilityPlan, and mimalloc-grade feature ordering.
Related:
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/inline-plan-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/return-proof-vocabulary-ssot.md
  - docs/reference/runtime/substrate-capabilities.md
---

# Rune Profile / Effect / Capability Plan SSOT

## Decision

`@rune Profile(...)` is useful, but it must not become a new semantic truth
source.

The accepted order is:

```text
source rune metadata / capability calls
-> MIR-owned InlinePlan / EffectPlan / CapabilityPlan / LayoutPlan / AttrPlan
-> verifier acceptance
-> MIR transform or route selection
-> backend emits the already-decided facts
```

Profile is sugar over lower-level facts. Backends must not read a profile name,
infer allocator semantics from it, or branch on it in `.inc` / ll_emit.

## Layering

The language remains split into three surfaces:

```text
ordinary .hako:
  box/method/String/Array/Map and safe app code

hako_kernel / hako_substrate:
  collection/runtime/allocator policy and state machines

capability modules:
  hako.mem / hako.buf / hako.ptr / hako.atomic / hako.tls /
  hako.osvm / hako.intrin
```

Low-level power is admitted through explicit capability modules, runes, MIR
facts, and verifiers. Do not add a broad C-style unsafe language shelf.

## Rune Profile Rule

Future source shape:

```hako
@rune Profile(allocator.fast)
method alloc_small(size: usize) -> Ptr<u8> {
  ...
}
```

Profile expansion is allowed only after the target primitive facts exist.

Example future expansion target:

```text
Profile(allocator.fast)
-> Hint(hot)
-> Lowering(inline_required)
-> Contract(no_alloc)
-> Contract(no_safepoint)
-> Contract(no_panic)
-> CapabilityPlan allow=[hako.ptr, hako.mem, hako.tls]
-> EffectPlan no_alloc/no_safepoint/no_panic verified before strict use
```

The profile string is not a backend contract. The expanded and verified MIR
facts are the contract.

Initial profile names are reserved only:

```text
allocator.fast
allocator.slow
substrate.leaf
intrinsic.leaf
raw.layout
```

No parser acceptance, expansion, verifier behavior, or backend behavior is
implied by this reservation.

## Plan Ownership

### InlinePlan

Owner: MIR metadata plus MIR optimizer/verifier.

Purpose: represent advisory and required inline decisions. `Hint(inline)` is
advisory. `Lowering(inline_required)` is strict only after verifier acceptance.
Backends emit already-inlined MIR or already-selected routes.

SSOT: `docs/development/current/main/design/inline-plan-ssot.md`

### EffectPlan

Owner: MIR metadata plus verifier.

Purpose: hold effect obligations and proofs such as:

```text
no_alloc
no_safepoint
no_panic
no_io
no_trace
no_registry_write
no_refcount_mutation
no_cache_write
```

`EffectPlan` is not a second effect dialect. It is the MIR-owned verifier-facing
summary used to decide whether rune contracts and profile expansions are
eligible for strict lowering.

M11d live surface:

```text
Contract(no_alloc)      -> EffectRequirement::NoAlloc
Contract(no_safepoint) -> EffectRequirement::NoSafepoint
-> MIR metadata.effect_plans
-> rune contract verifier consumes EffectPlan
```

`Contract(pure)` and `Contract(readonly)` are not live `EffectPlan`
requirements yet.

### CapabilityPlan

Owner: MIR metadata plus verifier.

Purpose: record which capability modules a declaration or strict lane is
allowed to use:

```text
hako.mem
hako.buf
hako.ptr
hako.atomic
hako.tls
hako.osvm
hako.intrin
```

Capability use must be checked structurally. Backends must not infer capability
rights from method names, file names, or profile names.

M11d live surface:

```text
metadata.capability_plans = []
```

There is no `@rune Capability(...)` syntax and no `@rune Profile(...)`
expansion yet.

### LayoutPlan

Owner: MIR layout facts.

Purpose: make raw layout truth explicit for future `repr(C)` / struct rows.
Existing `repr_c_v0` vocabulary covers fixed-width numeric fields only. Source
syntax, pointer fields, `sizeof`, `offsetof`, and backend-active native layout
remain future rows.

### AttrPlan

Owner: runtime-decl manifest / proof verifier / export service.

Purpose: export LLVM attributes only from proven ABI facts.

`handle_*` return classes are not native pointer attr targets. Only
`native_ptr_*` classes may later feed `nonnull`, `dereferenceable`, alignment,
or `noalias`, and only after proof/export gates accept them.

## Syntax Admission Rule

Before adding syntax, check whether the row can be expressed by an existing
lower-level mechanism:

1. capability function
2. rune / profile expansion
3. manifest row
4. MIR-owned Plan fact
5. syntax

Syntax candidates that may justify language surface:

```text
struct
static_assert
Ptr<T> / Handle<T> type spelling
usize/u64/u32 exact runtime semantics
```

Rune candidates:

```text
Profile(...)
Hint(...)
Contract(...)
Lowering(...)
IntrinsicCandidate(...)
PerfContract(...)
```

Capability-module candidates:

```text
mem.alloc / mem.free / mem.realloc
ptr.load / ptr.store / ptr.add
atomic.load / atomic.store / atomic.cas
tls.get / tls.set / tls.get_or_init
osvm.reserve / osvm.commit / osvm.decommit
intrin.ctz / intrin.popcnt / intrin.prefetch / intrin.assume
```

## Task Order

This SSOT refines the mimalloc taskboard order without making any new behavior
live.

Immediate order after M11d:

```text
1. M12 mimalloc raw-page proof [live-narrow]
   Prove a raw page/free-list fixture using explicit capability calls and
   existing contracts.

2. M12b Profile registry docs
   Reserve profile names and expansion targets in one registry. No parser
   acceptance yet unless the row explicitly includes parser parity.

3. M12c Profile expansion to facts
   Expand Profile(...) to primitive rune/Plan facts. Backend still reads only
   facts, not profile names.

4. M13 allocator fast-path EXE proof
   Use verified inline/effect/capability facts and route facts to prove the
   fast path in EXE.
```

Deferred rows:

```text
M14 raw layout source syntax / repr(C) struct
M15 logical shift and wrapping/checked arithmetic
M16 TLS slot + atomic load/store/CAS/fetch_add
M17 native pointer strong attr export after eligible proof rows
M18 restricted unsafe capability block
M19 static_assert / sizeof / offsetof
M20 final / sealed / private dispatch proof
M21 generic specialization for substrate types
M22 layout-aware Option/Result
M23 PerfContract / asm gate
```

The deferred order may be split further by later cards. It must not be used to
skip the immediate verifier and plan-boundary rows.

## Do Not Implement First

Do not implement these before their owners exist:

- `@rune Profile(...)` parser acceptance before EffectPlan/CapabilityPlan
  boundaries exist.
- `unsafe(...)` blocks before capability verification is defined.
- `struct` / `repr(C)` source syntax before LayoutPlan source acceptance is
  scoped.
- `noalias` / `nonnull` / `dereferenceable` export before native pointer proof
  rows are eligible.
- `PerfContract(...)` before a MIR/asm evidence gate exists.
- final/sealed/private dispatch shortcuts before direct-call proof is modeled.
- generic specialization before monomorphization ownership is documented.
- boxed or unboxed `Option` / `Result` layout before value-layout ownership is
  documented.

## Backend Boundary

Allowed backend behavior:

- emit already-verified facts
- emit already-selected capability/intrinsic routes
- reject with a diagnostic carried by MIR/lowering facts

Forbidden backend behavior:

- branch on `Profile(allocator.fast)`
- branch on `Mi*`, `Allocator*`, or app-specific names
- infer `no_alloc`, `no_safepoint`, `fresh`, `nonnull`, or capability rights
  from symbols
- implement inline planning in `.inc` / ll_emit
- reinterpret handle values as native pointers for LLVM attrs

## Current Reading

`M12 mimalloc raw-page proof` is live-narrow. `M12b Profile registry docs` is
live-docs. The registry SSOT is:

```text
docs/reference/mir/rune-profile-registry.md
```

The next implementation step is `M12c Profile expansion to facts`. Profile
exists only as a reserved design target until that row explicitly owns parser
parity and expansion. It must expand over existing facts rather than becoming a
backend-readable semantic string.
