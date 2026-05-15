# lang/src/hako_alloc — Hako Alloc Root

Scope
- Top-level physical root for the `hako_alloc` layer.
- First-wave home for policy-plane helpers that used to live under `lang/src/runtime/memory/`.
- Future home for `RawBuf`, `Layout`, `MaybeInit`, and collection/allocator policy layering.
- Current policy/state contract owner is fixed by `hako-alloc-policy-state-contract-ssot.md`.
- Substrate capability order is fixed by `substrate-capability-ladder-ssot.md`.

Current mimalloc policy queue
- `HakoAllocAlignmentPolicy` is the M177 alignment owner. It normalizes
  requested alignment, rejects unsupported inputs, and computes padded-size
  policy only.

Principles
- Keep this root as the alloc/policy anchor.
- Do not move OS VM, LLVM, or other thin native keep concerns here.
- Treat `runtime/memory/` as historical location only; new work should land under `hako_alloc/`.
- New state boxes in this root should use Unified Members stored declarations
  (`field: Type`) instead of legacy `init { ... }` slot lists.
- Do not treat this root as the owner for unrestricted raw memory, raw pointer,
  native layout, OS VM, or platform atomics/TLS.
- Current stop-line:
  - current live implementation row is GC trigger threshold policy
  - first landed policy rows are handle reuse policy and GC trigger threshold policy
  - third live allocator row is the page/free-list policy-state prototype
    (the original row name says VM-only; current EXE parity rides typed-object
    and pure-first routes, not native allocator fast-path ownership)
  - live Rust bodies still remain under `src/runtime/**`
  - `RawBuf` policy/state and `MaybeInit` stay reserved-only for now
  - `LayoutBox` is size-class policy only; it is not native layout/ABI ownership
  - `HakoAllocRemoteFreePolicy` owns the `.hako` remote-free retry policy only;
    pointer atomics remain substrate route facts.
  - `HakoAllocPageSourcePolicy` owns the `.hako` page-source policy seam only;
    OSVM reserve/commit/decommit metal remains substrate/native keep.
  - `SizeClassBox` owns mimalloc-shaped pure size-class policy. M187 adds
    `usize` input facades while keeping invalid/oversized sentinel results
    signed. `LayoutBox` remains the small/medium compatibility facade until
    the page heap migrates.
  - `HakoAllocPageModel` owns page-local `free` / `local_free` / `used` /
    `capacity` / `reserved` invariants, including same-thread local-free
    collection and empty-page retire observation. Page queues, OSVM sourcing,
    TLS, atomics, and remote-free integration stay in later rows.
  - `HakoAllocPageQueue` owns page ordering/direct-page cache state. It chooses
    pages by observing `freeCount()` and must not pop allocation blocks.
  - `HakoAllocFastPathHeap` composes page queue selection with page-local
    free-list pop. It does not source OS pages, collect local-free blocks, or
    own remote-free integration.
  - `HakoAllocOsVmBackedFastPathHeap` is the M168 adapter that backs fresh
    modeled pages through `HakoAllocPageSourcePolicy` reserve/commit/decommit.
    It must not add native OSVM leaves, local-free retire, remote-free
    integration, provider activation, hooks, or allocator replacement.
  - M169 local-free retire stays page-local in `HakoAllocPageModel`; M170 owns
    remote-free integration and any broader heap/queue consumption of retire
    state.
  - `HakoAllocRemoteFreePageInbox` is the M170 integration owner: it composes
    the existing bounded pointer remote-free policy with caller-provided
    page/block identity, then delegates page state mutation to
    `HakoAllocPageModel.releaseLocal(...)`.
  - `HakoAllocPageMap` is the M171 pointer ownership model. It resolves
    caller-visible pointer ids to page/block ids, but arbitrary free/realloc
    composition remains a later row.
  - `HakoAllocPageMapReleaseSeam` is the M172 release orchestration owner. It
    takes an explicit `HakoAllocPageMap`, performs lookup, delegates block
    mutation to `HakoAllocPageModel.releaseLocal(...)`, and unregisters only
    after page-local release succeeds.
  - `HakoAllocPageMapReleaseObserver` is the M173 invariant observer owner. It
    observes live-handle state around `HakoAllocPageMapReleaseSeam.releasePtr(...)`
    so realloc rows can reuse a frozen success-vs-reject contract without taking
    over register/release/unregister execution.
  - `HakoAllocPageMapReallocSameClassPath` is the M174 no-move owner. It reads
    live page-map identity and current page block size, then returns the same
    pointer only when the request still fits that block without releasing or
    unregistering.
  - `HakoAllocPageMapReallocAllocCopyReleasePath` is the M175 grow fallback
    owner. It allocates a replacement ptr, models copy count, and calls the
    M172 release seam only after replacement allocation succeeds.
  - `HakoAllocPageMapReallocFailureContract` is the M176 diagnostics owner. It
    classifies zero, oversized, unknown, stale, released, and alloc-fail
    outcomes while delegating same-class and grow execution back to M174/M175.
  - `HakoAllocAlignmentPolicy` is the M177 alignment owner. It normalizes and
    validates alignment plus computes padded-size policy, but aligned execution
    still stays outside the current realloc/release owners. M188 adds `usize`
    request-size/alignment facades without changing signed reject lanes.
  - `HakoAllocPageMapAlignedSmallPath` is the M178 aligned small-path owner. It
    attaches alignment metadata to normal page-map-backed small allocations
    while huge-page routing still stays outside this owner. M188 adds a typed
    `usize` input facade that delegates to the same execution path. C205c
    moves the aligned-small metadata columns behind
    `aligned_small_meta_store_box.hako` / `HakoAllocAlignedSmallMetaStore`,
    where record construction/read is used as the source-facing seam while
    storage remains scalar columns.
  - `HakoAllocHugeThresholdRouter` is the M179 huge threshold/routing owner. It
    routes padded requests above the last regular size-class to an explicit
    huge-unsupported fail-fast result while delegating small requests to M178.
    M188 adds `usize` request facades while keeping route/result kinds signed.
  - `HakoAllocHugePageModel` is the M180 huge page model owner. It registers
    one-allocation huge handles in the page map while keeping requested,
    committed, and live metadata separate from small page free lists. C205d
    moves those metadata columns behind `huge_page_meta_store_box.hako` /
    `HakoAllocHugePageMetaStore`, where record construction/read is used as the
    source-facing seam while storage remains scalar columns.
  - `HakoAllocObjectLifecycleFacadeHugeReleaseRoute` is the MIMAP-024A facade
    metadata release owner. It allocates a huge handle through the MIMAP-023A
    facade huge-page route and retires that same handle through
    `HakoAllocHugePageModel.markReleased(ptr)` while stopping before M181
    page-map unregister and OS page return.
  - `HakoAllocHugeReleaseSeam` is the M181 huge release seam owner. It retires
    huge handles through `HakoAllocHugePageModel` and unregisters page-map
    ownership without touching small page `releaseLocal(...)`.
  - `HakoAllocSecureFreeListDiagnostics` is the M183 diagnostics owner. It
    observes page-local free-list shape before secure-list encode/decode policy
    lands.
  - `HakoAllocSecureFreeListPolicy` is the M184 encoded-next policy owner. It
    provides reversible next-index encode/decode plus capacity validation with
    caller-provided cookies and no entropy-source claim.
  - `allocator_metadata_records.hako` is the C205a declaration-only owner for
    future allocator metadata records. It names aligned-small and huge-page
    metadata shapes. C205c consumes the aligned-small record shape through a
    record-facing metadata store, and C205d does the same for huge-page
    metadata. Packed `ArrayBox` compiler auto-use remains a later row.

