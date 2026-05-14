---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: mimalloc upstream source pin, Hakorune-shaped blueprint, and port task slicing.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
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

