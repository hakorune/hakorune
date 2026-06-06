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

`MIM-FMEM-017` starts as a report/check bridge, not a product activation row.
The goal is to connect `.hako` allocator policy/state truth to the
replacement-front descriptor surface without changing malloc/free execution.

First source truth:

```text
hako_alloc_size_class_source=lang/src/hako_alloc/memory/size_class_box.hako
```

This is intentionally narrower than page metadata, TLS owner residence, remote
free execution, or segment backing. `SizeClassBox` is the most stable first
policy source because it owns `size_to_bin`, `bin_size`, `good_size`, regular
bin count, and huge-bin classification.

Normalized bridge fields:

```text
replacement_front_product_shaped_bridge_v0=0|1
replacement_front_product_shaped_bridge_non_activating=1
replacement_front_product_shaped_bridge_report_only=1
replacement_front_product_shaped_bridge_route=replacement_front_benchmark_to_product_ldpreload_descriptor
replacement_front_product_shaped_bridge_source_truth=hako_alloc.size_class_box|unknown
replacement_front_product_shaped_bridge_evidence_ready=0|1
replacement_front_product_shaped_bridge_activation_ready=0
replacement_front_product_shaped_bridge_block_reason=...
replacement_front_product_shaped_bridge_missing=...

replacement_front_product_shaped_bridge_shape_ok=0|1
replacement_front_product_shaped_bridge_safety_ok=0|1
replacement_front_product_shaped_bridge_coverage_ok=0|1
replacement_front_product_shaped_bridge_preflight_ok=0|1

replacement_front_product_shaped_bridge_no_type_abi_hot_lookup=0|1
replacement_front_product_shaped_bridge_no_provider_dispatch=0|1
replacement_front_product_shaped_bridge_no_global_lock_hot_path=0|1
replacement_front_product_shaped_bridge_no_range_scan_hot_path=0|1
replacement_front_product_shaped_bridge_no_host_passthrough=0|1

replacement_front_product_shaped_bridge_requires_activation_row=1
replacement_front_product_shaped_bridge_requires_product_gate_open=1
```

`MIM-FMEM-017B` adds the first concrete bridge evidence below the
product-shaped bridge. It is still report/check-only and proves only that the
replacement-front size-class mirror is tied to `.hako` `SizeClassBox` policy.

SizeClassBox bridge fields:

```text
replacement_front_size_class_bridge_v0=0|1
replacement_front_size_class_bridge_report_only=1
replacement_front_size_class_bridge_source_truth=hako_alloc.size_class_box|unknown
replacement_front_size_class_bridge_source_file=lang/src/hako_alloc/memory/size_class_box.hako
replacement_front_size_class_bridge_mirror_source=hako_size_class_box_report_mirror|...
replacement_front_size_class_bridge_bound=0|1
replacement_front_size_class_bridge_missing=...

replacement_front_size_class_required_method_count=<n>
replacement_front_size_class_required_methods_present=0|1
replacement_front_size_class_missing_methods=none|...
replacement_front_size_class_word_size=8
replacement_front_size_class_max_regular_bin=72
replacement_front_size_class_huge_bin=73
replacement_front_size_class_huge_sentinel=-1
replacement_front_size_class_usize_facades_present=0|1

replacement_front_size_class_policy_methods_covered=0|1
replacement_front_size_class_policy_constants_covered=0|1
replacement_front_size_class_policy_huge_sentinel_covered=0|1
replacement_front_size_class_policy_mirror_matches_source=0|1
```

`replacement_front_size_class_bridge_bound=1` means:

```text
source file exists
required SizeClassBox methods are present
word_size/max_regular_bin/huge_bin constants match the source contract
good_size uses -1 as the huge sentinel
usize facades are present
replacement-front mirror source normalizes to hako_alloc.size_class_box
```

It does not mean:

```text
product bins are activated
page metadata is product-owned
remote-free behavior is complete
the replacement front is a full .hako mimalloc algorithm
```

`MIM-FMEM-017C` adds page-local state bridge evidence. It is still
report/check-only and proves that product-shaped page metadata / same-owner
local-free evidence is tied to `.hako` `HakoAllocPageModel` page-local state.

Page-local bridge fields:

```text
replacement_front_page_local_bridge_v0=0|1
replacement_front_page_local_bridge_report_only=1
replacement_front_page_local_bridge_source_truth=hako_alloc.page_box|unknown
replacement_front_page_local_bridge_source_file=lang/src/hako_alloc/memory/page_box.hako
replacement_front_page_local_bridge_mirror_source=hako_page_box_report_mirror|...
replacement_front_page_local_bridge_bound=0|1
replacement_front_page_local_bridge_missing=...

replacement_front_page_local_required_field_count=<n>
replacement_front_page_local_required_fields_present=0|1
replacement_front_page_local_missing_fields=none|...
replacement_front_page_local_required_method_count=<n>
replacement_front_page_local_required_methods_present=0|1
replacement_front_page_local_missing_methods=none|...

replacement_front_page_local_directarray_fields_present=0|1
replacement_front_page_local_counter_fields_present=0|1
replacement_front_page_local_acquire_release_methods_present=0|1
replacement_front_page_local_lifecycle_methods_present=0|1
replacement_front_page_local_typed_meta_matches_source=0|1
replacement_front_page_local_same_owner_route_matches_source=0|1
replacement_front_page_local_no_remote_free_claim=1
```

