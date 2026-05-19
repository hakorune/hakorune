---
Status: SSOT
Decision: accepted
Date: 2026-05-19
Scope: Task order for exact numeric types and memory substrate before the
  mimalloc `.hako` port can claim C-mimalloc-like memory/performance behavior.
Related:
  - docs/reference/language/types.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# Typed Numeric / Memory Substrate Task Order SSOT

## Decision

Hakorune should let `.hako` source spell exact integer intent clearly:

```hako
record Span {
    start: usize
    count: usize
}

method classFor(size: usize): u32 {
    ...
}
```

But spelling `usize`, `u32`, or `u64` is not enough to open allocator-grade
memory behavior. The mimalloc port may continue narrow exact-`usize` stored
field migrations for owner-local non-negative facts, but it must stop before
raw memory execution until the numeric and memory substrate gates below are
green.

## Current Meaning

Current accepted use:

- type annotations on fields, records, parameters, and returns;
- exact numeric storage/layout metadata;
- range-checked exact numeric facts and selected route facts;
- narrow hako_alloc stored-field migrations where values remain in the current
  `Integer(i64)` executable subset;
- backend fail-fast when exact numeric storage or route facts are unsupported.

Current non-goals:

- arbitrary exact typed arithmetic in source;
- silent fallback from `usize` / `u32` / `u64` to legacy dynamic `Integer(i64)`;
- raw pointer arithmetic;
- bitmap word arithmetic;
- allocator memory residence or arena execution;
- provider activation or process allocator replacement.

## Syntax Rule

The surface should be crisp. Prefer exact type names in places that document
contracts:

```hako
record SizeClassRow {
    size: usize
    class_id: u32
}

method reserve(bytes: usize, alignment: usize): usize
```

Do not rely on plain operators becoming allocator-grade semantics before their
rows exist:

```hako
// Not allocator-grade until exact op rows cover this shape.
local next: usize = current + delta
local mask: u64 = 1u64 << shift
```

Until then, use explicit helpers or keep the row scalar/model-only.

## Task Order

## Near-Term Current Order

Use this order when deciding whether to keep pushing mimalloc rows or switch to
Hakorune language/compiler work.

```text
0. Finish the active exact-usize closeout row.
   Current: HAKO-ALLOC-USIZE-FIELD-GROUP-048.

1. Finish the local-free reuse ledger release-apply numeric cluster.
   Remaining likely rows:
     - execution/capability release-apply reject counters
     - closeout for that counter group

2. Add narrow report-carrier record rows where the current `box` is only a
   scalar report carrier.
   Target first:
     HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport
   Do not migrate owner boxes or identity-bearing state in the same row.

3. Strengthen source exact numeric surface only where it removes repeated
   allocator friction:
     - exact annotations in fields/params/returns
     - literal suffixes such as 0usize / 1u32 / 1u64
     - source-near diagnostics for negative writes to unsigned exact fields

4. Strengthen exact operation semantics before opening bitmap or pointer math:
     - checked same-type add/sub/mul
     - exact compare
     - unsigned shifts / bitwise ops with fail-fast traps
     - PHI/Select exact-type preservation for identical exact types only

5. Improve return/route diagnostics when a normal `.hako` helper shape still
   requires unnecessary typed-local workarounds.

6. Return to memory substrate rows:
     - no-escape address residence
     - arena backing first execution
     - real segment-map execution
     - bitmap / atomic / TLS after exact numeric + memory substrate gates
```

Do not pause the mimalloc lane for broad language work. Switch to language work
only when it removes repeated allocator workarounds or blocks the next memory
substrate step.

### A. Narrow Exact Stored Fields

Continue the current 293x exact-`usize` field-group lane only for:

- owner-local byte/capacity/count/page-size facts;
- values that are never negative;
- fields that do not use `-1` or any signed sentinel;
- fields whose proof apps stay in the current executable `Integer(i64)` subset.

Do not migrate:

- reason/status vocabularies;
- indexes with `-1` not-found values;
- ids/tokens/handles;
- deltas;
- pointer-shaped addresses;
- counters whose signedness is intentionally diagnostic or uncertain.

### B. Source Type Surface Parity

Before relying on exact types in broader `.hako` code, finish source parity:

