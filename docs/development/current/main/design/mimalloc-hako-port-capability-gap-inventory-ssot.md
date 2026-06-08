---
Status: SSOT
Decision: accepted
Date: 2026-06-08
Scope: compact decision surface for the mimalloc port capability gaps that block a faithful `.hako` port.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
  - docs/reference/language/types.md
  - docs/reference/runtime/substrate-capabilities.md
  - tools/hako_check/README.md
  - docs/development/current/main/investigations/mimalloc-hako-port-capability-gap-inventory-investigation-2026-06-08.md
  - docs/development/current/main/investigations/mimalloc-hako-port-capability-gap-inventory-task-ledger-2026-06-08.md
---

# Mimalloc `.hako` Port Capability Gap Inventory

## Decision

The mimalloc port gap is not primarily a missing surface-syntax problem.
The parser and MIR already carry bitwise and shift operators:

```text
& | ^ << >>
```

The missing piece is a contract-bound memory fast-path sublanguage with
restricted low-level capabilities:

```text
address-derived page lookup
exact unsigned/pointer-sized arithmetic
typed page table / side-table storage
thread-local owner state
atomic remote-free push/drain
OS memory page backing
no-escape pointer/address capability proofs
```

Do not solve this by adding unrestricted C-like pointer arithmetic or broad
`RawPtr<T>` source semantics. The accepted route is a contract-bound memory
region:

```hako
fastmem PageMapV0 {
    local a = mem.addr(ptr)
    local key = (a >> PAGE_SHIFT) & PAGE_MASK
    local page = page_table[key]
}
```

`fastmem` is convenient inside the region, but every operation must lower to a
known `MemOp` row and pass verifier / `hako_check` inventory.

Post-consultation decision:

```text
RawPtr<T>:
  rejected as the mimalloc port surface

FastMemoryContract / fastmem source region:
  accepted as the first implementation boundary for low-level memory hot paths

allocator:
  first consumer of fastmem, not the only consumer

AddressToken / PageKey / PageMapBridge / PageMetaHandle:
  accepted as later safe capability wrappers over the same MemOp lowering

Type ABI:
  descriptor / report interpretation only

Provider ABI:
  execution boundary for provider calls only; not the replacement-front hot path

FastMemoryContract:
  compile-time / lowering-time memory contract; not a runtime dispatch table

Parser parity:
  required before `fastmem ContractName { ... }` is accepted as active source
  syntax. Rust-only parser support is not enough for this lane because the
  selfhost parser is a current migration target and source syntax must not
  fork silently.
```

## Current Facts

```text
bitwise syntax:
  present

shift syntax:
  present

current plain >>:
  signed i64 arithmetic shift in the current lane

exact unsigned logical shift:
  modeled through exact numeric route facts and reference execution, but not a
  blanket source-level promise for all product backends

usize/u64 exact numeric substrate:
  live-narrow; type names, facts, route checks, and selected backend support
  exist, while runtime values still travel mostly through Integer(i64)

RawPtr / pointer operators:
  intentionally absent as a broad source feature

.hako page map:
  modeled heavily through page ids, ledgers, proofs, and policy seams

current LD_PRELOAD benchmark hot path:
  generated C replacement front, not `.hako` source or MIR builder execution
```

## Gap Inventory Summary

| Area | Current state | Missing for faithful port | Recommended boundary |
| --- | --- | --- | --- |
| Bitwise operators | Parser/MIR support exists. | Product-grade unsigned semantics must be explicit where used for addresses/bitmaps. | Keep syntax; add capability-backed exact numeric rows, not ad hoc externcall. |
| Right shift | Plain `>>` remains signed i64 arithmetic in the current lane. | Logical right shift for address/page-key calculations with shift-count traps. | In `fastmem`, left-hand memory address values use exact logical route facts; ordinary `>>` stays unchanged. |
| Pointer values | Native pointer rows exist for selected `hako_mem` / atomic seams. | Region-local memory address/reference values without becoming general pointer arithmetic. | `MemAddr` / `MemRef<T>` inside `fastmem`; later safe wrapper may expose `AddressToken` / `PageKey`. |
| Page lookup | `.hako` has modeled page-map policy; replacement front has generated C side table. | Source-authorable page-key lookup with typed, bounded storage and clear ownership semantics. | `fastmem` `PageMapV0` over verified MemOps first; `PageMapBridge` wrapper later; no Type ABI hot lookup. |
| Arrays/tables | Static const tables and DirectArray-style plans exist for selected cases. | Nested or side-table page metadata layout suitable for page-key lookup and owner lookup. | Typed table/side-table substrate, with MIR metadata and `hako_check` explanation. |
| TLS owner state | Thread substrate and TLS proof rows exist in narrow form. | Per-worker/per-thread allocator arena identity and teardown semantics. | `WorkerId` + allocator TLS capability, separate from source-level `nowait`. |
| Remote free | Atomic pointer store/load/CAS proof rows and retry-loop proofs exist. | Product-shaped remote free head push, owner drain, abandoned owner handling. | `AtomicRemoteHead` capability with memory-order vocabulary and counters. |
| OS memory | OSVM reserve/commit/decommit rows exist in narrow form. | Segment/page backing lifecycle and reclaim policy. | OSVM page-source seam under allocator policy, not direct app calls. |
| hako_check | Source/MIR/report adapters exist; replacement-front report reader exists. | Capability coverage inventory that joins `.hako` model coverage with replacement-front execution evidence. | Extend `hako_check` only as observation; no keeper selection or rewriting. |

