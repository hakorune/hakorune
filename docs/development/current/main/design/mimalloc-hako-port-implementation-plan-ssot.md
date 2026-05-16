---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: upstream mimalloc reference, `.hako` port decomposition, and the next
  implementation order after the allocator-provider stop line.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - docs/reference/runtime/substrate-capabilities.md
  - lang/src/hako_alloc/
  - apps/mimalloc-lite/
---

# Mimalloc Hako Port Implementation Plan (SSOT)

## Decision

The mimalloc port resumes as derived `.hako` / `hako_alloc` implementation
work. The upstream C project is a reference for algorithm shape, invariants,
and naming, not a vendored runtime dependency and not a trigger for Hakorune
process allocator replacement.

Current implementation rows are selected by `CURRENT_STATE.toml` and the
phase-293x taskboard. The original size-class row is historical and already
landed; after `MIMAP-059A` the active row is `MIMAP-060A`, a reclaim
completion marker route. It must not reopen allocator-provider M104+, host
replacement behavior, secure entropy execution, thread scheduling, OSVM
release, or page-source behavior.

## Upstream Reference

Reference source:

```text
repository: https://github.com/microsoft/mimalloc
primary tag: v3.3.2
primary commit: 30b2d9d89099
comparison main snapshot: fef6b0dd70f9
local analysis copy: target/upstream/mimalloc-v3.3.2
license: MIT, copyright Microsoft Corporation and Daan Leijen
```

The upstream source under `target/upstream/` is analysis material only and
must not be committed into this repository. If later rows copy or closely adapt
non-trivial code, the row must preserve the MIT notice and state the source
file it derived from.

Use `v3.3.2` as the current algorithm reference because upstream marks it as
the recommended latest release. The default `main` snapshot is useful for
comparison, but it is not the port baseline unless this SSOT is updated.

## Port Boundary

Current target:

```text
.hako / hako_alloc
  size class policy
  page/free-list state
  page queues and thread-local heap shape
  local free and bounded remote-free policy
  OSVM-backed page source composition
  proof apps and production facade parity
```

Current non-target:

```text
allocator-provider M104+
host/process allocator replacement
#[global_allocator] / GlobalAlloc
implicit provider env toggles
activation hooks
.inc provider, mimalloc, hook, facade, or policy name matching
```

## Upstream Component Map

| Upstream area | Port reading | Hakorune owner |
| --- | --- | --- |
| `include/mimalloc/types.h` | names the durable model: heap, thread heap, page, arena, page queues, free lists, and bins | docs plus `lang/src/hako_alloc/` model vocabulary |
| `src/page-queue.c` | size-to-bin and good-size policy, then page queue search | first `SizeClassBox`, later page queue owner |
| `src/alloc.c` | fast allocation from a page free list and generic fallback | allocator facade over page queue/page model |
| `src/page.c` | page initialization, capacity/reserved growth, free-list extension, page collect/retire | page model and local free-list policy |
| `src/free.c` | local free, remote free, abandoned/reclaim behavior | local free first; remote-free and reclaim are later explicit rows |
| `src/arena.c`, `src/os.c` | OS-backed page source and arena ownership | existing OSVM page-source policy, widened only by explicit rows |
| `src/page-map.c` | pointer-to-page lookup for arbitrary frees | future only; current handles can carry page identity |
| `src/random.c`, secure free list paths | encoded/randomized free-list hardening | future optional hardening after plain policy works |
| stats/options/init files | production configuration and observability | after allocator algorithm parity, not before |

## Required Model Split

Keep the owner files small:

```text
lang/src/hako_alloc/memory/size_class_box.hako
  pure size -> bin and bin -> block-size policy

lang/src/hako_alloc/memory/layout_box.hako
  compatibility facade over size class policy until callers migrate

lang/src/hako_alloc/memory/page_box.hako
  page-local fields and free/local_free operations

lang/src/hako_alloc/memory/page_queue_box.hako
  bin queues and direct-page cache shape

lang/src/hako_alloc/memory/page_heap_box.hako
  orchestration only; no embedded size-class or remote-free algorithms

lang/src/hako_alloc/memory/allocator_facade_box.hako
  public production seam over page heap, remote-free policy, and page source
```

If a row makes `page_heap_box.hako` grow algorithm bodies again, stop and split
before adding behavior.

## Implementation Ladder

| Row | Scope | Acceptance |
| --- | --- | --- |
| `M163 mimalloc size-class policy owner` | add `SizeClassBox` with a mimalloc-shaped `size_to_bin` / `bin_size` subset, and keep `LayoutBox` as a wrapper | pure `.hako` proof app; no allocator state mutation |
| `M164 hako_alloc layout migration` | migrate current two-class layout calls to the size-class owner while preserving existing app behavior | `mimalloc-lite` and production facade proofs stay green |
| `M165 page model split` | introduce page-local `free`, `local_free`, `used`, `capacity`, and `reserved` vocabulary | local allocate/free proof, no atomics |
| `M166 page queue and direct-page cache` | model per-bin page queues and fast direct-page lookup | allocation chooses an available page without scanning app code |
| `M167 alloc fast path plus generic fallback` | mirror the upstream fast free-list pop and a small generic fallback | deterministic proof app for reuse and refill |
| `M168 OSVM page source composition` | route fresh page creation through existing OSVM page-source policy | no new native leaf unless a proof row requires it |
| `M169 local free collection and retire` | add local free collection and empty-page retirement policy | local free does not touch remote-free atomics |
| `M170 remote-free integration` | compose bounded remote-free retry policy with the page model | remote-free proof stays behind existing pointer atomics |

M171 starts the post-M170 ladder with the pointer-to-page map model. Realloc,
aligned allocation, huge/large pages, secure encoded free lists, purge, stats,
and options stay separate rows after the page-map-backed free seam is proven.

## Post-M170 Active Roadmap

