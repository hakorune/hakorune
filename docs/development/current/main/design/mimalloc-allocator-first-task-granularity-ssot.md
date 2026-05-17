# mimalloc allocator-first task granularity SSOT

Status: SSOT
Decision: accepted
Date: 2026-05-15
Scope: allocator-first implementation order and language-feature sidecar policy.

## Decision

Continue the mimalloc lane through allocator behavior first.

Hakorune language features should not be implemented speculatively. If the
allocator row hits a compiler/language blocker, split the blocker into a
minimal sidecar row with its own fixture and guard, then return to the allocator
row.

```text
primary:
  allocator behavior row

sidecar:
  smallest compiler/language acceptance row that unblocks the allocator row

defer:
  broad language semantics without allocator evidence
```

## Stop lines

Each allocator row must own one durable behavior only.

Do not mix:

- allocation and release
- release and realloc
- OSVM and in-memory page model
- provider/hook/global allocator activation and allocator facade behavior
- compiler BoxCount fixes and allocator behavior in the same row
- BoxShape cleanup and allocator behavior in the same row

Unsupported backends must fail fast. VM remains diagnostic for object-heavy
allocator routes. LLVM/EXE is the primary acceptance backend for MIMAP-014+.

## Current implementation slices

### MIMAP-014A single-page small allocation fast-path

Purpose:

```text
Prove that the facade-owned object lifecycle queue can select one reusable page
and allocate one block through HakoAllocPageModel.acquire(size).
```

Allowed:

- add a narrow allocation method on the thin object lifecycle facade
- return scalar observer data such as selected page id, block id, and reason code
- use one reusable page fixture
- validate with LLVM/EXE primary guard

Forbidden:

- release/free route
- realloc route
- alignment route
- OSVM/page-source activation
- provider/hook/global allocator activation
- binding or mutating a facade-returned selected object
- broad queue scan proof density beyond the row output contract

Acceptance shape:

```text
adds=<queue index>
alloc_page=<page id>
alloc_block=<block id>
alloc_reason=<success reason>
summary=ok
```

### MIMAP-014B active-page fallback and allocation miss

Purpose:

```text
Extend the allocation fast-path to prove reusable-page preference, active-page
fallback, and miss/fail reason when no page can satisfy the request.
```

Allowed:

- reusable page success
- active page success after reusable page is consumed
- miss reason when no candidate page remains
- scalar facade observers for selection kind and miss count

Forbidden:

- release/free route
- realloc route
- OSVM/page-source activation
- selected-object return through the facade

### MIMAP-014C allocation fast-path stats observers

Purpose:

```text
Expose read-only allocation counters for the facade allocation fast-path.
```

Allowed:

- allocation attempt count
- allocation success count
- allocation miss/fail count
- reusable/active allocation counters

Forbidden:

- changing queue selection semantics
- adding new storage substrate
- provider/hook/global allocator activation

## Follow-up allocator rows