## Stop Line

Do not add these as general language features:

```text
RawPtr<T>
pointer arithmetic operators outside fastmem
address dereference syntax
implicit pointer-to-integer conversion
Type ABI lookup on malloc/free hot path
Provider ABI dispatch on replacement-front hot path
contract-less unsafe {}
contract-less fastmem {}
```

Do not read a fast benchmark-only replacement front as a full `.hako`
mimalloc algorithm claim.

## FastMemory Contract Region

`fastmem` is the accepted near-term source boundary for memory hot paths. It is
backed by `FastMemoryContract`, and is not an allocator-only unsafe island or a
general unsafe escape hatch.

Naming note:

```text
fastmem:
  accepted near-term memory-profile pilot spelling

fastpath:
  reserved future general spelling for contract-bound fast-path regions
  across memory / io / simd / other profiles
```

Do not introduce `fastio`, `fastsimd`, or similar per-domain keywords. If a
second profile opens later, prefer the general form:

```hako
fastpath ContractName { ... }
```

with the profile selected by contract metadata.

Selfhost timing:

```text
before selfhost parser/MIRBuilder stability:
  keep `fastmem ContractName { ... }`
  document `fastpath` only
  avoid parser churn

after selfhost parser/MIRBuilder stability:
  consider whether `fastpath` becomes canonical and `fastmem` becomes a
  compatibility alias for profile=memory
```

```text
normal .hako:
  safe Box-centered language surface

fastmem region:
  low-level memory hot path sublanguage
  contract id required
  operations must be classified as MemOps
  no Type ABI hot lookup
  no Provider ABI hot dispatch

capability chain:
  later safe API surface over the same MemOps
```

The first contract family is allocator-oriented:

```text
allocator.page_map_v0
allocator.remote_free_v0
allocator.tls_arena_v0
```

Future families may cover other memory-heavy code without reopening raw pointer
semantics for the whole language:

```text
bytes.scan_v0
hash.table_probe_v0
linalg.kernel_v0
codec.buffer_v0
runtime.table_v0
```

`fastmem` must be explicit:

```text
canonical boundary:
  fastmem PageMapV0 { ... }

rejected:
  unsafe { ... }
  fastmem { ... }
  @rune FastMemory(PageMapV0) as a region boundary
  method-wide fastmem by annotation only
```

`@rune FastMemory(PageMapV0)` may remain as optional declaration metadata only.
It can say that a method/package participates in FastMemory work, but it does
not create a `FastMemRegion`, does not widen the method body into a fastmem
region, and is not a substitute for `fastmem ContractName { ... }`.

## Companion Investigation

The detailed design surfaces that were removed from the active SSOT live in:

```text
docs/development/current/main/investigations/mimalloc-hako-port-capability-gap-inventory-investigation-2026-06-08.md
```

That investigation carries the detailed plans for:

```text
MemOp region plan
PageKey exact route
PageMapBridge
TypedPageMetaHandle
safe capability wrappers
shape coverage scoring
producer transition
owner state
source/MIR/lowering visibility
pro-consultation prompt
```

The exhaustive task ledger remains archived here:

```text
docs/development/current/main/investigations/mimalloc-hako-port-capability-gap-inventory-task-ledger-2026-06-08.md
```

## Recommended Implementation Order

1. Lock the active contract surface: `FastMemoryContractV0`, `FastMemRegionV0`,
   `MemOpAllowlistV0`, `MemPointerClassV0`, and `MemLayoutContractV0`.
2. Keep `MemOp` as the single executable MIR instruction and leave
   `FastMemRegion` as side-table metadata.
3. Implement verifier gates before accepting source syntax.
4. Lock page-key arithmetic, `PageMapBridge`, and owner-state observation rows.
5. Add source syntax and remote-free execution only after the above gates are
   stable.

## Current Reading For Pro Consultation

The active question stays narrow:

```text
What is the smallest contract-bound fast memory sublanguage that lets `.hako`
express mimalloc-style address-derived page maps and remote-free paths without
opening general unsafe pointer arithmetic or making Type ABI / Provider ABI hot?
```
