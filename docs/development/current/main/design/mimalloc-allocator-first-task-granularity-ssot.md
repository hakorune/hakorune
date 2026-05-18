# mimalloc allocator-first task granularity SSOT

Status: SSOT
Decision: accepted
Date: 2026-05-15
Scope: allocator-first implementation order and language-feature sidecar policy.

## Current Navigation Contract

Current row and latest-card pointers are owned by:

```text
docs/development/current/main/CURRENT_STATE.toml
```

This SSOT owns durable granularity rules, stop lines, and row-selection
boundaries. It is not the live current-status ledger.

For the active phase:

```text
current row:
  MIMAP-219A

current choice boundary:
  lifecycle-token fact bridge
  or release-key migration precondition observer
  or the next modeled bridge that keeps real execution closed

closed until explicitly reopened:
  raw pointer residence
  real segment-map mutation
  real allocator free-list mutation
  arena backing
  atomic bitmap execution
  OSVM/page-source execution
  worker scheduling / source-level concurrency
  provider activation / host allocator replacement / hooks / #[global_allocator]
  cross-function Result direct ABI
  runtime sum materialization
  backend matchers
```

History-slimming rule:

```text
keep near-current MIMAP rows as full granularity text
keep older rows as stable anchors or compact table rows
do not paste latest-card history already owned by CURRENT_STATE.toml
```

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
| `MIMAP-146A` | post-report-record-cleanup row selection | landed; selected HAKO-ALLOC-RESULT-API-001 |
| `HAKO-ALLOC-RESULT-API-001` | allocator Result/Option guard-let inventory | landed; selected PURE-FIRST-GUARDLET-ENUMMATCH-001 |
| `PURE-FIRST-GUARDLET-ENUMMATCH-001` | direct MIR guard-let EnumMatchExpr acceptance | landed; selected HAKO-ALLOC-RESULT-API-002 |
| `HAKO-ALLOC-RESULT-API-002` | allocator local-free Result guard-let pilot | landed; selected MIMAP-147A |
| `MIMAP-147A` | post-Result-guard-let-pilot row selection | landed; selected HAKO-ALLOC-RESULT-API-003 |
| `HAKO-ALLOC-RESULT-API-003` | allocator local-free remaining Result guard-let boundaries | landed; selected MIMAP-148A |
| `MIMAP-148A` | post-local-free-Result-boundary row selection | landed; selected MIMAP-149A |
| `MIMAP-149A` | segment allocation blocked-substrate matrix proof | landed; selected MIMAP-150A |
| `MIMAP-150A` | post-blocked-substrate-matrix row selection | landed; selected MIMAP-151A |
| `MIMAP-151A` | segment-map scalar lookup boundary inventory | landed; selected MIMAP-152A |
| `MIMAP-152A` | post-segment-map-scalar-lookup row selection | landed; selected MIMAP-153A |
| `MIMAP-153A` | segment-map lookup guarded readiness composition | landed; selected MIMAP-154A |
| `MIMAP-154A` | post-lookup-guarded-readiness row selection | landed; selected MIMAP-155A |
| `MIMAP-155A` | segment-map readiness validation pack closeout guard | landed; selected MIMAP-156A |
| `MIMAP-156A` | post-segment-map-readiness-closeout row selection | landed; selected MIMAP-157A |
| `MIMAP-157A` | segment-map accepted readiness modeled consume ledger route | landed; selected MIMAP-158A |
| `MIMAP-158A` | segment-map modeled consume ledger diagnostics | landed; selected MIMAP-159A |
| `MIMAP-159A` | segment-map modeled consume ledger closeout pack | landed; selected MIMAP-160A |
| `MIMAP-160A..MIMAP-199A` | segment-map modeled release/recycle/local-free bridge progression | landed; summarized in phase taskboard |
| `MIMAP-200A` | segment-map local-free reuse ledger release apply bridge | landed; selected MIMAP-201A |
| `MIMAP-201A` | post-segment-map-local-free-reuse-ledger-release-apply-bridge row selection | landed; selected MIMAP-202A |
| `MIMAP-202A` | segment-map local-free reuse ledger release apply bridge closeout pack | landed; selected MIMAP-203A |
| `MIMAP-203A` | post-segment-map-local-free-reuse-ledger-release-apply-bridge-closeout row selection | landed; selected MIMAP-204A |
| `MIMAP-204A` | segment-map local-free reuse ledger release-applied recycle bridge | landed; selected MIMAP-205A |
| `MIMAP-205A` | post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge row selection | landed; selected MIMAP-206A |
| `MIMAP-206A` | segment-map local-free reuse ledger release-applied recycle bridge closeout pack | landed; selected MIMAP-207A |
| `MIMAP-207A` | post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-closeout row selection | landed; selected MIMAP-208A |
| `MIMAP-208A` | segment-map local-free reuse ledger release-applied recycle second-release diagnostic | landed; selected MIMAP-209A |
| `MIMAP-209A` | post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic row selection | landed; selected MIMAP-210A |
| `MIMAP-210A` | segment-map local-free reuse ledger release-applied recycle second-release diagnostic closeout pack | landed; selected MIMAP-211A |
| `MIMAP-211A` | post-segment-map-local-free-reuse-ledger-release-applied-recycle-second-release-diagnostic-closeout row selection | landed; selected MIMAP-212A |
| `MIMAP-212A` | segment-map local-free reuse ledger lifecycle-token pilot | landed; selected MIMAP-213A |
| `MIMAP-213A` | post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot row selection | landed; selected MIMAP-214A |
| `MIMAP-214A` | segment-map local-free reuse ledger lifecycle-token pilot closeout pack | landed; selected MIMAP-215A |
| `MIMAP-215A` | post-segment-map-local-free-reuse-ledger-lifecycle-token-pilot-closeout row selection | landed; selected MIMAP-216A |
| `MIMAP-216A` | segment-map local-free reuse ledger lifecycle-token observer diagnostic | landed; selected MIMAP-217A |
| `MIMAP-217A` | post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic row selection | landed; selected MIMAP-218A |
| `MIMAP-218A` | segment-map local-free reuse ledger lifecycle-token observer diagnostic closeout pack | landed; selected MIMAP-219A |
| `MIMAP-219A` | post-segment-map-local-free-reuse-ledger-lifecycle-token-observer-diagnostic-closeout row selection | landed; selected MIMAP-220A |
| `MIMAP-220A` | segment-map local-free reuse ledger lifecycle-token release-key precondition observer | landed; selected MIMAP-221A |
| `MIMAP-221A` | post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition row selection | landed; selected MIMAP-222A |
| `MIMAP-222A` | segment-map local-free reuse ledger lifecycle-token release-key precondition closeout pack | landed; selected MIMAP-223A |
| `MIMAP-223A` | post-segment-map-local-free-reuse-ledger-lifecycle-token-release-key-precondition-closeout row selection | landed; selected MIMAP-224A |
| `MIMAP-224A` | segment-map local-free reuse ledger lifecycle-keyed release shadow pilot | landed; selected MIMAP-225A |
| `MIMAP-225A` | post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow row selection | landed; selected MIMAP-226A |
| `MIMAP-226A` | segment-map local-free reuse ledger lifecycle-keyed release shadow closeout pack | landed; selected MIMAP-227A |
| `MIMAP-227A` | post-segment-map-local-free-reuse-ledger-lifecycle-keyed-release-shadow-closeout row selection | landed; selected MIMAP-228A |
| `MIMAP-228A` | source release-ledger lifecycle-key migration pilot | landed; selected MIMAP-229A |
| `MIMAP-229A` | source lifecycle-keyed release ledger diagnostics | landed; selected MIMAP-230A |
| `MIMAP-230A` | source release-ledger lifecycle-key migration closeout pack | selected current |


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