`replacement_front_page_local_bridge_bound=1` means:

```text
source file exists
required HakoAllocPageModel page-local fields are present
free/local_free/block_used are DirectArrayI64-backed fields
page-local counters/free heads are present
acquire/releaseLocal/releaseLocalKnownLive methods are present
typed page metadata exposes block_size/free/local_free/capacity/used evidence
same-owner local-free route evidence maps to page-local release shape
```

It does not mean:

```text
remote-free execution is complete
remote_head has .hako page-local source truth
segment backing is connected
product allocator activation is open
```

## Replacement Front Producer Transition Plan

Identity boundary:

```text
hako_alloc:
  .hako body/source truth of the mimalloc port
  not a separate allocator family

replacement_front C shim:
  temporary execution bridge for the same mimalloc port
  not the final semantic producer

runtime/bootstrap allocator:
  allocator used to run/build Hakorune itself
  separate from the application/product allocator under construction
```

Read the full naming and role split here:

```text
docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md
```

The current replacement front is a safe bridge, not the final producer. The
long-term goal is to remove Python-template C as a semantic producer while
keeping the producer-neutral `report.kv` / `hako_check` contract.

Producer roles:

```text
python_template_c_bridge:
  current bridge
  allowed only while product activation is closed
  must be tied to .hako source truth by bridge evidence
  must not become semantic SSOT
  retirement required
  retained through MIR-FMEM-005 as baseline evidence
  retired only after MIR-FMEM-006 producer-neutral parity

mir_to_c_lowering:
  optional debug/diff/bootstrap artifact producer
  C may exist, but only as backend artifact from MIR/FastMem lowering
  semantics live in .hako fastmem/capability surface and MIR MemOps
  not required before the primary LLVM/object producer

mir_to_llvm_lowering:
  primary product producer
  no C in the primary execution path
  same counters/report.kv contract as other producers
```

Producer-neutral fields:

```text
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=python_template_c_bridge|mir_to_c_lowering|mir_to_llvm_lowering
replacement_front_backend_artifact=c|llvm_ir|object|exe
replacement_front_source_truth=hako_fastmem|hako_alloc.size_class_box|hako_alloc.page_box|unknown
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=0|1
python_template_c_bridge_runtime_dependency_count
producer_neutral_report_schema=0|1
producer_neutral_parity_pass=0|1
replacement_front_mir_memop_enabled=0|1
replacement_front_mir_fastmem_region_enabled=0|1
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_transition_state=current_bridge|primary_llvm_transition|optional_c_artifact|final_primary
hako_alloc_mimalloc_port_identity=hako_alloc_is_mimalloc_hako_body
runtime_allocator_role=bootstrap_host_allocator
application_allocator_role=hako_alloc_mimalloc_port
hako_alloc_product_activation=0
```

Allowed C:

```text
MIR / FastMem lowering -> C backend artifact
debug/bootstrap/diff C backend
```

Rejected C:

```text
Python template C as allocator semantic truth
C-only sizeclass/page/remote-free policy
C-only keeper route without .hako/MIR source evidence
producer-specific report.kv schema
```

MIRBuilder boundary:

```text
MIRBuilder:
  emits MemOp instructions and FastMemRegion side-table metadata
  preserves source span and region identity
  records contract id on the region metadata
  does not choose C vs LLVM
  does not choose page-map route
  does not claim product readiness

Planner:
  chooses producer/route plan

Verifier:
  enforces fastmem contract, layout, escape, and ABI boundaries

Lowering:
  consumes the selected plan and emits C/LLVM/object artifacts

hako_check:
  verifies producer-neutral evidence from report.kv
```

MIRBuilder design consultation should happen before adding FastMem/MemOp
execution lowering. The consultation question is not whether Python-template C
is final; that is rejected. The question is how to represent FastMemRegion /
MemOp in MIR without letting MIRBuilder choose routes.

Candidate task split:

