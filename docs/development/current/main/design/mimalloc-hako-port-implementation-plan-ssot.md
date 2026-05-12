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

Rows after `M170` are deliberately not scheduled here. Realloc/aligned
allocation, pointer-to-page map, huge/large pages, secure encoded free lists,
purge, stats, and options each need fresh cards after the basic allocator
model is running.

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
- `M173` converged allocator numeric stored fields from `IntegerBox` spelling to
  `i64` scalar substrate annotations. The current runtime lane remains
  `Integer(i64)`; `usize`/exact-width behavior stays reserved.
- `M174` fixed the pre-port syntax/spec decision: `usize` remains accepted as
  annotation text and MIR numeric substrate metadata, but hako_alloc/mimalloc
  production state stays on `i64` until native exact numeric storage exists.
  State fields continue to use `i64` until exact pointer-sized unsigned
  semantics, range checks, and overflow behavior are live. Parameter and
  accepted return type annotations are now preserved through AST metadata and
  JSON transport, and stored field initializers are per-construction values.
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
  evidence, and the original M167 heap remains OSVM-free.
- `M169` owns same-thread local free collection only. Remote-free and abandoned
  reclaim remain out of scope until `M170+`.
- `M170` composes existing pointer atomics only. It does not add pointer
  `fetch_add`, page-map lookup, allocator hooks, or host replacement.

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
