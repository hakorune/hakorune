# Low-Level Capability Surface

Status: Current reference with staged implementation rows.

This document is the language-facing entry for low-level `.hako` code used by
allocator, collection, and runtime internals.

The current mimalloc port is the main proving workload for this surface. Its
goal is to make allocator algorithms expressible in `.hako` / `hako_alloc`,
not to replace the Hakorune process allocator.

Detailed runtime capability rows are owned by
`docs/reference/runtime/substrate-capabilities.md`.

## Core Rule

Hakorune does not expose a broad C-style `unsafe` block for allocator work.

Low-level code must use explicit capability modules, MIR-owned metadata facts,
and verifier-backed contracts:

```text
source .hako code
-> capability module call or @rune metadata
-> MIR-owned route / plan / effect facts
-> verifier acceptance or fail-fast rejection
-> backend emits already-decided facts
```

Backends, `.inc` files, and Stage0 matchers must not infer allocator behavior
from box names, method names, provider names, or profile names.

## Future Feature Map

The durable owner split for future low-level language features is tracked outside
this reference page to avoid duplicating semantics.

| SSOT | Role |
| --- | --- |
| `docs/development/current/main/design/language-minimal-surface-ssot.md` | Canonical minimal keyword/surface rule. |
| `docs/development/current/main/design/delegation-no-inheritance-ssot.md` | Canonical behavior-reuse rule: no inheritance, explicit field delegation only. |
| `docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md` | Canonical rule for what Stage0 may carry as syntax / metadata and what Stage1 must own as meaning. |
| `docs/development/current/main/design/language-feature-implementation-order-ssot.md` | Canonical Wave A/B/C task order and full feature inventory. |
| `docs/development/current/main/design/type-system-policy-ssot.md` | Type meaning policy. `MirType` is not language semantics. |
| `docs/development/current/main/design/record-and-packed-array-lowering-ssot.md` | Record and PackedArray lowering owner. |
| `docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md` | Rune effect/capability metadata lane. |

Stage0 may only parse, transport metadata, or perform trivial desugar for these
features. Stage1 owns semantic checks, verifier facts, CorePlan decisions, and
unsupported-backend fail-fast behavior.

The practical Stage0 / Stage1 usable surface matrix is intentionally owned by:

```text
docs/reference/language/stage-profiles.md
```

Keep this page focused on low-level capability vocabulary. Do not duplicate the
full language profile table here; it drifts quickly as Stage1 rows land.

## Current Language Surface

### Direct Memory Vocabulary

Current direction is view-based direct memory, not C-style pointer exposure.

Accepted source-visible split for v0:

```text
Array:
  public/general array semantics

DirectArrayI64:
  exact i64 direct-storage family member for verified internal hot paths
```

`DirectArray` is a family name in docs, not a standalone untyped v0 source
type. `DirectArrayI64` is not a subtype of `Array`, and implicit conversion in
either direction is not accepted. Materialization or copy must be explicit when
needed.

Implementation status: the compiler recognizes `DirectArrayI64` constructor
and `get` / `set` receiver shapes as DirectArray access-plan candidates, and
the LLVM exact front can lower the explicit constructor to the direct-i64 birth
symbol. A production source migration, such as changing allocator PageModel
fields from `ArrayBox` to `DirectArrayI64`, still requires a separate
initialized-length fixture and exact-front measurement. Do not treat the
metadata route shape as permission to silently replace existing `ArrayBox`
fields.

The following are not accepted source features:

```text
RawPtr<T>
NativePtr dereference
NativePtr indexing
NativePtr pointer arithmetic
& / * / -> pointer operators
```

`NativePtr` remains opaque. Future native memory access must create a bounded
view first.

### Box, Record, And Mutable State

`record` is the canonical source spelling for identity-free aggregate values.
It is appropriate for metadata rows, snapshots, report payloads, and local
scalarizable value bundles.

`record` is not the v0 spelling for an allocator owner object. In particular,
it must not be used to silently replace a `box` that owns lifecycle, method
dispatch, DirectArray storage, observer/debug boundaries, or public identity.

Use this split for low-level allocator code:

```text
box:
  identity / lifecycle / methods / storage ownership

record:
  identity-free aggregate value, snapshot, metadata row, or local state bundle

DirectArrayI64:
  owned variable-length exact-i64 table
```

The desired C-struct-like shape for mutable internal state is therefore not
"turn the owner box into a record". The narrow future direction is
`RecordStateResidencePlanV0`: a box-private record residence plan.

```hako
record PageState {
    used: i64
    free_top: i64
    local_free_top: i64
    peak_used: i64
}

box HakoAllocPageModel {
    state: PageState
    free: DirectArrayI64
    local_free: DirectArrayI64
}
```

