---
Status: SSOT
Decision: accepted
Date: 2026-06-05
Scope: inventory of language, MIR, runtime, and tooling gaps that block a faithful `.hako` mimalloc-style port.
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

The contract name is mandatory even while v0 checks do not branch per contract.
In v0, `PageMapV0` is a stable report/discovery id and future allowlist key,
not a broad type-system feature. Do not infer it from the enclosing method name:
renames must not silently change the memory contract.

`@rune FastMemory(PageMapV0)` may remain as optional declaration metadata only.
It can say that a method/package participates in FastMemory work, but it does
not create a `FastMemRegion`, does not widen the method body into a fastmem
region, and is not a substitute for `fastmem ContractName { ... }`.

The region may use convenient memory-shaped operations, but only if the
compiler can classify them:

```text
allowed MemOp families:
  MemOpKind::AddrOf
  MemOpKind::Add / MemOpKind::Sub
  MemOpKind::LogicalShr / MemOpKind::BitAnd
  MemOpKind::TableIndex
  MemOpKind::FieldLoad / MemOpKind::FieldStore
  future MemOpKind::TypedLoad / MemOpKind::TypedStore
  future MemOpKind::AtomicCas / MemOpKind::AtomicExchange / MemOpKind::AtomicFetchAdd

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
explicit memory operations and report them.

Post `MIR-FMEM-001` decision:

```text
MemOp:
  the single executable MIR instruction for fast memory dialect operations

MemOpKind:
  the dialect vocabulary

FastMemRegion:
  side-table metadata / contract truth

MemOp.region:
  carries FastMemRegionId
```

Do not model `FastMemRegionBegin` / `FastMemRegionEnd` as normal MIR
instructions. They are presentation-only debug comments at most. The v0
MIR/report vocabulary is:

```text
MemOpKind::AddrOf:
  user pointer -> MemAddr

MemOpKind::LogicalShr / MemOpKind::BitAnd:
  address/page-key arithmetic
  shift count and mask must be verifier-visible

MemOpKind::Add / MemOpKind::Sub:
  region-local address arithmetic only

MemOpKind::TableIndex:
  verified table/key lookup

MemOpKind::FieldLoad / MemOpKind::FieldStore:
  verified layout field access

MemOpKind::CurrentAllocOwnerId / MemOpKind::OwnerEq:
  allocator-local owner identity observation and equality check
```

Atomic operations remain a later dialect extension. The report vocabulary must
be present before execution lowering is accepted:

```text
fastmem_region_metadata_table
fastmem_region_instruction_markers
fastmem_region_count
fastmem_memop_instruction_enabled
fastmem_memop_kind_allowlist_v0
fastmem_memop_count
fastmem_unknown_memop_kind_count
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

## TypedPageMetaHandle Plan

`TypedPageMetaHandle` is the layout-verified metadata capability returned by
`PageMapBridge`. It is not a raw metadata pointer. It is the named handle that
allows allocator fast paths to read owner/size metadata and operate on page-local
free-list fields through a verified layout.

The v0 layout contract is:

```text
PageMetaLayoutV0:
  owner_worker_id
  block_size
  free_head
  local_free_head
  remote_head
  capacity
  used
```

Report invariants:

```text
typed_page_meta_handle=1
typed_page_meta_layout_verified=1
typed_page_meta_layout_id=PageMetaLayoutV0
typed_page_meta_layout_hash=<hash>
typed_page_meta_field_count=7
typed_page_meta_required_field_missing_count=0
fastmem_layout_verified=1
fastmem_unverified_offset_load_count=0
```

Stop line:

```text
TypedPageMetaHandle does not allow arbitrary offset loads.
TypedPageMetaHandle does not escape fastmem/replacement-front metadata scope.
TypedPageMetaHandle does not imply product allocator activation.
```

## Safe Capability Wrapper Plan

Safe capability wrappers are the readable, higher-level surface over the same
FastMemory MemOps. They do not introduce a second execution path.

```text
AddressToken:
  no-escape address fact
  no dereference
  no general pointer arithmetic

PageKey:
  exact address-width shift/mask result

PageMapBridge:
  PageKey / MemAddr-derived key -> typed page metadata

PageMetaHandle:
  layout-verified PageMeta capability

AllocOwnerId:
  allocator arena owner identity

AtomicRemoteHead:
  page-local remote-free atomic head capability
```

The wrapper route must lower to the same MemOps as the direct `fastmem` region:

```text
safe_capability_wrapper_route=fastmem_memop_alias
safe_capability_wrapper_lowering_route=fastmem_memop_alias
safe_capability_wrapper_memop_equivalence=1
safe_capability_wrapper_rawptr_surface=0
safe_capability_wrapper_deref_surface=0
safe_capability_wrapper_escape_count=0
```

Stop line:

```text
RawPtr<T> remains closed.
Pointer arithmetic outside fastmem remains closed.
Address dereference syntax remains closed.
Type ABI hot lookup remains closed.
Provider ABI replacement-front dispatch remains closed.
Product allocator activation remains closed.
```

## Mimalloc Shape Coverage Score Plan

Speed is not enough to promote a replacement-front route. A route can be fast
because it is a small native allocator or a shortcut, while still failing to
match the mimalloc shape Hakorune is trying to port. `MIM-FMEM-016` therefore
separates score families:

```text
mimalloc_speed_score:
  throughput interpretation only

mimalloc_shape_score:
  structural mimalloc-shape evidence

mimalloc_safety_score:
  boundary/safety evidence

mimalloc_coverage_score:
  required coverage evidence for keeper candidacy
```

