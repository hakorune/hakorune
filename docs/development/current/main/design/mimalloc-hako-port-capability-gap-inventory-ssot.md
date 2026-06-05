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
`RawPtr<T>` source semantics. The accepted route is a contract-bound memory
fast-path sublanguage:

```hako
fastmem PageMapV0 {
    local a = mem.addr(ptr)
    local key = (a >> PAGE_SHIFT) & PAGE_MASK
    local page = page_table[key]
}
```

`fastmem` is convenient inside the region, but every operation must lower to a
known `MemOp` row and pass verifier / hako_check inventory.

Post-consultation decision:

```text
RawPtr<T>:
  rejected as the mimalloc port surface

FastMemoryContract / fastmem source region:
  accepted as the first implementation boundary for low-level memory hot paths

allocator:
  first consumer of fastmem, not the only consumer

AddressToken / PageKey / PageMapBridge / PageMetaHandle:
  still accepted, but as later safe capability wrappers over the same MemOp
  lowering rather than the first required authoring surface

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
| Right shift | Plain `>>` remains signed i64 arithmetic in the current lane. | Logical right shift for address/page-key calculations with shift-count traps. | In `fastmem`, left-hand memory address values use exact logical route facts; ordinary `>>` stays unchanged. |
| Pointer values | Native pointer rows exist for selected `hako_mem` / atomic seams. | Region-local memory address/reference values without becoming general pointer arithmetic. | `MemAddr` / `MemRef<T>` inside `fastmem`; later safe wrapper may expose `AddressToken` / `PageKey`. |
| Page lookup | `.hako` has modeled page-map policy; replacement front has generated C side table. | Source-authorable page-key lookup with typed, bounded storage and clear ownership semantics. | `fastmem` `PageMapV0` over verified MemOps first; `PageMapBridge` wrapper later; no Type ABI hot lookup. |
| Arrays/tables | Static const tables and DirectArray-style plans exist for selected cases. | Nested or side-table page metadata layout suitable for page-key lookup and owner lookup. | Typed table/side-table substrate, with MIR metadata and hako_check explanation. |
| TLS owner state | Thread substrate and TLS proof rows exist in narrow form. | Per-worker/per-thread allocator arena identity and teardown semantics. | `WorkerId` + allocator TLS capability, separate from source-level `nowait`. |
| Remote free | Atomic pointer store/load/CAS proof rows and retry-loop proofs exist. | Product-shaped remote free head push, owner drain, abandoned owner handling. | `AtomicRemoteHead` capability with memory-order vocabulary and counters. |
| OS memory | OSVM reserve/commit/decommit rows exist in narrow form. | Segment/page backing lifecycle and reclaim policy. | OSVM page-source seam under allocator policy, not direct app calls. |
| hako_check | Source/MIR/report adapters exist; replacement-front report reader exists. | Capability coverage inventory that joins `.hako` model coverage with replacement-front execution evidence. | Extend hako_check only as observation; no keeper selection or rewriting. |

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
accepted:
  fastmem PageMapV0 { ... }
  @rune FastMemory(PageMapV0)

rejected:
  unsafe { ... }
  fastmem { ... }
```

The region may use convenient memory-shaped operations, but only if the
compiler can classify them:

```text
allowed MemOp families:
  MemAddrOf
  MemAdd / MemSub
  MemLogicalShr / MemAnd
  MemTableIndex
  MemFieldLoad / MemFieldStore
  MemTypedLoad / MemTypedStore
  MemAtomicCas / MemAtomicExchange / MemAtomicFetchAdd

forbidden in fastmem:
  arbitrary raw dereference
  arbitrary external call
  arbitrary Box method call
  allocation / safepoint
  await / nowait
  closure capture
  metadata pointer escape
  Provider ABI crossing
  Type ABI hot lookup
```

Pointer-like values inside fastmem are region-local values, not broad source
types:

```text
MemAddr:
  no-escape address value; arithmetic allowed only inside the contract

MemRef<T>:
  no-escape typed memory reference; load/store only through allowed MemOps

MemLayoutRef<PageMeta>:
  no-escape metadata reference; field access only through verified layout

user allocation pointer:
  ABI endpoint allowed for malloc/free/realloc surfaces, but not general .hako
  dereference
```

The contract owns layout facts:

```text
FastMemoryContract PageMapV0:
  domain=allocator
  backend=replacement_front
  address_width=target_usize
  endian=native
  safepoint=forbidden
  allocation=forbidden
  provider_abi_crossing=forbidden
  type_abi_hot_lookup=forbidden

  layouts:
    PageMeta:
      repr=C
      fields:
        owner_worker_id
        block_size
        free_head
        local_free_head
        remote_head
        capacity
        used
```

