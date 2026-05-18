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
| `MIMAP-125A` | post-source-cleanup row selection | landed; selected MIMAP-126A |
| `MIMAP-126A` | segment allocation modeled local-free reuse route | landed; selected MIMAP-127A |
| `MIMAP-127A` | post-local-free-reuse row selection | landed; selected MIMAP-128A |
| `MIMAP-128A` | segment allocation modeled local-free reuse closeout guard | landed; selected MIMAP-129A |
| `MIMAP-129A` | post-local-free-reuse-closeout row selection | landed; selected MIMAP-130A |
| `MIMAP-130A` | segment allocation modeled local-free reuse ledger route | landed; selected MIMAP-131A |
| `MIMAP-131A` | post-local-free-reuse-ledger row selection | landed; selected MIMAP-132A |
| `MIMAP-132A` | segment allocation modeled local-free reuse ledger closeout guard | landed; selected MIMAP-133A |
| `MIMAP-133A` | post-local-free-reuse-ledger-closeout row selection | landed; selected MIMAP-134A |
| `MIMAP-134A` | segment allocation modeled local-free reuse ledger release route | landed; selected MIMAP-135A |
| `MIMAP-135A` | post-local-free-reuse-ledger-release row selection | landed; selected MIMAP-136A |
| `MIMAP-136A` | segment allocation modeled local-free reuse ledger release closeout guard | landed; selected MIMAP-137A |
| `MIMAP-137A` | post-local-free-reuse-ledger-release-closeout row selection | landed; selected MIMAP-138A |
| `MIMAP-138A` | segment allocation modeled local-free reuse ledger release apply route | landed; selected MIMAP-139A |
| `MIMAP-139A` | segment allocation modeled local-free reuse ledger release apply closeout guard | landed; selected MIMAP-140A |
| `MIMAP-140A` | post-local-free-reuse-ledger-release-apply-closeout row selection | landed; selected GUARD-MANIFEST-012 |
| `GUARD-MANIFEST-012` | guard manifest batch migration inventory | landed; selected GUARD-MANIFEST-013 |
| `GUARD-MANIFEST-013` | declarative guard spec pilot | landed; selected MIMAP-141A |
| `MIMAP-141A` | post-guard-spec-pilot row selection | landed; selected MIMAP-142A |
| `MIMAP-142A` | segment allocation modeled local-free reuse ledger release-applied recycle proof | landed; selected MIMAP-143A |
| `MIMAP-143A` | segment allocation modeled local-free reuse ledger release-applied recycle closeout guard | landed; selected MIMAP-144A |
| `MIMAP-144A` | post-release-applied-recycle-closeout row selection | landed; selected HAKO-ALLOC-ID-BRAND-001 |
| `HAKO-ALLOC-ID-BRAND-001` | allocator scalar ID brand application inventory | landed; selected PURE-FIRST-BRAND-CONSTRUCT-001 |
| `PURE-FIRST-BRAND-CONSTRUCT-001` | brand constructor MIR acceptance | landed; selected HAKO-ALLOC-ID-BRAND-002 |
| `HAKO-ALLOC-ID-BRAND-002` | allocator scalar ID brand first pilot | landed; selected HAKO-ALLOC-ID-BRAND-003 |
| `HAKO-ALLOC-ID-BRAND-003` | allocator scalar ID brand pilot closeout guard | landed; selected MIMAP-145A |
| `MIMAP-145A` | post-ID-brand-pilot-closeout row selection | landed; selected HAKO-ALLOC-REPORT-RECORD-001 |
| `HAKO-ALLOC-REPORT-RECORD-001` | allocator report record cleanup inventory | landed; selected HAKO-ALLOC-REPORT-RECORD-002 |
| `HAKO-ALLOC-REPORT-RECORD-002` | local-free integration report record boundary cleanup | landed; selected MIMAP-146A |
| `MIMAP-146A` | post-report-record-cleanup row selection | selected current |


## Detailed Granularity Ledger Split

The historical per-row prose moved to:

```text
docs/development/current/main/design/archive/mimalloc-allocator-first-task-granularity-full-ledger-2026-05-18.md
```

This SSOT keeps the active decision, stop lines, current implementation slices,
the follow-up row table, and guard-compatible granularity anchors. New row
details should live in the active card / row-specific SSOT, not as another
long landed-history prose block here.

## Current Detailed Granularity

### MIMAP-142A granularity

MIMAP-142A is a modeled allocator behavior row after the release apply closeout.
It should prove that a modeled local-free reuse token whose source ledger row was
release-applied can be recorded again as a new live source row, while a
still-live duplicate remains rejected.