```text
MIM-FMEM-017C:
  Page-local state bridge evidence on the current python_template_c_bridge.

MIM-FMEM-017D:
  Add replacement_front_producer fields to report/check.
  No execution behavior change.

LLVM-PIPE-001:
  Inventory/report the current LLVM runner pipeline debt separately from
  replacement-front producer taxonomy:
    NYASH_REWRITE_FUTURE env forcing
    method_id_injector no-op mutation seam
    joinir_experiment hook/fallback
    pyvm/harness/mock fallback route visibility

LLVM-PIPE-002:
  Add pipeline/report fields:
    mir_future_rewrite_route
    pipeline_joinir_experiment_enabled
    method_id_injector_mutation_count
    execution_backend
    llvm_fallback_used
    llvm_fallback_reason

LLVM-PIPE-003:
  Move env side effects and runner ad-hoc stages toward
  CompileOptions / PipelinePlan / LoweringPlan.

MIR-FMEM-001:
  MIRBuilder FastMemRegion/MemOp design consultation and docs.
  Representation only; no lowering behavior.
  Decision: MemOp single instruction, MemOpKind dialect, FastMemRegion
  side-table metadata. No FastMemRegionBegin/End MIR instructions.

MIR-FMEM-002:
  mir/contracts `MemOp` instruction tag, MemOpKind allowlist, and JSON
  vocabulary for region metadata plus MemOp payloads.

MIR-FMEM-003:
  MIRBuilder source lowering to FastMemRegion/MemOp metadata.

MIR-FMEM-004:
  Verifier gates for fastmem escape/layout/ABI boundaries.

MIR-FMEM-005:
  MIR -> LLVM/object primary producer.
  Keep python_template_c_bridge as comparison baseline.

MIR-FMEM-006:
  Producer-neutral parity against the current python_template_c_bridge.

MIR-FMEM-007:
  Retire python_template_c_bridge after producer-neutral parity is proven.
  Do not leave a hidden fallback to the Python-template C producer.

MIR-FMEM-C-ARTIFACT:
  Optional MIR -> C debug/diff/bootstrap artifact producer.
  C remains a backend artifact, not a required product path.
```

Required blocker semantics while activation is closed:

```text
replacement_front_product_shaped_bridge_activation_ready=0
replacement_front_product_shaped_bridge_missing includes product_gate_open
replacement_front_product_shaped_bridge_missing includes activation_row
```

Stop line:

```text
MIM-FMEM-017A does not change generated C behavior.
MIM-FMEM-017A does not add source syntax.
MIM-FMEM-017A does not change the Rust parser or .hako parser.
Product bins/pages may become described, but product activation stays closed.
The bridge is not a full .hako mimalloc algorithm claim.
```

## AllocOwnerId / TLS Arena Owner State Plan

The next owner-state boundary is `AllocOwnerId`, not a source-level thread
identity.

```text
Host thread / pthread:
  host execution source

RuntimeWorkerId:
  future .hako scheduler/worker identity

AllocOwnerId:
  allocator-local TLS arena / page owner identity
```

`AllocOwnerId` is the semantic name for the value currently reported through
legacy-compatible `worker_id_*` fields. `PageMeta.owner_worker_id` stores an
`AllocOwnerId`; the field name stays for layout compatibility, but the meaning
is allocator arena owner identity.

Identity invariants:

```text
AllocOwnerId:
  allocator-local
  process-local
  run-stable claim = 0
  OS thread id claim = 0
  runtime worker id claim = 0
  .hako task id claim = 0
  no-escape
  equality check only on the hot path

0:
  unowned / invalid owner

nonzero:
  allocator TLS arena owner
```

Preferred representation:

```text
AllocOwnerId:
  slot: u32
  generation: u32
```

The generation is part of the plan from the beginning so arena reuse does not
silently look like the same owner.

MIM-FMEM-011 introduces owner truth, not remote-free behavior:

```text
free(ptr)
  -> PageMapBridge
  -> TypedPageMetaHandle
  -> current AllocOwnerId
  -> compare page.owner_worker_id
  -> count same / remote / unowned / stale / invalid
```

Actual same-owner local-free routing and remote `AtomicRemoteHead` mutation are
later rows. If remote owner is observed before `AtomicRemoteHead` is ready, it
must not be pushed to `local_free`; it remains a candidate/fallback observation.

Report vocabulary:

```text
alloc_owner_id_capability=0|1
alloc_owner_id_kind=allocator_arena_owner|unknown
alloc_owner_id_source=benchmark_c_pthread_tls|hako_runtime_worker_tls|unknown
alloc_owner_id_width_bits=64
alloc_owner_id_generation_enabled=0|1
alloc_owner_id_zero_is_unowned=1
alloc_owner_id_escape_count

worker_id_capability=0|1
worker_id_kind=allocator_arena_owner|unknown
worker_id_source=benchmark_c_pthread_tls|hako_runtime_worker_tls|unknown
worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0
worker_id_escape_count

allocator_tls_arena_enabled=0|1
allocator_tls_arena_mode=benchmark_c_tls|hako_runtime_tls|unknown
allocator_tls_arena_init_count
allocator_tls_arena_live_count
allocator_tls_arena_peak_count
allocator_tls_arena_reuse_count
allocator_tls_arena_init_fail_count
allocator_tls_arena_fallback_count
allocator_thread_exit_flush_supported=0|1
allocator_thread_exit_flush_count
allocator_abandoned_owner_count

page_owner_check_enabled=0|1
page_owner_check_route=page_meta_owner_worker_id|none
page_owner_check_count
page_owner_same_count
page_owner_remote_count
page_owner_unowned_count
page_owner_stale_generation_count
page_owner_invalid_count

same_owner_free_local_candidate_count
same_owner_free_local_push_count
same_owner_free_local_fallback_count
remote_owner_free_remote_candidate_count
remote_owner_free_remote_push_count
remote_owner_free_fallback_lock_count
```