## MIR MemOp Region Plan

`fastmem` source is not accepted until MIR can represent the region with
explicit memory operations and report them. The v0 MIR/report vocabulary is:

```text
FastMemRegionBegin:
  contract_id
  contract_family
  source_span

FastMemRegionEnd:
  contract_id

MemAddrOf:
  user pointer -> MemAddr

MemLogicalShr / MemAnd:
  address/page-key arithmetic
  shift count and mask must be verifier-visible

MemAdd / MemSub:
  region-local address arithmetic only

MemTableIndex:
  verified table/key lookup

MemFieldLoad / MemFieldStore:
  verified layout field access

MemTypedLoad / MemTypedStore:
  typed load/store allowed only by contract

MemAtomicCas / MemAtomicExchange / MemAtomicFetchAdd:
  atomic operations with explicit memory order
```

The report vocabulary must be present before source syntax is accepted:

```text
fastmem_memop_region_begin_count
fastmem_memop_region_end_count
fastmem_memop_unbalanced_region_count
fastmem_memop_unclassified_count
fastmem_memop_addr_of_count
fastmem_memop_add_count
fastmem_memop_sub_count
fastmem_memop_logical_shr_count
fastmem_memop_and_count
fastmem_memop_table_index_count
fastmem_memop_field_load_count
fastmem_memop_field_store_count
fastmem_memop_typed_load_count
fastmem_memop_typed_store_count
fastmem_memop_atomic_cas_count
fastmem_memop_atomic_exchange_count
fastmem_memop_atomic_fetch_add_count
fastmem_forbidden_allocation_count
fastmem_forbidden_safepoint_count
fastmem_forbidden_await_count
fastmem_forbidden_nowait_count
fastmem_forbidden_call_count
fastmem_type_abi_hot_lookup_count
fastmem_provider_abi_crossing_count
```

Before `MIM-FMEM-004`, all of these may be zero because there is no source
syntax and no MIR region yet. They still matter: the verifier task must turn
non-zero `unclassified` / `forbidden` / crossing counts into fail-fast errors.

## PageKey Exact Route Plan

`PageKey` derivation uses exact address-width arithmetic. This is narrower than
changing the whole language shift semantics.

```text
ordinary i64 >>:
  remains the current signed i64 route for this lane

fastmem MemAddr >> constant:
  logical target-usize shift
  shift count must satisfy 0 <= shift < address_width
  invalid count is verifier failure

fastmem MemAddr & constant_mask:
  exact target-usize mask
  mask must be verifier-visible

PageKeyMake:
  MemAddr -> PageKey
  consumes segment/page shift and mask facts
  emits report fields before backend lowering
```

The v0 route is intentionally narrow:

```text
accepted:
  page_key_numeric_route=uintptr_exact_logical_shift_mask

rejected:
  page_key_numeric_route=generic_i64 for allocator keeper work
  silent wrapping shift count
  implicit pointer-to-integer conversion outside fastmem
```

Report vocabulary:

```text
page_key_capability
page_key_numeric_route
page_key_shift_count_trap
page_key_segment_shift
page_key_page_shift
page_key_mask
```

## PageMapBridge Plan

`PageMapBridge` is the execution substrate that maps an exact page key to
layout-verified page metadata. It is not a Type ABI lookup and not Provider ABI
dispatch.

```text
input:
  PageKey or fastmem MemAddr-derived key

output:
  PageMetaHandle / MemLayoutRef<PageMeta>

accepted v0 bridge kinds:
  flat_side_table
  two_level_segment_table
  page_base_mask
  header_backptr

rejected keeper route:
  range_scan on the hot free path
```

Bridge invariants:

```text
page_map_bridge_type_abi_hot_lookup_count=0
page_map_bridge_provider_abi_hot_dispatch_count=0
fastmem_contract_runtime_lookup_count=0
fastmem_unverified_offset_load_count=0
```

The bridge can be consumed by generated C replacement-front evidence before it
is exposed through source syntax. Source-level wrappers are later
`AddressToken -> PageKey -> PageMapBridge -> PageMetaHandle`.

## Recommended Implementation Order

1. **FastMemory docs/report lock**
   - Define `FastMemoryContractV0`, `FastMemRegionV0`,
     `MemOpAllowlistV0`, `MemPointerClassV0`, and
     `MemLayoutContractV0`.
   - Keep ordinary `.hako` safe and Box-centered.