MIMAP-146A landed by selecting HAKO-ALLOC-RESULT-API-001, an inventory row for
applying the already-accepted Result/Option + guard-let surface to allocator
failure APIs.


### HAKO-ALLOC-RESULT-API-001 granularity

HAKO-ALLOC-RESULT-API-001 inventories scalar allocator status/reason surfaces
and decides whether one focused owner can use existing Result/Option +
guard-let language support without changing allocator behavior.

It must not add allocator behavior, broad report rewrites, implicit `?`, `try`,
`throw`, null sugar, provider activation, host allocator replacement, backend
matchers, or silent fallback.

HAKO-ALLOC-RESULT-API-001 landed by confirming that `hako_alloc` does not yet
use Result/Option/guard-let and that direct MIR currently rejects the
`EnumMatchExpr` emitted by guard-let sugar. It selects
PURE-FIRST-GUARDLET-ENUMMATCH-001.


### PURE-FIRST-GUARDLET-ENUMMATCH-001 granularity

PURE-FIRST-GUARDLET-ENUMMATCH-001 is a focused compiler acceptance row for the
existing guard-let sugar. It should lower the narrow `EnumMatchExpr` shapes
emitted by `guard let Type::Variant(binding) = expr else { ... }` in direct MIR.

It must not add broad pattern matching, implicit Result propagation, allocator
source rewrites, backend matchers, or silent fallback.

PURE-FIRST-GUARDLET-ENUMMATCH-001 landed by publishing enum metadata in direct
MIR, lowering known enum constructors to `VariantMake`, and accepting the two
guard-let generated `EnumMatchExpr` shapes. It selected
HAKO-ALLOC-RESULT-API-002.

### HAKO-ALLOC-RESULT-API-002 granularity

HAKO-ALLOC-RESULT-API-002 is the first allocator source pilot for the accepted
Result/guard-let surface. It should touch one local-free integration helper
boundary only, preserve report record fields and proof output, and keep broad
allocator report rewrites out of scope.

HAKO-ALLOC-RESULT-API-002 landed by keeping the Result boundary local to
`integrateLocalFree`, consuming it with guard-let, and adding pure-first
same-module support for local sum aggregates. Cross-function `Result` direct
ABI remains out of scope.

### MIMAP-147A granularity

MIMAP-147A is a planning row after the first Result/guard-let allocator pilot.
It decides whether the next row should extend Result to one more allocator
boundary, add a compiler sidecar for a specific missing Result shape, or return
to ordinary mimalloc behavior/proof work.

MIMAP-147A landed by selecting HAKO-ALLOC-RESULT-API-003. Cross-function
`Result` direct ABI remains closed; the next row stays inside the same
`integrateLocalFree` owner and uses only local `Result<i64, i64>` aggregates.

### HAKO-ALLOC-RESULT-API-003 granularity

HAKO-ALLOC-RESULT-API-003 converts the apply-plan and page-apply checks inside
`HakoAllocSegmentAllocationModeledLocalFreeIntegration.integrateLocalFree` to
local `Result<i64, i64>` values consumed by guard-let. It preserves the proof
output, report record fields, and HAKO-ALLOC-RESULT-API-002 candidate boundary.

It must not add cross-function `Result` direct ABI, runtime sum materialization,
implicit propagation sugar, broad allocator report rewrites, provider
activation, host allocator replacement, hooks, or backend matchers.

HAKO-ALLOC-RESULT-API-003 landed by converting the apply-plan and page-apply
local checks to local `Result<i64, i64>` guard-let boundaries. It selected
MIMAP-148A.

### MIMAP-148A granularity

MIMAP-148A is a planning row after the local-free Result guard-let cleanup. It
decides whether to stop the Result cleanup burst and return to ordinary
mimalloc behavior work, add one more allocator-local Result cleanup row, or open
a focused compiler row only if a concrete blocked Result shape appears.

MIMAP-148A landed by stopping the Result cleanup burst and selecting
MIMAP-149A.

### MIMAP-149A granularity

MIMAP-149A is a proof-only ordinary mimalloc row that names the still-closed
hard substrate blockers before real segment allocation/free can open. It should
compose already-landed scalar facts from segment allocation readiness,
segment/page membership, and segment/arena/bitmap boundary inventory.

It must report blockers without executing them: raw pointer residence,
segment-map lookup, arena backing allocation, atomic bitmap execution, OSVM,
thread scheduling, provider activation, and real segment allocation/free remain
closed.

MIMAP-149A landed by adding the proof-only matrix owner, proof app, guard, and
manifest/index/docs wiring. It selected MIMAP-150A.

### MIMAP-150A granularity

MIMAP-150A is a planning row after the blocked-substrate matrix. It chooses
exactly one next boundary from the MIMAP-149A matrix and records whether that
boundary should become allocator-only proof work, Hakorune compiler/language
acceptance work, substrate/capability inventory, or a park row.

It must not implement real segment allocation/free or open more than one
blocked substrate.

MIMAP-150A landed by selecting MIMAP-151A, a scalar segment-map lookup boundary
that keeps raw pointer residence parked.

### MIMAP-151A granularity

MIMAP-151A is a proof-only allocator row for segment-map lookup by explicit
scalar identities. It should accept a known `(segment_id, page_id, slice)` row
and reject unknown segment, wrong page, stale generation, out-of-range slice,
and raw-pointer lookup request cases with stable reasons.

It must not derive lookup identity from raw pointers or open arena backing,
atomic bitmap, OSVM, thread scheduling, provider activation, or backend matcher
behavior.

MIMAP-151A landed by adding the explicit-ID scalar lookup owner, proof app,
guard, and docs wiring. It selected MIMAP-152A.

### MIMAP-152A granularity

MIMAP-152A is a planning row after explicit-ID segment-map scalar lookup. It
chooses exactly one next follow-up: either compose the lookup into allocator
proof work or park the next boundary when it requires rawbuf, atomics, OSVM, or
scheduling substrate.

