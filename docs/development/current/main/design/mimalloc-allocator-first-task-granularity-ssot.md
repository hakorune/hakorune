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
| `MIMAP-052A` | reclaim execution preflight proposal | selected current |

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
