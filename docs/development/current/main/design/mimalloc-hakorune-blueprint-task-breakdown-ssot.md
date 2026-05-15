---
Status: SSOT
Decision: accepted
Date: 2026-05-15
Scope: mimalloc upstream source pin, Hakorune-shaped blueprint, and port task slicing.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Hakorune Blueprint Task Breakdown SSOT

## Purpose

Use upstream mimalloc as an algorithm and lifecycle oracle without copying the C
code into the repository or forcing Hakorune to mimic C pointer structure.

The goal is:

```text
near-transcribe what is naturally structural
rewrite lifecycle ownership explicitly
record missing Hakorune expressivity as evidence-backed tasks
```

## Source Policy

The upstream source tree must stay untracked.

Recommended local path:

```text
.external/upstream/mimalloc/
```

Tracked docs may keep only:

```text
upstream URL
tag / commit SHA
download date
license summary
files inspected
concept map
gap ledger
Hakorune blueprint
task rows
```

Do not vendor upstream source into `docs/`, `src/`, `lang/`, or phase folders.

## Current Upstream Pin

`MIMAP-001` pins the upstream reference tree as local-only input:

```text
upstream:
  https://github.com/microsoft/mimalloc.git

local path:
  .external/upstream/mimalloc/

commit:
  fef6b0dd70f9d7fa0750b0d0b9fbb471203b94cd

describe:
  fef6b0d
```

Pin details and the initial source inventory window live in:

```text
docs/development/current/main/investigations/mimalloc-upstream-pin.md
```

## Current Concept Inventory

`MIMAP-002` classifies the pinned source into concept families before any
Hakorune implementation starts:

```text
docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
```

The inventory separates near-transcription candidates from lifecycle rewrites,
substrate gaps, representation gaps, and deferred unsafe surfaces.

## Current Lifecycle Blueprint

`MIMAP-003` turns the lifecycle-rewrite concepts into explicit page, block, and
segment state vocabularies:

```text
docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
```

The blueprint keeps heap/TLS, OSVM, atomic cross-thread free, raw block
residence, hooks, and global allocator replacement out of the first executable
model.

## Current Gap Ledger

`MIMAP-004` records missing substrate and representation features as explicit
future rows:

```text
docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
```

Unsupported capabilities must fail fast. The first executable slice should avoid
hard OSVM, atomic, TLS, raw pointer, and global replacement gaps.

## Current Hakorune Vocabulary

`MIMAP-005A` defines the brand/type vocabulary used by the blueprint skeleton:

```text
docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
```

Identity values are brands. Measurement units are aliases unless mixing them
would hide a lifecycle bug.

## Current Record Vocabulary

`MIMAP-005B` defines identity-free records for refs, table rows, snapshots, and
stats:

```text
docs/development/current/main/design/mimalloc-hakorune-record-vocabulary-ssot.md
```

Records do not own behavior, raw memory, delegate declarations, or lifecycle
transitions.

## Current Lifecycle Skeleton

`MIMAP-005C` defines the non-executable enum/transition skeleton:

```text
docs/development/current/main/design/mimalloc-hakorune-lifecycle-skeleton-ssot.md
```

The skeleton uses `enum`, `transition`, `requires`, `ensures`, and `uses`
metadata only. It does not implement allocator behavior.

## Current Capability Surface

`MIMAP-005D` defines the capability surface and fail-fast boundaries:

```text
docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
```

The first executable slice should avoid hard OSVM, atomic, TLS, rawbuf, random,
and provider gaps.

## Current Executable Slice

`MIMAP-006` selects the first executable near-transcription slice:

```text
docs/development/current/main/design/mimalloc-first-executable-slice-ssot.md
```

The selected first slice is `MIMAP-007 size-class / bin map executable pilot`.

## Current Size-Class Pilot

`MIMAP-007` adopts the existing executable size-class policy as the first pilot:

```text
docs/development/current/main/design/mimalloc-size-class-bin-pilot-ssot.md
```

The pilot is deliberately no-OSVM, no-atomic, no-TLS, no-rawbuf, and no-provider.

## Collection / Automata Dependency Cut

Map/Set/FST tasks are tracked separately:

```text
docs/development/current/main/design/collection-set-map-fst-task-breakdown-ssot.md
```

They are not prerequisites for the current mimalloc sequence.

```text
MIMAP-008:
  continue with records, counters, Array, and existing SizeClassBox
  do not pull Set forward unless unique membership is the blocker

MIMAP-009:
  may use existing MapBox if a dynamic lookup table is genuinely needed
  does not need FST

FST:
  not part of mimalloc port unless a static dictionary/route matcher becomes evidence-backed
```

## Port Reading