The shape score is component based. Each component is worth 10 points:

```text
mimalloc_shape_component_page_map_bridge
mimalloc_shape_component_typed_page_meta
mimalloc_shape_component_tls_arena
mimalloc_shape_component_alloc_owner
mimalloc_shape_component_owner_check
mimalloc_shape_component_same_owner_local_free
mimalloc_shape_component_atomic_remote_head
mimalloc_shape_component_safe_wrappers
mimalloc_shape_component_no_global_lock_hot_path
mimalloc_shape_component_no_range_scan_hot_path
```

Keeper gating is explicit. Ordinary inventory reports do not fail just because
the shape score is low; the stricter gate opens only when a report marks itself
as a keeper candidate:

```text
mimalloc_keeper_candidate=0|1
mimalloc_keeper_eligible=0|1
mimalloc_keeper_block_reason=not_candidate|shape_below_threshold|safety_below_threshold|coverage_below_threshold|eligible
mimalloc_shape_threshold=<0..100>
mimalloc_safety_threshold=<0..100>
mimalloc_coverage_threshold=<0..100>
```

Accepted default thresholds:

```text
mimalloc_shape_threshold=80
mimalloc_safety_threshold=100
mimalloc_coverage_threshold=80
```

Stop line:

```text
throughput alone cannot set mimalloc_keeper_eligible=1
product activation remains closed
hook install remains closed
global allocator claim remains closed
winner claim remains closed
Type ABI hot lookup remains closed
Provider ABI replacement-front dispatch remains closed
source syntax remains unchanged in this row
Rust-only parser behavior remains rejected
```

## Product-Shaped Replacement Front Bridge Plan

`MIM-FMEM-017` is report/check only. It does not open product activation or
change execution. The active evidence surface is split into two compact
bridge checks:

- `MIM-FMEM-017B`: size-class bridge
  - source truth: `lang/src/hako_alloc/memory/size_class_box.hako`
  - proves the replacement-front mirror is tied to `.hako` size-class policy
  - covers regular-bin constants, huge-bin sentinel, and required methods
- `MIM-FMEM-017C`: page-local bridge
  - source truth: `lang/src/hako_alloc/memory/page_box.hako`
  - proves page-local metadata and same-owner local-free evidence are tied to
    `.hako` page state
  - covers typed metadata, counters, free heads, and lifecycle methods

Active invariants:

```text
report_only=1
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
remote_free_execution=closed
provider_type_abi_hot_lookup=closed
```

See the archived task ledger for the full field inventory.

## Replacement Front Producer Transition Plan

`hako_alloc` is the mimalloc `.hako` body, not a separate allocator family.
`python_template_c_bridge` is the explicit diagnostic baseline until the
MIR/LLVM producer reaches parity. `mir_to_c_lowering` remains optional backend
artifact output.

Current roles:

```text
MIRBuilder:
  emit MemOp + FastMemRegion metadata only
  preserve span/region/contract identity
  do not choose producer or route

Planner:
  choose producer/route plan

Verifier:
  enforce layout, escape, and ABI boundaries

Lowering:
  consume the selected plan
```

Keep the historical row-by-row producer transition ledger in the archive note.

## AllocOwnerId / TLS Arena Owner State Plan

`AllocOwnerId` is the allocator-local owner identity. It is distinct from OS
thread id, runtime worker id, and `.hako` task id.

Active rules:

```text
slot+generation representation
no escape
equality-only on hot path
zero means unowned/invalid
same/remote/unowned/stale/invalid counts stay observation-only here
```

Keep the detailed owner-state row inventory in the archive note.

## Recommended Implementation Order

1. Lock the active contract surface: `FastMemoryContractV0`, `FastMemRegionV0`,
   `MemOpAllowlistV0`, `MemPointerClassV0`, and `MemLayoutContractV0`.
2. Keep `MemOp` as the single executable MIR instruction and leave
   `FastMemRegion` as side-table metadata.
3. Implement verifier gates before accepting source syntax.
4. Lock page-key arithmetic, `PageMapBridge`, and owner-state observation rows.
5. Add source syntax and remote-free execution only after the above gates are
   stable.

## Task Breakdown

The detailed row-by-row ledger is archived here:

```text
docs/development/current/main/investigations/mimalloc-hako-port-capability-gap-inventory-task-ledger-2026-06-08.md
```

Keep the active SSOT focused on the current decision surface and the minimal
next implementation order.

## Report Fields For `MIM-FMEM-002`

The exhaustive field inventory is archived to the ledger note above. The active
SSOT only needs to remember that `MIM-FMEM-002` is report/check-only and that
inventory fields remain observation-only.

## Source/MIR/Lowering Visibility

Keep the layer split compact:

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

MIR / plan only:
  FastMemRegion metadata side table
  MirInstruction::MemOp
  MemOpKind::AddrOf
  MemOpKind::LogicalShr
  MemOpKind::BitAnd
  MemOpKind::TableIndex
  MemOpKind::FieldLoad
  MemOpKind::FieldStore
  future MemOpKind::AtomicCas
  future MemOpKind::AtomicExchange
  future MemOpKind::AtomicFetchAdd
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

The active question is narrow:

```text
What is the smallest contract-bound fast memory sublanguage that lets `.hako`
express mimalloc-style address-derived page maps and remote-free paths without
opening general unsafe pointer arithmetic or making Type ABI / Provider ABI hot?
```