It must not implement real segment allocation/free or open another blocked
substrate.

MIMAP-152A landed by selecting MIMAP-153A.

### MIMAP-153A granularity

MIMAP-153A is a proof-only allocator row that composes explicit-ID segment-map
scalar lookup with the existing segment/page membership and allocation
readiness scalar owners. It should accept one lookup -> membership -> readiness
path and reject lookup, membership, readiness, and raw-pointer request paths
with stable reason codes.

It must not derive lookup identity from raw pointers or open real segment-map
execution, arena backing, atomic bitmap, OSVM, thread scheduling, provider
activation, or backend matcher behavior.

MIMAP-153A landed by adding the guarded readiness composition owner, proof app,
guard, and docs wiring. It selected MIMAP-154A.

### MIMAP-154A granularity

MIMAP-154A is a planning row after lookup-guarded membership/readiness. It
chooses exactly one next follow-up, preferably a small row that composes the
accepted readiness path into the existing modeled allocation consume / ledger
proof lane.

It must not implement real segment allocation/free or open another blocked
substrate.

MIMAP-154A landed by selecting MIMAP-155A, the segment-map readiness validation
pack closeout guard.

### MIMAP-155A granularity

MIMAP-155A is a closeout guard row for the explicit-ID segment-map readiness
family. It freezes the relationship between MIMAP-149A blocked-substrate
matrix, MIMAP-151A scalar lookup inventory, MIMAP-153A lookup-guarded readiness
composition, and ROW-VALIDATION-PROFILE L2 split commands.

It must not add allocator behavior, real segment-map execution, raw pointer
residence, arena backing allocation, atomic bitmap execution, OSVM, thread
scheduling, provider activation, or backend matchers.

MIMAP-155A landed by adding the closeout SSOT, manifest-backed closeout guard,
and index/taskboard wiring. It selected MIMAP-156A.

### MIMAP-156A granularity

MIMAP-156A is a planning row after segment-map readiness validation closeout.
It selected MIMAP-157A as the next small behavior row.

MIMAP-156A must not add allocator behavior, compiler acceptance, backend
lowering, or validation infrastructure.

### MIMAP-157A granularity

MIMAP-157A consumes an accepted lookup-guarded readiness report into the
existing modeled consume / ledger lane. It proves the segment-map readiness
family can feed a modeled ledger entry without opening raw pointer residence,
real segment-map execution, arena backing, atomic bitmap execution, OSVM,
thread scheduling, provider activation, cross-function `Result` direct ABI, or
backend matchers.

MIMAP-157A uses L2 daily validation. L3 EXE is deferred to a future
consume-ledger closeout pack unless this row introduces a new backend route
shape.

MIMAP-157A landed by adding the composition owner, proof app, L2 guard, module
export, manifest entry, and docs wiring. It selected MIMAP-158A.

### MIMAP-158A granularity

MIMAP-158A adds blocked / duplicate / stale diagnostics around the same modeled
consume ledger boundary opened by MIMAP-157A.

It must not open raw pointer residence, real segment-map mutation, real segment
allocation/free, arena backing, atomic bitmap execution, OSVM/page-source
execution, worker scheduling, provider activation, cross-function `Result`
direct ABI, runtime sum materialization, or backend matchers.

MIMAP-158A landed by extending the MIMAP-157A owner and proof app with scalar
diagnostic vocabulary and counters. It selected MIMAP-159A.

### MIMAP-159A granularity

MIMAP-159A is the closeout pack for MIMAP-157A/MIMAP-158A. It carries
representative L3 EXE evidence for accepted, blocked, duplicate, and stale
diagnostics while preserving the daily L2 validation split.

It must not open raw pointer residence, real segment-map mutation, real segment
allocation/free, arena backing, atomic bitmap execution, OSVM/page-source
execution, worker scheduling, provider activation, cross-function `Result`
direct ABI, runtime sum materialization, or backend matchers.

MIMAP-159A landed by adding the closeout SSOT, manifest-backed closeout guard,
and representative exact-MIR L3 EXE evidence. It selected MIMAP-160A.

### MIMAP-160A granularity

MIMAP-160A is a planning row after the segment-map modeled consume ledger
closeout. It chooses the next narrow allocator behavior, compiler acceptance,
or cleanup row.

The expected allocator direction is the modeled release/recycle ledger lane.
This row must not implement the behavior directly.

It must not open raw pointer residence, real segment-map mutation, real segment
allocation/free, arena backing, atomic bitmap execution, OSVM/page-source
execution, worker scheduling, provider activation, cross-function `Result`
direct ABI, runtime sum materialization, or backend matchers.

MIMAP-160A landed by selecting MIMAP-161A.

### MIMAP-161A granularity

MIMAP-161A adds a scalar modeled release route at the segment-map modeled
consume ledger owner boundary. It should reuse the existing
`HakoAllocSegmentAllocationModeledLedger.releaseModeledToken` substrate rather
than creating a second release ledger.

It should prove accepted release, duplicate release, missing/invalid token, and
unsupported substrate rejections while staying in scalar/model space.

It must not open real segment free execution, raw pointer residence, real
segment-map mutation, real segment allocation/free, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-161A landed by adding the release report/method, proof app, proof
manifest row, local L2 guard, and SSOT. It selected MIMAP-162A.

### MIMAP-162A granularity

MIMAP-162A is the closeout pack for MIMAP-161A. It should carry representative
L3 EXE evidence for the segment-map modeled consume ledger release route while
preserving the daily L2 validation split.

It must not open real segment free execution, raw pointer residence, real
segment-map mutation, real segment allocation/free, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-162A landed by adding the release closeout SSOT, manifest-backed closeout
guard, and representative exact-MIR L3 EXE evidence. It selected MIMAP-163A.

### MIMAP-163A granularity

MIMAP-163A is a planning row after the segment-map modeled consume-ledger
release closeout. It chooses the next narrow allocator behavior, compiler
acceptance, or cleanup row.

The next row should prefer modeled recycle or released-span observation before
raw pointer residence, arena backing, real segment-map execution, or atomic
bitmap behavior.

It must not open real segment free execution, raw pointer residence, real
segment-map mutation, real segment allocation/free, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-163A landed by selecting MIMAP-164A.

### MIMAP-164A granularity

MIMAP-164A proves released-token recycle at the segment-map modeled
consume-ledger owner boundary.

It should reuse the existing MIMAP-100A modeled ledger recycle behavior without
adding a second recycle ledger:

```text
accepted readiness
  -> consume-ledger token live
  -> release token
  -> same scalar token accepted again as a new live row
```

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-164A landed by adding the segment-map owner-boundary recycle proof app,
L2 guard, proof manifest row, and SSOT. It selected MIMAP-165A.