In that future shape, the `box` still owns identity and storage, while the
compiler may lower `me.state.free_top` through a direct state/record residence
plan when verifier facts prove the access. Until that row exists, record-local
scalarization remains compiler-local and must not imply runtime record objects,
backend record lowering, automatic record-to-box conversion, or ordinary-box
auto-recordification.

`RecordStateResidencePlanV0` is not a new source surface. It may only accept
concrete box-private record fields with primitive scalar subfields. The v0
operation set is subfield load/store such as `me.state.free_top`; whole-record
read, whole-record assignment, record return ABI, helper argument ABI, public
materialization, handle fields, nested records, and record methods remain
rejected until separately accepted.

Internal representation work should use these layers:

```text
MemoryRegion:
  owned or borrowed storage region

MemoryView:
  typed way to see that region

MemoryAccessPlan:
  selected load/store route for one access site

Proof:
  bounds / alignment / alias / lifetime / stability / initialization facts
```

For current DirectArray lowering, the stable proof vocabulary is:

```text
RangeIndexFact:
  index interval, such as i in 0..capacity

DirectArrayExtentFact:
  receiver extent covers that interval

RegionStabilityFact:
  receiver storage is stable across the planned access region
```

Terminology:

```text
direct:
  route contract; relevant memory-like access must have a FastPathPlan when
  required

unsafe memory:
  region / provenance / caller-assumption contract for external/native memory

unchecked:
  bounds-check policy result, usually derived from proof
```

Do not merge these meanings. `direct` is not unsafe, `unsafe memory` does not
guarantee a fast route, and `unchecked` must come from proof.

### Direct FastPath Diagnostics

Decision: v0 does not add `direct {}` source syntax.

The next direct-memory contract is diagnostic-first:

```text
RequiredFastPathRegion:
  function or future source region where relevant access must have FastPathPlan

FastPathObligation:
  one relevant access site inside that region

FastPathPlan:
  DirectState / DirectArray / Span / future Bytes/LayoutSpan route plan
```

In v0, `RequiredFastPathRegion` is created by diagnostics, keeper
expectations, or CI gates. A future `direct { ... }` block may become a thin
source spelling for the same region contract, but it must not introduce a
separate verifier.

Relevant access kinds:

```text
direct_state_field load/store
direct_array_i64 load/store
span_i64 load
span_mut_i64 load/store
future bytes load/store
future layout field load/store
```

Non-relevant operations include local scalar assignment, integer arithmetic,
compare, branch, loop, and return.

Allowed:

```text
route=direct_array_i64_store, bounds_policy=checked
route=direct_array_i64_store, bounds_policy=proved_unchecked
verified Inline(required) call, if the active region policy allows it
```

Rejected when required:

```text
generic helper route
boxed fallback
dynamic dispatch route
runtime reflective access
unknown storage route
FastPathPlan present but backend/lowering used fallback
```

`direct` is therefore not a promise of branchless code. It only forbids slow or
ambiguous routes for relevant access sites.

Planned order:

1. keep current `Array` / `DirectArrayI64` source shape;
2. strengthen `DirectArrayAccessPlanV0` with `element_type` and `proof_ids`;
3. normalize `RangeIndexFact`, `DirectArrayExtentFact`, and
   `RegionStabilityFact`;
4. add `SpanI64` / `SpanMutI64` as no-escape views over DirectArray storage;
5. add `RequiredFastPathRegion` / `FastPathObligation` diagnostics;
6. add `direct {}` later only as syntax sugar over the diagnostic contract;
7. add `unsafe memory` / `Bytes` later, with `NativePtr` still opaque;
8. add `LayoutSpan` and bulk memory pattern recognition after Span/Bytes.

Span v0 is defined by
`docs/development/current/main/design/span-no-escape-ssot.md`: `SpanI64` and
`SpanMutI64` are no-escape views over `DirectArrayI64`, not pointer syntax and
not unsafe memory.

### Numeric Type Names

These integer type names are accepted as annotation text and classified by MIR
metadata:

```text
i8 i16 i32 i64 isize
u8 u16 u32 u64 usize
```

Current semantics remain narrow:

- runtime numeric values still execute on the dynamic `Integer(i64)` lane;
- typed-object and layout planning may use the names as storage hints;
- exact unsigned, overflow, wrapping, and pointer-sized arithmetic semantics
  are not implied by the names yet.

### Static Const Tables

The current accepted table shape is:

```hako
static const SIZE_CLASS: u16[] = [
  8, 16, 32, 64,
]
```

Current rules:

- the only accepted element type is `u16`;
- initializer elements may use narrow side-effect-free integer expressions;
- `NAME[index]` reads lower to MIR `StaticDataLoad`;
- reads return current-lane `Integer(i64)` values;
- negative or out-of-range reads fail fast.