This is the execution order for the `.hako` mimalloc port. Rows `M172-M184`
match the recommended algorithm ladder. Rows `M185-M190` are adjusted to the
current repo truth: broad numeric field inventory and facade-local exact
`usize` stats already landed in phase 294x, so future rows must not redo them.

### Main Algorithm Lane

| Row | Status | Goal | Stop line |
| --- | --- | --- | --- |
| `M171 page-map model` | Complete | record and resolve caller-visible pointer ownership to `page_id` / `block_id` | no arbitrary free/realloc, no pointer arithmetic, no OSVM release |
| `M172 page-map-backed release seam` | Complete | compose page-map lookup/unregister with page-local release | no realloc, no byte copy, no host replacement |
| `M173 pre-realloc release invariant freeze` | Complete | freeze handle lifetime, page-map registration/unregistration timing, and release observers before realloc | no realloc body, no byte copy |
| `M174 realloc same-class/no-move path` | Complete | keep the same handle when the new request fits the current usable block/class | no alloc-copy-release fallback |
| `M175 realloc alloc-copy-release fallback` | Complete | allocate a replacement handle, model copy count, and release the old handle only after success | no aligned/huge allocation |
| `M176 realloc negative matrix / failure contract` | Complete | fix stale/unknown/released/zero/oversized failure behavior | no new allocator API surface beyond realloc diagnostics |
| `M177 alignment policy object` | Complete | add alignment normalization, power-of-two validation, padded-size policy | no native aligned allocation route or ABI alignment claim |
| `M178 aligned allocation small path` | Complete | attach alignment metadata to normal page-map-backed small allocations | no huge path |
| `M179 huge threshold and routing` | Complete | classify huge requests and fail-fast unsupported huge behavior | no huge page model yet |
| `M180 huge page model` | Complete | model one-allocation huge pages separately from small page free lists | no OS unreserve/release widening |
| `M181 huge release seam` | Complete | unregister and release huge handles through the page-map owner | no small-page free-list mixing |
| `M182 secure free-list policy inventory` | Complete | decide encoded/randomized free-list responsibilities | no implementation |
| `M183 secure-list diagnostics-only` | Complete | detect duplicate/out-of-range/free-list shape errors | no encode/decode |
| `M184 secure-list encode/decode small path` | Complete | add the smallest encoded-next policy once diagnostics are stable | no cryptographic randomness claim without compiler support |

### Numeric / API Completion Lane

| Row | Status | Goal | Stop line |
| --- | --- | --- | --- |
| `M185 hako_alloc field inventory delta` | Complete | inventory numeric fields introduced by `M173-M184` and reconcile them with `NUMERIC_FIELDS.md` / `294x-16` | no migration in this row |
| `M186 exact usize facade stats` | Already complete as `294x-19e` | facade-local event counters are exact `usize` | do not schedule duplicate facade migration |
| `M187 exact usize for size-class policy` | Complete | migrate size-class policy inputs/outputs that are truly non-negative and backend-supported | lookup failure sentinels stay signed |
| `M188 exact usize for request path` | Complete | migrate allocation request sizes and alignments where verifier/lowering support is live | page ids, block ids, and failure sentinels stay signed |
| `M189 object-return allocate/realloc EXE parity` | Complete | prove semantic object-return allocator APIs in EXE instead of relying on scalar observers | no scalar-only proof substitution for API parity |
| `M190 nullable / failure handle contract` | Complete | define explicit success/failure handle shape for allocation APIs | no silent null fallback or unchecked invalid handle |

Stats/options surface remains post-`M190` unless a concrete algorithm row needs
it earlier. Environment toggles require the environment-variable SSOT and a
removal/rollback note.

### Cross-Cutting Compiler / Backend Lane

These rows may run in parallel only when their write sets do not collide with
the active `.hako` algorithm row.