| Row | Purpose | Pull forward when |
| --- | --- | --- |
| `MIMAP-015A` | release/free one known block through the facade | allocation fast-path returns stable page/block observer data |
| `MIMAP-015B` | double-release / stale-release fail-fast route | `MIMAP-015A` is green |
| `MIMAP-016A` | alignment request metadata and observer result | small allocation route is stable |
| `MIMAP-016B` | aligned allocation success/fail route | alignment metadata has a guard |
| `MIMAP-017A` | realloc shrink / same-page route | release and alignment are stable |
| `MIMAP-017B` | realloc grow / move route | `MIMAP-017A` is green |
| `MIMAP-018A` | stats snapshot observer integration | allocation/release counters are stable |
| `MIMAP-019A` | purge/reclaim/decommit policy route | stats and lifecycle observers are stable |
| `MIMAP-020A` | OSVM/page-source capability pilot; first closeout is existing M49 owner adoption | in-memory allocator facade route is stable |
| `MIMAP-021A` | post-020 allocator row selection | MIMAP-020A and metadata post-promotion reconcile are closed |
| `MIMAP-021B` | facade page-source fresh-page attach | MIMAP-021A selects the facade-facing seam |
| `MIMAP-021C` | facade page-source allocation-miss fallback | MIMAP-021B is green |
| `MIMAP-022A` | post-lifecycle allocator row selection | lifecycle construction/reuse cleanup rows are closed |
| `MIMAP-022B` | facade huge-request fail-fast routing | MIMAP-022A selects the facade huge-request boundary |
| `MIMAP-022C` | post-huge-failfast allocator row selection | MIMAP-022B is green |
| `MIMAP-023A` | facade huge-page model route | MIMAP-022C selects the facade huge-page model seam |
| `MIMAP-023B` | post-huge-page-model allocator row selection | MIMAP-023A is green |
| `MIMAP-024A` | facade huge-release metadata route | MIMAP-023B selects the narrow huge-handle lifetime seam |
| `MIMAP-024B` | post-huge-release allocator row selection | MIMAP-024A is green |
| `MIMAP-025A` | facade huge-release fail-fast diagnostics route | MIMAP-024B selects double-release / stale-pointer diagnostics |
| `MIMAP-025B` | post-huge-release-failfast allocator row selection | MIMAP-025A is green |
| `MIMAP-026A` | facade huge-release page-map unregister route | MIMAP-025B selects the M181 success seam |
| `MIMAP-026B` | post-huge-unregister allocator row selection | MIMAP-026A is green |
| `MIMAP-027A` | facade huge-unregister fail-fast diagnostics route | MIMAP-026B selects M181 post-unregister reject diagnostics |
| `MIMAP-027B` | post-huge-unregister-failfast allocator row selection | MIMAP-027A is green |
| `MIMAP-028A` | facade huge page-source backing route | landed after MIMAP-027B |
| `MIMAP-028B` | post-backed-huge allocator row selection | landed; selected MIMAP-029A |
| `MIMAP-029A` | facade huge decommit-after-unregister success route | landed after MIMAP-028B |
| `MIR-EMIT-SSOT-001` | pure-first MIR artifact exactness sidecar | landed before MIMAP-029B |
| `MIR-ROUTE-PREFLIGHT-001` | lowering-plan route preflight sidecar | landed after MIR-EMIT-SSOT-001 |
| `SELFHOST-PROGRESS-001` | selfhost/pure-first progress diagnostics sidecar | landed after MIR-ROUTE-PREFLIGHT-001 |
| `MIR-EMIT-SSOT-002` | canonical external source-to-MIR route entry | landed after progress diagnostics |
| `MIMAP-029B` | post-huge-decommit allocator row selection | landed; selected MIMAP-030A |
| `MIMAP-030A` | facade huge decommit fail-fast diagnostics | landed after MIMAP-029B |
| `MIMAP-030B` | post-huge-decommit-failfast allocator row selection | landed; selected MIMAP-031A |
| `MIMAP-031A` | OSVM unreserve capability inventory / planning row | landed; selected MIMAP-032A |
| `MIMAP-032A` | OSVM unreserve substrate route | landed after MIMAP-031A |
| `MIMAP-032B` | post-OSVM-unreserve allocator row selection | landed; selected MIMAP-033A |
| `MIMAP-033A` | page-source unreserve adapter | landed after MIMAP-032B |
| `MIMAP-033B` | post-page-source-unreserve allocator row selection | landed; selected MIMAP-034A |
| `MIMAP-034A` | facade huge unreserve-after-decommit success route | landed after MIMAP-033B |
| `MIMAP-034B` | post-huge-unreserve allocator row selection | landed; selected MIMAP-035A |
| `MIMAP-035A` | facade huge unreserve fail-fast diagnostics | landed after MIMAP-034B |
| `MIMAP-035B` | post-huge-unreserve-failfast row selection | landed; selected MIMAP-036A |
| `MIMAP-036A` | post-huge-unreserve closeout guard | landed after MIMAP-035B |
| `MIMAP-036B` | post-huge-unreserve-closeout row selection | landed; selected MIMAP-037A |
| `MIMAP-037A` | facade huge backing-set helper cleanup | landed after MIMAP-036B |
| `MIMAP-037B` | post-backing-set-helper row selection | landed; selected MIMAP-038A |
| `MIMAP-038A` | object-lifecycle known-page loop cleanup | landed after MIMAP-037B |
| `MIMAP-038B` | post-known-page-loop row selection | landed; selected MIMAP-039A |
| `MIMAP-039A` | remote-free retry-bound named owner cleanup | landed after MIMAP-038B |
| `MIMAP-039B` | post-remote-free-retry-bound row selection | landed; selected MIR-ROW-C |
| `MIR-ROW-C` | nullable user-box object return sidecar | landed after MIMAP-039B |
| `MIMAP-039C` | post-nullable-object-return row selection | landed; selected MIMAP-040A |
| `MIMAP-040A` | object-lifecycle selectPage queue-length loop cleanup | landed after MIMAP-039C |
| `MIMAP-040B` | post-selectPage-loop row selection | landed; selected PURE-FIRST-DIAG-001 |
| `PURE-FIRST-DIAG-001` | pure-first acceptance layer diagnostics | landed after MIMAP-040B |
| `MIMAP-040C` | post-diagnostics row selection | landed; selected MIMAP-041A |
| `MIMAP-041A` | record report boundary cleanup for bounded purge/decommit scheduler | landed after MIMAP-040C |
| `MIMAP-041B` | post-record-report row selection | landed; selected MIR-EXTERN-SPEC-001 |
| `MIMAP-NEXT-BEHAVIOR-SELECTION-001` | post-cleanup row selection | landed; selected MIMAP-042A |
| `MIMAP-042A` | OSVM-backed fast-path bounded purge route | landed after MIMAP-NEXT-BEHAVIOR-SELECTION-001 |
| `MIMAP-042B` | post-fast-path-purge route row selection | landed; selected MIMAP-043A |
| `MIMAP-043A` | OSVM-backed fast-path recommit/reuse route | landed after MIMAP-042B |
| `MIMAP-043B` | post-fast-path-reuse route row selection | landed; selected MIMAP-044A |
| `MIMAP-044A` | OSVM-backed fast-path route closeout guard | landed after MIMAP-043B |
| `MIMAP-044B` | post-fast-path-closeout row selection | landed; selected MIMAP-045A |
| `MIMAP-045A` | OSVM-backed fast-path unreserve route | landed after MIMAP-044B |
| `MIMAP-045B` | post-fast-path-unreserve row selection | landed; selected MIMAP-046A |
| `MIMAP-046A` | OSVM-backed fast-path unreserve fail-fast diagnostics | landed after MIMAP-045B |
| `MIMAP-046B` | post-fast-path-unreserve-failfast row selection | landed; selected MIMAP-047A |
| `MIMAP-047A` | OSVM-backed fast-path unreserve closeout guard | landed after MIMAP-046B |
| `MIMAP-047B` | post-fast-path-unreserve-closeout row selection | landed; selected MIMAP-048A |
| `MIMAP-048A` | OSVM release capability inventory | landed after MIMAP-047B |
| `MIMAP-048B` | post-release-inventory row selection | landed; selected MIMAP-049A |
| `MIMAP-049A` | secure entropy source inventory | landed after MIMAP-048B |
| `MIMAP-049B` | post-secure-entropy-inventory row selection | landed; selected RANDOM-CAP-001 |
| `RANDOM-CAP-001` | uses random capability decision + fail-fast contract | landed; selected RANDOM-CAP-002 |
| `RANDOM-CAP-002` | random capability unsupported-route preflight | landed; selected MIMAP-050A |
| `MIMAP-050A` | secure entropy route proposal-or-park | landed; parked entropy execution and selected MIMAP-051A |
| `MIMAP-051A` | reclaim owner-transfer contract inventory | landed; selected MIMAP-051B |
| `MIMAP-051B` | post-reclaim-contract row selection | landed; selected USES-002A |
| `USES-002A` | declared uses capability plan mapping | landed; selected MIMAP-052A |
| `MIMAP-052A` | reclaim execution preflight proposal | landed; selected MIMAP-052B |
| `MIMAP-052B` | reclaim execution intent marker preflight | landed; selected MIMAP-053A |
| `MIMAP-053A` | reclaim execution support row selection | landed; selected MIMAP-054A |
| `MIMAP-054A` | reclaim atomic-claim contract | landed; selected MIMAP-055A |
| `MIMAP-055A` | reclaim owner-transfer first execution route | landed; selected MIMAP-056A |
| `MIMAP-056A` | reclaim remote-free drain contract inventory | landed; selected MIMAP-057A |
| `MIMAP-057A` | reclaim remote-free drain first execution route | landed; selected MIMAP-058A |
| `MIMAP-058A` | reclaim post-drain owner-transfer integration route | landed; selected MIMAP-059A |
| `MIMAP-059A` | post-reclaim-integration row selection | landed; selected MIMAP-060A |
| `MIMAP-060A` | reclaim completion marker route | landed; selected MIMAP-061A |
| `MIMAP-061A` | reclaim scalar lane closeout guard | landed; selected MIMAP-062A |
| `MIMAP-062A` | post-reclaim-scalar-closeout row selection | landed; selected MIMAP-063A |
| `MIMAP-063A` | reclaim scheduler boundary inventory | landed; selected MIMAP-064A |
| `MIMAP-064A` | reclaim scheduler request marker contract | landed; selected MIMAP-065A |
| `MIMAP-065A` | reclaim scheduler marker closeout guard | landed; selected MIMAP-066A |
| `MIMAP-066A` | post-scheduler-marker row selection | landed; selected MIMAP-067A |
| `MIMAP-067A` | reclaim scheduler substrate proposal-or-park | landed; selected MIMAP-068A |
| `MIMAP-068A` | reclaim scheduler request ledger route | landed; selected MIMAP-069A |
| `MIMAP-069A` | reclaim scheduler request ledger closeout guard | landed; selected MIMAP-070A |
| `MIMAP-070A` | post-scheduler-ledger row selection | landed; selected MIMAP-071A |
| `MIMAP-071A` | reclaim scheduler request ledger consume route | landed; selected MIMAP-072A |
| `MIMAP-072A` | reclaim scheduler ledger consume closeout guard | landed; selected MIMAP-073A |
| `MIMAP-073A` | post-scheduler-consume row selection | landed; selected MIMAP-074A |
| `MIMAP-074A` | reclaim scheduler request ledger roundtrip route | landed; selected MIMAP-075A |
| `MIMAP-075A` | reclaim scheduler request ledger roundtrip closeout guard | landed; selected MIMAP-076A |
| `MIMAP-076A` | post-scheduler-roundtrip row selection | landed; selected MIMAP-077A |
| `MIMAP-077A` | reclaim scheduler scalar lane closeout guard | landed; selected MIMAP-078A |
| `MIMAP-078A` | post-scheduler-scalar-closeout row selection | landed; selected MIMAP-079A |
| `MIMAP-079A` | segment arena bitmap boundary inventory | landed; selected MIMAP-080A |
| `MIMAP-080A` | segment arena bitmap inventory closeout guard | landed; selected MIMAP-081A |
| `MIMAP-081A` | post-segment-arena-bitmap-inventory row selection | landed; selected MIMAP-082A |
| `MIMAP-082A` | segment lifecycle scalar state contract | landed; selected MIMAP-083A |
| `MIMAP-083A` | segment lifecycle scalar state closeout guard | landed; selected MIMAP-084A |
| `MIMAP-084A` | post-segment-lifecycle-closeout row selection | landed; selected MIMAP-085A |
| `MIMAP-085A` | segment page membership scalar contract | landed; selected MIMAP-086A |
| `MIMAP-086A` | segment page membership closeout guard | landed; selected MIMAP-087A |
| `MIMAP-087A` | post-segment-page-membership-closeout row selection | landed; selected MIMAP-088A |
| `MIMAP-088A` | segment allocation readiness scalar contract | landed; selected MIMAP-089A |
| `MIMAP-089A` | segment allocation readiness closeout guard | landed; selected MIMAP-090A |
| `MIMAP-090A` | post-segment-allocation-readiness row selection | landed; selected MIMAP-091A |
| `MIMAP-091A` | segment allocation modeled consume route | landed; selected MIMAP-092A |
| `MIMAP-092A` | segment allocation modeled consume closeout guard | landed; selected MIMAP-093A |
| `MIMAP-093A` | post-segment-allocation-modeled-consume row selection | landed; selected MIMAP-094A |
| `MIMAP-094A` | segment allocation modeled ledger route | landed; selected MIMAP-095A |
| `MIMAP-095A` | segment allocation modeled ledger closeout guard | landed; selected MIMAP-096A |
| `MIMAP-096A` | post-segment-allocation-modeled-ledger row selection | landed; selected MIMAP-097A |
| `MIMAP-097A` | segment allocation modeled ledger release route | landed; selected MIMAP-098A |
| `MIMAP-098A` | segment allocation modeled ledger release closeout guard | landed; selected MIMAP-099A |
| `MIMAP-099A` | post-segment-allocation-modeled-release row selection | landed; selected MIMAP-100A |
| `MIMAP-100A` | segment allocation modeled ledger released-token recycle route | landed; selected MIMAP-101A |
| `MIMAP-101A` | segment allocation modeled ledger released-token recycle closeout guard | landed; selected MIMAP-102A |
| `MIMAP-102A` | post-segment-allocation-modeled-recycle row selection | landed; selected HAKO-ALLOC-SRC-CLEAN-001 |
| `HAKO-ALLOC-SRC-CLEAN-001` | segment counter compound assignment cleanup | landed; selected MIMAP-103A |
| `MIMAP-103A` | post-segment-counter-cleanup row selection | landed; selected MIMAP-104A |
| `MIMAP-104A` | segment allocation modeled ledger release span facts route | landed; selected MIMAP-105A |
| `MIMAP-105A` | post-release-span-facts row selection | landed; selected MIMAP-ROW-CADENCE-001 |
| `MIMAP-ROW-CADENCE-001` | mimalloc row validation cadence SSOT | landed; selected MIMAP-106A |
| `MIMAP-106A` | post-validation-cadence row selection | landed; selected MIMAP-107A |
| `MIMAP-107A` | segment allocation modeled released-span ledger route | landed; selected MIMAP-108A |
| `MIMAP-108A` | post-released-span-ledger row selection | landed; selected MIMAP-109A |
| `MIMAP-109A` | segment allocation modeled local-free candidate ledger route | landed; selected MIMAP-110A |
| `MIMAP-110A` | post-local-free-candidate-ledger row selection | landed; selected MIMAP-111A |
| `MIMAP-111A` | segment allocation modeled local-free apply plan route | landed; selected MIMAP-112A |
| `MIMAP-112A` | post-local-free-apply-plan row selection | landed; selected MIMAP-113A |
| `MIMAP-113A` | segment allocation modeled local-free scalar lane closeout guard | landed; selected MIMAP-114A |
| `MIMAP-114A` | post-local-free-scalar-closeout row selection | landed; selected MIMAP-115A |
| `MIMAP-115A` | segment allocation modeled local-free page-model apply route | landed; selected MIMAP-116A |
| `MIMAP-116A` | post-local-free-page-apply row selection | landed; selected MIMAP-117A |
| `MIMAP-117A` | segment allocation modeled local-free page-apply closeout guard | landed; selected MIMAP-118A |
| `MIMAP-118A` | post-local-free-page-apply-closeout row selection | landed; selected MIMAP-119A |
| `MIMAP-119A` | segment allocation modeled local-free integration route | landed; selected MIMAP-120A |
| `MIMAP-120A` | post-local-free-integration row selection | landed; selected MIMAP-121A |
| `MIMAP-121A` | segment allocation modeled local-free integration closeout guard | landed; selected MIMAP-122A |
| `MIMAP-122A` | post-local-free-integration-closeout row selection | landed; selected PURE-FIRST-GLOBAL-CALL-001 |
| `PURE-FIRST-GLOBAL-CALL-001` | same-module static helper global-call route support | landed; selected MIMAP-123A |
| `MIMAP-123A` | post-same-module-global-call row selection | landed; selected ROUTE-FIXPOINT-001 |
| `ROUTE-FIXPOINT-001` | route refresh fixpoint owner extraction | landed; selected ROUTE-DIAG-VOCAB-001 |
| `ROUTE-DIAG-VOCAB-001` | route diagnostics vocabulary SSOT | landed; selected ROUTE-DIAG-VOCAB-002 |
| `GUARD-MANIFEST-011` | pure-first route thin wrapper pilot | landed; selected ROUTE-DIAG-VOCAB-001 |
| `ROUTE-DIAG-VOCAB-002` | preflight vocabulary drift guard | landed; selected MIMAP-124A |
| `MIMAP-124A` | post-route-diagnostics cleanup row selection | landed; selected RUNTIME-UNWRAP-001 |
| `RUNTIME-UNWRAP-001` | runtime lock expect messages | landed; selected WASM-LOG-001 |
| `WASM-LOG-001` | WAT2WASM stable tags | landed; selected MIMAP-125A |
| `MIMAP-125A` | post-source-cleanup row selection | selected current |

### MIMAP-020A granularity

MIMAP-020A is not a provider-activation row. Its first task is to adopt and
document the already-live page-source capability owner:

```text
HakoAllocPageSourcePolicy
HakoAllocProductionFacade.pageSource*
tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
```

The row may close with no new code when the existing proof still demonstrates
reserve/commit/decommit through MIR-owned OSVM extern routes. A new owner,
proof app, or guard is allowed only if the existing proof misses a
MIMAP-specific acceptance seam. Provider selection, hooks, host allocator
replacement, and `#[global_allocator]` remain parked outside this row.

### MIMAP-021A / MIMAP-021B granularity

MIMAP-021A is a planning-only row. It selects the next allocator behavior row
after the page-source capability checkpoint and must not implement allocator
behavior.

MIMAP-021B is the selected behavior row. It should add one narrow facade-facing
adapter that proves:

```text
HakoAllocPageSourcePolicy.reservePage(bytes)
HakoAllocPageSourcePolicy.commitPage(base, bytes)
new HakoAllocPageModel(...)
HakoAllocObjectLifecycleFacade.objectLifecycleAddPage(page)
```

The row must stop before allocation-on-miss retry. It must not change
release/realloc/alignment, purge/reclaim/decommit/recommit execution,
remote-free/TLS/atomic behavior, page-map lookup, provider hooks, host allocator
replacement, or `#[global_allocator]`.