It may add a focused proof app, manifest row, guard, SSOT, and small owner helper
only if needed to expose the existing source ledger contract.

It must not execute real segment allocation/free, mutate real page arrays, use a
segment-map pointer lookup, allocate arena backing, execute atomic bitmap claims,
call page-source or OSVM seams, schedule threads, activate providers, replace
the host allocator, add backend matchers, or silently fallback.

MIMAP-142A landed by adding a proof app, proof-app manifest row, route guard,
SSOT, check-script index entry, and memory README owner note. It selects
MIMAP-143A.


### MIMAP-143A granularity

MIMAP-143A is a closeout guard row for the release-applied local-free reuse
ledger token recycle proof. It should freeze MIMAP-142A owner/proof/docs/guard
wiring and inactive stop lines before any broader allocator row is selected.

It must not add allocator behavior, compiler route behavior, source syntax,
provider activation, host allocator replacement, backend matchers, or silent
fallback.

MIMAP-143A landed by adding a closeout SSOT, manifest-backed guard, thin
wrapper, and check-script index entry. It selects MIMAP-144A.


### MIMAP-144A granularity

MIMAP-144A is a planning-only row after the release-applied local-free reuse
ledger token recycle closeout. It should read MIMAP-143A evidence and select
exactly one next allocator / compiler / language task.

It must not implement allocator behavior, compiler route behavior, source
syntax, provider activation, host allocator replacement, backend matchers, or
silent fallback by itself.

MIMAP-144A landed by selecting HAKO-ALLOC-ID-BRAND-001, a language/allocator
boundary row that inventories existing allocator scalar IDs against the already
accepted brand/type vocabulary before any broader allocator execution row.


### HAKO-ALLOC-ID-BRAND-001 granularity

HAKO-ALLOC-ID-BRAND-001 is a language/allocator boundary row after MIMAP-144A.
It should inventory page/block/segment/token scalar IDs in the current
`hako_alloc` modeled allocator lane, classify where existing `brand` / `type`
semantics can be applied, and decide whether a first source pilot is already
covered by current Stage1 brand checks.

It must not add allocator behavior, new brand syntax, broad type-system
semantics, field/return/cross-module brand inference, provider activation, host
allocator replacement, backend matchers, or silent fallback.

HAKO-ALLOC-ID-BRAND-001 landed by documenting the allocator scalar ID candidates
and selecting PURE-FIRST-BRAND-CONSTRUCT-001 because direct MIR currently treats
declared brand constructors such as `BlockId(7)` as unresolved function calls.


### PURE-FIRST-BRAND-CONSTRUCT-001 granularity

PURE-FIRST-BRAND-CONSTRUCT-001 is a focused compiler acceptance row. It should
collect declared brand names in the direct MIR source route and lower
`BrandName(value)` as a transparent single-value wrapper when the brand is known.

It must not add full brand type checking, field/return/typed-local propagation,
cross-module inference, allocator behavior, provider activation, host allocator
replacement, backend matchers, or silent fallback.

PURE-FIRST-BRAND-CONSTRUCT-001 landed by registering direct MIR brand
declarations and lowering declared `BrandName(value)` constructors as
transparent single-value wrappers. It selects HAKO-ALLOC-ID-BRAND-002.


### HAKO-ALLOC-ID-BRAND-002 granularity

HAKO-ALLOC-ID-BRAND-002 is the first allocator source pilot for existing brand
semantics. It should add `SegmentId`, `PageId`, and `BlockId` only at one
same-box helper boundary in the local-free reuse ledger owner, fed by explicit
brand constructors.

It must not add field/return/typed-local/cross-module brand inference, token
brand vocabulary expansion, allocator behavior, provider activation, host
allocator replacement, backend matchers, or silent fallback.

HAKO-ALLOC-ID-BRAND-002 landed by adding `SegmentId`, `PageId`, and `BlockId`
to the local-free reuse ledger owner and applying them only to the
`makeReuseToken(...)` helper boundary. It selects HAKO-ALLOC-ID-BRAND-003.


### HAKO-ALLOC-ID-BRAND-003 granularity

HAKO-ALLOC-ID-BRAND-003 is a closeout guard row for the first allocator scalar
ID brand pilot. It should freeze the `SegmentId` / `PageId` / `BlockId`
declarations, the `makeReuseToken(...)` brand-typed parameter boundary, and the
explicit constructor call site while keeping token storage and reports scalar.

It must not add allocator behavior, field/return/typed-local/cross-module brand
inference, provider activation, host allocator replacement, backend matchers, or
silent fallback.