| Row | Goal | Required before |
| --- | --- | --- |
| `C191 typed object field initializer hardening` | prove declaration-site object defaults are per-construction across VM/backend paths | wider state-box migration |
| `C192 method return object lowering` | make object-return methods reliable enough for allocator API EXE parity | `M189` and full object-return `realloc` parity |
| `C193 exact numeric backend fail-fast coverage` | keep unsupported exact numeric routes fail-fast instead of silently using legacy lanes | any broader `usize` production migration |
| `C194 verifier-owned allocation invariants` | move release/page-map/size invariants from observer proofs toward verifier-owned contracts | broad allocator API hardening |
| `C201 ordinary user-box field-index fast path` | keep ordinary `box` semantics while lowering typed fields as field-index/typed-slot fast paths | `C202-C205` aggregate/value lane |
| `C202 record surface and semantics` | lock `record` as the explicit identity-free aggregate surface | `C203-C205` and any pretty allocator metadata surface |
| `C203a record declaration metadata transport` | carry `record_decls` through JSON/MIR surfaces without mixing records into ordinary user-box declarations | `C203b-C205` |
| `C203b record layout plans` | derive layout facts for `record` declarations before any local scalar rewrite consumes them | `C203c-C205` |
| `C203c record local scalar replacement metadata` | expose concrete record layouts in folded `agg_local` / placement metadata before any scalar rewrite consumes them | record construction/read lowering and `C204-C205` |
| `C204a ArrayBox inline-record storage descriptors` | derive metadata-only packed column descriptors from record layout plans | `C204b-C205` |
| `C204b ArrayBox inline-record storage vocabulary` | add private runtime storage vocabulary and materialization boundaries while keeping public ArrayBox behavior unchanged | `C205` |
| `C205a allocator metadata record declarations` | add declaration-only record shapes for aligned-small and huge-page metadata while preserving current scalar columns | record construction/read lowering |
| `C205b allocator record construction/read lowering` | make record values usable enough for metadata probes through builder-local scalarization | live hako_alloc metadata migration |
| `C205c aligned-small metadata record migration` | move M178 `meta_ptrs/meta_alignments/meta_padded_sizes` behind a record-shaped metadata store | huge-page metadata migration |
| `C205d huge-page metadata record migration` | replace M180 `page_ids/ptrs/requested_sizes/committed_sizes/live_flags` with record-backed storage | broader allocator/table cleanup |
| `C207 packed ArrayBox compiler auto-use eligibility gate` | classify record-array sites as eligible/rejected/fail-fast candidates without enabling runtime auto-use | `C208-C209` packed auto-use pilot |
| `C208 inline-record materialization / escape boundary` | reject visible element escape until materialization exists while allowing non-escaping direct field-read shapes | `C209` |
| `C209 non-escaping packed ArrayBox compiler auto-use pilot` | consume C207 eligibility for integer-lane, non-escaping record arrays | `C210-C211` metadata packed-store pilots |
| `C210 aligned-small metadata packed-store pilot` | let aligned-small metadata stores use compiler-selected packed storage without `.hako` knowing compiler internals | `C211` |
| `C211 huge-page metadata packed-store pilot` | extend the packed-store pilot to huge-page metadata while preserving live/sentinel semantics | verifier/backend hardening |
| `C212 packed record backend fail-fast hardening` | keep unsupported packed record routes fail-fast rather than silently falling back | allocator algorithm rows after packed storage |
| `C194 verifier-owned allocation invariants` | move C210/C211 metadata row invariants into MIR verifier-owned contracts | M191 allocator-owned stats/options surface |
| `M191 hako_alloc stats surface` | add allocator-owned stats snapshot observability without mutable options or behavior changes | purge/decommit policy inventory |
| `M192 purge/decommit policy inventory` | classify empty-retired page purge candidates without page-source execution | purge/decommit dry-run observer |
| `M193 purge/decommit dry-run observer` | observe existing OSVM-backed heap page/backing state through the M192 policy without execution | purge/decommit execution fail-fast entry |
| `M194 purge/decommit execution fail-fast` | add an explicit execution attempt owner that always returns blocked reports | bounded decommit execution policy |
| `M195 bounded decommit execution policy` | call a caller-provided decommit executor exactly once for eligible in-bound decisions | page-source decommit adapter |
| `M196 page-source decommit adapter` | connect M195 bounded policy to `HakoAllocPageSourcePolicy.decommitPage` through a decommit-only adapter | purge decommit heap integration |
| `M197 purge decommit heap integration` | compose dry-run observation, bounded policy, and page-source adapter for heap page/backing state | purge decommit state marker |
| `M198 purge decommit state marker` | record successful decommit report page ids in a separate state owner without heap/page mutation | purge state-aware duplicate guard |
| `M199 purge state-aware duplicate guard` | block already-marked pages before M197/M196 source execution can run again | decommitted page reuse precondition |
| `M200 decommitted page reuse precondition` | classify decommitted pages as unavailable until a future recommit path exists | recommit fail-fast entry |
| `M201 recommit fail-fast entry` | expose explicit recommit attempt reports while keeping actual recommit/source execution blocked | future recommit execution policy |
| `M202 bounded recommit policy` | execute bounded caller-provided recommit source calls only after M200 requires recommit | future recommit page-source adapter |
| `M203 page-source recommit adapter` | connect M202 bounded policy to `HakoAllocPageSourcePolicy.commitPage` through a recommit-only adapter | future recommit marker transition / heap integration |
| `M204 recommit marker transition` | transition decommit marker state after successful recommit using generation counts | future recommit heap integration |
| `M205 recommit heap integration` | compose recommit precondition/policy/adapter/marker transition and reactivate page-local state | reuse proof closeout |
| `M206 reuse proof closeout` | prove the M199+M205 purge/recommit reuse loop for two marker generations without new allocator owners | post-closeout task selection |
| `M207 page lifecycle invariant freeze` | name and prove the active/retired/decommitted/recommitted-active lifecycle table without new allocator behavior | C194b verifier-owned lifecycle invariants |
| `C194b verifier-owned page lifecycle invariants` | move selected M207 lifecycle invariants into verifier-owned contracts | M208 heap reuse priority policy |
| `M208 heap reuse priority policy` | define active/retired/recommitted/fresh selection priority without scheduler or OS release widening | lifecycle observability |
| `M209 lifecycle stats observer surface` | expose read-only lifecycle event stats without behavior/options changes | EXE/backend hardening |
| `M210 decommit/recommit/reuse EXE hardening` | keep lifecycle proof completion out of VM-only or silent fallback routes | purge candidate policy inventory |
| `M211 purge candidate policy inventory` | classify future purge candidates after lifecycle invariants are frozen | bounded purge scheduler |
| `M212 bounded purge/decommit scheduler small path` | scan a bounded page set and call existing decommit seams for one candidate | abandoned/reclaim inventory |
| `M213 abandoned/reclaim inventory` | inventory thread/abandoned/reclaim vocabulary without threads, atomics expansion, or replacement | future row |

Post-C205 phase split:

- `C201-C205` is the aggregate metadata lane. After C205d it is closed for the
  current allocator metadata migration goal.
- `C206+` was reserved for cleanup/probe work only and is closed for now
  through C206e. Further C206 rows require a concrete blocker that directly
  simplifies C207 acceptance.
- `C207+` opens the packed `ArrayBox` compiler auto-use lane, starting with an
  eligibility gate only. Runtime auto-use, hako_alloc migration, materialized
  record elements, and backend lowering stay later rows.
- Allocator algorithm rows resume separately; do not use a C206 cleanup row to
  add realloc/aligned/huge/secure-list behavior.

C201 status:
complete as `293x-207`. The MIR JSON user-box declaration surface now carries
`field_index_fast_path`, `layout_id`, `field_index`, and `storage` for fields
that already have a legal typed-object plan. Ordinary `box` identity semantics
remain unchanged, and unsupported fields stay on the names/dynamic route.

C202 status:
complete as `293x-208`. The parser now accepts `record` declarations as the
explicit identity-free aggregate surface with typed non-weak fields only. Local
scalar replacement, packed `ArrayBox` storage, and allocator metadata migration
remain in `C203-C205`.

