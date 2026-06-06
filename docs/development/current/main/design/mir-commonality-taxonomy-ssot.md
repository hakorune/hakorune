---
Status: SSOT
Decision: accepted
Date: 2026-06-06
Scope: Shared-boundary taxonomy for escape, allowlist/gate, and owner concepts.
Related:
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - docs/development/current/main/design/contract-region-v0-ssot.md
  - src/mir/escape_barrier.rs
  - src/mir/contracts/README.md
---

# MIR Commonality Taxonomy

## Decision

Commonize only the thin layer that removes duplicate cause classification or
report vocabulary. Keep domain policy payloads owned by their domain.

Short form:

```text
Shared:
  cause classification / envelope / vocabulary

Domain-owned:
  acceptance policy / proof payload / lowering plan / runtime identity
```

This avoids a generic abstraction that sounds elegant but hides important
FastMemory, DirectArray, backend, and runtime ownership differences.

## Escape Boundary

Accepted shared owner:

```text
src/mir/escape_barrier.rs
  EscapeBarrier
  EscapeUse
  classify_escape_uses()
```

This module owns cause-side classification: what kind of MIR use crosses a
value boundary.

Domain owners still decide what the classification means:

```text
FastMemory:
  no-escape MemOp policy
  MemOp-to-MemOp allowed flow
  ordinary MIR consumer rejection
  layout/table/pointer provenance rules

DirectMemory / Span:
  span borrow lifetime and stable-region policy

Record / string / AOT:
  materialization or escape_kind interpretation
```

Do not make `escape_kind` a global SSOT until a real AOT consumer opens. It can
be an adapter from `EscapeBarrier` later.

## Allowlist / Gate Boundary

Use the existing contract stack as the common boundary:

```text
src/mir/contracts:
  accepted MIR/backend vocabulary

src/mir/backend_capability.rs:
  backend support gate entry
```

Profile-specific allowlists remain profile-specific:

```text
FastMemory MemOp dialect:
  src/mir/contracts/fastmem_ops.rs

Call dialect canonicalization:
  closure/callsite SSOT modules

CoreBox / route surfaces:
  their own phase-specific allowlists and guards
```

Do not add a generic `AllowlistGate<T>` framework now. The current shape is
already "shared entry, specialized checks".

Vocabulary:

```text
allowlist:
  accepted vocabulary / surface

gate:
  fail-fast enforcement point

backend capability gate:
  route/backend support check
```

## Owner Boundary

Do not commonize owner concepts into one generic `Owner` abstraction.

Keep the axes separate:

```text
AllocOwnerId:
  allocator arena owner identity
  runtime memory-management identity
  slot + generation lifecycle

page owner:
  page metadata owner for allocator-local state
  derived from / compared with AllocOwnerId in allocator flows

semantic owner:
  architecture/source-truth owner
  examples: ArrayCoreBox, SizeClassBox, PageModel
```

These all answer "who owns this?", but at different layers. Collapsing them
would blur runtime identity with design responsibility.

## Task Order

```text
COMMON-TAX-001:
  docs-only taxonomy for escape / allowlist-gate / owner boundaries

ESCAPE-COMMON-001:
  refactor FastMemory escape verifier to use classify_escape_uses() for
  ordinary MIR consumers
  no policy change

ESCAPE-COMMON-002:
  add focused verifier fixtures for return/store/call/capture/debug/Phi
  preserve FastMemory-owned no-escape error/report shape

AOT-ESCAPE-LATER:
  add adapter from EscapeBarrier to escape_kind only when AOT consumer opens

ALLOWLIST-GATE-LATER:
  no code work unless a second concrete gate needs the same helper shape

OWNER-TAXONOMY-LATER:
  no code work; keep glossary only unless an ambiguity causes a real bug
```

## No-Go

```text
generic EscapePolicy replacing FastMemory no-escape verifier
generic Owner type spanning AllocOwnerId and semantic owner
generic AllowlistGate<T> with no second concrete consumer
using AOT escape_kind as current MIR escape classifier
changing TableIndex / FastMemory proof payloads in this taxonomy slice
```