Consistency invariant:

```text
page_owner_check_count
  == page_owner_same_count
   + page_owner_remote_count
   + page_owner_unowned_count
   + page_owner_stale_generation_count
   + page_owner_invalid_count
```

Boundary invariants:

```text
benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
hako_mimalloc_algorithm_claim=0
replacement_front_is_full_hako_algorithm=0
```

Fail-fast conditions for keeper work:

```text
worker_id_kind != allocator_arena_owner
worker_id_escape_count > 0
worker_id_equals_os_thread_id_claim != 0
worker_id_equals_runtime_worker_id_claim != 0
worker_id_equals_hako_task_id_claim != 0
allocator_tls_arena_enabled=1 and allocator_tls_arena_init_count=0
allocator_tls_arena_init_fail_count > 0
page_owner_check_enabled != 1 for owner-state profiles
page_owner_check_count == 0 for mixed-ws/free-path profiles
page_owner_check_count != same + remote + unowned + stale + invalid
page_owner_unowned_count > 0 unless route explicitly permits bootstrap pages
page_owner_stale_generation_count > 0
remote owner enters local_free
```

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
   - Add `MemOp` as the single executable MIR instruction and keep
     `FastMemRegion` as side-table metadata.
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
   - Add `fastmem ContractName { ... }` only after the MemOp/verifier contract,
     first bridge plan, and parser parity gate are stable.
   - Contract name is mandatory.
   - `@rune FastMemory(ContractName)` is metadata-only and never a region
     boundary.
   - Initial row is parse-only; lowering/execution stays closed until a later
     implementation row.

9. **AllocOwnerId / TLS arena owner state**
   - Bind allocator owner identity to TLS arena/page ownership.
   - Keep `AllocOwnerId`, OS thread id, runtime worker id, and `.hako` task id
     separate.
   - Count same/remote/unowned/stale/invalid owner-check outcomes before
     changing local/remote free behavior.
   - Keep source-level concurrency claims separate from C pthread benchmark
     evidence.

