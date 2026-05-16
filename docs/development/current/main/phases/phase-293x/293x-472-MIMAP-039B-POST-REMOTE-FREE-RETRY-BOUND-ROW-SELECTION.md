# 293x-472 MIMAP-039B Post-Remote-Free-Retry-Bound Row Selection

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-039B` is the planning-only row after `MIMAP-039A`.

It must select exactly one next row after the remote-free retry-bound cleanup
lands.

It must not land code.

## Candidate Set

```text
candidate:
  pick a narrow allocator behavior row if cleanup no longer blocks the next
  mimalloc completeness seam
candidate:
  park object-lifecycle queue loop cleanup behind a compiler acceptance sidecar
candidate:
  continue narrow cleanup if another concrete hardcoded allocator shape remains
candidate:
  switch to a language/compiler sidecar only if the next allocator row exposes
  an acceptance blocker
```

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