### MIMAP-165A granularity

MIMAP-165A is a planning row after segment-map modeled consume-ledger
released-token recycle. It should choose between a recycle closeout pack,
released-span observation at the segment-map owner boundary, or a cleanup
sidecar before opening raw pointer residence, arena backing, real segment-map
execution, or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-165A landed by selecting MIMAP-166A.

### MIMAP-166A granularity

MIMAP-166A is the closeout pack for MIMAP-164A. It should carry
representative L3 EXE evidence for the segment-map modeled consume-ledger
released-token recycle route while preserving the daily L2 validation split.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-166A landed by adding the recycle closeout SSOT, manifest-backed closeout
guard, and representative exact-MIR L3 EXE evidence. It selected MIMAP-167A.

### MIMAP-167A granularity

MIMAP-167A is a planning row after the segment-map modeled consume-ledger
released-token recycle closeout. It should choose between released-span
observation at the segment-map owner boundary, recycle diagnostics, or a
cleanup sidecar before opening raw pointer residence, arena backing, real
segment-map execution, or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-167A landed by selecting MIMAP-168A.

### MIMAP-168A granularity

MIMAP-168A proves that a successful segment-map modeled consume-ledger release
report can be observed by the existing MIMAP-107A released-span ledger. It adds
the missing scalar `modeled_block_end` to the segment-map release report and
keeps the bridge in scalar/model space.

It must not open real segment allocation/free execution, free-list mutation,
raw pointer residence, real segment-map mutation, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-168A landed by adding the released-span observation proof app, L2 guard,
proof manifest row, owner README note, and SSOT. It selected MIMAP-169A.

### MIMAP-169A granularity

MIMAP-169A is a planning row after segment-map modeled consume-ledger
released-span observation. It should choose between a released-span observation
closeout pack, local-free/free-list bridge preparation, or a cleanup sidecar
before opening raw pointer residence, arena backing, real segment-map
execution, or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-169A landed by selecting MIMAP-170A.

### MIMAP-170A granularity

MIMAP-170A is the closeout pack for MIMAP-168A. It should carry
representative L3 EXE evidence for segment-map modeled consume-ledger
released-span observation while keeping daily validation on L2.

It must not open real segment allocation/free execution, free-list mutation,
raw pointer residence, real segment-map mutation, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-170A landed by adding the released-span observation closeout SSOT,
manifest-backed closeout guard, and representative exact-MIR L3 EXE evidence.
It selected MIMAP-171A.

### MIMAP-171A granularity

MIMAP-171A is a planning row after segment-map modeled consume-ledger
released-span observation closeout. It should choose between local-free /
free-list bridge preparation, modeled free-list observation, or a cleanup
sidecar before opening raw pointer residence, arena backing, real segment-map
execution, or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-171A landed by selecting MIMAP-172A.

### MIMAP-172A granularity

MIMAP-172A proves that a released-span row produced through the segment-map
modeled consume-ledger boundary can be consumed by the existing MIMAP-109A
local-free candidate ledger. It prepares modeled free-list planning without
mutating a real free-list.

It must not open real segment allocation/free execution, free-list mutation,
raw pointer residence, real segment-map mutation, arena backing, atomic bitmap
execution, OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-172A landed by adding the local-free candidate bridge proof app, L2
guard, proof manifest row, owner README note, and SSOT. It selected MIMAP-173A.

### MIMAP-173A granularity

MIMAP-173A is a planning row after the segment-map released-span local-free
candidate bridge. It should choose between local-free apply-plan composition,
modeled free-list observation, or a cleanup sidecar before opening raw pointer
residence, arena backing, real segment-map execution, real free-list mutation,
or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-173A landed by selecting MIMAP-174A.

### MIMAP-174A granularity

MIMAP-174A is the closeout pack for MIMAP-172A. It should carry representative
L3 EXE evidence for the segment-map released-span local-free candidate bridge
while keeping daily validation on L2.

It must not open real segment allocation/free execution, real free-list
mutation, raw pointer residence, real segment-map mutation, arena backing,
atomic bitmap execution, OSVM/page-source execution, worker scheduling,
provider activation, cross-function `Result` direct ABI, runtime sum
materialization, or backend matchers.

MIMAP-174A landed by adding the local-free candidate bridge closeout SSOT,
manifest-backed closeout guard, and representative exact-MIR L3 EXE evidence.
It selected MIMAP-175A.

### MIMAP-175A granularity

MIMAP-175A is a planning row after segment-map released-span local-free
candidate bridge closeout. It should choose between local-free apply-plan
composition, modeled free-list observation, or a cleanup sidecar before opening
raw pointer residence, arena backing, real segment-map execution, real
free-list mutation, or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-175A landed by selecting MIMAP-176A.

### MIMAP-176A granularity

MIMAP-176A proves that a local-free candidate row produced through the
segment-map bridge can be consumed by the existing MIMAP-111A local-free
apply-plan ledger. It prepares modeled page/free-list planning without mutating
a real free-list or page state.

It must not open real segment allocation/free execution, free-list mutation,
page-state mutation, raw pointer residence, real segment-map mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-176A landed by adding the local-free apply-plan bridge proof app, L2
guard, proof manifest row, owner README note, and SSOT. It selected MIMAP-177A.

### MIMAP-177A granularity

MIMAP-177A is a planning row after the segment-map local-free apply-plan
bridge. It should choose between an apply-plan bridge closeout, modeled
page-apply/free-list observation, or a cleanup sidecar before opening raw
pointer residence, arena backing, real segment-map execution, real free-list
mutation, page-state mutation, or atomic bitmap behavior.

It must not open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-177A landed by selecting MIMAP-178A.

### MIMAP-178A granularity

MIMAP-178A closes the segment-map local-free apply-plan bridge pack with
representative exact-MIR L3 EXE evidence. It does not add allocator behavior;
it only proves the MIMAP-176A bridge stays stable on VM, MIR preflight, and
EXE from the exact MIR artifact.

It must not open real segment allocation/free execution, free-list mutation,
page-state mutation, raw pointer residence, real segment-map mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-178A landed by adding the apply-plan bridge closeout SSOT,
manifest-backed closeout guard, exact-MIR L3 EXE evidence, and current
pointers. It selected MIMAP-179A.

### MIMAP-179A granularity

MIMAP-179A is a planning row after the segment-map local-free apply-plan
bridge closeout. It should choose between modeled page-apply/free-list
observation, an apply-plan diagnostic/observer sidecar, or a cleanup sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-179A landed by selecting MIMAP-180A.

### MIMAP-180A granularity

MIMAP-180A proves that a segment-map-derived local-free apply-plan row can be
consumed by the existing MIMAP-115A modeled page-apply owner. The row may use
`HakoAllocPageModel.releaseLocal` through that owner, but it does not mutate a
real allocator free-list or real page state.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-180A landed by adding the segment-map local-free page-apply bridge proof
app, L2 guard, proof manifest row, owner README note, and SSOT. It selected
MIMAP-181A.