10. **Remote-free plan and pilot**
   - Define `AtomicRemoteHead` push/drain semantics with counters.
   - Pilot `AtomicRemoteHead` only after owner-state rows are visible.

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
| `MIM-FMEM-003 MIR MemOp region docs/report plan` | done | Name the early FastMemory MemOp report vocabulary. | Superseded by `MIR-FMEM-001` for final MIR representation: `MemOp` instruction + `MemOpKind` dialect + `FastMemRegion` side table. |
| `MIM-FMEM-004 FastMem verifier implementation` | done | Implement region verifier for MemOp allowlist, no-escape values, and forbidden operations. | Unclassified memory behavior, allocation, safepoint, await/nowait, arbitrary calls, Type ABI lookup, and Provider ABI dispatch are rejected. |
| `MIM-FMEM-005 PageKey exact route docs/report lock` | done | Name `PageKeyExactRoutePlanV0`: exact `usize/u64` logical shift, mask, shift-count trap, address-width facts. | Report vocabulary and fail-fast expectations are documented before code; plain `>>` semantics remain unchanged outside fastmem. |
| `MIM-FMEM-006 PageKey exact route implementation` | done | Implement the narrow exact numeric route needed for allocator page-key derivation. | Route facts and backend/reference behavior agree; invalid shift/range traps are observable. |
| `MIM-FMEM-007 PageMapBridge plan` | done | Define `PageMapBridgePlanV0`: fastmem/PageKey -> PageMetaHandle via typed side-table/table route. | Type ABI hot lookup count is zero; Provider ABI hot dispatch count is zero; range-scan replacement is explicit. |
| `PARSER-FMEM-001 parser parity inventory contract` | done | Freeze the Rust/.hako parser gap list that blocks fastmem source syntax. | No source syntax behavior change; Rust-only active grammar remains rejected. |
| `PARSER-FMEM-002 parser parity gate surface` | done | Define one reusable probe/smoke entry for Rust parser and `.hako` parser parse-only parity. | Gate is reusable and does not add lowering/runtime behavior. |
| `PARSER-FMEM-003 general bitwise/shift expression parity` | done | Catch up `.hako` parser expression parse for `<< >> & | ^` outside static const-only use. | Fastmem examples can parse shift/mask expressions without externcall escape. |
| `PARSER-FMEM-004 rune contract-name parity` | done | Catch up `.hako` parser rune metadata names needed by current Rust parser surfaces. | Rune metadata remains parse/noop unless a separate consumer row opens behavior. |
| `PARSER-FMEM-005 fastmem block parse-only dual parser pilot` | done | Add `fastmem IDENT { ... }` parse-only surface to both parsers. | Execution/lowering stay closed; contract name is mandatory. |
| `PARSER-FMEM-006 fastmem contractless fail-fast parity` | done | Reject `fastmem { ... }` and `unsafe { ... }` in both parsers. | Contract-less unsafe escape remains closed. |
| `MIM-FMEM-008 fastmem source syntax pilot` | done | Connect parser output to MIR MemOp region metadata after `PARSER-FMEM-001..006` proved dual parser parse-only parity. | Source-derived fastmem inventory/check works; contract-less `unsafe` / `fastmem` remains rejected; execution/lowering beyond metadata stays closed. |
| `MIM-FMEM-009 PageMapBridge benchmark-front pilot` | done | Replace the current free-path page lookup shape with the selected bridge in generated C replacement-front evidence. | `free_path_page_lookup_route != range_scan`; report keeps product activation and hako algorithm claim closed. |
| `MIM-FMEM-010 TypedPageMetaHandle plan` | done | Define metadata capability for owner, size, free/local_free/remote_head access. | Page metadata stays layout-verified; unverified offset loads are counted and rejected for keeper work. |
| `MIM-FMEM-011A AllocOwnerId / TLS owner-state schema` | done | Define `AllocOwnerIdV0`, compatibility `worker_id_*` report fields, TLS arena owner-state fields, and page-owner check fields. | Owner identity is allocator-local; OS thread/runtime worker/.hako task equality claims stay zero. |
| `MIM-FMEM-011B fastmem-check owner-state gates` | done | Add fail-fast checks for owner identity kind/escape, TLS arena init failures, page-owner count consistency, and boundary claims. | Owner-state profiles cannot pass with missing owner checks, stale generation, unowned pages, Type ABI hot lookup, Provider ABI hot dispatch, or source thread-support claims. |
| `MIM-FMEM-011C replacement-front owner shadow counters` | done | Add generated C replacement-front shadow evidence for current `AllocOwnerId`, TLS arena init, page owner assignment, and same/remote/unowned/stale owner comparison counts. | Behavior remains observation-first; product activation, hooks, winner claims, remote CAS push, and full `.hako` algorithm claims stay closed. |
| `MIM-FMEM-012 same-owner local-free route` | done | Use owner truth to route same-owner frees to local-free where safe, while remote-owner frees stay fallback/locked until AtomicRemoteHead exists. | Same-owner local push counters are observable; remote-owner never enters local_free. |
| `MIM-FMEM-013 AtomicRemoteHead plan` | done | Define remote-free push/drain contract, memory-order vocabulary, and counters. | Remote-free is a page/fastmem capability, not a general `AtomicPtr<T>` surface. |
| `MIM-FMEM-014 AtomicRemoteHead pilot` | done | Pilot the remote-free push/drain route after owner-state and same-owner rows exist. | Push/drain counters are observable; product activation and winner claims stay closed. |
| `MIM-FMEM-015 safe capability wrapper plan` | done | Layer `AddressToken`, `PageKey`, `PageMapBridge`, `PageMetaHandle`, `AllocOwnerId`, and `AtomicRemoteHead` over MemOps. | Wrapper route lowers to the same MemOps as fastmem and does not reopen RawPtr. |
| `MIM-FMEM-016 Mimalloc shape coverage score` | done | Add speed/shape/safety/coverage separation to report acceptance. | Fast but non-mimalloc-shaped routes cannot become keeper by throughput alone. |
| `MIM-FMEM-017A Product-shaped bridge report normalization` | done | Normalize non-activating product-shaped bridge evidence and bind the first source truth to `SizeClassBox`. | Report/check only; activation, hook install, global allocator claim, and winner claim remain closed. |
| `MIM-FMEM-017B SizeClassBox bridge evidence` | done | Prove the replacement-front size-class mirror is formally tied to `.hako` `SizeClassBox` policy. | Product bins/pages execution remains benchmark-only; no page metadata or remote-free behavior change. |
| `MIM-FMEM-017C Page-local state bridge evidence` | done | Start connecting `PageBox` page-local shape to product-shaped metadata evidence after size-class truth is bound. | No activation; page-map/TLS/remote-free semantics remain explicit later rows. |
| `MIM-FMEM-017D Replacement-front producer taxonomy` | done | Add producer-neutral report fields that distinguish `python_template_c_bridge`, `mir_to_c_lowering`, and `mir_to_llvm_lowering`. | Report/check only; does not implement MIR lowering or remove the current bridge. |
| `LLVM-PIPE-001 LLVM runner pipeline debt inventory` | done | Report the current env rewrite, method-id seam, JoinIR experiment hook, and PyVM/harness/mock fallback visibility risks. | Static hako_check inventory only; PyVM remains diagnostic-reachable but daily route stays zero. |
| `LLVM-PIPE-002 LLVM runner pipeline report fields` | done | Add explicit pipeline report fields for future rewrite route, JoinIR experiment, method-id mutation count, backend executor, and fallback reason. | Opt-in runtime report via `NYASH_LLVM_PIPELINE_REPORT_OUT`; no route change. |
| `LLVM-PIPE-003 CompileOptions / PipelinePlan cleanup` | done | Move env side effects and runner ad-hoc stages toward explicit plan objects. | Current defaults flow through named `LlvmCompileOptions` / `LlvmPipelinePlan`; executor behavior unchanged. |
| `MIR-FMEM-001 MIRBuilder FastMemRegion/MemOp design consultation` | done | Lock the MIRBuilder representation boundary before implementing FastMem execution lowering. | `MemOp` is the single executable instruction, `MemOpKind` is the dialect, and `FastMemRegion` is side-table metadata. |
| `MIR-FMEM-002 mir/contracts FastMem MemOp vocabulary` | done | Add `MemOp` to MIR instruction contracts and add a `MemOpKind` allowlist surface. | Backend adapters cannot keep hidden MemOpKind allowlists; JSON/VM/LLVM/C support remains closed until dedicated rows. |
| `MIR-FMEM-003 MIRBuilder source lowering to FastMemRegion/MemOp metadata` | done | Connect parsed fastmem source to the new MIR representation metadata. | Function metadata now owns FastMemRegion rows; MIR instruction streams carry only MemOp operations. Backend support remains closed. |
| `MIR-FMEM-004 verifier gates for fastmem escape/layout/ABI boundaries` | done | Add verifier gates for no-escape, region metadata, MemOp kind/arity/effect shape, and ABI boundary escape bans. | Verifier guards landed; lowering remains closed until dedicated producer rows. |
| `MIR-FMEM-005 MIR-to-LLVM/object primary producer` | done | Lower verified value-only FastMemory MemOps to the primary LLVM/object producer path. | No C layer is required on the primary path; layout/table and allocator-owner runtime MemOps remain closed. |
| `MIR-FMEM-006 producer-neutral parity against python_template_c_bridge` | done | Compare MIR-to-LLVM evidence with the current temporary bridge using the same report.kv / hako_check contract. | Added `hako_check fastmem-producer-parity`; parity is evidence-only and does not delete the bridge. |
| `MIR-FMEM-007 Python template C bridge retirement first slice` | done | Remove the Python-template C semantic bridge from normal runtime entrypoints after producer-neutral parity is proven. | Replacement-front Python-template C generation now requires an explicit diagnostic baseline flag, and report producer inference no longer defaults `replacement_front_c_shim` to `python_template_c_bridge`. |
| `MIR-FMEM-007B Remaining Python template C quarantine/delete inventory` | done | Inventory remaining Python-template C bridge template/report/smoke files and remove or quarantine non-baseline runtime entrypoints. | Build helpers now use explicit diagnostic-baseline names and require the shared bridge guard. Optional MIR-to-C debug/diff artifact support remains separate and must be generated from MIR MemOps. |
| `MIR-FMEM-007C Python template C diagnostic import guard` | done | Add a lightweight static guard for diagnostic bridge imports. | Normal allocator tools must not import Python-template C payload modules directly; diagnostic payloads route through the bridge guard/build support boundary. |
| `MIR-FMEM-007D Python template C diagnostic payload keep/archive decision` | done | Decide whether the remaining diagnostic payloads stay until allocator-owner layout/table MemOps cover replacement-front behavior, or whether fixed-slot-only payloads can be archived first. | Keep all remaining payloads quarantined for now; do not delete parity/report fixtures before MIR-to-LLVM replacement-front evidence can replace their baseline role. |
| `MIR-FMEM-C-ARTIFACT optional MIR-to-C debug/diff artifact` | deferred | Optionally emit C from MIR MemOps for debug, diff, or bootstrap inspection. | This is not a required product path and must not become semantic SSOT. |
| `MIM-FMEM-018A AllocOwner lifecycle state machine` | done | Define thread-exit / abandoned-owner lifecycle as AllocOwnerId page ownership truth, not just cleanup. | Persistent states are Active / ExitingFlush / Abandoned / Reclaimed; ReclaimAttempt is transient; AllocOwnerId is packed slot/generation from v0. |
| `MIM-FMEM-018B lifecycle report/check fields` | done | Add report schema and fastmem-check gates for owner lifecycle, generation reuse, stale detection, reclaim blocking, and boundary claims. | Bad lifecycle reports fail fast; behavior remains conservative and reclaim is not opened. |
| `MIM-FMEM-018C lifecycle shadow counters` | done | Add producer-side shadow evidence for thread-exit flush, owner state transitions, abandoned pages, and reclaim-block observations. | Counters observe lifecycle truth without enabling unsafe abandoned reclaim. |
| `MIM-FMEM-019 AtomicRemoteHead drain` | done | Implement remote-free drain after owner lifecycle truth can block unsafe reclaim. | Remote candidates can be handled before reclaim work opens. |
| `MIM-FMEM-020 abandoned reclaim` | done | Implement abandoned page reclaim with generation-safe owner transition and remote-drain preconditions. | Empty abandoned owner-page entries can transition to Reclaimed only after remote candidates are handled; TLS backing transfer remains closed. |
| `MIR-FMEM-008A producer-slice selection` | done | Select the smallest MIR-to-LLVM replacement-front producer slice after the diagnostic lifecycle bridge evidence is complete. | Selected layout/table MemOps (`TableIndex`, `FieldLoad`, `FieldStore`) first and deferred owner-runtime MemOps (`CurrentAllocOwnerId`, `OwnerEq`); report/check fields only, no lowering behavior change. |
| `MIR-FMEM-008B layout/table producer pilot` | done | Build verified layout/table access proofs for MemOps such as `TableIndex`, `FieldLoad`, and `FieldStore`. | Symbolic `MemOpAccess` ids, `fastmem_access_plans[]`, canonical layout/table contracts, and complete TableIndex proof checks are landed. This row did not open LLVM lowering. |
| `ContractRegionV0 docs-only` | done | Define the future common contract-region envelope without renaming current FastMemory code. | Commonize region/contract/obligation/verifier-report concepts only. `FastMemRegion` remains the memory-profile wrapper, and `MemOp` / `VerifiedMemAccessPlan` remain memory-specific. |
| `MIR-FMEM-008C layout/table LLVM producer pilot` | done | Open LLVM/object lowering for complete verified layout/table MemOps. | TableIndex lowers to backend-private LayoutRef, FieldLoad consumes LayoutRef into ordinary scalar values, FieldStore writes allowlisted mutable plain fields, and report/check requires positive lowered counts. Owner-runtime MemOps remain deferred. |
| `MIR-FMEM-008D owner-runtime producer pilot` | done | Open LLVM/object lowering for allocator owner runtime MemOps such as `CurrentAllocOwnerId` and `OwnerEq` plus matching report counters. | `CurrentAllocOwnerId` lowers to producer-local owner-id observation and `OwnerEq` lowers to equality only. TLS backing transfer, owner slot reuse as active owner, hooks, global allocator claim, and winner claim remain closed. |
| `MIR-FMEM-008E producer-neutral parity/readiness` | done | Prove MIR-to-LLVM layout/table/owner-runtime evidence can replace the quarantined Python-template C diagnostic baseline. | `fastmem-producer-parity` now has a candidate-only readiness profile requiring positive layout/table and owner-runtime lowered-count evidence. Reference closeout may run next; remaining payload deletion is still separate. |
| `FASTMEM-REFERENCE-CLOSEOUT-AFTER-PRODUCER-BODY-296X-001` | planned | Resync reference/current/tool docs after the MIR-FMEM layout/table/owner runtime producer body is implemented. | Retire stale Python-template C wording or mark it diagnostic-only; do not use this docs closeout to open product activation. |

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
typed_page_meta_layout_verified=0|1
typed_page_meta_layout_id=PageMetaLayoutV0|unknown
typed_page_meta_layout_hash=<hash|unknown>
typed_page_meta_field_count=<int>
typed_page_meta_required_field_missing_count
typed_page_meta_field_owner_worker_id=0|1
typed_page_meta_field_block_size=0|1
typed_page_meta_field_free_head=0|1
typed_page_meta_field_local_free_head=0|1
typed_page_meta_field_remote_head=0|1
typed_page_meta_field_capacity=0|1
typed_page_meta_field_used=0|1
typed_page_table_mode=none|side_table|segment_slices|compressed_index

