# 293x-474 MIMAP-039C Post-Nullable-Object-Return Row Selection

Status: selected current
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