C203a status:
complete as `293x-209`. `record_decls` now has its own metadata-only transport
lane through Program JSON v0, JSON bridge, MIR metadata, and MIR JSON. Records
remain separate from ordinary `user_box_decls`, and no record layout/lowering
consumer is installed yet.

C203b status:
complete as `293x-210`. Concrete `record_decls` now derive dedicated
`record_layout_plans` with field slots and storage classes. The layout lane is
metadata-only, separate from typed-object/user-box plans, and does not yet drive
local scalar replacement or packed `ArrayBox` residence.

C203c status:
complete as `293x-211`. Concrete record layout plans now publish
`record_local_layout` rows through folded `agg_local` and placement/effect
metadata. This is still metadata-only: no record construction/read lowering,
no user-box scalar seed route, and no MIR scalar rewrite.

C204a status:
complete as `293x-212`. `array_record_storage_plans` now derives metadata-only
column descriptors from `record_layout_plans`. Runtime `ArrayStorage` variants,
public ArrayBox behavior, compiler auto-use, and hako_alloc migration remain
future work.

C204b status:
complete as `293x-213`. `ArrayStorage::InlineRecord` now exists as private
runtime storage vocabulary with columnar scalar storage, len/capacity/clone/
equality/debug support, and explicit unmaterialized boundaries for visible
record values. Compiler auto-use, boxed record materialization, hako_alloc
metadata migration, backend lowering, and provider/native allocator coupling
remain out of scope.

C205a status:
complete as `293x-214`. `allocator_metadata_records.hako` declares
`HakoAllocAlignedSmallMeta` and `HakoAllocHugePageMeta` as identity-free record
shapes for future metadata migration. The M178 and M180 scalar `ArrayBox`
columns remain authoritative runtime state, and no hako_alloc owner constructs
or reads these records yet.

C205b status:
complete as `293x-215`. Record construction/read lowering is now usable for
small metadata probes through a builder-local scalarization seam. Record values
do not emit `NewBox`, do not materialize objects, and fail fast if they escape
the direct field-read route. `hako_alloc` scalar metadata columns remain the
runtime truth until `C205c-C205d`.

C205c status:
complete as `293x-216`. M178 aligned-small metadata now lives behind
`HakoAllocAlignedSmallMetaStore`. The store constructs
`HakoAllocAlignedSmallMeta` at the append boundary and reads fields locally, but
storage remains scalar columns until packed `ArrayBox` compiler auto-use lands.

C205d status:
complete as `293x-217`. M180 huge-page metadata now lives behind
`HakoAllocHugePageMetaStore`. The store constructs `HakoAllocHugePageMeta` at
the append boundary and reads fields locally, but storage remains scalar
columns until packed `ArrayBox` compiler auto-use lands.

C206a status:
complete as `293x-218`. `HakoAllocAlignedSmallMetaStore` now owns a single
`findIndex(ptr)` lookup seam, and both aligned metadata read APIs delegate to
that seam. This is behavior-preserving cleanup only; it does not enable packed
`ArrayBox` auto-use or a generic store abstraction.

C206b status:
complete as `293x-219`. `ArrayInlineRecordProbe` now provides an explicit
test-only owner for constructing `ArrayStorage::InlineRecord` arrays. The probe
is `#[cfg(test)]`, keeps visible record materialization disabled, and does not
enable compiler auto-use or `hako_alloc` packed storage migration.

C206c status:
complete as `293x-220`. The explicit inline-record probe now fixes its first
negative contract: ragged columns are rejected before an `ArrayBox` is built.
This keeps the packed residence invariant local to the probe owner.

C206d status:
complete as `293x-221`. `ArrayInlineRecordPlanProbe` now connects MIR
`ArrayRecordStoragePlan` metadata to the explicit test-only runtime probe for
integer-lane columns. Unsupported storage, such as handle columns, is rejected
before any `ArrayBox` is built. This is still not compiler auto-use.

C206e status:
complete as `293x-223`. The aligned-small and huge-page metadata stores now
expose index-based read seams, and pointer-based APIs delegate through those
seams. This removes repeated pointer lookup in callers that already resolved an
index, without changing allocator behavior or enabling packed ArrayBox
compiler auto-use.

C207 status:
complete as `293x-224`. C207 adds only a compiler eligibility gate for packed
`ArrayBox` auto-use. It emits conservative eligible/rejected metadata from
`ArrayRecordStoragePlan` rows, leaves `production_auto_use_enabled=false`, and
does not enable production `ArrayStorage::InlineRecord` construction,
hako_alloc migration, materialization, or backend lowering.

C208 status:
complete as `293x-225`. C208 adds metadata-only
`array_record_materialization_boundary_plans` from C207 eligible rows. It allows
future non-escaping direct indexed field-read consumption while keeping public
`ArrayBox.get(i)` record values, returned record elements, host/backend escape,
visible record materialization, runtime auto-use, hako_alloc migration, and
backend lowering closed.

C209 status:
complete as `293x-226`. C209 adds
`array_record_packed_autouse_pilot_plans` and a crate-private runtime
inline-record i64 column construction/read seam for non-escaping direct field
reads. Public record materialization, hako_alloc live migration, backend
lowering, and visible `ArrayBox.get(i)` record values remain closed.

C210 status:
complete as `293x-227`. C210 adds
`hako_alloc_aligned_small_packed_store_pilot_plans` for
`HakoAllocAlignedSmallMeta` / `HakoAllocAlignedSmallMetaStore` and proves the
aligned-small metadata shape can use the private C209 i64-column seam. The live
`.hako` store remains record-shaped and scalar-column compatible, with no
`InlineRecord` / compiler feature names in hako_alloc source. Huge-page metadata
and backend lowering remain C211/C212 work.

C211 status:
complete as `293x-228`. C211 adds
`hako_alloc_huge_page_packed_store_pilot_plans` for `HakoAllocHugePageMeta` /
`HakoAllocHugePageMetaStore` and proves the huge-page metadata shape can use the
private C209 i64-column seam. The row preserves live-flag and released-sentinel
contracts, keeps `.hako` scalar-column compatibility, and leaves backend
lowering to C212.

