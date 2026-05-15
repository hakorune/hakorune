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

MIMAP-023B is the next planning-only row. It must pick exactly one
post-huge-page-model allocator behavior slice and record the owner, proof app,
guard, and stop lines before implementation begins.

## Compiler / language sidecar triggers

| Sidecar | Trigger | Return condition |
| --- | --- | --- |
| `MIR-ROW-B` | helper-call object-loop shape blocks allocator row | MIR JSON and LLVM/EXE guard pass for the minimized helper-call fixture |
| `MIR-ROW-C` | facade must return or store nullable selected object | nullable object field/return fixture passes LLVM/EXE |
| `MIR-ROW-D` | dense observer proof reads block MIR emit | dense read fixture passes without broadening allocator row |
| `CONTRACT-003A` | allocator row needs runtime `assert`/`requires` semantics | runtime-check insertion guard passes |
| `TRANS-002A` | allocator row needs transition legality checks | transition legality diagnostics are guarded |
| `USES-002A` | OSVM/rawbuf/atomic route starts | unsupported capability fails fast and supported route is guarded |
| `PACKED-BACKEND-001` | allocator metadata requires packed record residence | PackedArray backend proof passes without silent fallback |

## Rule of thumb

If a row can be proven with scalar observers, keep it scalar. Do not pull
selected-object return, dense proof reads, or backend capability activation into
the row unless they are the smallest blocker.