alloc_owner_id_capability=0|1
alloc_owner_id_kind=allocator_arena_owner|unknown
alloc_owner_id_source=benchmark_c_pthread_tls|hako_runtime_worker_tls|unknown
alloc_owner_id_width_bits=64
alloc_owner_id_generation_enabled=0|1
alloc_owner_id_zero_is_unowned=1
alloc_owner_id_escape_count

worker_id_capability=0|1
worker_id_kind=allocator_arena_owner|unknown
worker_id_source=benchmark_c_pthread_tls|hako_runtime_worker_tls|unknown
worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0
worker_id_escape_count

allocator_tls_arena_enabled=0|1
allocator_tls_arena_mode=benchmark_c_tls|hako_runtime_tls|unknown
allocator_tls_arena_init_count
allocator_tls_arena_live_count
allocator_tls_arena_peak_count
allocator_tls_arena_reuse_count
allocator_tls_arena_init_fail_count
allocator_tls_arena_fallback_count
allocator_tls_arena_count
allocator_thread_exit_flush_supported=0|1
allocator_thread_exit_flush_count
allocator_abandoned_owner_count

page_owner_check_enabled=0|1
page_owner_check_route=page_meta_owner_worker_id|none
page_owner_check_count
page_owner_same_count
page_owner_remote_count
page_owner_unowned_count
page_owner_stale_generation_count
page_owner_invalid_count
same_owner_free_local_candidate_count
same_owner_free_local_push_count
same_owner_free_local_fallback_count
remote_owner_free_remote_candidate_count
remote_owner_free_remote_push_count
remote_owner_free_fallback_lock_count

