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

The next implementation row is a small `.hako` size-class policy row. It must
not reopen allocator-provider M104+ or host replacement behavior.

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
| `C203 record local scalar replacement` | keep non-escaping `record` values in local aggregate carriers before objectization | `C204-C205` |
| `C204 ArrayBox inline-record storage` | add packed record columns for `record` payloads while keeping `ArrayBox` as authority | `C205` |
| `C205 allocator metadata record migration` | replace hand-written scalar metadata arrays with `record` surface on top of packed storage | revisiting `M178` metadata surface |

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

### Docs / Guard Checkpoints

| Row | Goal | Trigger |
| --- | --- | --- |
| `D195 hako_alloc SSOT refresh` | refresh ownership, handle lifetime, release, realloc, alignment, huge, secure-list, and non-goal docs | after `M176`, then again after `M184` if secure-list lands |
| `D196 stop-the-line guard refresh` | keep guards focused on real stop lines instead of growing every row-specific shell check | after `M184`, or earlier if guard runtime starts blocking normal development |

Post-M184 D195 refresh is complete in
`docs/development/current/main/phases/phase-293x/293x-196-D195-HAKO-ALLOC-SSOT-REFRESH.md`.
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
No M191 allocator API row is scheduled until a concrete stats/options or
algorithm row needs it.

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