### MIMAP-181A granularity

MIMAP-181A is a planning row after the segment-map local-free page-apply
bridge. It should choose between a page-apply bridge closeout, local-free
integration observation from the segment-map chain, or a cleanup sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-181A landed by selecting MIMAP-182A.

### MIMAP-182A granularity

MIMAP-182A closes the segment-map local-free page-apply bridge pack with
representative exact-MIR L3 EXE evidence. It does not add allocator behavior;
it only proves the MIMAP-180A bridge stays stable on VM, MIR preflight, and
EXE from the exact MIR artifact.

It must not open real segment allocation/free execution, real allocator
free-list mutation, raw pointer residence, real segment-map mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-182A landed by adding the page-apply bridge closeout SSOT,
manifest-backed closeout guard, exact-MIR L3 EXE evidence, and current
pointers. It selected MIMAP-183A.

### MIMAP-183A granularity

MIMAP-183A is a planning row after the segment-map local-free page-apply
bridge closeout. It should choose between local-free integration observation
from the segment-map chain, a page-apply diagnostic/observer sidecar, or a
cleanup sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-183A landed by selecting MIMAP-184A.

### MIMAP-184A granularity

MIMAP-184A proves that a segment-map-derived released-span row can be consumed
by the existing modeled local-free integration owner. It connects the
segment-map chain to the MIMAP-119A integration route while keeping the same
scalar/model boundary:

```text
segment-map released-span row
  -> modeled local-free integration owner
```

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-184A landed by adding the segment-map local-free integration bridge proof
app, SSOT, L2 guard, proof manifest row, and current pointers. It selected
MIMAP-185A.

### MIMAP-185A granularity

MIMAP-185A is a planning row after the segment-map local-free integration
bridge. It should choose between an integration bridge closeout pack, a modeled
reuse bridge from the segment-map integration chain, or a small
diagnostic/observer sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-185A landed by selecting MIMAP-186A.

### MIMAP-186A granularity

MIMAP-186A closes the segment-map local-free integration bridge pack with
representative exact-MIR L3 EXE evidence. It adds no new allocator behavior;
it only proves the MIMAP-184A bridge stays stable on VM, MIR preflight, and
EXE.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-186A landed by adding the integration bridge closeout SSOT,
manifest-backed closeout guard, exact-MIR L3 EXE evidence, and current
pointers. It selected MIMAP-187A.

### MIMAP-187A granularity

MIMAP-187A is a planning row after the segment-map local-free integration
bridge closeout. It should choose between a modeled reuse bridge from the
segment-map integration chain, a local-free integration diagnostic/observer
sidecar, or a cleanup sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-187A landed by selecting MIMAP-188A.

### MIMAP-188A granularity

MIMAP-188A proves that a segment-map-derived released-span row can be consumed
by the existing modeled local-free reuse owner. It connects the segment-map
chain to the MIMAP-126A reuse route while keeping the same scalar/model
boundary:

```text
segment-map released-span row
  -> modeled local-free reuse owner
```

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-188A landed by adding the segment-map local-free reuse bridge proof app,
SSOT, L2 guard, proof manifest row, and current pointers. It selected
MIMAP-189A.

### MIMAP-189A granularity

MIMAP-189A is a planning row after the segment-map local-free reuse bridge. It
should choose between a reuse bridge closeout pack, a segment-map local-free
reuse ledger bridge, or a small diagnostic/observer sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-189A landed by selecting MIMAP-190A.

### MIMAP-190A granularity

MIMAP-190A closes the segment-map local-free reuse bridge pack with
representative exact-MIR L3 EXE evidence. It adds no new allocator behavior;
it only proves the MIMAP-188A bridge stays stable on VM, MIR preflight, and
EXE.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-190A landed by adding the reuse bridge closeout SSOT,
manifest-backed closeout guard, exact-MIR L3 EXE evidence, and current
pointers. It selected MIMAP-191A.

### MIMAP-191A granularity

MIMAP-191A is a planning row after the segment-map local-free reuse bridge
closeout. It should choose between a segment-map local-free reuse ledger
bridge, a local-free reuse diagnostic/observer sidecar, or the next modeled
allocator boundary.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-191A landed by selecting MIMAP-192A.

### MIMAP-192A granularity

MIMAP-192A proves that a segment-map-derived local-free reuse report can be
recorded by the existing modeled local-free reuse ledger owner. It connects the
segment-map chain to the MIMAP-130A reuse ledger route while keeping the same
scalar/model boundary:

```text
segment-map local-free reuse report
  -> modeled local-free reuse ledger owner
```

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-192A landed by adding the segment-map local-free reuse ledger bridge
proof app, SSOT, L2 guard, proof manifest row, and current pointers. It
selected MIMAP-193A.

### MIMAP-193A granularity

MIMAP-193A is a planning row after the segment-map local-free reuse ledger
bridge. It should choose between a reuse ledger bridge closeout pack, a
reuse-ledger release bridge, or a small diagnostic/observer sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-193A landed by selecting MIMAP-194A.

### MIMAP-194A granularity

MIMAP-194A closes the segment-map local-free reuse ledger bridge pack with
representative exact-MIR L3 EXE evidence. It does not add new allocator
behavior; it proves that the MIMAP-192A L2 bridge remains executable through
the pure-first EXE path from the same MIR artifact.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-194A landed by adding the segment-map local-free reuse ledger bridge
closeout SSOT, manifest-backed closeout guard, representative exact-MIR L3 EXE
evidence, and current pointers. It selected MIMAP-195A.

### MIMAP-195A granularity

MIMAP-195A is a planning row after the segment-map local-free reuse ledger
bridge closeout. It should choose between a reuse-ledger release bridge, a
ledger observer/diagnostic sidecar, or a narrow pack-level cleanup if the
closeout finds one.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-195A landed by selecting MIMAP-196A.

### MIMAP-196A granularity

MIMAP-196A proves that a segment-map-derived local-free reuse ledger row can be
recorded by the existing modeled local-free reuse ledger release owner. It
connects the segment-map chain to the MIMAP-134A release route while keeping
the same scalar/model boundary:

```text
segment-map local-free reuse ledger row
  -> modeled local-free reuse ledger release owner
```

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-196A landed by adding the segment-map local-free reuse ledger release
bridge proof app, SSOT, L2 guard, proof manifest row, and current pointers. It
selected MIMAP-197A.

### MIMAP-197A granularity

MIMAP-197A is a planning row after the segment-map local-free reuse ledger
release bridge. It should choose between a release bridge closeout pack, a
release-apply bridge, or a small observer/diagnostic sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-197A landed by selecting MIMAP-198A.