MIMAP-021C may add allocation-on-miss retry, but only as a one-fresh-page
fallback. It must reuse the MIMAP-021B adapter, retry once, and expose scalar
proof fields. It must not loop over multiple fresh pages, add page-source
decommit/recommit policy, use page-map lookup, or reopen provider hooks / host
allocator replacement.

### MIMAP-022A / MIMAP-022B granularity

MIMAP-022A is a planning-only row. It selects the next allocator behavior row
after the lifecycle construction/reuse cleanup rows and must not implement
allocator behavior.

MIMAP-022B is the selected behavior row. It should add one narrow
object-lifecycle facade boundary that proves:

```text
request size classification
huge request -> fail-fast scalar report
non-huge request -> existing MIMAP-021C allocation-miss fallback
```

The row must stop before a huge page model. It must not change release,
realloc, alignment, purge/reclaim/decommit/recommit execution, remote-free,
TLS, atomic behavior, page-map lookup, provider hooks, host allocator
replacement, or `#[global_allocator]`.

MIMAP-022C is a planning-only row. It selects exactly one post-huge allocator
behavior slice and records the owner, proof app, guard, and stop lines before
implementation begins.

### MIMAP-023A granularity

MIMAP-023A is the selected behavior row. It should add one narrow
object-lifecycle facade route that proves:

```text
request size classification through the MIMAP-022B threshold
huge request -> existing HakoAllocHugePageModel allocation
non-huge request -> existing MIMAP-022B / MIMAP-021C forwarding
```

The row must reuse the existing M180 huge-page model owner. It must not add a
new huge page model, huge release/unregister/unreserve/decommit behavior,
page-map lookup route, provider hooks, host allocator replacement, or
`#[global_allocator]`.

MIMAP-023B is a landed planning-only row. It selects MIMAP-024A as the next
allocator behavior slice.

### MIMAP-024A granularity

MIMAP-024A is the selected behavior row. It should add one narrow
object-lifecycle facade route that proves:

```text
huge request -> MIMAP-023A huge-page model allocation
released live huge pointer -> HakoAllocHugePageModel.markReleased(ptr)
scalar report -> live-count transition and release counters
non-huge request -> existing MIMAP-021C fallback forwarding
```

The row must reuse the existing MIMAP-023A facade route and the existing M180
huge-page model metadata release. It must not adopt the wider M181
`HakoAllocHugeReleaseSeam` as the facade owner yet, because that seam also owns
page-map lookup/unregister behavior. It must not add page-map lookup,
page-map unregister, OSVM release/unreserve/decommit, small release/free,
double-release / stale-pointer facade fail-fast, realloc, alignment,
purge/reclaim, remote-free, TLS, atomic, provider hooks, host allocator
replacement, or `#[global_allocator]`.

MIMAP-024B is a landed planning-only row. It selects MIMAP-025A as the next
allocator behavior slice.

### MIMAP-025A granularity

MIMAP-025A is the selected behavior row. It should add one narrow
object-lifecycle facade diagnostics route that proves:

```text
first huge release -> MIMAP-024A success
second release of same huge pointer -> reject
stale/unknown huge pointer -> reject
scalar report -> live-count stability and reject counters
```

The row must reuse the existing MIMAP-024A route and the existing M180
huge-page model metadata rejection behavior. It must not adopt the wider M181
`HakoAllocHugeReleaseSeam`, use page-map lookup/unregister, release OS pages,
add small release/free, realloc, alignment, purge/reclaim, remote-free, TLS,
atomic, provider hooks, host allocator replacement, or `#[global_allocator]`.

MIMAP-025B is a landed planning-only row. It selects MIMAP-026A as the next
allocator behavior slice.

### MIMAP-026A granularity

MIMAP-026A is the selected behavior row. It should add one narrow
object-lifecycle facade success route that proves:

```text
huge request -> MIMAP-023A huge-page model allocation
release selected live huge pointer -> M181 HakoAllocHugeReleaseSeam
scalar report -> huge-model live_count 1->0 and page-map live_count 1->0
```

The row may use the existing M181 `HakoAllocHugeReleaseSeam` for page-map
lookup/unregister on the success path. It must not add OSVM
release/unreserve/decommit, purge/reclaim, small release/free, realloc,
alignment, remote-free, TLS, atomic, provider hooks, host allocator
replacement, or `#[global_allocator]`.

MIMAP-026B is a landed planning-only row. It selects MIMAP-027A as the next
allocator behavior slice.

### MIMAP-027A granularity

MIMAP-027A is the selected behavior row. It should add one narrow
object-lifecycle facade diagnostics route that proves:

```text
first huge unregister -> MIMAP-026A success
second release of same pointer -> M181 lookup-miss reject
stale/unknown huge pointer -> M181 lookup-miss reject
scalar report -> page-map live_count remains 0 and reject counters advance
```

The row must reuse the existing MIMAP-026A route and the existing M181
`HakoAllocHugeReleaseSeam` for rejection. It must not call page-map
lookup/unregister or `HakoAllocHugePageModel.markReleased(ptr)` directly from
the facade diagnostics owner, and it must not add OSVM release/unreserve/
decommit, purge/reclaim, small release/free, realloc, alignment, remote-free,
TLS, atomic, provider hooks, host allocator replacement, or
`#[global_allocator]`.

MIMAP-027B is a landed planning-only row. It inspected the post-MIMAP-027A state
and selected exactly one next allocator behavior row without implementing that
behavior in the selection card. Candidate directions included a narrow OS page
return / unreserve planning row, or an owner-triggered CorePlan / verifier
contract promotion if the next behavior needed a stronger no-fallback contract
before OSVM release.

Selection rubric:

```text
1. prefer scalar .hako owner rows when they can prove the next invariant
2. choose CorePlan / verifier promotion only for a real no-fallback blocker
3. do not select OSVM release/unreserve/decommit before a page-source-backed
   huge allocation contract exists
4. keep provider activation / host replacement / hooks parked
```

MIMAP-028A is a landed facade huge page-source backing route. It proves that a
huge allocation can carry scalar backing identity before any later
release/decommit row:

```text
huge request -> page-source reserve/commit backing identity
backing identity -> huge model allocation/register
scalar report -> page id / ptr / base / bytes / requested size / committed size
scalar report -> no release/unregister/decommit executed
```

`MIMAP-028A` may reuse the existing page-source reserve/commit owner, but it
must not release/unregister the huge handle, call decommit, add unreserve or OS
release vocabulary, activate providers/hooks, replace the host allocator, add
small release/free, realloc, alignment, remote-free, TLS, atomic, or backend
`.inc` shortcuts. If the backed-huge proof cannot be expressed as a scalar
`.hako` owner, `MIMAP-027B` should select a narrow CorePlan / verifier contract
row instead of jumping to OS page return.

MIMAP-028B is a landed planning-only row. It inspected the backed huge
allocation state and selected `MIMAP-029A`: a facade huge
decommit-after-unregister success route. The selected row composes:

```text
MIMAP-028A backed huge allocation
M181 huge unregister bound to the MIMAP-028A route's huge model
M196 page-source decommit of the MIMAP-028A backing range
scalar report -> huge/page-map live state is zero and decommit succeeds
```

MIMAP-029A must not call `MIMAP-026A allocateThenUnregisterHuge(...)`
directly, because that route owns a separate huge model/page-map and therefore
does not prove decommit of the MIMAP-028A backing range.

MIMAP-029A is a landed behavior row. It started with the scalar owner split
below before any compiler sidecar:

```text
page_source_route.allocateHugeWithPageSource(facade, size)
release_seam = new HakoAllocHugeReleaseSeam(page_source_route.huge_route.huge_model)
release_seam.releaseHugePtr(result.huge_ptr)
decommit_adapter.decommitPage(result.source_base, result.source_bytes)
```

If that minimal owner fails pure-first route acceptance with
`target_body_supported=false`, `user_box_method_contract_missing`,
`structured_call_no_route`, or `mir_call_no_route`, stop the allocator row and
cut `USERBOX-METHOD-COMPOSITE-001`. That sidecar must be allocator-name neutral
and pin only the generic shape:

```text
typed report object-return method
  + field_set
  + generic_i64 global call
  + cross-owner same-module user-box method call
```

The implementation exposed a PHI base-query work explosion before a missing
user-box route contract. The closeout fixed `src/mir/phi_query.rs` by replacing
per-branch `BTreeSet` cloning with memoized backtracking. That compiler sidecar
does not add an allocator behavior or backend capability.

MIMAP-029A does not add unreserve/recommit, duplicate decommit diagnostics,
provider activation, host allocator replacement, hooks, or backend matcher
shortcuts.

Before returning to MIMAP-029B, the lane must close the pure-first/selfhost
BoxShape sidecar documented in:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

Reason:

```text
MIMAP-029A proved the allocator behavior, but the pure-first guard path still
lets preflight inspect one MIR artifact while selfhost EXE build may re-emit
another MIR artifact to the same path. Route unsupported diagnostics can also
arrive too late, after ny-llvmc / C shim work has started.
```

The sidecar order is:

```text
MIR-EMIT-SSOT-001:
  split --mir-in / --mir-out and require exact same MIR artifact for preflight
  and EXE build

MIR-ROUTE-PREFLIGHT-001:
  landed
  classify lowering-plan misses from MIR metadata before backend emission

SELFHOST-PROGRESS-001:
  landed
  add phase progress / timeout closeout for slow/stuck/unsupported diagnosis

MIR-EMIT-SSOT-002:
  landed
  make the canonical external source-to-MIR route explicit; prefer the existing
  tools/smokes/v2/lib/emit_mir_route.sh route SSOT or a thin facade over it
```

These are BoxShape rows. They must not add allocator behavior, widen
MIMAP-029A, or add backend name matchers.

MIMAP-029B landed after the pure-first sidecar and selected MIMAP-030A.
MIMAP-030A landed duplicate/stale huge decommit diagnostics through
allocator-side state without relying on OSVM/page-source decommit itself to
detect duplicate decommit. MIMAP-030B selected MIMAP-031A, which inventoried
OSVM unreserve/release as closed. MIMAP-031A selected MIMAP-032A as a substrate
route row only. MIMAP-032A landed `hako_osvm_unreserve_bytes_i64` /
`OsVmCoreBox.unreserve_bytes_i64` without page-source/facade adoption, and
selected MIMAP-032B as the post-unreserve allocator row selection. MIMAP-032B
selected MIMAP-033A as the page-source unreserve adapter row so allocator owner
adoption happens before any facade huge-unreserve route. MIMAP-033A landed
`HakoAllocPageSourcePolicy.unreservePage` and
`HakoAllocPageSourceUnreserveAdapter`; MIMAP-033B selected MIMAP-034A as the
facade huge unreserve-after-decommit success route without opening duplicate /
stale unreserve diagnostics, provider activation, or host allocator
replacement. MIMAP-034A landed that success route by composing MIMAP-029A and
MIMAP-033A. MIMAP-034B selected MIMAP-035A, which landed duplicate/stale
facade huge unreserve diagnostics by recording the successful unreserved
backing range and rejecting before a second adapter call. MIMAP-035B selected
MIMAP-036A as a post-huge-unreserve closeout guard before any broader
allocator behavior or provider/replacement work is reopened. MIMAP-036B
selected MIMAP-037A to extract duplicate/stale unreserve backing-set storage
from the fail-fast route into a helper before the next behavior row. MIMAP-037A
landed that BoxShape cleanup without adding allocator behavior. MIMAP-037B
selected MIMAP-038A, which replaced the object-lifecycle facade fixed
three-page known-page lookup with a queue-length loop while leaving queue
selection policy unchanged. MIMAP-038B probed the remaining object-lifecycle
queue fixed-shape selection cleanup, but that exposed a compiler acceptance
sidecar around loop-returned page objects. It selected the smaller MIMAP-039A
remote-free retry-bound named owner cleanup instead. MIMAP-039A landed without
changing retry behavior. MIMAP-039B selected `MIR-ROW-C` because the remaining
page queue loop cleanup needs compiler acceptance for nullable selected-object
returns before allocator source can be simplified. MIR-ROW-C landed that
acceptance in same-module user-box route metadata without changing allocator
behavior. MIMAP-039C selected MIMAP-040A, and MIMAP-040A replaced the fixed
`selectPage()` slots with a queue-length loop that returns the selected page
object directly. MIMAP-040B selected PURE-FIRST-DIAG-001, which added
layer/contract preflight diagnostics for pure-first acceptance failures.
MIMAP-040C then selected the MIMAP-041A record report cleanup.