atomic_remote_head_enabled=0|1
remote_free_push_count
remote_free_drain_count
remote_free_cas_retry_count
remote_free_memory_order=missing|acq_rel|release_acquire

mimalloc_shape_page_free_lists=missing|free_only|free_local_remote
mimalloc_shape_thread_local_heap=0|1
mimalloc_shape_segment_slice_lookup=0|1
mimalloc_shape_component_count=<0..10>
mimalloc_shape_component_page_map_bridge=0|1
mimalloc_shape_component_typed_page_meta=0|1
mimalloc_shape_component_tls_arena=0|1
mimalloc_shape_component_alloc_owner=0|1
mimalloc_shape_component_owner_check=0|1
mimalloc_shape_component_same_owner_local_free=0|1
mimalloc_shape_component_atomic_remote_head=0|1
mimalloc_shape_component_safe_wrappers=0|1
mimalloc_shape_component_no_global_lock_hot_path=0|1
mimalloc_shape_component_no_range_scan_hot_path=0|1
mimalloc_speed_score=<0..100>
mimalloc_shape_score=<0..100>
mimalloc_safety_score=<0..100>
mimalloc_coverage_score=<0..100>
mimalloc_shape_threshold=<0..100>
mimalloc_safety_threshold=<0..100>
mimalloc_coverage_threshold=<0..100>
mimalloc_keeper_candidate=0|1
mimalloc_keeper_eligible=0|1
mimalloc_keeper_block_reason=not_candidate|shape_below_threshold|safety_below_threshold|coverage_below_threshold|eligible
safety_score=<0..100>
coverage_score=<0..100>

