# 293x-474 MIMAP-039C Post-Nullable-Object-Return Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-039C` is the planning-only row after `MIR-ROW-C`.

It must select exactly one next row now that same-module nullable selected
object returns are accepted by MIR route metadata and pure-first EXE.

It must not land code.

## Candidate Set

```text
candidate:
  return to the object-lifecycle page queue selectPage() loop cleanup that
  exposed MIR-ROW-C
candidate:
  run a focused probe first if the queue loop cleanup exposes another
  independent compiler acceptance blocker
candidate:
  choose the next narrow allocator behavior row if page queue cleanup no
  longer blocks the next mimalloc completeness seam
```

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when one next row is selected with clear owner/proof/guard
names and provider/host allocator replacement still inactive unless explicitly
reopened.

## Selection Result

`MIMAP-039C` selects `MIMAP-040A`.

Rationale:

- `MIR-ROW-C` removed the compiler acceptance blocker for nullable
  loop-carried user-box object returns.
- The remaining object-lifecycle page queue cleanup is now a narrow BoxShape
  row: replace the fixed `page0/page1/page2` selection shape with a
  queue-length loop while preserving selection behavior.
- The row must stay allocator-local and must not activate provider, hook, host
  allocator replacement, OSVM, remote-free, or backend matcher shortcuts.

Selected row:

```text
row:
  MIMAP-040A object-lifecycle selectPage queue-length loop
owner:
  lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
proof apps:
  apps/mimalloc-object-lifecycle-queue-proof/main.hako
  apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako
guards:
  tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
  tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
primary proof:
  fourth page can be selected after earlier decommitted/reusable/unavailable
  pages, with `selectPage()` returning the selected object directly
stop lines:
  no facade selected-object exposure API
  no allocation behavior change
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

Closeout:

```text
current blocker moves to MIMAP-040A object-lifecycle selectPage loop cleanup.
```