2. **Inventory/report first**
   - Keep using `hako_check replacement-front-report` to identify whether the
     current measured owner is generated C replacement-front, Provider ABI,
     `.hako` source, or MIR builder.
   - Add a future coverage adapter that reports which mimalloc-required
     fastmem/capability substrate rows are modeled, route-backed, and actually
     executed.

3. **MIR MemOp region plan**
   - Add `FastMemRegionBegin/End` and explicit MemOp rows before product
     source syntax broadening.
   - Document how unclassified memory behavior becomes verifier failure.

4. **FastMem verifier implementation**
   - Implement the region verifier before accepting source syntax.
   - Reject allocation, safepoints, arbitrary calls, unmanaged escape, Type ABI
     hot lookup, and Provider ABI hot dispatch.

5. **Exact page-key arithmetic**
   - Lock `usize/u64` logical right shift and bit-mask route facts for
     allocator page-key calculations.
   - Require verifier-backed shift-count and range diagnostics.

6. **PageMapBridge plan**
   - Provide a typed table / side-table bridge from `PageKey` to page metadata.
   - Keep Type ABI as descriptor/control plane only.
   - Keep Provider ABI as execution boundary only.

7. **Parser parity catch-up**
   - Catch up the `.hako` parser with the Rust parser for the subset needed by
     fastmem before accepting source syntax.
   - First parity subset: general bitwise/shift expression parse, rune
     contract-name metadata, and `fastmem IDENT { ... }` parse-only.
   - Rust-only active grammar is rejected.

8. **Source fastmem syntax**
   - Add `fastmem ContractName { ... }` or method annotation only after the
     MemOp/verifier contract, first bridge plan, and parser parity gate are
     stable.
   - Contract name is mandatory.
   - Initial row is parse-only; lowering/execution stays closed until a later
     implementation row.

9. **Worker-local arena and remote-free capability**
   - Bind allocator owner identity to runtime worker/thread registry.
   - Add `AtomicRemoteHead` push/drain semantics with counters.
   - Keep source-level concurrency claims separate from C pthread benchmark
     evidence.

10. **Remote-free pilot**
   - Pilot `AtomicRemoteHead` only after the plan and owner-state rows are
     visible.

11. **Capability wrappers**
   - Add `AddressToken`, `PageKey`, `PageMapBridge`, `PageMetaHandle`, and
     `AtomicRemoteHead` as safer wrappers over the same MemOps.

12. **Product allocator bridge**
   - Only after the above, connect `.hako` policy/state to a product-shaped
     replacement front.
   - Activation, hooks, global allocator claims, and winner claims remain
     closed until a later explicit activation row.

## Task Breakdown

Use this task list before adding capability code. Each row is intentionally
narrow; do not mix source syntax expansion, runtime activation, and benchmark
keeper work in one task.