C212 status:
complete as `293x-229`. C212 adds the shared
`enforce_mir_backend_supported(...)` gate and a packed record backend
fail-fast checker. It does not enable packed record backend lowering; it only
ensures future required packed record routes fail fast on unsupported backends
instead of silently falling back.

C194 status:
complete as `293x-230`. C194 adds the MIR verifier-owned hako_alloc metadata
invariant checker for C210/C211 rows. It verifies source pilot presence,
integer-lane backing layouts, fixed metadata column order, materialization
closure, and huge-page released sentinels without changing hako_alloc runtime
behavior or enabling backend lowering.

M191 status:
complete as `293x-231`. M191 adds `HakoAllocStatsSurface`,
`HakoAllocStatsSnapshot`, and `HakoAllocProductionFacade.statsSnapshot()`.
The row is stats-first: mutable options, env toggles, purge/decommit, provider
activation, hooks, and process allocator replacement remain out of scope.

M192 status:
complete as `293x-232`. M192 adds `HakoAllocPurgePolicyInventory` and
`HakoAllocPurgeDecision` as a read-only purge/decommit candidate inventory. It
classifies missing-backing, live, not-retired, and empty-retired cases, but all
execution booleans remain false and no page-source / OSVM release behavior is
opened.

M193 status:
complete as `293x-233`. M193 adds `HakoAllocPurgeDryRunObserver`, which reads
existing OSVM-backed heap page/backing state and delegates to the M192 policy.
It proves live, eligible empty-retired, and missing-backing dry-run cases while
keeping page-source / OSVM release execution closed.

M194 status:
complete as `293x-234`. M194 adds `HakoAllocPurgeExecutionFailFastEntry` and
`HakoAllocPurgeExecutionReport`. Missing, ineligible, and eligible decisions
all return blocked reports; all execution fields remain false and no
page-source / OSVM release behavior is opened.

M195 status:
complete as `293x-235`. M195 adds `HakoAllocBoundedDecommitPolicy` and
`HakoAllocBoundedDecommitReport`. It validates missing/ineligible decisions,
base, and byte bounds, then calls a caller-provided `decommitPage(base, bytes)`
executor at most once for eligible in-bound requests. Unreserve and OS release
remain closed.

M196 status:
complete as `293x-236`. M196 adds `HakoAllocPageSourceDecommitAdapter`, a
decommit-only executor for M195 that delegates to
`HakoAllocPageSourcePolicy.decommitPage(base, bytes)`. Reserve/commit stay in
the existing page-source facade, and unreserve / OS release remain closed.

M197 status:
complete as `293x-237`. M197 adds `HakoAllocPurgeHeapDecommitIntegration`,
which composes dry-run observation, bounded decommit policy, and the page-source
decommit adapter for existing heap page/backing state. The row does not mutate
heap/page state and leaves unreserve / OS release closed.

### Docs / Guard Checkpoints

| Row | Goal | Trigger |
| --- | --- | --- |
| `D195 hako_alloc SSOT refresh` | refresh ownership, handle lifetime, release, realloc, alignment, huge, secure-list, and non-goal docs | after `M176`, then again after `M184` if secure-list lands |
| `D196 stop-the-line guard refresh` | keep guards focused on real stop lines instead of growing every row-specific shell check | after `M184`, or earlier if guard runtime starts blocking normal development |

D196 status:
complete as `293x-222`. `C206+` cleanup/probe guards are local-run and
index-listed by default. They must not be promoted into quick/dev or
allocator-wide gates unless a later card explicitly names the production stop
line that requires promotion.

Post-M184 D195 refresh is complete in
`docs/development/current/main/phases/phase-293x/293x-196-D195-HAKO-ALLOC-SSOT-REFRESH.md`.
D196 guard refresh is complete in
`docs/development/current/main/phases/phase-293x/293x-222-D196-STOP-THE-LINE-GUARD-REFRESH.md`.
`C206+` cleanup/probe guards are local-run and index-listed by default; they
must not be promoted into `dev_gate.sh` or `k2_wide_allocator_gate.sh` unless a
card names the production stop line that requires promotion.
M185 field inventory delta is complete in
`docs/development/current/main/phases/phase-293x/293x-197-M185-HAKO-ALLOC-FIELD-INVENTORY-DELTA.md`.
M187 size-class `usize` facades are complete in
`docs/development/current/main/phases/phase-293x/293x-198-M187-SIZE-CLASS-USIZE-POLICY.md`.
M188 request-path `usize` facades are complete in
`docs/development/current/main/phases/phase-293x/293x-199-M188-REQUEST-PATH-USIZE.md`.
M189 object-return allocator API parity is complete in
`docs/development/current/main/phases/phase-293x/293x-200-M189-OBJECT-RETURN-ALLOCATOR-API.md`.
M190 nullable/failure handle contract is complete in
`docs/development/current/main/phases/phase-293x/293x-201-M190-NULLABLE-FAILURE-HANDLE-CONTRACT.md`.
M198 status:
complete as `293x-238`. `HakoAllocPurgeDecommitStateMarker` records page ids
from successful decommit reports and rejects non-decommitted, duplicate, or
widened release reports without calling page-source APIs or mutating heap/page
state.

M199 status:
complete as `293x-239`. `HakoAllocPurgeStateAwareDecommitGuard` consults the
M198 marker before M197 heap decommit integration, blocks duplicate decommit
attempts before source execution, and marks successful first execution.

M200 status:
complete as `293x-240`. `HakoAllocDecommittedPageReusePrecondition` classifies
committed/unmarked pages as reusable and decommitted pages as requiring future
recommit, without page-source calls or heap/page mutation.