MIMAP-041A landed the record-local report payload cleanup for the existing M212
bounded purge/decommit scheduler. MIMAP-041B selected a compiler cleanup chain,
and `MIMAP-NEXT-BEHAVIOR-SELECTION-001` returned the lane to allocator behavior
with `MIMAP-042A`. MIMAP-042A landed the OSVM-backed fast-path bounded purge
route and selected `MIMAP-042B` as the next row-selection checkpoint.

### MIMAP-042A granularity

MIMAP-042A was a narrow allocator behavior row. It added one `.hako` route owner
that composes existing owners:

```text
HakoAllocOsVmBackedFastPathHeap
HakoAllocPurgeStateAwareDecommitGuard
HakoAllocBoundedPurgeDecommitScheduler
```

Allowed:

- allocation / release through the OSVM-backed fast-path heap;
- one bounded scheduler purge attempt through M212;
- duplicate purge observation through the M199 state-aware guard;
- scalar report fields proving first purge vs duplicate purge behavior.

Forbidden:

- direct page-source / OSVM calls from the new route owner;
- unreserve, recommit, OS release, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`;
- remote-free, TLS, atomic, worker scheduling, or user-facing concurrency
  syntax expansion;
- compiler acceptance widening unless the route preflight exposes a real
  independent blocker.

### MIMAP-042B granularity

MIMAP-042B was a planning-only row. It read the MIMAP-042A proof result and
selected `MIMAP-043A` as the next allocator behavior task. It did not implement
allocator behavior, compiler acceptance, or cleanup by itself.

### MIMAP-043A granularity

MIMAP-043A was a narrow allocator behavior row. It added one `.hako` route owner
that composes:

```text
HakoAllocOsVmFastPathPurgeRoute
HakoAllocRecommitHeapIntegration
```

Allowed:

- allocation / release / bounded purge through the existing MIMAP-042A route;
- one explicit recommit attempt through M205 for a caller-provided page id;
- one post-recommit allocation through the same route;
- scalar report fields proving pre-recommit rejection, recommit success, and
  post-recommit allocation on the same page.

Forbidden:

- direct page-source / OSVM calls from the new route owner;
- unreserve, OS release, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`;
- remote-free, TLS, atomic, worker scheduling, or user-facing concurrency
  syntax expansion;
- scheduler policy changes, page queue policy changes, or fresh-page fallback
  changes;
- compiler acceptance widening unless route preflight exposes a real
  independent blocker.

### MIMAP-043B granularity

MIMAP-043B is a planning-only row. It reads the MIMAP-043A proof result and
selects exactly one next allocator/compiler/language task. It must not implement
allocator behavior, compiler acceptance, or cleanup by itself.

### MIMAP-044A granularity

MIMAP-044A is a closeout guard row. It freezes the completed OSVM-backed
fast-path route surface:

```text
MIMAP-042A:
  allocation / release / bounded purge

MIMAP-043A:
  recommit / post-recommit allocation reuse
```

Allowed:

- closeout SSOT for the combined route;
- one guard that pins cards, owners, proof apps, focused guards, docs, and
  inactive surfaces;
- taskboard/current pointer updates.

Forbidden:

- `.hako` behavior changes;
- compiler acceptance widening;
- provider activation, hooks, host allocator replacement, or `#[global_allocator]`;
- unreserve, OS release, remote-free/TLS/atomic execution changes, thread
  scheduling, or user-facing concurrency syntax expansion.

### MIMAP-044B granularity

MIMAP-044B is a planning-only row. It reads the MIMAP-044A closeout evidence and
selects exactly one next allocator/compiler/language task. It must not implement
allocator behavior, compiler acceptance, or cleanup by itself.

MIMAP-044B landed by selecting `MIMAP-045A` because no compiler/language blocker
was exposed by the OSVM-backed fast-path closeout and the smallest explicit
allocator continuation is the fast-path unreserve seam.

### MIMAP-045A granularity

MIMAP-045A composes the landed fast-path route with the existing page-source
unreserve adapter:

```text
HakoAllocOsVmFastPathReuseRoute
  -> allocate / release / purge / marker state

HakoAllocPageSourceUnreserveAdapter
  -> page-source unreserve executor
```

Allowed:

- add `HakoAllocOsVmFastPathUnreserveRoute`;
- prove one allocate -> release -> bounded purge/decommit -> unreserve sequence;
- expose scalar report fields for page id, backing range, purge state, adapter
  counters, and unreserve status.

Forbidden:

- direct page-source / OSVM calls from the new route owner or proof app;
- provider activation, hooks, host allocator replacement, or `#[global_allocator]`;
- remote-free execution, TLS/atomic execution changes, thread scheduling,
  reclaim execution, page ownership migration, or user-facing concurrency work;
- backend `.inc` app/name matchers;
- post-unreserve reuse behavior.

### MIMAP-045B granularity

MIMAP-045B is a planning-only row. It reads the MIMAP-045A unreserve evidence
and selects exactly one next allocator/compiler/language task. It must not
implement allocator behavior, compiler acceptance, or cleanup by itself.

MIMAP-045B landed by selecting `MIMAP-046A`, the fail-fast companion for
duplicate/stale/unknown fast-path unreserve requests.

### MIMAP-046A granularity

MIMAP-046A adds scalar diagnostics for invalid fast-path unreserve requests
without widening the success route:

```text
first unreserve:
  success via MIMAP-045A / MIMAP-033A

duplicate unreserve:
  rejected by the diagnostics owner

unknown or not-decommitted page:
  rejected before adapter execution
```

Forbidden:

- direct page-source / OSVM calls from the diagnostics owner or proof app;
- post-unreserve reuse behavior;
- provider activation, hooks, host allocator replacement, or `#[global_allocator]`;
- remote-free execution, TLS/atomic execution changes, thread scheduling,
  reclaim execution, page ownership migration, or user-facing concurrency work;
- backend `.inc` app/name matchers.

### MIMAP-046B granularity

MIMAP-046B is a planning-only row. It reads the MIMAP-046A fail-fast evidence
and selects exactly one next allocator/compiler/language task. It must not
implement allocator behavior, compiler acceptance, or cleanup by itself.

MIMAP-046B landed by selecting `MIMAP-047A`, the closeout guard for the
OSVM-backed fast-path unreserve success and fail-fast rows.

### MIMAP-047A granularity

MIMAP-047A is a closeout guard row. It freezes the completed OSVM-backed
fast-path unreserve surface:

```text
MIMAP-045A:
  success route via HakoAllocOsVmFastPathUnreserveRoute

MIMAP-046A:
  duplicate / unknown / not-decommitted diagnostics via
  HakoAllocOsVmFastPathUnreserveFailFastRoute
```

It must not implement allocator behavior, compiler acceptance, post-unreserve
reuse, OS release, provider activation, hooks, host allocator replacement,
remote-free/TLS/atomic execution changes, reclaim execution, or user-facing
concurrency work.

### MIMAP-047B granularity

MIMAP-047B is a planning-only row. It reads the MIMAP-047A closeout evidence and
selects exactly one next allocator/compiler/language task. It must not implement
allocator behavior, compiler acceptance, or cleanup by itself.

MIMAP-047B landed by selecting `MIMAP-048A`, the OSVM release capability
inventory.

### MIMAP-048A granularity

MIMAP-048A is an inventory / proposal row. It freezes the distinction between
the landed OSVM unreserve surface and the still-inactive OS release surface.

It may add docs and a guard proving that release remains closed, but it must not
add `hako_osvm_release*`, `release_bytes_*`, `releasePage`, fast-path release
behavior, provider activation, hooks, host allocator replacement,
`#[global_allocator]`, reclaim execution, or backend `.inc` app/name matchers.

MIMAP-048A landed by adding the OSVM release capability inventory SSOT and guard,
then selecting `MIMAP-048B`.

### MIMAP-048B granularity

MIMAP-048B is a planning-only row. It reads the MIMAP-048A inventory evidence and
selects exactly one next allocator/compiler/language task. It must not implement
allocator behavior, compiler acceptance, or cleanup by itself.

MIMAP-048B landed by selecting `MIMAP-049A`, the secure entropy source
inventory.

### MIMAP-049A granularity

MIMAP-049A is a read-only `.hako` inventory row. It names the secure
entropy/randomness boundary after secure-list encode/decode landed with
caller-provided cookies only.

It may add an inventory owner, proof app, and guard. It must not add random or
entropy extern routes, source entropy, mutate secure-list policy behavior, claim
cryptographic hardening, activate provider/hooks/host replacement, or add
backend `.inc` app/name matchers.

MIMAP-049A landed by adding the secure entropy inventory owner, VM proof app,
guard, and manifest/index/docs wiring, then selecting `MIMAP-049B`.

### MIMAP-049B granularity

MIMAP-049B is a planning-only row. It reads the MIMAP-049A inventory evidence and
selects exactly one next allocator/compiler/language task. It must not implement
allocator behavior, compiler acceptance, or cleanup by itself.

### MIMAP-050A granularity

MIMAP-050A is a planning-only row. It reads the secure entropy inventory,
`uses random` metadata contract, and unsupported random route preflight, then
chooses whether to propose a real entropy route or keep execution parked.

MIMAP-050A landed by keeping secure entropy execution parked. Current
secure-list proofs continue to use caller-provided cookies and deterministic
proof keys. The row selects MIMAP-051A rather than secure-list hardening,
because no current allocator row needs runtime entropy.

### MIMAP-051A granularity

MIMAP-051A is an allocator contract / inventory row. It names reclaim
owner-transfer preconditions before any execution is opened.

It may add a `.hako` owner, proof app, guard, and SSOT for reclaim readiness
facts. It must not mutate ownership, execute reclaim, perform atomic claims,
drain remote-free queues, schedule threads, call page-source APIs, unreserve or
release OSVM pages, activate providers, install hooks, replace the process
allocator, or add backend `.inc` app/name matchers.