HAKO-ALLOC-ID-BRAND-003 landed by adding a manifest-backed closeout guard for
the first allocator scalar ID brand pilot. It selects MIMAP-145A.


### MIMAP-145A granularity

MIMAP-145A is a planning-only row after the scalar ID brand pilot closeout. It
should read the closeout evidence and select exactly one next allocator,
Hakorune core, or BoxShape cleanup row.

It must not implement allocator behavior, compiler route behavior, source
syntax, provider activation, host allocator replacement, backend matchers, or
silent fallback by itself.

MIMAP-145A landed by selecting HAKO-ALLOC-REPORT-RECORD-001, an inventory row
for allocator proof report record cleanup.


### HAKO-ALLOC-REPORT-RECORD-001 granularity

HAKO-ALLOC-REPORT-RECORD-001 inventories current wide allocator proof report
shapes and decides whether existing record support can safely reduce one report
surface without changing behavior.

It must not add allocator behavior, broad report rewrites, packed/backend record
lowering, provider activation, host allocator replacement, backend matchers, or
silent fallback.

HAKO-ALLOC-REPORT-RECORD-001 landed by selecting the widest focused report
helper boundary: `segment_allocation_modeled_local_free_integration_box.hako`
`report(...)` with 22 scalar arguments. It selects HAKO-ALLOC-REPORT-RECORD-002.


### HAKO-ALLOC-REPORT-RECORD-002 granularity

HAKO-ALLOC-REPORT-RECORD-002 replaces the local-free integration report/22
helper boundary with owner-local record payload construction/read, while keeping
the returned report box and proof output unchanged.

It must not add allocator behavior, broad report rewrites, record-payload pass,
return, or store escape, packed/backend record lowering, provider activation,
host allocator replacement, backend matchers, or silent fallback.

HAKO-ALLOC-REPORT-RECORD-002 landed by adding
`HakoAllocSegmentAllocationModeledLocalFreeIntegrationReportFields`, replacing
the legacy scalar `report(...)` helper, and extending the MIMAP-119A guard to
reject the old helper boundary. It selects MIMAP-146A.


### MIMAP-146A granularity

MIMAP-146A is a planning-only row after the report-record cleanup. It should
read HAKO-ALLOC-REPORT-RECORD-002 evidence and select exactly one next
allocator, Hakorune core, or BoxShape cleanup row.

It must not implement allocator behavior, compiler route behavior, source
syntax, provider activation, host allocator replacement, backend matchers, or
silent fallback by itself.


## Historical Granularity Anchors

These headings are retained so older guard rows that assert `MIMAP-* granularity`
keep a stable anchor while the full historical prose is archived.

### MIMAP-020A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-021A / MIMAP-021B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-022A / MIMAP-022B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-023A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-024A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-025A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-026A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-027A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-042A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-042B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-043A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-043B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-044A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-044B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-045A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-045B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-046A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-046B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-047A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-047B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-048A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-048B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-049A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-049B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-050A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-051A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-051B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### USES-002A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-052A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-052B granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-053A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-054A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-055A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-056A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-057A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-058A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-059A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-060A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-061A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-062A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-063A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-064A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-065A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-066A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-067A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-068A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-069A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-070A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-071A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-072A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-073A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-074A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-075A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-076A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-077A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-078A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-079A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-080A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-081A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-082A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-083A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-084A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-085A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-086A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-087A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-088A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-089A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-104A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-105A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-ROW-CADENCE-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-106A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-107A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-116A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-117A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-118A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-119A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-120A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-121A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-122A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### PURE-FIRST-GLOBAL-CALL-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-123A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### ROUTE-FIXPOINT-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### ROUTE-DIAG-VOCAB-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### GUARD-MANIFEST-011 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### ROUTE-DIAG-VOCAB-002 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-124A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### RUNTIME-UNWRAP-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### WASM-LOG-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-125A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-126A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-127A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-128A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-129A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-130A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-131A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-132A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-133A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-134A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-135A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-136A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-137A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-138A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-139A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-140A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### GUARD-MANIFEST-012 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### GUARD-MANIFEST-013 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-141A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-112A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-113A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-114A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-115A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-108A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-109A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-110A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-111A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### HAKO-ALLOC-SRC-CLEAN-001 granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-103A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-090A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-091A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-092A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-093A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-094A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-095A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-096A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-097A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-098A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-099A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-100A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-101A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.

### MIMAP-102A granularity

Historical detail moved to the full ledger archive. The row remains summarized
in the follow-up allocator rows table above.