mimalloc should not be line-by-line translated.

The right unit is a lifecycle-aware structural map:

```text
mimalloc C:
  segment/page/block/heap/os/stat structures are pointer-heavy and macro-heavy

Hakorune:
  brand/type/record/enum/box/transition/uses/Result make the lifecycle explicit
```

## Classification

Every source finding should be classified as one of:

| Class | Meaning | Example |
| --- | --- | --- |
| `near-transcription` | Can be written close to the C algorithm in current Hakorune. | size-class lookup, stats counters, local scans |
| `lifecycle-rewrite` | Shape is similar, but state ownership must be explicit. | page retire/decommit/recommit/reuse |
| `substrate-gap` | Needs host/runtime capability before faithful implementation. | OS virtual memory, atomics, TLS, aligned raw allocation |
| `representation-gap` | Needs language/storage expressivity. | bitmaps, packed metadata, const tables, raw spans |
| `deferred-unsafe` | Should not be modeled until a capability/fail-fast gate exists. | global allocator replacement, hooks, provider activation |

## Row Plan

| Row | Scope | Output | Stop line |
| --- | --- | --- | --- |
| `MIMAP-001 upstream source pin` | Clone/download upstream under `.external/`, record commit/tag/license/files. | pin doc | no vendored source |
| `MIMAP-002 source concept inventory` | Inventory segment/page/block/heap/free-list/size-class/os/stats. | concept map | no Hakorune implementation |
| `MIMAP-003 lifecycle rewrite blueprint` | Map page/segment/block states to enum + transition + guard contracts. | lifecycle blueprint | no executable allocator behavior |
| `MIMAP-004 substrate and representation gap ledger` | List missing capability/language/runtime features with source evidence. | gap ledger | no speculative feature admission |
| `MIMAP-005 Hakorune blueprint skeleton` | Write non-executable `.hako` sketch using current canonical surface. | blueprint files/docs | provisional syntax must be marked |
| `MIMAP-006 first executable near-transcription slice` | Select one small slice that current Hakorune can run. | implementation card | no full allocator port |
| `MIMAP-007 size-class table pilot` | Implement size-class/bin map slice if it stays near-transcription. | executable pilot | no const evaluator dependency unless row says so |
| `MIMAP-008 page/free-list model pilot` | Implement local page/free-list model with explicit lifecycle state. | executable pilot | no OSVM/global allocator activation |
| `MIMAP-009 lifecycle integration pilot` | Connect decommit/recommit/reuse model to existing lifecycle observers. | proof app / diagnostics | no host allocator replacement |
| `MIMAP-010 page queue lifecycle selection` | Select reusable/active pages and skip unavailable pages. | executable pilot | no object-heavy facade route |
| `MIMAP-011 facade lifecycle route` | Expose lifecycle selection through a facade with LLVM/EXE primary acceptance. | proof app / guard | no provider/hook/global allocator activation |
| `MIMAP-012 object-backed lifecycle queue` | Retain page objects in a queue and return selected page object. | proof app / guard | no facade allocation behavior |
| `MIMAP-013 facade object lifecycle queue` | Compose the object queue through a thin facade and scalar observers. | proof app / guard | no selected-object return through facade |
| `MIMAP-014A single-page small allocation fast-path` | Select one reusable page through the facade and acquire one block. | proof app / guard | no release/realloc/alignment/OSVM |
| `MIMAP-014B active fallback and miss` | Prove reusable preference, active fallback, and miss reason. | proof app / guard | no release/realloc/OSVM |
| `MIMAP-014C allocation stats observers` | Add read-only allocation fast-path counters. | proof app / guard | no selection semantic change |
| `MIMAP-015A/B release route` | Release known blocks, then double/stale release fail-fast. | proof app / guard | no realloc |
| `MIMAP-016A/B alignment route` | Add alignment request metadata, then aligned allocation success/fail. | proof app / guard | no OSVM unless row says so |
| `MIMAP-017A/B realloc route` | Prove same-page shrink and grow/move behavior. | proof app / guard | no provider/global allocator |
| `MIMAP-018A stats snapshot` | Integrate allocator stats snapshot observers. | proof app / guard | no purge policy change |
| `MIMAP-019A purge/reclaim policy` | Integrate purge/reclaim/decommit policy. | proof app / guard | no OSVM activation unless explicitly selected |
| `MIMAP-020A OSVM page-source pilot` | Start capability-gated OSVM/page-source route. | proof app / guard | unsupported backend fail-fast |

## Allocator-first granularity

Implementation order and sidecar trigger rules live in:

```text
docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
```

The short rule is:

```text
allocator row first
smallest compiler/language sidecar only when blocked
return to allocator row after the sidecar guard is green
```

