---
Status: SSOT
Decision: accepted
Date: 2026-06-05
Scope: inventory of language, MIR, runtime, and tooling gaps that block a faithful `.hako` mimalloc-style port.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
  - docs/reference/language/types.md
  - docs/reference/runtime/substrate-capabilities.md
  - tools/hako_check/README.md
---

# Mimalloc `.hako` Port Capability Gap Inventory

## Decision

The current mimalloc port gap is not primarily a missing surface syntax problem.
The parser and MIR already carry bitwise and shift operators:

```text
& | ^ << >>
```

The gap is that mimalloc's allocator shape needs restricted low-level
capabilities that are not yet product-grade `.hako` execution substrate:

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
`RawPtr<T>` source semantics.

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

## Why Mimalloc Is Hard To Write "Straight" Today

Mimalloc's core lookup shape is roughly:

```text
segment = top_table[address >> segment_shift]
page = segment.pages[(address >> page_shift) & page_mask]
```

To express this faithfully and quickly in `.hako`, the system needs more than
ordinary arrays and ordinary `i64` arithmetic.

## Gap Inventory

| Area | Current state | Missing for faithful port | Recommended boundary |
| --- | --- | --- | --- |
| Bitwise operators | Parser/MIR support exists. | Product-grade unsigned semantics must be explicit where used for addresses/bitmaps. | Keep syntax; add capability-backed exact numeric rows, not ad hoc externcall. |
| Right shift | Plain `>>` remains signed i64 arithmetic in the current lane. | Logical right shift for `usize/u64` page-key calculations with shift-count traps. | Use exact numeric shift route facts / verifier-backed lowering. |
| Pointer values | Native pointer rows exist for selected `hako_mem` / atomic seams. | A no-escape address token that can be indexed/masked without becoming general pointer arithmetic. | `AddressToken` / `PageKey` capability, allocator-only and no-escape. |
| Page lookup | `.hako` has modeled page-map policy; replacement front has generated C side table. | Source-visible page-key lookup with typed, bounded storage and clear ownership semantics. | `PageMapBridge` over typed tables; no Type ABI hot lookup. |
| Arrays/tables | Static const tables and DirectArray-style plans exist for selected cases. | Nested or side-table page metadata layout suitable for page-key lookup and owner lookup. | Typed table/side-table substrate, with MIR metadata and hako_check explanation. |
| TLS owner state | Thread substrate and TLS proof rows exist in narrow form. | Per-worker/per-thread allocator arena identity and teardown semantics. | `WorkerId` + allocator TLS capability, separate from source-level `nowait`. |
| Remote free | Atomic pointer store/load/CAS proof rows and retry-loop proofs exist. | Product-shaped remote free head push, owner drain, abandoned owner handling. | `AtomicRemoteHead` capability with memory-order vocabulary and counters. |
| OS memory | OSVM reserve/commit/decommit rows exist in narrow form. | Segment/page backing lifecycle and reclaim policy. | OSVM page-source seam under allocator policy, not direct app calls. |
| hako_check | Source/MIR/report adapters exist; replacement-front report reader exists. | Capability coverage inventory that joins `.hako` model coverage with replacement-front execution evidence. | Extend hako_check only as observation; no keeper selection or rewriting. |

## Stop Line

Do not add these as general language features:

```text
RawPtr<T>
pointer arithmetic operators
address dereference syntax
implicit pointer-to-integer conversion
Type ABI lookup on malloc/free hot path
Provider ABI dispatch on replacement-front hot path
```

Do not read a fast benchmark-only replacement front as a full `.hako`
mimalloc algorithm claim.

## Recommended Implementation Order

1. **Inventory/report first**
   - Keep using `hako_check replacement-front-report` to identify whether the
     current measured owner is generated C replacement-front, Provider ABI,
     `.hako` source, or MIR builder.
   - Add a future coverage adapter that reports which mimalloc-required
     substrate capabilities are modeled, route-backed, and actually executed.

2. **Exact page-key arithmetic**
   - Lock `usize/u64` logical right shift and bit-mask route facts for
     allocator page-key calculations.
   - Require verifier-backed shift-count and range diagnostics.

3. **No-escape address token**
   - Add an allocator-only `AddressToken` / `PageKey` concept.
   - It may be shifted, masked, compared, and used as a lookup key.
   - It may not be dereferenced or exposed as a general pointer.

4. **PageMapBridge**
   - Provide a typed table / side-table bridge from `PageKey` to page metadata.
   - Keep Type ABI as descriptor/control plane only.
   - Keep Provider ABI as execution boundary only.

5. **Worker-local arena and remote-free capability**
   - Bind allocator owner identity to runtime worker/thread registry.
   - Add `AtomicRemoteHead` push/drain semantics with counters.
   - Keep source-level concurrency claims separate from C pthread benchmark
     evidence.

6. **Product allocator bridge**
   - Only after the above, connect `.hako` policy/state to a product-shaped
     replacement front.
   - Activation, hooks, global allocator claims, and winner claims remain
     closed until a later explicit activation row.

## Current Reading For Pro Consultation

The question to ask is not:

```text
Should Hakorune add C-style pointer syntax?
```

The better question is:

```text
What is the smallest safe capability surface that lets `.hako` express a
mimalloc-style address-derived page map and remote-free path without opening
general unsafe pointer arithmetic or making Type ABI / Provider ABI hot?
```