Runtime `ArrayBox` / `MapBox` construction is not the implementation strategy
for static tables.

### Rune Metadata

Canonical declaration metadata uses `@rune`.

Current accepted allocator-relevant rows include:

```hako
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
@rune Inline(prefer)
@rune Inline(avoid)
@rune Inline(required)
@rune Hint(hot)
@rune Hint(cold)
```

Current rules:

- `Contract(no_alloc)` and `Contract(no_safepoint)` are checked by the MIR
  verifier;
- canonical `Inline(prefer|avoid|required)` rows produce MIR `InlinePlan`
  facts;
- compat `Hint(inline/noinline)` and `Lowering(inline_required)` remain
  accepted during the migration window and map to the equivalent `Inline(...)`
  request;
- `Hint(hot|cold)` remains advisory tuning metadata;
- verified required inline may be consumed by the MIR optimizer for narrow
  same-module leaf bodies;
- `Profile(...)` names are reserved in the MIR profile registry, but new source
  should prefer primitive runes and explicit contracts;
- backend route selection must not read profile names.

`@rune Capability(...)` is not accepted parser surface yet.

### Capability Modules

The current low-level vocabulary is split by capability family:

| Family | Current role |
| --- | --- |
| `hako.mem` | allocation/reallocation/free facade rows below `RawBuf` |
| `hako.buf` | buffer length/capacity/reserve/grow shape below `RawArray` |
| `hako.ptr` | pointer/span and direct slot/native-pointer route vocabulary |
| `hako.atomic` | fixed-slot i64 atomics plus direct native-pointer store/load/CAS route facts |
| `hako.tls` | diagnostic TLS rows plus narrow allocator cache-slot get/set |
| `hako.worker` | single-worker current-id substrate row for allocator-internal policy proof |
| `hako.gc` | first write-barrier facade |
| `hako.osvm` | page-size and reserve/commit/decommit rows |
| `hako.intrin` | current-lane non-negative i64 bit-count rows |

These modules are low-level vocabulary, not allocator policy owners.

### Raw Substrate And `hako_alloc`

Current layering:

```text
hako.mem / hako.buf / hako.ptr / hako.atomic / hako.tls / hako.worker / hako.osvm
  -> RawBuf / RawArray substrate helpers
  -> hako_alloc policy/state/facade
  -> mimalloc-style allocator algorithms in .hako
```

`RawBuf` is a byte-buffer allocation facade, not an allocator state machine.
`RawArray` is an explicit slot substrate, not a semantic collection owner.
`hako_alloc` owns allocator policy/state/facade names for current mimalloc
work.

## Current Mimalloc Reading

Continue:

- mimalloc `.hako` algorithm slices;
- internal read-only `hako_alloc` inventory surfaces for options/defaults and owner-token facts;
- `hako_alloc` policy/state/facade ownership;
- `RawBuf` / `RawArray` proof-backed substrate use;
- size-class static tables;
- page/free-list policy;
- TLS cache-slot and atomic remote-free proofs;
- OSVM page-source proofs;
- EXE / pure-first proof apps.

M214/M215 note: options/defaults inventory and thread heap owner-token inventory are internal read-only `hako_alloc` surfaces. They do not add user syntax, environment variables, mutable runtime options, allocation policy changes, provider activation, hooks, process allocator replacement, scheduling, atomics, or reclaim execution.

Stop by default:

- allocator-provider M104+;
- activation;
- host allocator replacement;
- process allocator replacement.

Keep only as guardrails:

- no global allocator;
- no provider environment toggle;
- no `.inc` provider or hook matcher;
- no activation hook.

## Reserved Surface

The following are not language features today:

- unrestricted `unsafe(...)` blocks;
- source-level `repr(C)` / `sizeof` / `alignof`;
- `MaybeInit` as a live language/runtime surface;
- exact-width numeric runtime semantics beyond the current i64 lane;
- generic pointer arithmetic;
- generic TLS cells;
- generic atomic operations with user-selected memory-order arguments beyond
  the rows documented in the runtime substrate reference;
- backend-readable profile names;
- implicit allocator-provider discovery;
- host/process allocator replacement.

## Reference Map

- Types and static tables: `docs/reference/language/types.md`
- Grammar: `docs/reference/language/EBNF.md`
- Stage0 / Stage1 usable surface profiles:
  `docs/reference/language/stage-profiles.md`
- Rune metadata and MIR hints: `docs/reference/mir/hints.md`
- Rune profile registry: `docs/reference/mir/rune-profile-registry.md`
- MIR metadata facts: `docs/reference/mir/metadata-facts-ssot.md`
- ABI boundary: `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- Runtime substrate capability rows:
  `docs/reference/runtime/substrate-capabilities.md`
- Current mimalloc purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