M201 status:
complete as `293x-245`. `HakoAllocRecommitFailFastEntry` exposes an explicit
recommit attempt report by reading the M200 precondition. It remains
blocked/report-only and does not call page-source APIs, recommit, unreserve,
release OSVM pages, clear markers, or mutate heap/page state.

M202 status:
complete as `293x-246`. `HakoAllocBoundedRecommitPolicy` executes at most one
caller-provided `commitPage(base, bytes)` source call after M200 reports
`requires_recommit`, while direct page-source adapter wiring, marker clearing,
heap/page mutation, unreserve, and OS release remain closed.

M203 status:
complete as `293x-247`. `HakoAllocPageSourceRecommitAdapter` connects M202 to
`HakoAllocPageSourcePolicy.commitPage(base, bytes)` through a recommit-only
adapter. Marker transition/clearing, heap/page mutation, unreserve, OS release,
and allocator replacement remain closed.

M204 status:
complete as `293x-248`. `HakoAllocPurgeDecommitStateMarker` now records
recommitted page ids in a separate generation lane. M200 treats a page as
decommitted only while decommit generations outnumber recommit generations, so
successful recommit can reopen reuse eligibility without physically removing
marker entries or mutating heap/page state.

M205 status:
complete as `293x-249`. `HakoAllocRecommitHeapIntegration` composes M200,
M202, M203, and M204, then calls `HakoAllocPageModel.reactivate()` so an empty
retired page can become selectable and allocatable again after recommit. Page
sourcing, unreserve, OS release, and allocator replacement remain closed.

M206 status:
complete as `293x-250`. The reuse closeout proof composes the M199 duplicate
guard and M205 recommit heap integration for two decommit/recommit generations,
then selects and acquires from the same heap page after each recommit. It adds
no new allocator owner and keeps object-return allocator API expansion,
unreserve, OS release, provider activation, hooks, and process allocator
replacement closed.

Post-M206 allocator task selection must be explicit; do not keep extending the
purge/recommit closeout ladder without a new blocker.

Post-M206 selected ladder:

1. `M207 page lifecycle invariant freeze`
2. `C194b verifier-owned page lifecycle invariants`
3. `M208 heap reuse priority policy`
4. `M209 lifecycle stats observer surface`
5. `M210 decommit/recommit/reuse EXE hardening`
6. `M211 purge candidate policy inventory`
7. `M212 bounded purge/decommit scheduler small path`
8. `M213 abandoned/reclaim inventory`

M207 status:
complete as `293x-251`. `HakoAllocPageLifecycleInvariantObserver` freezes the
active/retired/decommitted/recommitted-active state vocabulary as a read-only
observer over heap page/backing state and marker generations. It adds no
allocation, release, decommit, recommit, reactivation, scheduler, OS release,
provider, hook, or replacement behavior.

C194b status:
complete as `293x-252`. C194b adds the MIR verifier-owned page lifecycle
surface checker for the frozen M207 owner. It verifies required lifecycle
functions plus the selected strong `i64` report fields that carry the
active/retired/decommitted/recommitted-active vocabulary and generation facts,
without changing allocator behavior or enabling backend lowering.

M208 status:
complete as `293x-253`. M208 adds the read-only
`HakoAllocHeapReusePriorityPolicy`, which ranks active, recommitted-active,
retired-reactivate, and fresh fallback routes from the frozen M207 lifecycle
facts without mutating heap/page state or widening page-source behavior.

M209 status:
complete as `293x-254`. M209 adds
`HakoAllocLifecycleStatsObserverSurface`, which snapshots existing M207
lifecycle observer counters and M208 reuse priority policy counters without
triggering observation/selection, mutating allocator state, adding options, or
widening backend/provider behavior.

M210 status:
complete as `293x-255`. M210 adds a proof-only pure-first EXE hardening app
and guard for the M195-M209 decommit/recommit/reuse lifecycle path. It proves
two marker generations plus lifecycle/reuse/stats observation without adding a
new allocator owner, VM-only completion, provider/hook behavior, unreserve, or
OS release.

M211 status:
complete as `293x-257`. M211 adds
`HakoAllocPurgeCandidatePolicyInventory`, a read-only classifier over already
built M207 lifecycle reports. It inventories future purge candidates after the
lifecycle vocabulary is frozen without observing heap pages, scanning queues,
scheduling purge work, decommitting, recommitting, calling page-source APIs,
mutating allocator state, unreserving, releasing OSVM pages, or widening
provider/backend behavior.

M212 status:
complete as `293x-259`. M212 adds
`HakoAllocBoundedPurgeDecommitScheduler`, which scans at most a caller-provided
page count, observes M207 lifecycle facts, classifies them through M211, and
delegates the first eligible page to the M199 state-aware decommit guard. It
does not call M197/M195/M196 or page-source APIs directly, does not unreserve
or release OSVM pages, and does not change allocation/release/reuse behavior.

M213 status:
complete as `293x-261`. M213 adds
`HakoAllocAbandonedReclaimInventory`, a read-only vocabulary owner for
abandoned ownership and reclaim candidates over scalar owner/page facts. It
does not schedule threads, add atomics, execute reclaim, call page-source APIs,
decommit, recommit, unreserve, release OSVM pages, or change allocator
behavior.

### Proof App Ergonomics Queue

These rows improve proof readability without changing allocator semantics.
They run before the next algorithm row when a proof app becomes harder to read
than the allocator state it is proving.

Decision: support both ordinary boolean chains and proof check blocks, but do
not treat them as aliases.

- `&&` / `||` are ordinary expression/control-flow operators. They keep
  short-circuit semantics and belong in production logic, `if`, `loop`, and
  guard-like code.
- `check "name" { "label": expr }` is a proof-list surface. It evaluates
  every item eagerly, gives each assertion a stable label, and returns one
  scalar pass/fail value for proof apps.
- Long proof summaries should not be represented as a giant `&&` chain once the
  `check` surface exists. Conversely, normal control-flow logic should not use
  `check` as a replacement for short-circuit boolean expressions.