Design owners
- Policy/state stop-line:
  `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- Capability ladder:
  `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
- Minimal memory/pointer substrate:
  `docs/development/current/main/design/minimal-capability-modules-ssot.md`
- Minimum verifier:
  `docs/development/current/main/design/minimum-verifier-ssot.md`

Allocator fast-path rule
- `mimalloc-lite` and allocator policy models can live here as policy/state rows.
- mimalloc-grade native fast paths require the substrate ladder first.
- post-M176 realloc behavior is fixed by the M171-M176 owners above; do not fold
  alignment, huge-page, or secure-list execution into those boxes.
- M189 object-return API parity is intentionally limited to the legacy
  `HakoAllocHeap` semantic API. Page-map-backed result wrappers and failure
  reason objects remain M190+ work.
- M190 adds `HakoAllocHandleResult` for explicit `ok/reason/handle` allocation
  failures while keeping the M189 `allocate/realloc` compatibility APIs intact.
  Reason codes are scalar and local to the `.hako` policy row.
- post-M184 secure-list behavior is intentionally split: diagnostics observe
  page-local list shape, while encoded-next policy only transforms next indices.
  Neither owner mutates page state or claims hardening.
- `RawBuf` policy/state, `MaybeInit`, native `Layout`, `repr`-like layout,
  `sizeof`, `alignof`, `no_alloc`, `no_safepoint`, TLS, atomics, and OS VM rows
  stay reserved until their docs/gates are named.
- The narrow `RawBufCoreBox` allocation facade lives under
  `lang/src/runtime/substrate/raw_buf/`; it is not this layer's allocator
  policy/state owner.

Production allocator port entry
- M45 fixes the production allocator port boundary:
  - `hako_alloc` owns allocator policy/control/facade names.
  - `runtime/substrate` owns raw capability facades.
  - native metal keep owns final libc/syscall/platform bodies.
- First implementation order:
  1. production facade boundary
  2. local page policy proof
  3. remote-free policy proof
  4. OSVM page-source proof
  5. stress production-facade parity
- Do not add allocator replacement hooks, pointer fetch_add, native pointer
  attrs, or app-specific `.inc` matchers as part of the entry plan.

Allocator replacement hook boundary
- M52 fixes the hook boundary before implementation:
  - `hako_alloc` owns policy/control shape; it does not install the process hook.
  - MIR/manifest HookPlan facts must become the backend-readable truth before
    any backend/runtime hook is activated.
  - `.inc` must not infer hook ownership from app, facade, or policy names.
  - hook environment toggles stay inactive until a named future row documents
    them with defaults and removal/rollback conditions.

Current modules
- `memory.arc_box`
- `memory.alignment_policy_box`
- `memory.alloc_fast_path_heap_box`
- `memory.allocator_facade_box`
- `memory.layout_box`
- `memory.osvm_backed_fast_path_heap_box`
- `memory.page_box`
- `memory.page_heap_box`
- `memory.page_map_box`
- `memory.page_map_aligned_small_path_box`
- `memory.page_map_release_box`
- `memory.page_map_release_invariant_box`
- `memory.page_map_realloc_alloc_copy_release_box`
- `memory.page_map_realloc_failure_contract_box`
- `memory.page_map_realloc_same_class_box`
- `memory.page_queue_box`
- `memory.page_queue_lifecycle_box`
- `memory.page_source_policy_box`
- `memory.remote_free_page_integration_box`
- `memory.remote_free_policy_box`
- `memory.refcell_box`
- `memory.size_class_box`
- `memory.usize_field_probe_box`
