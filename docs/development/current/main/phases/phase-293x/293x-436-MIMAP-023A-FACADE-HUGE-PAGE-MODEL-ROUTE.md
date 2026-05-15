# 293x-436 MIMAP-023A Facade Huge-Page Model Route

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-023A` is the allocator behavior row selected by `MIMAP-022C`.

It should add one narrow object-lifecycle facade route that sends huge requests
to the existing M180 huge-page model owner while keeping the small allocation
path exactly on the MIMAP-022B / MIMAP-021C route.

This is not provider activation and not process allocator replacement. It is a
`.hako` / `hako_alloc` completeness row for the facade allocator model.

## Selected Owner

Planned narrow owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
```

Existing collaborators:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_failfast_box.hako
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
lang/src/hako_alloc/memory/huge_page_model_box.hako
lang/src/hako_alloc/memory/page_map_box.hako
lang/src/hako_alloc/memory/size_class_box.hako
```

Proof and guard:

```text
apps/mimalloc-facade-huge-page-model-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
```

## Scope

- Classify request size using the existing size-class / MIMAP-022B threshold.
- For huge requests, allocate through the existing `HakoAllocHugePageModel`.
- Preserve the current small-request forwarding path through MIMAP-022B and
  MIMAP-021C.
- Return scalar proof fields such as `huge_allocated`, `huge_page_id`,
  `huge_ptr`, `huge_requested_size`, `huge_committed_size`,
  `small_forwarded`, `final_ok`, and `final_reason`.

## Stop Lines

- No new huge page model owner; reuse the existing M180 owner.
- No huge release, unregister, unreserve, decommit, or OS release behavior.
- No page-map lookup route.
- No release/realloc/alignment behavior changes.
- No purge/reclaim/decommit/recommit execution.
- No remote-free, TLS, or atomic behavior changes.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Planned Task Order

| Step | Task | Output | Stop line |
| --- | --- | --- | --- |
| `023A.1` | Add the facade huge-page model route owner. | One `.hako` owner with scalar report fields. | no huge release / page-map lookup |
| `023A.2` | Add one proof app and guard. | MIR JSON + EXE prove huge allocation and small forwarding. | no provider ladder |
| `023A.3` | Update hako_alloc memory docs / task docs. | Owner boundary and stop lines are discoverable. | no unrelated cleanup |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
bash tools/checks/k2_wide_mimap022c_next_row_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

Implemented the selected facade huge-page model route in:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
apps/mimalloc-facade-huge-page-model-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
```

The route classifies request size through the same size-class threshold used by
MIMAP-022B. Huge requests are allocated through the existing M180
`HakoAllocHugePageModel`; non-huge requests preserve the MIMAP-022B non-huge
behavior by forwarding to the MIMAP-021C allocation-miss fallback.

The proof app is intentionally scalar-print based with compact aggregate checks.
The detailed route values are fixed by the guard output assertions so the app
does not expand into a large MIR proof shape.

The next current row is planning-only:

```text
docs/development/current/main/phases/phase-293x/293x-437-MIMAP-023B-POST-HUGE-PAGE-MODEL-ROW-SELECTION.md
```