| Row | Status | Goal | Stop line |
| --- | --- | --- | --- |
| `293x-182 M172 proof check cleanup` | Complete | replace the M172 proof app's giant conjunction with an app-local `ProofCheck` helper | no parser, language syntax, allocator algorithm, or guard-scope widening |
| `C197 logical condition surface hardening` | Complete | make ordinary `&&` / `||` chains and parenthesized multiline conditions pleasant and reliable as normal control-flow expressions | no eager proof-list semantics, no `all(...)` macro, no allocator-specific condition DSL |
| `C198 check block surface` | Complete | add a general proof-oriented `check "name" { "label": expr }` expression with eager item evaluation | no short-circuit macro, variadic `all(...)`, allocator DSL, or backend route selector |
| `C199 compound assignment surface` | Complete | promote `+=` style sugar where it lowers to the existing assignment form | no hidden overflow policy or allocator-specific meaning |
| `C200 guard else surface` | Complete | add early-return guard syntax that lowers to `if !(cond) { ... }` | no exception/fallback semantics |

## Granular Row Contracts

Each row below is allowed to split into `A/B` cards if the listed write set
becomes too large or if one row would mix state-model work with orchestration
work. Splitting is mandatory if a row starts adding algorithm bodies back into
`page_heap_box.hako`.

| Row | Primary write set | Proof target | Must not touch | Done when |
| --- | --- | --- | --- | --- |
| `M164 layout migration closeout` | `layout_box.hako`, existing hako_alloc proof docs, at most callsite cleanup in `page_heap_box.hako` | `mimalloc-size-class-policy-proof`, `mimalloc-lite`, `allocator-stress`, representative production facade EXE guard | new page model, page queues, full-bin heap exposure, app output changes | all remaining size-class decisions outside `SizeClassBox` are compatibility facade decisions explicitly owned by `LayoutBox` |
| `M165 page model split` | new `page_box.hako`, `hako_module.toml`, focused proof app | `apps/mimalloc-page-model-proof/` | page queues, OSVM, TLS, atomics, remote-free policy | page-local `free`, `local_free`, `used`, `capacity`, and `reserved` invariants are testable without a heap |
| `M166 page queue and direct-page cache` | new `page_queue_box.hako`, narrow heap adapter only | `apps/mimalloc-page-queue-proof/` | alloc fast path, OSVM, remote-free, page-map | a request bin can find the current page through queue/direct-cache state without app-side scanning |
| `M167 alloc fast path plus generic fallback` | allocator facade/heap orchestration over `page_box` + `page_queue_box` | `apps/mimalloc-alloc-fast-path-proof/` | OSVM fresh page sourcing, local-free retire, remote-free atomics | fast path pops from a page free list; fallback refills deterministically through the queue model |
| `M168 OSVM page source composition` | `page_source_policy_box.hako`, heap adapter, proof app | `apps/mimalloc-osvm-page-source-composition-proof/` | new native leaf, OSVM unreserve/release, hook/provider behavior | fresh modeled pages can be backed by existing reserve/commit/decommit rows |
| `M169 local free collection and retire` | `page_box.hako`, local-free proof app | `apps/mimalloc-local-free-retire-proof/` | remote-free atomics, abandoned-page reclaim, OSVM release | local frees collect into reusable free blocks and empty-page retire state is observable |
| `M170 remote-free integration` | `remote_free_policy_box.hako`, page model integration, proof app | `apps/mimalloc-remote-free-page-integration-proof/` | pointer fetch_add, page-map, arbitrary pointer free, process replacement | existing pointer load/store/CAS rows push remote frees into page-owned state behind a bounded retry policy |

### Split Rules

- `M164` is a closeout row because M163 already made `LayoutBox` delegate to
  `SizeClassBox`. If M164 has no useful code delta, it may land as a focused
  guard/card that proves no stale layout owner remains.
  M164 landed as this closeout guard: `SizeClassBox` remains the truth owner,
  `LayoutBox` is documented as the legacy two-class compatibility facade, and
  `page_heap_box.hako` still consumes that facade without bypassing it.
- `M165` must create a page-local owner before `M166` creates queues. Do not
  add queue traversal to prove page fields.
  M165 landed as `HakoAllocPageModel` in `page_box.hako`: `free`,
  `local_free`, `used`, `capacity`, and `reserved` are testable without a heap,
  and `local_free` remains non-reusable until the later collection row.
- `M166` must choose pages, not allocate blocks. If it pops a free block, that
  belongs to `M167`.
  M166 landed as `HakoAllocPageQueue` in `page_queue_box.hako`: per-bin page
  selection and direct-page cache refresh are testable without queue-owned
  block pops, OSVM sourcing, or remote-free integration.
- `M166B` cleaned the new M165/M166 state boxes to the current Unified Members
  stored-field style. New mimalloc `.hako` state boxes should use `field: Type`
  rather than legacy `init { ... }` slot lists.
- `M172` converged the active mimalloc/hako_alloc state owners onto stored
  field initializers after the parser row accepted `field = expr` and
  `field: Type = expr`. Fixed defaults and owner construction now live at
  declaration site; constructor parameters remain in `birth(...)`.
- `293x-173` converged allocator numeric stored fields from `IntegerBox` spelling to
  `i64` scalar substrate annotations. The current runtime lane remains
  `Integer(i64)`; `usize`/exact-width behavior stays reserved.
- `293x-174` fixed the pre-port syntax/spec decision: `usize` remains accepted as
  annotation text and MIR numeric substrate metadata, but hako_alloc/mimalloc
  production state stays on `i64` until native exact numeric storage exists.
  State fields continue to use `i64` until exact pointer-sized unsigned
  semantics, range checks, and overflow behavior are live. Parameter and
  accepted return type annotations are now preserved through AST metadata and
  JSON transport, and stored field initializers are per-construction values.
- `M173` landed as `HakoAllocPageMapReleaseObserver` in
  `page_map_release_invariant_box.hako`: it observes the existing
  `HakoAllocPageMapReleaseSeam.releasePtr(...)` contract so successful releases
  expire the handle with one release/unregister/live-count delta while reject
  paths keep live ownership and zero release/unregister/page-local delta.
