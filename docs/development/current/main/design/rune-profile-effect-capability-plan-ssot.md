---
Status: SSOT
Decision: source surface minimized; Profile parked/reserved
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

`@rune Profile(...)` is reserved, not part of the active v0 source surface.
The active source surface should stay small:

```text
@rune Inline(prefer)
@rune Inline(avoid)
@rune Inline(required)
```

Profile may become useful later as a named bundle over lower-level facts, but
it must not become a new semantic truth source and must not be required for
simple inlining.

The accepted order is:

```text
source rune metadata / capability calls
-> MIR-owned InlinePlan / EffectPlan / CapabilityPlan / LayoutPlan / AttrPlan
-> verifier acceptance
-> MIR transform or route selection
-> backend emits the already-decided facts
```

If Profile is ever enabled, it is sugar over lower-level facts. Backends must
not read a profile name, infer allocator semantics from it, or branch on it in
`.inc` / ll_emit.

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

Deferred source shape:

```hako
@rune Profile(allocator.fast)
method alloc_small(size: usize) -> Ptr<u8> {
  ...
}
```

Profile expansion is allowed only after the target primitive facts already
exist as MIR-owned plan rows and repeated source boilerplate proves a real
need.

M12c expansion target:

```text
Profile(allocator.fast)
-> Hint(hot)
-> Inline(required)
-> Contract(no_alloc)
-> Contract(no_safepoint)
-> CapabilityPlan allow=[hako.ptr, hako.mem, hako.tls]
-> EffectPlan no_alloc/no_safepoint verified before strict use
```

The profile string is not a backend contract. The expanded and verified MIR
facts would be the contract.

Deferred Profile use:

```hako
@rune Profile(allocator.fast)
sizeToClass(size) {
  ...
}
```

This is not the active v0 path. It is a possible future surface for allocator
hot policy helpers only if explicit lower-level annotations become too noisy.
It is not required for small leaf helpers whose shape can be verified directly.

Small receiver-local reset helpers should use the lighter spelling:

```hako
@rune Inline(required)
beginSelection() {
  me.last_selected_index = -1
  me.last_selected_page_id = -1
  me.last_selected_kind = 0
  me.last_selected_page = null
}
```

The required-inline verifier may infer `no_alloc` and `no_safepoint` from a
narrow receiver-fieldset leaf shape. `Inline(required)` is still only a proof
request: if the verifier cannot prove the shape, it must fail fast.

If `allocator.fast` is ever enabled, it must be read as a registry expansion
for larger bundles, not as a backend-visible magic name:

```text
Profile(allocator.fast)
-> InlinePlan request=required
-> EffectPlan no_alloc
-> EffectPlan no_safepoint
-> verifier shape=allocator_fast_leaf
-> fallback=fail_fast
```

`allocator.fast` does not mean `pure` or `readonly`. Receiver-local field
mutation is allowed only when the verifier accepts a narrow allocator fast leaf
shape. The first intended mutation shape is receiver-local scalar/null
`FieldSet`, with no nested call, no allocation, no safepoint, no branch/loop,
and no dynamic dispatch.

`@rune Inline(required)` by itself does not grant `no_alloc` or
`no_safepoint`. It asks the verifier to prove a supported leaf shape.
`Profile` expansion may provide explicit contracts before verifier acceptance
when a named bundle is useful.

Reserved profile names are:

```text
allocator.fast
allocator.slow
substrate.leaf
intrinsic.leaf
raw.layout
```

Parser acceptance and expansion are not part of the active v0 surface. These
names are reserved only to avoid accidental semantic drift. Backend behavior is
still not implied by the profile string.

## Plan Ownership

### InlinePlan

Owner: MIR metadata plus MIR optimizer/verifier.

Purpose: represent advisory and required inline decisions. `Inline(prefer)` is
advisory. `Inline(required)` is strict only after verifier acceptance. Compat
`Hint(inline/noinline)` and `Lowering(inline_required)` remain accepted during
the migration window. Backends emit already-inlined MIR or already-selected
routes.

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

Active v0 surface:

```text
Contract(no_alloc)       -> EffectRequirement::NoAlloc
Contract(no_safepoint)   -> EffectRequirement::NoSafepoint
-> MIR metadata.effect_plans
-> rune contract verifier consumes EffectPlan
```

Profile-to-EffectPlan expansion is deferred. Small required-inline leaf helpers
should rely on verifier shape inference instead of source-level effect
boilerplate when the body shape is narrow enough.

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

Deferred Profile-to-CapabilityPlan surface:

```text
Profile(allocator.fast) -> CapabilityPlan allow=[hako.mem,hako.ptr,hako.tls]
Profile(allocator.slow) -> CapabilityPlan allow=[hako.mem,hako.osvm,hako.gc]
Profile(substrate.leaf) -> CapabilityPlan allow=[hako.mem,hako.buf,hako.ptr]
Profile(raw.layout)     -> CapabilityPlan allow=[hako.ptr]
```

There is no `@rune Capability(...)` syntax. CapabilityPlan rows produced from
profiles are metadata only; profile expansion and capability verifier/backend
use remain future.

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

2. M13 allocator fast-path EXE proof [live-narrow]
   Use verified inline/effect/capability facts to prove a scalar fast path in
   pure-first EXE without backend profile-name handling.
```

Deferred rows:

```text
M12b Profile registry docs, if repeated boilerplate proves demand
M12c Profile expansion to facts, after registry docs and primitive facts
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
skip the immediate verifier and plan-boundary rows. Profile rows are especially
deferred: do not introduce them until explicit `Inline` / `Contract` source
spelling has become real, repeated user-facing noise.

## Do Not Implement First

Do not implement these before their owners exist:

- new or widened `@rune Profile(...)` names before their primitive
  EffectPlan/CapabilityPlan/InlinePlan targets exist.
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

`M12 mimalloc raw-page proof` remains the narrow proof row. Profile registry
docs and Profile expansion are parked. The reserved registry note is:

```text
docs/reference/mir/rune-profile-registry.md
```

`M13 allocator fast-path EXE proof` is live-narrow. It should consume explicit
InlinePlan / EffectPlan / verifier facts, not `Profile(...)`. The next widening
must be a separate raw-substrate EXE row rather than a broad allocator special
case.