| Task | Status | Scope | Acceptance |
| --- | --- | --- | --- |
| `MIM-FMEM-001 FastMemoryContract docs/report lock` | done | Define `fastmem` as the source spelling backed by the general FastMemory memory fast-path contract, with allocator as first consumer. | Contract id required; broad unsafe/raw pointer semantics stay rejected; Type ABI/Provider ABI hot path counts stay zero. |
| `MIM-FMEM-002 hako_check fastmem capability inventory` | done | Add an observation-only report that joins replacement-front counters with fastmem/capability coverage. | Emits fastmem fields; no source rewrite, benchmark run, keeper selection, provider activation, hooks, global allocator claim, or product readiness claim. |
| `MIM-FMEM-003 MIR MemOp region docs/report plan` | done | Name `FastMemRegionBegin/End` plus MemAddr/field/table/atomic MemOps. | Fail-fast vocabulary is documented; behavior change waits for verifier implementation. |
| `MIM-FMEM-004 FastMem verifier implementation` | done | Implement region verifier for MemOp allowlist, no-escape values, and forbidden operations. | Unclassified memory behavior, allocation, safepoint, await/nowait, arbitrary calls, Type ABI lookup, and Provider ABI dispatch are rejected. |
| `MIM-FMEM-005 PageKey exact route docs/report lock` | done | Name `PageKeyExactRoutePlanV0`: exact `usize/u64` logical shift, mask, shift-count trap, address-width facts. | Report vocabulary and fail-fast expectations are documented before code; plain `>>` semantics remain unchanged outside fastmem. |
| `MIM-FMEM-006 PageKey exact route implementation` | done | Implement the narrow exact numeric route needed for allocator page-key derivation. | Route facts and backend/reference behavior agree; invalid shift/range traps are observable. |
| `MIM-FMEM-007 PageMapBridge plan` | done | Define `PageMapBridgePlanV0`: fastmem/PageKey -> PageMetaHandle via typed side-table/table route. | Type ABI hot lookup count is zero; Provider ABI hot dispatch count is zero; range-scan replacement is explicit. |
| `PARSER-FMEM-001 parser parity inventory contract` | next | Freeze the Rust/.hako parser gap list that blocks fastmem source syntax. | No source syntax behavior change; Rust-only active grammar remains rejected. |
| `PARSER-FMEM-002 parser parity gate surface` | pending | Define one reusable probe/smoke entry for Rust parser and `.hako` parser parse-only parity. | Gate is reusable and does not add lowering/runtime behavior. |
| `PARSER-FMEM-003 general bitwise/shift expression parity` | pending | Catch up `.hako` parser expression parse for `<< >> & | ^` outside static const-only use. | Fastmem examples can parse shift/mask expressions without externcall escape. |
| `PARSER-FMEM-004 rune contract-name parity` | pending | Catch up `.hako` parser rune metadata names needed by current Rust parser surfaces. | Rune metadata remains parse/noop unless a separate consumer row opens behavior. |
| `PARSER-FMEM-005 fastmem block parse-only dual parser pilot` | pending | Add `fastmem IDENT { ... }` parse-only surface to both parsers. | Execution/lowering stay closed; contract name is mandatory. |
| `PARSER-FMEM-006 fastmem contractless fail-fast parity` | pending | Reject `fastmem { ... }` and `unsafe { ... }` in both parsers. | Contract-less unsafe escape remains closed. |
| `MIM-FMEM-008 fastmem source syntax pilot` | next | Connect parser output to MIR MemOp region metadata after `PARSER-FMEM-001..006` proved dual parser parse-only parity. | Contract-less `unsafe` / `fastmem` remains rejected; region-local MemAddr/MemRef cannot escape; execution/lowering beyond metadata stays closed until this row opens it explicitly. |
| `MIM-FMEM-009 PageMapBridge benchmark-front pilot` | pending | Replace the current free-path page lookup shape with the selected bridge in generated C replacement-front evidence. | `free_path_page_lookup_route != range_scan`; report keeps product activation and hako algorithm claim closed. |
| `MIM-FMEM-010 TypedPageMetaHandle plan` | pending | Define metadata capability for owner, size, free/local_free/remote_head access. | Page metadata stays layout-verified; unverified offset loads are counted and rejected for keeper work. |
| `MIM-FMEM-011 WorkerId / TLS arena owner state` | pending | Bind allocator owner identity to runtime worker/thread registry and thread-exit flush counters. | Source-level thread support claims remain separate from C pthread benchmark evidence. |
| `MIM-FMEM-012 AtomicRemoteHead plan` | pending | Define remote-free push/drain contract, memory-order vocabulary, and counters. | Remote-free is a page/fastmem capability, not a general `AtomicPtr<T>` surface. |
| `MIM-FMEM-013 AtomicRemoteHead pilot` | pending | Pilot the remote-free push/drain route after owner-state and plan rows exist. | Push/drain counters are observable; product activation and winner claims stay closed. |
| `MIM-FMEM-014 safe capability wrapper plan` | pending | Layer `AddressToken`, `PageKey`, `PageMapBridge`, and `PageMetaHandle` over MemOps. | Wrapper route lowers to the same MemOps as fastmem and does not reopen RawPtr. |
| `MIM-FMEM-015 Mimalloc shape coverage score` | pending | Add speed/shape/safety/coverage separation to report acceptance. | Fast but non-mimalloc-shaped routes cannot become keeper by throughput alone. |
| `MIM-FMEM-016 Product-shaped replacement front bridge` | pending | Connect `.hako` policy/state to a product-shaped replacement front after fastmem/capabilities are present. | Activation, hook install, global allocator claim, and winner claim remain closed. |

## Report Fields For `MIM-FMEM-002`

The first task should prefer stable key-value fields over prose:

