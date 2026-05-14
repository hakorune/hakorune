# Mimalloc backend acceptance policy SSOT

Decision: accepted.

This document fixes the backend acceptance split for the mimalloc blueprint lane.

## Policy

```text
VM:
  semantic reference executor
  small scalar policy proof
  small route-contract smoke
  not the MIMAP-011+ object-heavy acceptance backend

LLVM/EXE:
  primary acceptance backend for MIMAP-011+
  page queue / heap facade / lifecycle object route proof
  object-return allocator API proof
```

VM remains valuable and must not be removed. It is not the product/mainline owner
for mimalloc object-heavy routes.

## VM role

VM guards are appropriate for:

- size-class scalar policy
- page-local lifecycle state transitions
- scalar lifecycle queue selection policy
- small reason-code and fail-fast proof apps

VM guards are not required to prove:

- object-heavy page queue retention
- heap facade object route orchestration
- page object storage inside queue-like boxes
- allocator provider activation
- host allocator replacement

## LLVM/EXE role

MIMAP-011 and later must use LLVM/EXE as the primary acceptance backend when the
row proves page queue, heap facade, lifecycle, or object-return allocator routes.

VM green is useful evidence, but VM support for object-heavy page/facade routes is
not a completion requirement for MIMAP-011+.

## Timeout rule

VM failure or hang must not be ignored.

Every MIMAP VM guard must run VM execution through a timeout helper. Timeout means
fail-fast for that guard. If the timed-out route is object-heavy and outside the
VM acceptance scope, the limitation must be recorded in:

```text
docs/development/current/main/design/vm-known-limitations-ssot.md
```

Timeout is never a silent pass.

## Forbidden

- Do not reshape mimalloc design just to fit VM object-heavy limitations.
- Do not treat VM timeout as success.
- Do not make VM object-heavy route green a MIMAP-011+ blocker.
- Do not allow LLVM/EXE silent fallback because VM has a known limitation.
