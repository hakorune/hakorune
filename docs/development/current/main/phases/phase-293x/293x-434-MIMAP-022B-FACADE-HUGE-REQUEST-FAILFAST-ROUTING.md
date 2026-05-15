# 293x-434 MIMAP-022B Facade Huge-Request Fail-Fast Routing

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-022B` is the allocator behavior row selected by `MIMAP-022A`.

It should add one narrow object-lifecycle facade / page-source boundary that
classifies oversized requests and fails fast before the MIMAP-021C
allocation-miss fallback sources a fresh page.

The row exists to keep the facade page-source path honest: huge requests must
not silently widen the small allocation path, allocate a modeled small page, or
fall into provider / host replacement work.

## Selected Owner

Planned narrow owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_failfast_box.hako
```

Existing collaborators:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
lang/src/hako_alloc/memory/size_class_box.hako
```

Proof and guard:

```text
apps/mimalloc-facade-huge-failfast-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_huge_failfast_exe_guard.sh
```

## Scope

- Classify request size using the existing size-class policy threshold.
- Reject huge requests before invoking page-source attach/retry.
- Preserve the current small allocation miss fallback for non-huge requests.
- Return scalar proof fields such as `huge_rejected`, `small_forwarded`,
  `fallback_attempted`, `final_ok`, and `final_reason`.

## Stop Lines

- No huge page model.
- No page-map lookup.
- No release/realloc/alignment behavior changes.
- No purge/reclaim/decommit/recommit execution.
- No remote-free, TLS, or atomic behavior changes.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Planned Task Order

| Step | Task | Output | Stop line |
| --- | --- | --- | --- |
| `022B.1` | Add the facade huge-request fail-fast owner. | One `.hako` owner with scalar report fields. | no huge model |
| `022B.2` | Add one proof app and guard. | MIR JSON + EXE prove huge reject and small forwarding. | no broad smoke or provider ladder |
| `022B.3` | Update memory README / task docs. | Owner boundary and stop lines are discoverable. | no unrelated cleanup |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_failfast_exe_guard.sh
bash tools/checks/k2_wide_mimap022a_next_row_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
