# Low-Level Capability Surface

Status: provisional reference

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

## Current Language Surface

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
@rune Hint(inline)
@rune Hint(noinline)
@rune Hint(hot)
@rune Hint(cold)
@rune Lowering(inline_required)
@rune Profile(allocator.fast)
@rune Profile(allocator.slow)
@rune Profile(substrate.leaf)
@rune Profile(intrinsic.leaf)
@rune Profile(raw.layout)
```

Current rules:

- `Contract(no_alloc)` and `Contract(no_safepoint)` are checked by the MIR
  verifier;
- `Hint(...)` and `Lowering(inline_required)` produce MIR `InlinePlan` facts;
- verified required inline may be consumed by the MIR optimizer for narrow
  same-module leaf bodies;
- `Profile(...)` is authoring sugar only and expands to primitive MIR plan
  facts;
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
| `hako.gc` | first write-barrier facade |
| `hako.osvm` | page-size and reserve/commit/decommit rows |
| `hako.intrin` | current-lane non-negative i64 bit-count rows |

These modules are low-level vocabulary, not allocator policy owners.

### Raw Substrate And `hako_alloc`

Current layering:

```text
hako.mem / hako.buf / hako.ptr / hako.atomic / hako.tls / hako.osvm
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
- `hako_alloc` policy/state/facade ownership;
- `RawBuf` / `RawArray` proof-backed substrate use;
- size-class static tables;
- page/free-list policy;
- TLS cache-slot and atomic remote-free proofs;
- OSVM page-source proofs;
- EXE / pure-first proof apps.

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
- Rune metadata and MIR hints: `docs/reference/mir/hints.md`
- Rune profile registry: `docs/reference/mir/rune-profile-registry.md`
- MIR metadata facts: `docs/reference/mir/metadata-facts-ssot.md`
- ABI boundary: `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- Runtime substrate capability rows:
  `docs/reference/runtime/substrate-capabilities.md`
- Current mimalloc purpose:
  `docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md`