MIMAP-051A landed by adding `HakoAllocReclaimOwnerTransferContract`, a proof
app, guard, and accepted SSOT. It selects MIMAP-051B.

### MIMAP-051B granularity

MIMAP-051B is a planning-only row. It reads MIMAP-051A evidence and selects
exactly one next allocator/compiler/language task. It must not implement
reclaim execution, capability checking, or cleanup by itself.

MIMAP-051B landed by selecting USES-002A.

### USES-002A granularity

USES-002A is a Hakorune core metadata row. It maps declared method-level
`uses osvm`, `uses atomic`, and `uses rawbuf` metadata to canonical MIR
CapabilityPlan ids while keeping execution unsupported unless a later route
row proves the capability.

It must not add `cap` block syntax, source-level TLS, random/entropy execution,
backend helper-name inference, reclaim execution, provider activation, hooks,
host allocator replacement, or broad capability solving.

USES-002A landed by extending `src/mir/effect_capability_plan.rs` and selecting
MIMAP-052A.

### MIMAP-052A granularity

MIMAP-052A is a planning / preflight row. It reads the reclaim owner-transfer
contract and declared capability mapping evidence, then selects exactly one
fail-fast/preflight or implementation row. It must not execute reclaim, mutate
ownership, perform atomic claims, drain remote frees, schedule threads, or call
page-source APIs by itself.

MIMAP-052A landed by selecting MIMAP-052B. The decision is that reclaim
execution must not be inferred from generic `hako.atomic` or `hako.osvm`
capabilities; it needs its own MIR-visible intent marker.

### MIMAP-052B granularity

MIMAP-052B is a fail-fast / metadata gate row. It adds the metadata-only
`uses alloc_reclaim` marker as `hako.alloc.reclaim` and an explicit pure-first
preflight option that rejects the marker before backend emission until a later
row opens reclaim execution.

It must not execute reclaim, mutate ownership, perform atomic claims, drain
remote-free queues, schedule threads, call page-source APIs, add backend
matchers, or activate allocator providers.

MIMAP-052B landed by adding the marker/preflight and selecting MIMAP-053A.

### MIMAP-053A granularity

MIMAP-053A is a planning-only row. It reads the reclaim contract and the
`hako.alloc.reclaim` preflight evidence, then selects exactly one next row:
first guarded reclaim execution, an atomic-claim contract sidecar, a
remote-free drain fail-fast row, or another no-execution allocator row.

It must not execute reclaim or mutate ownership by itself.

MIMAP-053A landed by selecting MIMAP-054A.

### MIMAP-054A granularity

MIMAP-054A is an allocator prerequisite / no-execution contract row. It proves
the owner-token atomic claim vocabulary before a future reclaim execution row
mutates page ownership.

It may add a `.hako` contract owner, proof app, guard, and SSOT. It must not
execute reclaim, mutate production page ownership, drain remote frees, schedule
threads, call page-source APIs, or activate providers.

MIMAP-054A landed by adding `HakoAllocReclaimAtomicClaimContract`, a proof app,
guard, and accepted SSOT. It selects MIMAP-055A.

### MIMAP-055A granularity

MIMAP-055A is the first guarded reclaim execution row. It may compose the
owner-transfer readiness contract and atomic-claim contract, then change only
an executor-local modeled owner token for one ready page.

It must not drain remote frees, schedule threads, call page-source APIs,
unreserve or release OSVM pages, activate providers, or add backend matchers.

MIMAP-055A landed by adding `HakoAllocReclaimOwnerTransferExecution`, a proof
app, guard, and accepted SSOT. It selects MIMAP-056A.

### MIMAP-056A granularity

MIMAP-056A is an allocator prerequisite / no-execution contract row. It should
name the reclaim remote-free drain readiness vocabulary before a later row
opens any actual drain execution.

It may add a `.hako` contract owner, proof app, guard, and SSOT. It must not
drain remote frees, schedule threads, call page-source APIs, unreserve or
release OSVM pages, activate providers, mutate production page ownership, or
add backend matchers.

MIMAP-056A landed by adding `HakoAllocReclaimRemoteFreeDrainContract`, a proof
app, guard, and accepted SSOT. It selects MIMAP-057A.

### MIMAP-057A granularity

MIMAP-057A is the first narrow modeled remote-free drain execution row. It may
compose the MIMAP-056A drain contract and execute at most one modeled
remote-free entry through a dedicated owner.

It must not schedule threads, call page-source APIs, unreserve or release OSVM
pages, activate providers, execute full reclaim, mutate production page-map
ownership outside the modeled drain owner, or add backend matchers.

MIMAP-057A landed by adding `HakoAllocReclaimRemoteFreeDrainExecution`, a proof
app, guard, and accepted SSOT. It selects MIMAP-058A.

### MIMAP-058A granularity

MIMAP-058A is a narrow integration row. It may compose the MIMAP-057A modeled
drain execution route and the MIMAP-055A modeled owner-transfer execution
route to prove the ordering around pending remote-free work.

It must not schedule threads, call page-source APIs, unreserve or release OSVM
pages, activate providers, execute full reclaim, or add backend matchers.

MIMAP-058A landed by adding `HakoAllocReclaimPostDrainOwnerTransfer`, a proof
app, guard, and accepted SSOT. It selects MIMAP-059A.

### MIMAP-059A granularity

MIMAP-059A is a planning-only row. It should select exactly one post-reclaim
integration follow-up: full reclaim success route, scalar closeout guard, or a
focused compiler/language sidecar if evidence requires it.

It must not add implementation code, full reclaim execution, scheduler
behavior, page-source calls, OSVM unreserve/release, providers, or backend
matchers.

MIMAP-059A landed by selecting MIMAP-060A.

### MIMAP-060A granularity

MIMAP-060A is a scalar reclaim completion marker route. It may compose the
MIMAP-058A post-drain owner-transfer integration route and set only an
executor-local completion marker when that route succeeds.

It must not schedule threads, call page-source APIs, unreserve or release OSVM
pages, activate providers, replace the host allocator, or add backend matchers.

MIMAP-060A landed by adding `HakoAllocReclaimCompletionMarker`, a proof app,
guard, and accepted SSOT. It selects MIMAP-061A.

### MIMAP-061A granularity

MIMAP-061A is a closeout/guard row for the landed scalar reclaim lane. It should
lock the MIMAP-051A, MIMAP-054A, MIMAP-055A, MIMAP-056A, MIMAP-057A,
MIMAP-058A, and MIMAP-060A proof rows before broader reclaim behavior is
opened.

It must not add new allocator behavior, schedule threads, call page-source
APIs, unreserve or release OSVM pages, activate providers, replace the host
allocator, or add backend matchers.

MIMAP-061A landed by adding a scalar reclaim lane closeout SSOT and guard. It
selects MIMAP-062A.

### MIMAP-062A granularity

MIMAP-062A is a planning-only row. It should select exactly one follow-up after
the scalar reclaim closeout: a narrow allocator behavior row, a scheduler /
substrate row, a language-feature row, or a compiler acceptance sidecar.

It must not add allocator behavior, schedule threads, call page-source APIs,
unreserve or release OSVM pages, activate providers, replace the host
allocator, or add backend matchers.

MIMAP-062A landed by selecting MIMAP-063A.

### MIMAP-063A granularity

MIMAP-063A is an allocator-internal reclaim scheduler boundary inventory. It
should define the boundary facts needed before broader reclaim can request
modeled scheduling, without adding real thread scheduling or source-level
concurrency semantics.

It must not add real scheduler execution, source-level `nowait`, `Channel`,
`task_scope`, `co`, `sync box`, `context`, or `worker_local` behavior, call
page-source APIs, unreserve or release OSVM pages, activate providers, replace
the host allocator, or add backend matchers.

MIMAP-063A landed by adding a scheduler boundary inventory SSOT and guard. It
selects MIMAP-064A.

### MIMAP-064A granularity

MIMAP-064A is a scalar scheduler request marker contract. It may add a `.hako`
owner that classifies whether a completed scalar reclaim result would request a
modeled scheduler handoff or stay local/suppressed.

It must not execute real scheduling, add source-level concurrency semantics,
call page-source APIs, unreserve or release OSVM pages, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-064A landed by adding `HakoAllocReclaimSchedulerRequestMarker`, a proof
app, guard, and accepted SSOT. It selects MIMAP-065A.

### MIMAP-065A granularity

MIMAP-065A is a closeout/guard row for the scheduler boundary/request marker
slice. It should lock MIMAP-063A and MIMAP-064A before broader reclaim behavior
or real scheduling is considered.

It must not add allocator behavior, execute real scheduling, add source-level
concurrency semantics, call page-source APIs, unreserve or release OSVM pages,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-065A landed by adding a scheduler marker closeout SSOT and guard. It
selects MIMAP-066A.

### MIMAP-066A granularity

MIMAP-066A is a planning-only row. It should select exactly one follow-up after
the scheduler marker closeout: allocator behavior, a real scheduler substrate
row, a language-feature row, or a compiler acceptance sidecar.

It must not add allocator behavior, execute real scheduling, add source-level
concurrency semantics, call page-source APIs, unreserve or release OSVM pages,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-066A landed by selecting MIMAP-067A.

### MIMAP-067A granularity

MIMAP-067A is a planning-only row. It decides whether to open a narrow
allocator-internal scheduler substrate implementation row, park real scheduling,
or switch to a concrete Hakorune language/compiler prerequisite.

It must not add allocator behavior, execute real scheduling, add source-level
concurrency semantics, call page-source APIs, unreserve or release OSVM pages,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-067A landed by parking real scheduler substrate for now and selecting
MIMAP-068A.

### MIMAP-068A granularity

MIMAP-068A is a narrow allocator behavior row. It may add a scalar
allocator-owned reclaim scheduler request ledger that composes the MIMAP-064A
request marker and records at most one pending modeled scheduler request.

It must not execute real scheduling, spawn workers, add source-level
concurrency semantics, call page-source APIs, unreserve or release OSVM pages,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-068A landed by adding `HakoAllocReclaimSchedulerRequestLedger`, a proof
app, guard, and accepted SSOT. It selects MIMAP-069A.

### MIMAP-069A granularity

MIMAP-069A is a closeout/guard row for the scheduler request ledger slice. It
should lock MIMAP-068A before broader reclaim behavior, real scheduler
substrate work, or language feature work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-069A landed by adding a scheduler request ledger closeout SSOT and guard.
It selects MIMAP-070A.

### MIMAP-070A granularity

MIMAP-070A is a planning-only row. It should select exactly one follow-up after
the scheduler request ledger closeout: scalar allocator behavior, real
scheduler substrate, a language feature row, or a compiler acceptance sidecar.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-070A landed by selecting MIMAP-071A.

### MIMAP-071A granularity

MIMAP-071A is a narrow allocator behavior row. It may extend
`HakoAllocReclaimSchedulerRequestLedger` with a local consume/clear route for
one pending modeled scheduler request.

It must not execute real scheduling, spawn workers, add source-level
concurrency semantics, call page-source APIs, unreserve or release OSVM pages,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-071A landed by adding a consume report/method, proof app, guard, and
accepted SSOT. It selects MIMAP-072A.

### MIMAP-072A granularity