### MIMAP-198A granularity

MIMAP-198A closes the segment-map local-free reuse ledger release bridge pack
with representative exact-MIR L3 EXE evidence. It keeps daily validation on
L2 and verifies that the MIMAP-196A proof app still reaches identical VM and
EXE output from the same emitted MIR artifact.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-198A landed by adding the release bridge closeout SSOT, manifest-backed
closeout guard, representative L3 EXE evidence, and current pointers. It
selected MIMAP-199A.

### MIMAP-199A granularity

MIMAP-199A is a planning row after the segment-map local-free reuse ledger
release bridge closeout. It should choose between a release-apply bridge, a
recycle bridge, or a small observer/diagnostic sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-199A landed by selecting MIMAP-200A.

### MIMAP-200A granularity

MIMAP-200A proves that a segment-map-derived local-free reuse ledger release
row can be applied by the existing source local-free reuse ledger release apply
route. It connects the segment-map chain to the MIMAP-138A apply route while
keeping the same scalar/model boundary:

```text
segment-map local-free reuse ledger release row
  -> source local-free reuse ledger release apply owner
```

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-200A landed by adding the segment-map local-free reuse ledger release
apply bridge proof app, SSOT, L2 guard, proof manifest row, and current
pointers. It selected MIMAP-201A.

### MIMAP-201A granularity

MIMAP-201A is a planning row after the segment-map local-free reuse ledger
release apply bridge. It should choose between a release-apply bridge closeout
pack, a release-applied recycle bridge, or a small observer/diagnostic sidecar.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-201A landed by selecting MIMAP-202A.

### MIMAP-202A granularity

MIMAP-202A closes the segment-map local-free reuse ledger release apply bridge
pack with representative exact-MIR L3 EXE evidence. It keeps daily validation
on L2 and verifies that the MIMAP-200A proof app still reaches identical VM
and EXE output from the same emitted MIR artifact.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-202A landed by adding the release apply bridge closeout SSOT,
manifest-backed closeout guard, representative L3 EXE evidence, and current
pointers. It selected MIMAP-203A.

### MIMAP-203A granularity

MIMAP-203A is a planning row after the segment-map local-free reuse ledger
release apply bridge closeout. It selected MIMAP-204A, the release-applied
recycle bridge.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

### MIMAP-204A granularity

MIMAP-204A proves that a segment-map-derived local-free reuse ledger row whose
release was applied to the source reuse ledger can be recorded again as a new
live source row through the existing source-ledger recycle route.

It must reuse the existing `applyReuseLedgerRelease` and
`recordLocalFreeReuse` routes. It must not add a segment-map-specific recycle
owner, mutate real page state, or widen the bump-shaped modeled ledger.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-204A landed by adding a proof app, proof-app manifest row, L2 guard,
SSOT, check-script index entry, and memory README owner note. It selected
MIMAP-205A.

### MIMAP-205A granularity

MIMAP-205A is a planning row after the segment-map local-free reuse ledger
release-applied recycle bridge. It selected MIMAP-206A, the closeout pack for
representative exact-MIR L3 evidence before another behavior row.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

### MIMAP-206A granularity

MIMAP-206A closes the segment-map local-free reuse ledger release-applied
recycle bridge pack with representative exact-MIR L3 EXE evidence. Daily
validation for MIMAP-204A remains L2, while this closeout guard proves VM/EXE
parity from the exact MIR artifact.

It must keep the same source-ledger `applyReuseLedgerRelease` and
`recordLocalFreeReuse` route shape from MIMAP-204A. It must not add a new
allocator behavior, mutate real page/segment state, or widen the source ledger
with a segment-map-specific backend matcher.

MIMAP-206A landed by adding the closeout SSOT, manifest-backed closeout guard,
guard manifest row, check-script index entry, phase card, and current pointers.
It selected MIMAP-207A.

### MIMAP-207A granularity

MIMAP-207A is a planning row after the segment-map local-free reuse ledger
release-applied recycle bridge closeout. It selected MIMAP-208A, a diagnostic
sidecar for the one-release-per-modeled-reuse-token boundary after source
ledger recycle.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

### MIMAP-208A granularity

MIMAP-208A proves that, after the source reuse ledger recycles the same modeled
reuse token as a new live row, the release owner still rejects a second release
record for that token. This is a diagnostic row: it records the current
one-release-per-token contract before a future row decides whether to add a
generation/lifecycle token.

It must not add generation/lifecycle IDs, mutate source ledger release state,
or open real execution. It keeps L3 EXE deferred to a later closeout pack.

MIMAP-208A landed by adding the proof app, diagnostic SSOT, L2 guard,
proof-app manifest row, check-script index entry, phase card, and current
pointers. It selected MIMAP-209A.

### MIMAP-209A granularity

MIMAP-209A is a planning row after the release-applied recycle second-release
diagnostic. It selected MIMAP-210A, the closeout pack for representative
exact-MIR L3 evidence before choosing generation/lifecycle-token semantics.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

### MIMAP-210A granularity

MIMAP-210A closes the release-applied recycle second-release diagnostic pack
with representative exact-MIR L3 EXE evidence. Daily validation for MIMAP-208A
remains L2, while this closeout guard proves VM/EXE parity from the exact MIR
artifact.

It must not add generation/lifecycle IDs, mutate real allocator state, or open
raw pointer residence, real segment-map execution, or provider activation.

MIMAP-210A landed by adding the closeout SSOT, manifest-backed closeout guard,
guard manifest row, check-script index entry, phase card, and current pointers.
It selected MIMAP-211A.

### MIMAP-211A granularity

MIMAP-211A is a planning row after the release-applied recycle second-release
diagnostic closeout. It should choose between a generation/lifecycle-token
decision row, a small observer/diagnostic sidecar, or the next modeled bridge
that keeps real allocator execution closed.

It must not open real segment allocation/free execution, raw pointer
residence, real segment-map mutation, real allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime
sum materialization, or backend matchers.

MIMAP-211A landed by selecting MIMAP-212A, a lifecycle-token pilot sidecar that
keeps the release ledger key unchanged.

### MIMAP-212A granularity

MIMAP-212A adds a dedicated scalar lifecycle-token owner. It derives
`reuse_lifecycle_token = modeled_reuse_token * 1000 + lifecycle_id`, records
accepted rows, and rejects invalid-shape, duplicate, and unsupported-requirement
branches.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, or open real segment allocation/free execution, raw
pointer residence, real segment-map mutation, real allocator free-list
mutation, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, cross-function `Result` direct ABI,
runtime sum materialization, or backend matchers.

MIMAP-212A landed by adding the lifecycle-token owner, proof app, accepted
SSOT, L2 guard, proof manifest row, check-script index entry, phase cards, and
current pointers. It selected MIMAP-213A.

### MIMAP-213A granularity