## Dependency with Language Rows

Before a serious executable port, the preferred prerequisites are:

```text
LOOP-003C/D:
  LoopRange facts and carrier policy

PACKED-003/004:
  source PackedArray direct-read consumption and fail-fast backend guard

CONTRACT-003A/B:
  assert / requires runtime-check insertion

TRANS-002A:
  transition legality checker

USES-002A:
  capability checker

COLL/AUTO:
  not prerequisites for MIMAP-008/009; use existing MapBox only if needed
```

The blueprint rows can start earlier because they are docs/inventory only.

## Worker Task Shape

When delegating to an agent, ask for bounded outputs:

```text
read only upstream files for one concept family
write a concept map section
list lifecycle states and transitions
classify gaps using the five classes above
do not implement code
do not copy large source snippets
```

## Stop Lines

```text
no C source vendoring
no line-by-line translation as the target design
no provider activation
no host allocator replacement
no hooks / #[global_allocator]
no silent fallback for unsupported substrate
no new language syntax without Decision docs
```

## MIMAP-008 page/free-list executable pilot

Decision: accepted.

`MIMAP-008` adopts the existing `HakoAllocPageModel` instead of introducing a
second page model. The durable owner remains:

```text
lang/src/hako_alloc/memory/page_box.hako
```

The row is fixed by the direct proof app and guard:

```text
apps/mimalloc-page-free-list-pilot-proof/main.hako
tools/checks/k2_wide_mimalloc_page_free_list_pilot_guard.sh
```

This closes the page/free-list slice for the mimalloc blueprint lane. Decommit,
recommit, and reuse integration are intentionally left to `MIMAP-009`.

## MIMAP-009 lifecycle integration pilot

Decision: accepted.

`MIMAP-009` adds page-local lifecycle state and methods to `HakoAllocPageModel`:

```text
decommit()
recommit()
canReuse()
reuse()
```

The executable proof is:

```text
apps/mimalloc-lifecycle-integration-pilot-proof/main.hako
```

This keeps OSVM, segment ownership, provider activation, and host allocator
replacement out of scope. The next mimalloc row is `MIMAP-010 page queue lifecycle
selection pilot`.

## MIMAP-010 page queue lifecycle selection pilot

Decision: accepted.

`MIMAP-010` adds a lifecycle-aware page queue owner:

```text
lang/src/hako_alloc/memory/page_queue_lifecycle_box.hako
```

It selects pages by skipping decommitted pages, explicitly reusing eligible retired
pages, then falling through to active pages. The executable proof is:

```text
apps/mimalloc-page-queue-lifecycle-selection-proof/main.hako
```

The next mimalloc row is `MIMAP-011 allocator facade lifecycle route pilot`.

## MIMAP backend acceptance policy

Decision: accepted.

Before `MIMAP-011`, backend acceptance is split deliberately:

```text
VM:
  semantic reference and small scalar policy proof
  timeout required for every MIMAP VM guard

LLVM/EXE:
  primary acceptance backend for MIMAP-011+ object-heavy page queue, heap facade,
  lifecycle, and object-return allocator routes
```

SSOT:

```text
docs/development/current/main/design/mimalloc-backend-acceptance-policy-ssot.md
docs/development/current/main/design/vm-known-limitations-ssot.md
```

`VM-LIM-001` records the object-heavy page queue/facade route limitation observed
while shaping `MIMAP-010`. This limitation is not a blocker for LLVM/EXE
acceptance, but it must not become a silent pass.

## MIMAP-011 facade lifecycle route pilot

Decision: accepted.

`MIMAP-011` exposes the lifecycle-aware scalar selection policy through
`HakoAllocProductionFacade` and proves the route with LLVM/EXE as the primary
acceptance backend.

Proof and guard:

```text
apps/mimalloc-facade-lifecycle-route-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_lifecycle_route_exe_guard.sh
```

The row intentionally does not pass page objects through queue/facade retention.
That route remains covered by `VM-LIM-001` and is reserved for `MIMAP-012`.

## MIMAP-012 object-backed lifecycle queue pilot

Decision: accepted.

`MIMAP-012` adds `HakoAllocObjectLifecyclePageQueue`, which retains
`HakoAllocPageModel` objects in `ArrayBox`, skips decommitted pages, calls
`page.canReuse()` / `page.reuse()` for retired reusable pages, selects active
pages by `page.freeCount()`, and returns the selected page object.

Acceptance backend remains LLVM/EXE primary. VM remains diagnostic-only for this
object-heavy route.

Proof and guard:

```text
apps/mimalloc-object-lifecycle-queue-proof/main.hako
tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
```

Next candidate row: `MIMAP-013 facade composition over object-backed lifecycle
queue`.