- `M174` landed as `HakoAllocPageMapReallocSameClassPath` in
  `page_map_realloc_same_class_box.hako`: it returns the same live pointer when
  the new request still fits the current page block and rejects grow, stale,
  released-block, and unknown cases without release or unregister side effects.
- `M175` landed as `HakoAllocPageMapReallocAllocCopyReleasePath` in
  `page_map_realloc_alloc_copy_release_box.hako`: it allocates a replacement
  ptr, models one copy, and releases the old ptr only after replacement
  registration succeeds, while same-class and no-capacity cases reject without
  extra release side effects.
- `M167` resumed after the 294x `usize` preflight as
  `HakoAllocFastPathHeap` in `alloc_fast_path_heap_box.hako`: page selection is
  delegated to `HakoAllocPageQueue`, block pops are delegated to
  `HakoAllocPageModel.acquire(...)`, and deterministic fallback creates modeled
  pages without OSVM sourcing, local-free retire, remote-free, or page-map
  behavior.
- `M167` may create deterministic model pages in memory; it must not reserve
  OS memory. OSVM enters only in `M168`.
- `M168` landed as `HakoAllocOsVmBackedFastPathHeap` in
  `osvm_backed_fast_path_heap_box.hako`: fresh modeled pages are backed by
  `HakoAllocPageSourcePolicy` reserve/commit rows, decommit remains cleanup
  evidence, and the original M167 heap remains OSVM-free. Its executable proof
  uses scalar-return `addFreshPage()` only as a fresh-page composition proof
  seam; semantic allocation remains the object-return `allocate(size)` API and
  allocator API object-return parity is a future row.
- `M169` owns same-thread local free collection only. Remote-free and abandoned
  reclaim remain out of scope until `M170+`.
  M169 landed this page-local behavior in `HakoAllocPageModel`:
  `acquire(...)` collects one same-thread `local_free` entry back to a reusable
  free stack slot when the normal free stack is empty, and the final
  `releaseLocal(...)` records empty-page retire state without remote-free
  atomics, abandoned reclaim, OSVM release, or page-map lookup.
- `M170` composes existing pointer atomics only. It does not add pointer
  `fetch_add`, page-map lookup, allocator hooks, or host replacement.
  M170 landed as `HakoAllocRemoteFreePageInbox` in
  `remote_free_page_integration_box.hako`: bounded pointer publish stays in
  `HakoAllocRemoteFreePolicy`, page state mutation stays in
  `HakoAllocPageModel.releaseLocal(...)`, and caller-provided block identity is
  the proof seam until a future page-map row exists.
- `M171` landed as `HakoAllocPageMap` in `page_map_box.hako`: it records
  caller-visible pointer ownership and resolves it to page/block identity.
  It does not call page release yet; `M172` owns the page-map-backed release
  seam.
- `M172` owns only the composition of page-map lookup/unregister with
  `HakoAllocPageModel.releaseLocal(...)`. It should use a separate orchestration
  box so `page_map_box.hako` stays a pure ownership map and `page_box.hako`
  stays page-local.
  M172 landed as `HakoAllocPageMapReleaseSeam` in
  `page_map_release_box.hako`: pointer registration remains owned by
  `HakoAllocPageMap`, while the release seam takes an explicit page-map owner,
  resolves pointer ownership, delegates page-local mutation to
  `HakoAllocPageModel.releaseLocal(...)`, and unregisters only after release
  succeeds. VM execution and MIR route contracts are the current proof; full
  pure-first EXE parity remains blocked by later object-return/lowering work,
  not by M172 ownership.

## First Concrete Row

`M163` should be implemented first because it is pure policy:

```text
owner: lang/src/hako_alloc/memory/size_class_box.hako
compat facade: lang/src/hako_alloc/memory/layout_box.hako
proof app: apps/mimalloc-size-class-policy-proof/
blocked behavior: provider activation, hook install, process replacement
```

The current two-class `LayoutBox` may remain as a compatibility shell while
`HakoAllocHeap` migrates. This avoids changing allocator state and size-class
policy in the same row.

M163 landed this owner as `lang/src/hako_alloc/memory/size_class_box.hako`.
`LayoutBox` now delegates size decisions to `SizeClassBox` while preserving the
current small/medium compatibility behavior for existing allocator apps.

## Stop-The-Line Signals

Stop and split the row if any implementation attempts to:

- add host allocator replacement behavior;
- add provider or hook activation;
- branch in `.inc` / backend code on mimalloc, provider, hook, facade, or
  policy names;
- copy a large upstream C function verbatim instead of extracting a `.hako`
  policy shape;
- mix size-class policy with page/free-list mutation in the same commit;
- grow `page_heap_box.hako` into a catch-all algorithm owner.

## Verification Policy

Daily rows use focused proof apps and the current pointer guard:

```bash
apps/<proof-app>/test.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Full real-app and EXE boundary suites are milestone checks, not required for
every pure policy row unless the row touches shared compiler/runtime behavior.

M214 status:

Complete as `293x-266`. M214 adds `HakoAllocOptionsInventory` as a read-only
allocator options/defaults inventory owner. It proves static option/default
facts and keeps mutable runtime options, environment toggles, allocation policy
changes, provider activation, hooks, process allocator replacement, reclaim
execution, unreserve, and OS release inactive.

M215 status:
complete as `293x-268`. M215 adds `HakoAllocThreadHeapOwnerInventory` as a
read-only owner-token inventory surface for future abandoned/reclaim work. It
classifies unknown owner, same-thread owner, active foreign owner, remote-free
pending, decommitted, and abandoned inactive owner-token facts while keeping
thread scheduling, atomic claim, remote-free drain, owner mutation, reclaim
execution, page-source calls, unreserve, OS release, provider activation, hooks,
and process allocator replacement inactive.