MIMAP-213A is a planning row after the segment-map local-free reuse ledger
lifecycle-token pilot. It should choose between a lifecycle-token closeout pack,
a small observer/diagnostic sidecar, or the next modeled bridge that keeps real
allocator execution closed.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, or open real segment allocation/free execution, raw
pointer residence, real segment-map mutation, real allocator free-list
mutation, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, cross-function `Result` direct ABI,
runtime sum materialization, or backend matchers.

MIMAP-213A landed by selecting MIMAP-214A, the closeout pack for representative
exact-MIR L3 EXE evidence.

### MIMAP-214A granularity

MIMAP-214A closes the lifecycle-token pilot pack with representative exact-MIR
L3 EXE evidence. Daily validation for MIMAP-212A remains L2, while this
closeout guard proves VM/EXE parity from the exact MIR artifact.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, or open real segment allocation/free execution, raw
pointer residence, real segment-map mutation, real allocator free-list
mutation, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, cross-function `Result` direct ABI,
runtime sum materialization, or backend matchers.

MIMAP-214A landed by adding the closeout SSOT, manifest-backed closeout guard,
guard manifest row, check-script index entry, phase card, and current pointers.
It selected MIMAP-215A.

### MIMAP-215A granularity

MIMAP-215A is a planning row after the lifecycle-token pilot closeout. It
should choose between a lifecycle-token observer/diagnostic sidecar, connecting
lifecycle-token facts to a later modeled release/recycle row, or the next
modeled bridge that keeps real allocator execution closed.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, or open real segment allocation/free execution, raw
pointer residence, real segment-map mutation, real allocator free-list
mutation, arena backing, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, cross-function `Result` direct ABI,
runtime sum materialization, or backend matchers.

MIMAP-215A landed by selecting MIMAP-216A, a lifecycle-token observer
diagnostic sidecar.

### MIMAP-216A granularity

MIMAP-216A adds a dedicated observer that reads lifecycle-token pilot state and
the release-owner duplicate diagnostic. It reports that lifecycle-token facts
exist while the release ledger remains keyed by modeled reuse token.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, mutate source reuse ledger or release owner state,
or open real segment allocation/free execution, raw pointer residence, real
segment-map mutation, real allocator free-list mutation, arena backing, atomic
bitmap execution, OSVM/page-source execution, worker scheduling, provider
activation, cross-function `Result` direct ABI, runtime sum materialization, or
backend matchers.

MIMAP-216A landed by adding the observer owner, proof app, accepted SSOT, L2
guard, proof manifest row, check-script index entry, phase cards, and current
pointers. It selected MIMAP-217A.

### MIMAP-217A granularity

MIMAP-217A is a planning row after the lifecycle-token observer diagnostic. It
should choose between an observer closeout pack, connecting lifecycle-token
facts to a later modeled release/recycle row, or the next modeled bridge that
keeps real allocator execution closed.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, mutate source reuse ledger or release owner state,
or open real segment allocation/free execution, raw pointer residence, real
segment-map mutation, real allocator free-list mutation, arena backing, atomic
bitmap execution, OSVM/page-source execution, worker scheduling, provider
activation, cross-function `Result` direct ABI, runtime sum materialization, or
backend matchers.

MIMAP-217A landed by selecting MIMAP-218A, the closeout pack for representative
exact-MIR L3 EXE evidence.

### MIMAP-218A granularity

MIMAP-218A closes the lifecycle-token observer diagnostic pack with
representative exact-MIR L3 EXE evidence. Daily validation for MIMAP-216A
remains L2, while this closeout guard proves VM/EXE parity from the exact MIR
artifact.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, mutate source reuse ledger or release owner state,
or open real segment allocation/free execution, raw pointer residence, real
segment-map mutation, real allocator free-list mutation, arena backing, atomic
bitmap execution, OSVM/page-source execution, worker scheduling, provider
activation, cross-function `Result` direct ABI, runtime sum materialization, or
backend matchers.

MIMAP-218A landed by adding the closeout SSOT, manifest-backed closeout guard,
guard manifest row, check-script index entry, phase card, and current pointers.
It selected MIMAP-219A.

### MIMAP-219A granularity

MIMAP-219A is a planning row after the lifecycle-token observer diagnostic
closeout. It should choose between connecting lifecycle-token facts to a later
modeled release/recycle row, adding a release-key migration precondition
observer, or the next modeled bridge that keeps real allocator execution closed.

It must not migrate release-ledger keys, define generation/lifecycle semantics
for real allocator cycles, mutate source reuse ledger or release owner state,
or open real segment allocation/free execution, raw pointer residence, real
segment-map mutation, real allocator free-list mutation, arena backing, atomic
bitmap execution, OSVM/page-source execution, worker scheduling, provider
activation, cross-function `Result` direct ABI, runtime sum materialization, or
backend matchers.

MIMAP-219A landed by selecting MIMAP-220A, the lifecycle-token release-key
precondition observer.

### MIMAP-220A granularity

MIMAP-220A adds a scalar precondition observer that classifies whether the
accepted lifecycle-token observer facts are sufficient for a future row to
consider release-ledger key migration. It reports `migration_candidate = 1`
only when the observer accepted, a duplicate release was seen, and
`lifecycle_count >= 2`.

This row must keep `would_migrate_release_ledger_key = 0`. It must not migrate
release-ledger keys, define real generation/lifecycle semantics, mutate source
reuse ledger or release owner state, or open real segment allocation/free
execution, raw pointer residence, real segment-map mutation, allocator
free-list mutation, arena backing, atomic bitmap execution, OSVM/page-source
execution, worker scheduling, provider activation, cross-function `Result`
direct ABI, runtime sum materialization, or backend matchers.

MIMAP-220A uses daily L2 validation and defers representative exact-MIR L3 EXE
evidence to a future release-key precondition closeout pack. It landed by
adding the owner, proof app, accepted SSOT, L2 guard, manifest/index rows, and
current pointers. It selected MIMAP-221A.

### MIMAP-221A granularity

MIMAP-221A is a planning row after the lifecycle-token release-key precondition
observer. It should choose whether to close the precondition observer pack, add
one more blocked-precondition diagnostic, or continue toward a later modeled
release/recycle bridge while real allocator execution remains closed.

It must not migrate release-ledger keys unless the next row explicitly selects
that row. It must keep real generation/lifecycle semantics, real segment
allocation/free execution, raw pointer residence, real segment-map mutation,
allocator free-list mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, and backend
matchers closed.

MIMAP-221A landed by selecting MIMAP-222A, the release-key precondition closeout
pack.

### MIMAP-222A granularity

MIMAP-222A closes the lifecycle-token release-key precondition pack with
representative exact-MIR L3 EXE evidence. Daily behavior remains owned by
MIMAP-220A and its L2 guard; MIMAP-222A proves the same exact MIR artifact can
lower to an executable and produce the same precondition diagnostic output.

