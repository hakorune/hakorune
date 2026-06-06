---
Status: Investigation
Date: 2026-06-06
Scope: active current-doc length audit before MIR-FMEM-008D.
Related:
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/phases/phase-296x/296x-480-DIRECTARRAY-FMEM-COMMONALITY-AND-DOC-LENGTH-CLEANUP.md
---

# Current Docs Length Audit 2026-06-06

## Decision

Do not move thousands of lines inside a FastMemory implementation card. Queue
dedicated docs-slim rows instead.

Archive and investigation docs are allowed to be long. Active entry docs,
workstream cards, taskboards, and design SSOTs should remain compact enough for
restart use. When an active doc grows past roughly 1000 lines, add a slimming
task that keeps the current surface in place and moves old evidence into an
archive/investigation owner.

## Active Docs Over Threshold

Measured 2026-06-06:

```text
3184 docs/development/current/main/workstreams/mimalloc-current.md
1678 docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
1595 docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
1277 docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
1007 docs/development/current/main/design/recipe-tree-and-parts-ssot.md
```

Current priority is the first three because they are on the active FastMemory /
mimalloc restart path.

## Queued Cleanup Rows

```text
DOCS-SLIM-296X-001:
  owner: workstreams/mimalloc-current.md
  action: keep current decisions/task order/parking lot; move historical
  evidence anchors and old algorithm-port ledgers to investigation/archive.

DOCS-SLIM-296X-002:
  owner: phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  action: keep current blocker and queue summary; move old row prose and
  historical restart queues to phase-local archive with a stub.

DOCS-SLIM-FMEM-SSOT-001:
  owner: design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  action: keep decisions/current implementation order; move detailed report
  field ledgers and old consultation packet text to a companion investigation.

DOCS-SLIM-BACKLOG:
  owner: mimalloc-allocator-first-task-granularity-ssot.md and
  recipe-tree-and-parts-ssot.md
  action: audit later unless they return to the active blocker path.
```

## Stop Line

```text
do_not_delete_history=1
do_not_update_thin_mirrors_for_length_only=1
leave_stub_when_moving_doc=1
current_entry_docs_should_point_to_archive_not_duplicate_it=1
```