```text
output_contract=hako-check-fastmem-capability-inventory-v0
tool_surface=hako_check_fastmem_capability_inventory
observation_only=1
rewrite_executed=0
benchmark_run_executed=0
keeper_selection=0
provider_activation=0
hook_installed=0
global_allocator_product_claim=0

measured_hot_path_owner=generated_c_replacement_front|provider_abi|hako_source|mir_builder|unknown
replacement_front_subowner=free_path_page_lookup|remote_free_queue|global_lock_hot_path|counter_instrumentation|unknown

fastmem_region_count
fastmem_contract_count
fastmem_contract_id=PageMapV0|unknown
fastmem_contract_family=allocator.page_map|allocator.remote_free|allocator.tls_arena|unknown
fastmem_general_rawptr_type=0
fastmem_general_deref_outside_region=0
fastmem_general_pointer_arithmetic_outside_region=0
fastmem_region_pointer_arithmetic_count
fastmem_region_typed_load_count
fastmem_region_typed_store_count
fastmem_region_atomic_op_count
fastmem_escape_count
fastmem_metadata_ptr_escape_count
fastmem_user_ptr_abi_return_count
fastmem_closure_capture_count
fastmem_box_field_store_count
fastmem_array_store_count
fastmem_layout_verified=0|1
fastmem_layout_id=PageMetaLayoutV0|unknown
fastmem_layout_hash=<hash|unknown>
fastmem_unverified_offset_load_count
fastmem_contract_runtime_lookup_count=0

address_token_capability=0|1
address_token_escape_check=missing|pass|fail
address_token_deref_allowed=0
address_token_pointer_arithmetic_allowed=0

page_key_capability=0|1
page_key_numeric_route=missing|uintptr_exact_logical_shift_mask|generic_i64
page_key_shift_count_trap=0|1
page_key_segment_shift=<int|unknown>
page_key_page_shift=<int|unknown>
page_key_mask=<hex|unknown>

free_path_page_lookup_route=range_scan|page_index_side_table|page_map_bridge|unknown
free_path_page_lookup_range_scan_count
page_map_bridge_kind=none|flat_side_table|two_level_segment_table|page_base_mask|header_backptr
page_map_bridge_type_abi_hot_lookup_count
page_map_bridge_provider_abi_hot_dispatch_count

typed_page_meta_handle=0|1
typed_page_table_mode=none|side_table|segment_slices|compressed_index

worker_id_capability=0|1
allocator_tls_arena_enabled=0|1
allocator_tls_arena_count
allocator_thread_exit_flush_count
allocator_abandoned_owner_count

atomic_remote_head_enabled=0|1
remote_free_push_count
remote_free_drain_count
remote_free_cas_retry_count
remote_free_memory_order=missing|acq_rel|release_acquire

mimalloc_shape_page_free_lists=missing|free_only|free_local_remote
mimalloc_shape_thread_local_heap=0|1
mimalloc_shape_segment_slice_lookup=0|1
mimalloc_shape_score=<0..100>
safety_score=<0..100>
coverage_score=<0..100>

replacement_front_is_full_hako_algorithm=0
hako_mimalloc_algorithm_claim=0
product_activation_ready=0
summary=ok|failed
```

## Source/MIR/Lowering Visibility

Do not put every concept at the same layer.

```text
ordinary .hako:
  no RawPtr<T>
  no broad pointer arithmetic
  no dereference syntax

fastmem source region:
  fastmem ContractName { ... }
  mem.addr
  mem.load / mem.store
  mem.atomic*
  logical address shift/mask
  verified layout field access

.hako allocator model:
  PageKey
  PageMapModel
  PageMeta
  WorkerTlsCache
  RemoteFreePolicy

allocator-internal .hako vocabulary:
  PageKey
  PageMeta
  WorkerId
  SizeClass
  AllocBlockToken
  RemoteFreeToken

MIR / plan only:
  FastMemRegionBegin
  FastMemRegionEnd
  MemAddrOf
  MemLogicalShr
  MemAnd
  MemTableIndex
  MemFieldLoad
  MemFieldStore
  MemAtomicCas
  MemAtomicExchange
  MemAtomicFetchAdd
  AddressToken
  PageKeyMake
  LogicalShrExact
  BitMaskExact
  PageMapBridgeLookup
  AtomicRemotePush
  AtomicRemoteDrain

generated C replacement front only:
  uintptr_t addr
  shift/mask constants
  table pointer
  CAS loop
  metadata pointer
  TLS arena pointer
```

## Current Reading For Pro Consultation

The question to ask is not:

```text
Should Hakorune add C-style pointer syntax?
```

The better question is:

```text
What is the smallest contract-bound fast memory sublanguage that lets `.hako`
express mimalloc-style address-derived page maps and remote-free paths without
opening general unsafe pointer arithmetic or making Type ABI / Provider ABI hot?
```