MIMAP-072A is a closeout/guard row for the scheduler request ledger consume
route. It should lock MIMAP-071A before broader reclaim behavior, real
scheduler substrate work, or language feature work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-072A landed by adding a scheduler request ledger consume closeout SSOT
and guard. It selects MIMAP-073A.

### MIMAP-073A granularity

MIMAP-073A is a planning-only row. It should select exactly one follow-up after
the scheduler request ledger consume closeout: scalar allocator behavior, real
scheduler substrate, a language feature row, or a compiler acceptance sidecar.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-073A landed by selecting MIMAP-074A.

### MIMAP-074A granularity

MIMAP-074A is an allocator behavior row for a scalar scheduler request ledger
roundtrip. It composes the existing request ledger record and consume routes to
prove one local lifecycle:

```text
record scheduler request
  -> pending request exists
  -> consume matching page id
  -> pending request cleared
```

It must not execute real scheduling, spawn workers, add source-level
concurrency semantics, call page-source APIs, unreserve or release OSVM pages,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-074A landed by adding a scheduler request ledger roundtrip owner, proof
app, guard, and accepted SSOT. It selects MIMAP-075A.

### MIMAP-075A granularity

MIMAP-075A is a closeout/guard row for the scheduler request ledger roundtrip
route. It should lock MIMAP-074A before broader allocator behavior, real
scheduler substrate work, or language feature work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-075A landed by adding a scheduler request ledger roundtrip closeout SSOT
and guard. It selects MIMAP-076A.

### MIMAP-076A granularity

MIMAP-076A is a planning-only row. It should select exactly one follow-up after
the scheduler request ledger roundtrip closeout: scalar allocator behavior,
real scheduler substrate, a language feature row, or a compiler acceptance
sidecar.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-076A landed by selecting MIMAP-077A.

### MIMAP-077A granularity

MIMAP-077A is a closeout/guard row for the scheduler scalar lane. It should
lock the scheduler boundary, request marker, request ledger record, consume,
and roundtrip rows before broader allocator behavior, real scheduler substrate
work, or language feature work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-077A landed by adding a scheduler scalar lane closeout SSOT and guard. It
selects MIMAP-078A.

### MIMAP-078A granularity

MIMAP-078A is a planning-only row. It should select exactly one follow-up after
the scheduler scalar lane closeout: scalar allocator behavior, real scheduler
substrate, a language feature row, or a compiler acceptance sidecar.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, call page-source APIs, unreserve or release
OSVM pages, activate providers, replace the host allocator, or add backend
matchers.

MIMAP-078A landed by selecting MIMAP-079A.

### MIMAP-079A granularity

MIMAP-079A is a scalar allocator inventory row for segment / arena / bitmap
boundaries. It should name tiny proof-only facts and explicit blocked reasons
for raw pointer residence, atomic bitmap execution, OSVM execution, provider
activation, and invalid shapes.

It must not add allocation/free behavior, execute real scheduling, spawn
workers, add source-level concurrency semantics, add raw pointer residence,
execute atomic bitmap claims, call page-source APIs, unreserve or release OSVM
pages, activate providers, replace the host allocator, or add backend matchers.

MIMAP-079A landed by adding a segment / arena / bitmap inventory owner, proof
app, guard, and accepted SSOT. It selects MIMAP-080A.

### MIMAP-080A granularity

MIMAP-080A is a closeout/guard row for the segment / arena / bitmap boundary
inventory. It should lock MIMAP-079A before broader allocator behavior, real
bitmap/OSVM substrate work, or language feature work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, add raw pointer residence, execute atomic
bitmap claims, call page-source APIs, unreserve or release OSVM pages, activate
providers, replace the host allocator, or add backend matchers.

MIMAP-080A landed by adding the local-run closeout SSOT and guard for
MIMAP-079A. It selects MIMAP-081A.

### MIMAP-081A granularity

MIMAP-081A is a planning row after the segment / arena / bitmap inventory
closeout. It should review the landed allocator evidence through MIMAP-080A and
select exactly one next row.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, add raw pointer residence, execute atomic
bitmap claims, call page-source APIs, unreserve or release OSVM pages, activate
providers, replace the host allocator, or add backend matchers.

MIMAP-081A landed by selecting MIMAP-082A.

### MIMAP-082A granularity

MIMAP-082A is a scalar allocator contract row for segment lifecycle states. It
may add a `.hako` owner, proof app, local-run guard, and accepted SSOT for the
state/transition vocabulary named by the lifecycle rewrite blueprint.

It must stay proof-only and same-owner. It must not add raw pointer residence,
atomic bitmap claim/unclaim, OSVM execution, real scheduling, source-level
concurrency semantics, arena backing allocation, segment map pointer
membership, provider activation, process allocator replacement, or backend
matchers.

MIMAP-082A landed by adding a scalar segment lifecycle state owner, proof app,
guard, accepted SSOT, and manifest/module/docs wiring. It selects MIMAP-083A.

### MIMAP-083A granularity

MIMAP-083A is a closeout/guard row for the segment lifecycle scalar state
contract. It should lock MIMAP-082A before broader segment behavior, real
bitmap/OSVM substrate work, or language feature work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, add raw pointer residence, execute atomic
bitmap claims, call page-source APIs, unreserve or release OSVM pages, activate
providers, replace the host allocator, or add backend matchers.

MIMAP-083A landed by adding the local-run closeout SSOT and guard for
MIMAP-082A. It selects MIMAP-084A.

### MIMAP-084A granularity

MIMAP-084A is a planning row after the segment lifecycle scalar state closeout.
It should review the landed allocator evidence through MIMAP-083A and select
exactly one next row.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, add raw pointer residence, execute atomic
bitmap claims, call page-source APIs, unreserve or release OSVM pages, activate
providers, replace the host allocator, or add backend matchers.

MIMAP-084A landed by selecting MIMAP-085A.

### MIMAP-085A granularity

MIMAP-085A is a scalar allocator contract row for segment/page membership. It
may add a `.hako` owner, proof app, local-run guard, and accepted SSOT for
page/slice membership facts using scalar IDs only.

It must stay proof-only and same-owner. It must not add raw pointer residence,
segment-map pointer membership, arena backing allocation, atomic bitmap
claim/unclaim, OSVM execution, real scheduling, source-level concurrency
semantics, provider activation, process allocator replacement, or backend
matchers.

MIMAP-085A landed by adding a scalar segment/page membership owner, proof app,
guard, accepted SSOT, and manifest/module/docs wiring. It selects MIMAP-086A.

### MIMAP-086A granularity

MIMAP-086A is a closeout/guard row for the segment page membership scalar
contract. It should lock MIMAP-085A before broader segment behavior, real
segment-map/raw-pointer work, bitmap/OSVM substrate work, or language feature
work is selected.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, add raw pointer residence, segment-map
pointer membership, arena backing allocation, execute atomic bitmap claims,
call page-source APIs, unreserve or release OSVM pages, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-086A landed by adding the local-run closeout SSOT and guard for
MIMAP-085A. It selects MIMAP-087A.

### MIMAP-087A granularity

MIMAP-087A is a planning row after the segment/page membership closeout. It
should review the landed allocator evidence through MIMAP-086A and select
exactly one next row.

It must not add allocator behavior, execute real scheduling, spawn workers, add
source-level concurrency semantics, add raw pointer residence, segment-map
pointer membership, arena backing allocation, execute atomic bitmap claims,
call page-source APIs, unreserve or release OSVM pages, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-087A landed by selecting MIMAP-088A.

### MIMAP-088A granularity

MIMAP-088A is a scalar allocator contract row after segment/page membership. It
classifies whether a known segment/page pair is ready for a small allocation
request using only scalar facts:

```text
segment_id
page_id
segment_state
page_used / page_capacity
request block count
unsupported substrate requirement
```

It must not execute segment allocation/free, allocate arena backing, add raw
pointer residence, use a segment-map pointer lookup, execute atomic bitmap
claims, call page-source or OSVM seams, schedule threads, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-088A landed by adding a scalar readiness owner, proof app, manifest
entry, and local-run guard. It selects MIMAP-089A.

### MIMAP-089A granularity

MIMAP-089A is a closeout/guard row for the segment allocation readiness scalar
contract. It must lock MIMAP-088A owner/proof/guard wiring and inactive stop
lines before any broader segment execution row is selected.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-103A landed by selecting MIMAP-104A.

### MIMAP-104A granularity

MIMAP-104A is a modeled scalar allocator behavior row after the released-token
recycle and focused source cleanup rows. It may extend the segment allocation
modeled ledger release report with deterministic scalar span facts for a live
token release:

```text
modeled block start
request block count
modeled block end
allocation-time new page used
allocation-time remaining blocks
```

It must not execute real segment allocation/free, mutate a free-list, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-104A landed by adding release span facts to the modeled ledger release
report. It selects MIMAP-105A.

### MIMAP-105A granularity

MIMAP-105A is a planning row after the modeled segment allocation release span
facts route. It should review the landed scalar segment allocation evidence
and select exactly one next row.

It must not add allocator behavior, execute segment allocation/free, mutate a
free-list, allocate arena backing, add raw pointer residence, use segment-map
pointer lookup, execute atomic bitmap claims, call page-source or OSVM seams,
schedule threads, activate providers, replace the host allocator, or add
backend matchers.

MIMAP-105A landed by selecting MIMAP-ROW-CADENCE-001.

### MIMAP-ROW-CADENCE-001 granularity

MIMAP-ROW-CADENCE-001 is a process cleanup row. It should define validation
levels for mimalloc / hako_alloc rows so future allocator behavior rows can
choose the smallest sufficient guard set without weakening stop-line safety.

It must not add allocator behavior, parser/compiler behavior, remove landed
guards, weaken evidence, grow dev_gate / allocator-wide defaults, activate
providers, replace the host allocator, or add backend matchers.

MIMAP-ROW-CADENCE-001 landed by adding
`docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
It selects MIMAP-106A.

### MIMAP-106A granularity

MIMAP-106A is a planning row after the validation cadence SSOT. It should
select exactly one next row and cite the validation level expected for that row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-106A landed by selecting MIMAP-107A.

### MIMAP-107A granularity

MIMAP-107A is a modeled scalar allocator behavior row after release span facts.
It should add a separate released-span ledger that consumes successful
`MIMAP-104A` release reports and records deterministic token / segment / page /
block-span rows.

Validation cadence:

```text
L2 proof row
```

Allowed:

- add one released-span ledger owner;
- consume scalar `HakoAllocSegmentAllocationModeledLedgerReleaseReport` facts;
- record successful release spans as scalar rows;
- reject invalid, span-missing, duplicate, or unsupported requests;
- add one focused proof app and guard.

Forbidden:

- real segment allocation/free execution
- free-list mutation
- page state mutation outside the new scalar ledger
- arena backing allocation
- raw pointer residence
- segment-map lookup
- atomic bitmap execution
- page-source / OSVM execution
- thread scheduling or worker spawning
- source-level concurrency changes
- provider activation / hooks / host allocator replacement
- backend matchers

MIMAP-115A landed by adding the page-model local-free apply owner, proof app,
SSOT, manifest entry, module export, README entry, and local guard. It selects
MIMAP-116A.

### MIMAP-116A granularity

MIMAP-116A is a planning row after the page-model local-free apply route. It
should review the current segment allocation modeled lane and select exactly
one next row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-116A landed by selecting MIMAP-117A.

### MIMAP-117A granularity

MIMAP-117A is a closeout row for the page-model local-free apply seam through
MIMAP-115A. It should add a manifest-backed closeout guard that freezes the
selected owner, proof app, guard, SSOT, index, and stop-line set.

It must not add allocator behavior, mutate page arrays directly, add raw
pointer residence, use segment-map lookup, execute atomic bitmap claims,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-117A landed by adding the page-apply closeout SSOT, a manifest-backed
closeout guard, a public wrapper, a guard manifest entry, and check-script index
wiring. It selects MIMAP-118A.

### MIMAP-118A granularity

MIMAP-118A is a planning row after the page-model local-free apply closeout. It
should review the current segment allocation modeled lane and select exactly
one next row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-118A landed by selecting MIMAP-119A.

### MIMAP-119A granularity

MIMAP-119A is a narrow allocator behavior row after the page-model local-free
apply closeout. It should add one integration owner that composes:

```text
HakoAllocSegmentAllocationModeledReleasedSpanLedgerReport
  -> HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger
  -> HakoAllocSegmentAllocationModeledLocalFreeApplyPlan
  -> HakoAllocSegmentAllocationModeledLocalFreePageApply