replacement_front_product_shaped_bridge_v0=0|1
replacement_front_product_shaped_bridge_non_activating=1
replacement_front_product_shaped_bridge_report_only=1
replacement_front_product_shaped_bridge_route=replacement_front_benchmark_to_product_ldpreload_descriptor|none
replacement_front_product_shaped_bridge_source_truth=hako_alloc.size_class_box|unknown
replacement_front_product_shaped_bridge_evidence_ready=0|1
replacement_front_product_shaped_bridge_activation_ready=0
replacement_front_product_shaped_bridge_block_reason=<stable reason>
replacement_front_product_shaped_bridge_missing=<comma list>
replacement_front_product_shaped_bridge_shape_ok=0|1
replacement_front_product_shaped_bridge_safety_ok=0|1
replacement_front_product_shaped_bridge_coverage_ok=0|1
replacement_front_product_shaped_bridge_preflight_ok=0|1
replacement_front_product_shaped_bridge_no_type_abi_hot_lookup=0|1
replacement_front_product_shaped_bridge_no_provider_dispatch=0|1
replacement_front_product_shaped_bridge_no_global_lock_hot_path=0|1
replacement_front_product_shaped_bridge_no_range_scan_hot_path=0|1
replacement_front_product_shaped_bridge_no_host_passthrough=0|1
replacement_front_product_shaped_bridge_requires_activation_row=1
replacement_front_product_shaped_bridge_requires_product_gate_open=1

replacement_front_size_class_bridge_v0=0|1
replacement_front_size_class_bridge_report_only=1
replacement_front_size_class_bridge_source_truth=hako_alloc.size_class_box|unknown
replacement_front_size_class_bridge_source_file=lang/src/hako_alloc/memory/size_class_box.hako
replacement_front_size_class_bridge_mirror_source=hako_size_class_box_report_mirror|unknown
replacement_front_size_class_bridge_bound=0|1
replacement_front_size_class_bridge_missing=<comma list>
replacement_front_size_class_required_method_count=<n>
replacement_front_size_class_required_methods_present=0|1
replacement_front_size_class_missing_methods=none|<comma list>
replacement_front_size_class_word_size=8
replacement_front_size_class_max_regular_bin=72
replacement_front_size_class_huge_bin=73
replacement_front_size_class_huge_sentinel=-1
replacement_front_size_class_usize_facades_present=0|1
replacement_front_size_class_policy_methods_covered=0|1
replacement_front_size_class_policy_constants_covered=0|1
replacement_front_size_class_policy_huge_sentinel_covered=0|1
replacement_front_size_class_policy_mirror_matches_source=0|1

replacement_front_page_local_bridge_v0=0|1
replacement_front_page_local_bridge_report_only=1
replacement_front_page_local_bridge_source_truth=hako_alloc.page_box|unknown
replacement_front_page_local_bridge_source_file=lang/src/hako_alloc/memory/page_box.hako
replacement_front_page_local_bridge_mirror_source=hako_page_box_report_mirror|unknown
replacement_front_page_local_bridge_bound=0|1
replacement_front_page_local_bridge_missing=<comma list>
replacement_front_page_local_required_field_count=<n>
replacement_front_page_local_required_fields_present=0|1
replacement_front_page_local_missing_fields=none|<comma list>
replacement_front_page_local_required_method_count=<n>
replacement_front_page_local_required_methods_present=0|1
replacement_front_page_local_missing_methods=none|<comma list>
replacement_front_page_local_directarray_fields_present=0|1
replacement_front_page_local_counter_fields_present=0|1
replacement_front_page_local_acquire_release_methods_present=0|1
replacement_front_page_local_lifecycle_methods_present=0|1
replacement_front_page_local_typed_meta_matches_source=0|1
replacement_front_page_local_same_owner_route_matches_source=0|1
replacement_front_page_local_no_remote_free_claim=1

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