It must not migrate release-ledger keys, define real generation/lifecycle
semantics, mutate source reuse ledger or release owner state, or open real
segment allocation/free execution, raw pointer residence, real segment-map
mutation, allocator free-list mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, or backend
matchers.

MIMAP-222A landed by adding the closeout SSOT, manifest-backed closeout guard,
guard manifest row, check-script index entry, phase card, and current pointers.
It selected MIMAP-223A.

### MIMAP-223A granularity

MIMAP-223A is a planning row after the lifecycle-token release-key precondition
closeout. It should choose whether to keep release-key migration parked and move
to another modeled bridge, add a narrower pre-migration diagnostic, or
explicitly select a release-ledger key migration row.

It must not migrate release-ledger keys unless the next row explicitly selects
that row. It must keep real generation/lifecycle semantics, real segment
allocation/free execution, raw pointer residence, real segment-map mutation,
allocator free-list mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, and backend
matchers closed.

MIMAP-223A landed by selecting MIMAP-224A, the lifecycle-keyed release shadow
pilot.

### MIMAP-224A granularity

MIMAP-224A adds a shadow release ledger keyed by `reuse_lifecycle_token`. It
consumes the MIMAP-220A release-key precondition report and an accepted
lifecycle-token report, then records a shadow row only when the modeled reuse
token matches and the lifecycle-keyed row is not already present.

This row must not migrate the source release ledger key. It must keep the source
release owner keyed by modeled reuse token, keep real generation/lifecycle
semantics closed, and avoid real segment allocation/free execution, raw pointer
residence, real segment-map mutation, allocator free-list mutation, arena
backing, atomic bitmap execution, OSVM/page-source execution, worker scheduling,
provider activation, cross-function `Result` direct ABI, runtime sum
materialization, and backend matchers.

MIMAP-224A uses daily L2 validation and defers representative exact-MIR L3 EXE
evidence to a future lifecycle-keyed release shadow closeout pack. It landed by
adding the owner, proof app, accepted SSOT, L2 guard, manifest/index rows, and
current pointers. It selected MIMAP-225A.

### MIMAP-225A granularity

MIMAP-225A is a planning row after the lifecycle-keyed release shadow pilot. It
should choose whether to close the shadow-ledger pack, add one more
shadow-ledger diagnostic, or continue toward a modeled release/recycle bridge
while source release-ledger migration remains closed.

It must not migrate the source release ledger key unless the next row
explicitly selects that row. It must keep real generation/lifecycle semantics,
real segment allocation/free execution, raw pointer residence, real segment-map
mutation, allocator free-list mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, and backend
matchers closed.

MIMAP-225A landed by selecting MIMAP-226A, the lifecycle-keyed release shadow
closeout pack.

### MIMAP-226A granularity

MIMAP-226A closes the lifecycle-keyed release shadow pack with representative
exact-MIR L3 EXE evidence. Daily behavior remains owned by MIMAP-224A and its
L2 guard; MIMAP-226A proves the same exact MIR artifact can lower to an
executable and produce the same shadow ledger output.

It must not migrate the source release ledger key, define real
generation/lifecycle semantics, mutate source reuse ledger or release owner
state, or open real segment allocation/free execution, raw pointer residence,
real segment-map mutation, allocator free-list mutation, arena backing, atomic
bitmap execution, OSVM/page-source execution, worker scheduling, provider
activation, cross-function `Result` direct ABI, runtime sum materialization, or
backend matchers.

MIMAP-226A landed by adding the closeout SSOT, manifest-backed closeout guard,
guard manifest row, check-script index entry, phase card, and current pointers.
It selected MIMAP-227A.

### MIMAP-227A granularity

MIMAP-227A is a planning row after the lifecycle-keyed release shadow closeout.
It should select the controlled source release-ledger lifecycle-key migration
pilot unless a closeout-only blocker is found.

It must not migrate the source release ledger key unless the next row
explicitly selects that row. It must keep real generation/lifecycle semantics,
real segment allocation/free execution, raw pointer residence, real segment-map
mutation, allocator free-list mutation, arena backing, atomic bitmap execution,
OSVM/page-source execution, worker scheduling, provider activation,
cross-function `Result` direct ABI, runtime sum materialization, and backend
matchers closed.

MIMAP-227A landed by selecting MIMAP-228A.

### MIMAP-228A granularity

MIMAP-228A introduces a new lifecycle-keyed source release ledger owner keyed by
`reuse_lifecycle_token`. It keeps the old modeled-reuse-token keyed release
owner unchanged and preserves `modeled_reuse_token` as a backref field in the
new owner report.

This is the first source-key migration pattern, so it uses first-pattern L3
validation. It proves one accepted lifecycle-keyed release row, duplicate
reject, precondition reject, lifecycle-report reject, modeled/lifecycle token
mismatch reject, unsupported-requirement reject, MIR report shape, route
preflight, and exact-MIR EXE parity.

It must not define real generation/lifecycle semantics, mutate old source
release owner state in place, or open real segment allocation/free execution,
raw pointer residence, real segment-map mutation, allocator free-list mutation,
arena backing, atomic bitmap execution, OSVM/page-source execution, worker
scheduling, provider activation, cross-function `Result` direct ABI, runtime sum
materialization, or backend matchers.

MIMAP-228A landed by adding the lifecycle-keyed source release ledger owner,
proof app, first-pattern L3 guard, proof manifest entry, check-script index
entry, accepted design SSOT, phase card, and current pointers. It selected
MIMAP-229A.

### MIMAP-229A granularity

MIMAP-229A is the diagnostics row after the source release-ledger lifecycle-key
migration pilot. It should keep the same route shape and add narrower duplicate
lifecycle-key, stale/mismatched lifecycle-report, and migrated-key reject
summary coverage before the source-key migration closeout pack.

It must keep real allocator execution, raw pointer residence, arena backing,
real segment-map mutation, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, cross-function `Result` direct ABI,
runtime sum materialization, and backend matchers closed.

MIMAP-229A landed by adding an observer-only diagnostics owner, proof app, L2
guard, proof manifest entry, check-script index entry, accepted design SSOT,
phase card, and current pointers. It selected MIMAP-230A.

### MIMAP-230A granularity

MIMAP-230A is the closeout pack for the source release-ledger lifecycle-key
migration family. It should provide representative exact-MIR L3 EXE evidence
for the MIMAP-228A source-key migration and MIMAP-229A diagnostics before the
next release/recycle lifecycle continuation bridge is selected.

It must keep real allocator execution, raw pointer residence, arena backing,
real segment-map mutation, atomic bitmap execution, OSVM/page-source execution,
worker scheduling, provider activation, cross-function `Result` direct ABI,
runtime sum materialization, and backend matchers closed.


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
