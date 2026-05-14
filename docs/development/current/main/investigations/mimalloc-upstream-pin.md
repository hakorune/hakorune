---
Status: Active
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-001 upstream mimalloc source pin and untracked local checkout policy.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/phases/phase-293x/293x-331-MIMAP-001-UPSTREAM-SOURCE-PIN.md
---

# mimalloc Upstream Source Pin

## Decision

Use upstream mimalloc as a local-only reference tree and keep all copied source
out of git.

```text
local path:
  .external/upstream/mimalloc/

tracked output:
  docs only
```

`.external/` is ignored by git before the checkout is used.

## Pin Snapshot

| Field | Value |
| --- | --- |
| Upstream | `https://github.com/microsoft/mimalloc.git` |
| Local path | `.external/upstream/mimalloc/` |
| Checkout mode | shallow clone |
| Commit | `fef6b0dd70f9d7fa0750b0d0b9fbb471203b94cd` |
| Describe | `fef6b0d` |
| Commit date | `2026-04-29 15:46:57 -0700` |
| Subject | `Merge pull request #1284 from microsoft/dev-main` |
| Pin date | `2026-05-14` |
| License | MIT-style license, owned by Microsoft Corporation and Daan Leijen in the upstream license file. |

## Source Inventory Window

MIMAP-002 should start with bounded concept-family reads rather than a full port.

| Concept family | Primary upstream files | Initial classification target |
| --- | --- | --- |
| Public API and user surface | `include/mimalloc.h`, `include/mimalloc-stats.h` | `near-transcription` vs `deferred-unsafe` |
| Internal types and constants | `include/mimalloc/types.h`, `include/mimalloc/internal.h` | `representation-gap` |
| Allocation/free path | `src/alloc.c`, `src/free.c`, `src/heap.c` | `near-transcription` / `lifecycle-rewrite` |
| Page and queue lifecycle | `src/page.c`, `src/page-queue.c` | `lifecycle-rewrite` |
| Segment and arena lifecycle | `src/segment.c`, `src/segment-map.c`, `src/arena.c`, `src/arena-abandon.c` | `lifecycle-rewrite` / `substrate-gap` |
| Size class and bitmap helpers | `src/bitmap.c`, `src/bitmap.h`, `src/options.c` | `near-transcription` / `representation-gap` |
| OS and primitive substrate | `src/os.c`, `include/mimalloc/prim.h`, `src/prim/prim.c`, `src/prim/unix/prim.c`, `src/prim/windows/prim.c`, `src/prim/osx/prim.c`, `src/prim/wasi/prim.c` | `substrate-gap` |
| Stats and diagnostics | `src/stats.c`, `include/mimalloc-stats.h` | `near-transcription` |
| Override / global allocator hooks | `include/mimalloc-override.h`, `include/mimalloc-new-delete.h`, `src/alloc-override.c`, `src/alloc-posix.c` | `deferred-unsafe` |

## Stop Lines

```text
no vendored mimalloc source
no line-by-line translation target
no global allocator replacement
no provider activation
no hooks
no copied C snippets in design docs
```

## Next Row

`MIMAP-002 source concept inventory` should classify each concept family as:

```text
near-transcription
lifecycle-rewrite
substrate-gap
representation-gap
deferred-unsafe
```