```

Validation cadence:

```text
L2 proof row
```

Allowed:

- add one integration owner under `lang/src/hako_alloc/memory/`;
- require an explicit `HakoAllocPageModel`;
- record candidate / apply-plan / page-apply reports through existing owners;
- expose scalar integration report facts and inactive substrate flags;
- add one focused proof app and guard.

Forbidden:

- real segment allocation/free execution beyond the existing page-local model
- direct page array mutation outside `HakoAllocPageModel.releaseLocal`
- raw pointer residence
- segment-map lookup
- arena backing allocation
- atomic bitmap execution
- page-source / OSVM execution
- thread scheduling or worker spawning
- source-level concurrency changes
- provider activation / hooks / host allocator replacement
- backend matchers

MIMAP-119A landed by adding the local-free integration owner, proof app, SSOT,
module export, README entry, proof manifest row, and dedicated guard. It
selects MIMAP-120A.

### MIMAP-120A granularity

MIMAP-120A is a planning row after the local-free integration route. It should
review the current segment allocation modeled lane and select exactly one next
row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-120A landed by selecting MIMAP-121A.

### MIMAP-121A granularity

MIMAP-121A is a closeout row for the local-free integration seam through
MIMAP-119A. It should add a manifest-backed closeout guard that freezes the
selected owner, proof app, guard, SSOT, index, module export, README entry, and
stop-line set.

It must not add allocator behavior, mutate page arrays directly, add raw
pointer residence, use segment-map lookup, execute atomic bitmap claims,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-121A landed by adding the integration closeout SSOT, a manifest-backed
closeout guard, a public wrapper, a guard manifest entry, and check-script
index wiring. It selects MIMAP-122A.

### MIMAP-122A granularity

MIMAP-122A is a planning row after the local-free integration closeout. It
should review the current segment allocation modeled lane and select exactly
one next row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-122A landed by selecting `PURE-FIRST-GLOBAL-CALL-001`, a narrow compiler
acceptance sidecar exposed by the local-free integration proof.

### PURE-FIRST-GLOBAL-CALL-001 granularity

PURE-FIRST-GLOBAL-CALL-001 is a compiler acceptance sidecar, not an allocator
behavior row. It should accept same-module static helper global calls only when
the callee exists, arity matches, the body is supported by
`same_module_body_shape`, and a return contract can be published into
`global_call_routes` / `lowering_plan`.

It must not add allocator behavior, source syntax, cross-module global-call
widening, recursive broadening, backend app/name matchers, or silent fallback.

PURE-FIRST-GLOBAL-CALL-001 landed by adding same-module static helper
`global_call_routes` / `lowering_plan` support for supported bodies with
published scalar or object return contracts. It selects MIMAP-123A.

### MIMAP-123A granularity

MIMAP-123A is a planning row after the same-module static helper compiler
sidecar. It should review the current segment allocation modeled lane and
select exactly one next row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, backend matchers, or silent
fallback.

MIMAP-123A landed by selecting `ROUTE-FIXPOINT-001`, a BoxShape compiler
cleanup row for the route refresh convergence owner.

### ROUTE-FIXPOINT-001 granularity

ROUTE-FIXPOINT-001 is a compiler cleanup row. It should move the module route
convergence sequence out of `semantic_refresh.rs` and behind a dedicated
RouteFixpoint owner while preserving current behavior.

It must not add allocator behavior, route acceptance shapes, proof vocabulary,
preflight reason vocabulary, source syntax, backend matchers, provider
activation, host allocator replacement, or silent fallback.

ROUTE-FIXPOINT-001 landed by extracting the route refresh convergence sequence
to `src/mir/route_fixpoint.rs`. It selects ROUTE-DIAG-VOCAB-001.

### ROUTE-DIAG-VOCAB-001 granularity

ROUTE-DIAG-VOCAB-001 is a compiler cleanup row. It should define a route
diagnostics vocabulary SSOT and map existing preflight reason strings to it.

It must not add allocator behavior, route acceptance shapes, proof vocabulary,
source syntax, backend matchers, provider activation, host allocator
replacement, or silent fallback.

ROUTE-DIAG-VOCAB-001 landed by adding
`docs/reference/mir/route-diagnostics-vocabulary.md` and pointing existing MIR /
pure-first route docs to it. It selects ROUTE-DIAG-VOCAB-002.

### GUARD-MANIFEST-011 granularity

GUARD-MANIFEST-011 is a guard cleanup row. It should move one recent pure-first
`k2_wide_*` guard behind `guard_rows.toml` and `run_row_guard.sh`, keeping the
public command stable.

It must not add allocator behavior, compiler route behavior, proof app source
changes, new generators, shell `eval`, `shell=True`, dev_gate wiring, provider
activation, host allocator replacement, or silent fallback.

GUARD-MANIFEST-011 landed by moving the pure-first same-module static helper
global-call guard body to `tools/checks/impl/`, adding a `guard_rows.toml`
entry, and keeping the public `k2_wide_*` command as a thin wrapper. It selects
ROUTE-DIAG-VOCAB-001.

### ROUTE-DIAG-VOCAB-002 granularity

ROUTE-DIAG-VOCAB-002 is a compiler cleanup row. It should add one lightweight
static guard that prevents drift between `tools/checks/pure_first_route_preflight.py`
and `docs/reference/mir/route-diagnostics-vocabulary.md`.

It must not add allocator behavior, route acceptance shapes, proof vocabulary,
source syntax, backend matchers, provider activation, host allocator
replacement, broad guard generator work, or silent fallback.

ROUTE-DIAG-VOCAB-002 landed by adding a static guard that keeps
`tools/checks/pure_first_route_preflight.py` reason tokens aligned with
`docs/reference/mir/route-diagnostics-vocabulary.md`. It selects MIMAP-124A.

### MIMAP-124A granularity

MIMAP-124A is a planning-only row after the route diagnostics cleanup wave. It
should review the current segment allocation modeled lane and select exactly
one next mimalloc / hako_alloc or Hakorune compiler row.

It must not add allocator behavior, compiler route behavior, source syntax,
guard bundles, provider activation, host allocator replacement, backend
matchers, or silent fallback.

MIMAP-124A landed by selecting RUNTIME-UNWRAP-001.

### RUNTIME-UNWRAP-001 granularity

RUNTIME-UNWRAP-001 is a source cleanup row. It should replace focused production
runtime lock / global-registry `unwrap()` calls with explicit `expect(...)`
messages in `box_registry.rs`, `plugin_loader_unified.rs`, and
`unified_registry.rs`.

It must not add poison recovery policy, allocator behavior, route behavior,
source syntax, provider activation, host allocator replacement, backend
matchers, broad unwrap cleanup, or silent fallback.

RUNTIME-UNWRAP-001 landed by replacing focused production runtime
lock/global-registry `unwrap()` calls with explicit `expect(...)` messages. It
selects WASM-LOG-001.

### WASM-LOG-001 granularity

WASM-LOG-001 is a source cleanup row. It should replace emoji WAT-to-WASM debug
messages in `src/backend/wasm/mod.rs` with stable `[wasm/wat2wasm]` tags.

It must not change WAT/WASM conversion behavior, runtime logging APIs,
allocator behavior, source syntax, backend routes, or silent fallback.

WASM-LOG-001 landed by replacing WAT-to-WASM emoji debug messages with stable
`[wasm/wat2wasm]` tags. It selects MIMAP-125A.

### MIMAP-125A granularity

MIMAP-125A is a planning-only row after focused source cleanup. It should
review the current segment allocation modeled lane and select exactly one next
mimalloc / hako_alloc or Hakorune compiler row.

It must not add allocator behavior, compiler route behavior, source syntax,
cleanup bundles, provider activation, host allocator replacement, backend
matchers, or silent fallback.

MIMAP-111A landed by adding the local-free apply-plan ledger owner, proof app,
SSOT, manifest entry, module export, README entry, and local guard. It selects
MIMAP-112A.

### MIMAP-112A granularity

MIMAP-112A is a planning row after the local-free apply-plan ledger. It should
review the current segment allocation modeled lane and select exactly one next
row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-112A landed by selecting MIMAP-113A.

### MIMAP-113A granularity

MIMAP-113A is a closeout row for the scalar local-free lane through MIMAP-111A.
It should add a manifest-backed closeout guard that freezes the selected owner,
proof app, guard, SSOT, index, and stop-line set for:

```text
MIMAP-107A released-span ledger
MIMAP-109A local-free candidate ledger
MIMAP-111A local-free apply-plan ledger
```

It must not add allocator behavior, mutate a free-list or page state, execute
real segment allocation/free, add parser/compiler behavior, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-113A landed by adding the scalar local-free lane closeout SSOT, a
manifest-backed closeout guard, a public wrapper, a guard manifest entry, and
check-script index wiring. It selects MIMAP-114A.

### MIMAP-114A granularity

MIMAP-114A is a planning row after the scalar local-free closeout. It should
review the current segment allocation modeled lane and select exactly one next
row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-114A landed by selecting MIMAP-115A.

### MIMAP-115A granularity

MIMAP-115A is a narrow allocator behavior row after the scalar local-free
closeout. It should consume a successful `MIMAP-111A` apply-plan report and an
explicit `HakoAllocPageModel`, then release each block in the plan span through
`HakoAllocPageModel.releaseLocal(block_id)`.

Validation cadence:

```text
L2 proof row
```

Allowed:

- add one page-model apply route owner;
- consume scalar `HakoAllocSegmentAllocationModeledLocalFreeApplyPlanReport`
  facts;
- validate the explicit page model page id and block span;
- call only `HakoAllocPageModel.releaseLocal(block_id)` to mutate page-local
  state;
- expose scalar page used/local-free before/after counters and inactive
  substrate flags;
- add one focused proof app and guard.

Forbidden:

- real segment allocation/free execution beyond the existing page-local model
- direct page array mutation outside `HakoAllocPageModel.releaseLocal`
- raw pointer residence
- segment-map lookup
- arena backing allocation
- atomic bitmap execution
- page-source / OSVM execution
- thread scheduling or worker spawning
- source-level concurrency changes
- provider activation / hooks / host allocator replacement
- backend matchers

MIMAP-107A landed by adding the released-span ledger owner, proof app, SSOT,
manifest entry, module export, README entry, and local guard. It selects
MIMAP-108A.

### MIMAP-108A granularity

MIMAP-108A is a planning row after the released-span ledger. It should review
the current segment allocation modeled lane and select exactly one next row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-108A landed by selecting MIMAP-109A.

### MIMAP-109A granularity

MIMAP-109A is a modeled scalar allocator behavior row after the released-span
ledger. It should add a separate local-free candidate ledger that consumes
successful `MIMAP-107A` released-span reports and records deterministic page /
segment / token / block-span candidate rows.

Validation cadence:

```text
L2 proof row
```

Allowed:

- add one local-free candidate ledger owner;
- consume scalar `HakoAllocSegmentAllocationModeledReleasedSpanLedgerReport`
  facts;
- record successful local-free candidate spans as scalar rows;
- reject invalid, source-rejected, duplicate, or unsupported requests;
- add one focused proof app and guard.

Forbidden:

- real segment allocation/free execution
- free-list mutation
- page state mutation outside the new scalar candidate ledger
- arena backing allocation
- raw pointer residence
- segment-map lookup
- atomic bitmap execution
- page-source / OSVM execution
- thread scheduling or worker spawning
- source-level concurrency changes
- provider activation / hooks / host allocator replacement
- backend matchers

MIMAP-109A landed by adding the local-free candidate ledger owner, proof app,
SSOT, manifest entry, module export, README entry, and local guard. It selects
MIMAP-110A.

### MIMAP-110A granularity

MIMAP-110A is a planning row after the local-free candidate ledger. It should
review the current segment allocation modeled lane and select exactly one next
row.

It must not add allocator behavior, parser/compiler behavior, cleanup bundles,
provider activation, host allocator replacement, or backend matchers.

MIMAP-110A landed by selecting MIMAP-111A.

### MIMAP-111A granularity

MIMAP-111A is a modeled scalar allocator behavior row after the local-free
candidate ledger. It should add a separate local-free apply-plan ledger that
consumes successful `MIMAP-109A` candidate reports and records deterministic
page / segment / token / block-span apply-plan rows.

Validation cadence:

```text
L2 proof row
```

Allowed:

- add one local-free apply-plan ledger owner;
- consume scalar `HakoAllocSegmentAllocationModeledLocalFreeCandidateLedgerReport`
  facts;
- record successful local-free apply plans as scalar rows;
- reject invalid, source-rejected, duplicate, or unsupported requests;
- add one focused proof app and guard.

Forbidden:

- real segment allocation/free execution
- free-list mutation
- page state mutation outside the new scalar apply-plan ledger
- arena backing allocation
- raw pointer residence
- segment-map lookup
- atomic bitmap execution
- page-source / OSVM execution
- thread scheduling or worker spawning
- source-level concurrency changes
- provider activation / hooks / host allocator replacement
- backend matchers

### HAKO-ALLOC-SRC-CLEAN-001 granularity

HAKO-ALLOC-SRC-CLEAN-001 is a focused source cleanup sidecar for the current
segment allocation modeled lane. It may rewrite exact same-field diagnostic
counter increments in the selected segment memory owners from
`me.FIELD = me.FIELD + 1` to `me.FIELD += 1`.

It must not add language semantics, parser/compiler behavior, allocator
behavior, proof-app formatting, non-segment rewrites, real segment execution,
page-source/OSVM execution, thread scheduling, provider activation, host
allocator replacement, or backend matchers.

HAKO-ALLOC-SRC-CLEAN-001 landed by rewriting exact same-field segment counter
increments in the selected memory owners. It selects MIMAP-103A.

### MIMAP-103A granularity

MIMAP-103A is a planning row after the focused segment counter cleanup. It
should select exactly one next mimalloc / hako_alloc row.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-089A landed by adding the closeout SSOT and guard. It selects
MIMAP-090A.

### MIMAP-090A granularity

MIMAP-090A is a planning row after the segment allocation readiness closeout.
It should review the landed scalar segment evidence and select exactly one next
row.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-090A landed by selecting MIMAP-091A.

### MIMAP-091A granularity

MIMAP-091A is a modeled scalar allocator behavior row after segment allocation
readiness. It may consume accepted readiness facts and report modeled
`page_used`, `remaining_blocks`, `modeled_block_start`, and a stable scalar
modeled allocation token.

It must not execute real segment allocation/free, allocate arena backing, add
raw pointer residence, use a segment-map pointer lookup, execute atomic bitmap
claims, call page-source or OSVM seams, schedule threads, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-091A landed by adding a modeled consume owner, proof app, manifest entry,
module export, memory README entry, and local-run guard. It selects MIMAP-092A.

### MIMAP-092A granularity

MIMAP-092A is a closeout/guard row for the modeled segment allocation consume
route. It must lock MIMAP-091A owner/proof/guard wiring and inactive stop lines
before any broader segment allocation row is selected.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-092A landed by adding the closeout SSOT and manifest-backed guard. It
selects MIMAP-093A.

### MIMAP-093A granularity

MIMAP-093A is a planning row after the modeled segment allocation consume
closeout. It should review the landed scalar segment evidence and select
exactly one next row.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-093A landed by selecting MIMAP-094A.

### MIMAP-094A granularity

MIMAP-094A is a modeled scalar allocator behavior row after segment allocation
consume. It may record accepted modeled consume results into a deterministic
scalar ledger so later rows can find and reason about modeled allocation tokens.

It must not execute real segment allocation/free, allocate arena backing, add
raw pointer residence, use segment-map pointer lookup, execute atomic bitmap
claims, call page-source or OSVM seams, schedule threads, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-094A landed by adding a modeled ledger owner, proof app, manifest entry,
module export, memory README entry, accepted SSOT, and local-run guard. It
selects MIMAP-095A.

### MIMAP-095A granularity

MIMAP-095A is a closeout/guard row for the modeled segment allocation ledger
route. It must lock MIMAP-094A owner/proof/guard wiring and inactive stop lines
before any broader segment allocation row is selected.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-095A landed by adding the closeout SSOT and manifest-backed guard. It
selects MIMAP-096A.

### MIMAP-096A granularity

MIMAP-096A is a planning row after the modeled segment allocation ledger
closeout. It should review the landed scalar segment evidence and select
exactly one next row.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-096A landed by selecting MIMAP-097A.

### MIMAP-097A granularity

MIMAP-097A is a modeled scalar allocator behavior row after the modeled
allocation ledger. It may mark exactly one live modeled allocation token as
released in the ledger and expose deterministic scalar release facts.

It must not execute real segment allocation/free, allocate arena backing, add
raw pointer residence, use a segment-map pointer lookup, execute atomic bitmap
claims, call page-source or OSVM seams, schedule threads, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-097A landed by adding the ledger release report/method, proof app,
manifest entry, release SSOT, memory README entry, and local-run guard. It
selects MIMAP-098A.

### MIMAP-098A granularity

MIMAP-098A is a closeout/guard row for the modeled segment allocation ledger
release route. It must lock MIMAP-097A owner/proof/guard wiring and inactive
stop lines before any broader segment allocation/free row is selected.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-098A landed by adding the closeout SSOT, manifest row, wrapper guard,
guard index entry, and next-card pointer. It selects MIMAP-099A.

### MIMAP-099A granularity

MIMAP-099A is a planning row after the modeled segment allocation ledger release
closeout. It should review the landed scalar segment allocation evidence and
select exactly one next row.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-099A landed by selecting MIMAP-100A.

### MIMAP-100A granularity

MIMAP-100A is a modeled scalar allocator behavior row after the modeled ledger
release closeout. It proves that a released allocation token can be recorded
again as the live allocation for the same scalar block while live duplicate
tokens remain rejected.

It may add a proof app, local-run guard, and SSOT for the released-token recycle
contract. Owner changes are allowed only if the existing scalar ledger needs a
small helper to expose the contract clearly.

It must not execute real segment allocation/free, allocate arena backing, add
raw pointer residence, use segment-map pointer lookup, execute atomic bitmap
claims, call page-source or OSVM seams, schedule threads, activate providers,
replace the host allocator, or add backend matchers.

MIMAP-100A landed by adding the released-token recycle SSOT, proof app, local
guard, manifest entry, memory README entry, and a small release lookup fix that
prefers the live row before historical duplicate-release diagnostics. It
selects MIMAP-101A.

### MIMAP-101A granularity

MIMAP-101A is a closeout/guard row for the modeled segment allocation ledger
released-token recycle route. It must lock MIMAP-100A owner/proof/guard wiring
and inactive stop lines before any broader segment allocation/free row is
selected.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

MIMAP-101A landed by adding the closeout SSOT and manifest-backed guard. It
selects MIMAP-102A.

### MIMAP-102A granularity

MIMAP-102A is a planning row after the modeled segment allocation released-token
recycle closeout. It should review the landed scalar segment allocation
evidence and select exactly one next row.

It must not add allocator behavior, execute segment allocation/free, allocate
arena backing, add raw pointer residence, use segment-map pointer lookup,
execute atomic bitmap claims, call page-source or OSVM seams, schedule threads,
activate providers, replace the host allocator, or add backend matchers.

## Compiler / language sidecar triggers

| Sidecar | Trigger | Return condition |
| --- | --- | --- |
| `MIR-EMIT-SSOT-001` | pure-first preflight and EXE build can consume different MIR emissions | preflight MIR artifact is the exact EXE input artifact |
| `MIR-ROUTE-PREFLIGHT-001` | route unsupported is only discovered late in ny-llvmc / C shim output | missing/unsupported route is classified from MIR metadata before backend emission |
| `SELFHOST-PROGRESS-001` | no-output build cannot distinguish slow/stuck/unsupported route | failure/timeout log includes the active selfhost phase |
| `MIR-EMIT-SSOT-002` | guard/selfhost callers each choose their own source-to-MIR environment | canonical external emit wrapper is used by guard/selfhost callers |
| `MIR-ROW-B` | helper-call object-loop shape blocks allocator row | MIR JSON and LLVM/EXE guard pass for the minimized helper-call fixture |
| `MIR-ROW-C` | facade must return or store nullable selected object | nullable object field/return fixture passes LLVM/EXE |
| `MIR-ROW-D` | same-module object route result carries a void placeholder before nested receiver checks | route contract refines the placeholder and MIMAP-042A guard passes |
| `CONTRACT-003A` | allocator row needs runtime `assert`/`requires` semantics | runtime-check insertion guard passes |
| `TRANS-002A` | allocator row needs transition legality checks | transition legality diagnostics are guarded |
| `USES-002A` | OSVM/rawbuf/atomic route starts | unsupported capability fails fast and supported route is guarded |
| `PACKED-BACKEND-001` | allocator metadata requires packed record residence | PackedArray backend proof passes without silent fallback |

## Rule of thumb

If a row can be proven with scalar observers, keep it scalar. Do not pull
selected-object return, dense proof reads, or backend capability activation into
the row unless they are the smallest blocker.