```text
TYPE-NUM-SURFACE-001
  .hako parser accepts exact numeric annotations in fields/params/returns.

TYPE-NUM-SURFACE-002
  .hako parser accepts range-checked literal suffixes:
  0usize, 1u32, 1u64, 42i64.

TYPE-NUM-SURFACE-003
  diagnostics reject negative assignment to unsigned exact fields close to the
  source site.
```

These rows are language/compiler rows, not mimalloc behavior rows.

### C. Exact Operation Semantics

Before bitmap, pointer offset, or allocator-size arithmetic becomes real, the
following must be backend-visible:

```text
EXACT-NUM-OPS-001
  checked add/sub/mul for same exact type.

EXACT-NUM-OPS-002
  exact compare and logical shift rules for unsigned types.

EXACT-NUM-OPS-003
  bitwise and/or/xor plus explicit shift-count traps for u32/u64/usize.

EXACT-NUM-OPS-004
  PHI/Select/loop merge keeps exact facts only for identical exact types and
  fails fast on unsafe exact/dynamic merges.

EXACT-NUM-OPS-005
  backend parity and fail-fast coverage for VM, pure-first EXE, and active
  codegen routes.
```

Wrapping behavior must be explicit vocabulary, for example
`wrapping_add_usize` or `wrapping_shl_u64`. Plain operators stay checked unless
the `usize` foundation SSOT is updated by accepted decision.

### D. Static Tables And Layout

Mimalloc needs dense tables and fixed layout before real memory:

```text
MEM-LAYOUT-001
  static const table source/MIR owner for u8/u16/u32/u64/usize rows.

MEM-LAYOUT-002
  record/layout carrier for allocator metadata reports that are pure scalar
  fields.

MEM-LAYOUT-003
  sizeof/alignof/offsetof vocabulary for exact-layout records.

MEM-LAYOUT-004
  no silent user-box dynamic layout for allocator native metadata.
```

Report-like data with only scalar fields should prefer `record` once the
constructor/lowering and backend-read surface is ready for that report family.

### E. Memory Substrate

Open memory execution in this order:

```text
MEM-SUBSTRATE-001
  RawBuf / Span / MaybeInit vocabulary, metadata-only first.

MEM-SUBSTRATE-002
  bounds and initialized-range verifier contracts.

MEM-SUBSTRATE-003
  no-escape pointer/address residence with explicit lifetime boundary.

MEM-SUBSTRATE-004
  raw load/store helpers for exact integer element types.

MEM-SUBSTRATE-005
  arena backing first execution using exact sizes and layout facts.

MEM-SUBSTRATE-006
  real segment-map execution after arena backing is closed out.
```

Rows before `MEM-SUBSTRATE-005` may model memory behavior, but they must not
claim real allocator residence.

### F. Bitmap, Atomic, TLS

Bitmap and concurrency come after exact numeric and memory substrate:

```text
BITMAP-001
  u64/u32 bitmap word policy with exact bitwise ops.

BITMAP-002
  bitmap claim/unclaim model lane.

BITMAP-003
  first real bitmap execution row.

ATOMIC-001
  atomic load/store/CAS/fetch_add route vocabulary.

TLS-001
  allocator-internal worker-local/TLS cache slots.
```

Do not use `Channel`, `co`, or user-facing concurrency semantics as allocator
substrate. Mimalloc needs runtime/internal atomics and TLS first.

## Mimalloc Gate

The mimalloc port may continue scalar/model rows while the following remain
closed:

```text
real raw pointer residence
real arena backing allocation
real segment-map mutation
atomic bitmap execution
OSVM execution beyond already guarded facades
provider activation
host allocator replacement
hooks / #[global_allocator]
```

To open a real memory row, the card must name:

- exact numeric operations it uses;
- memory substrate capability it consumes;
- verifier contract;
- backend route or explicit fail-fast;
- representative L3 EXE evidence when the route is first-pattern or closeout.

## Recommended Next Planning Use

When `HAKO-ALLOC-USIZE-FIELD-GROUP-042` selects a next field group, it should
choose another narrow field group only if it remains in section A. If the next
candidate needs arithmetic, layout, raw memory, bitmap, atomics, or TLS, stop
the field migration lane and select the corresponding substrate row from this
SSOT instead.
