---
Status: Active
Date: 2026-06-01
Scope: direct memory / DirectArray language substrate workstream.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# Direct Memory Current Workstream

## Goal

Pause mimalloc source tweaking long enough to make the direct-memory substrate
tasks readable and reusable.

The target is not C pointer syntax. The target is a small verified view system
that lets allocator code stay ordinary `.hako` while compiler plans choose
direct storage routes.

## Stop Line

- no `RawPtr<T>` source type
- no `&`, `*`, `->`, or pointer arithmetic
- no `NativePtr` dereference or indexing
- no untyped `DirectArray` source type in v0
- no implicit `Array` <-> `DirectArrayI64` conversion
- no public ArrayBox semantics change
- no broad `unsafe` block
- no new numbered row or row-specific `.sh` guard by default

## Task Order

### Active Slice

- [x] LANG-DM-001: reference policy lock
  - output: `docs/reference/language/low-level-capabilities.md` says
    RawPtr/pointer operators are not accepted, NativePtr is opaque, and
    `direct` / `unsafe memory` / `unchecked` are distinct
  - no code unless a reference mismatch is discovered

- [x] LANG-DM-002: DirectArrayAccessPlan cleanup
  - output: plan schema carries `element_type` and `proof_ids`
  - prepare `region_id` / `view_id` as metadata only if it keeps code simpler
  - no Span, Bytes, LayoutSpan, or source syntax

- [ ] LANG-DM-003: proof fact normalization
  - output: current DirectArray proof consumers can read stable fact names:
    `RangeIndexFact`, `DirectArrayExtentFact`, `RegionStabilityFact`
  - do not widen the accepted loop forms unless the fixture/smoke is included

### Next Slice

- [ ] LANG-DM-004: Span no-escape SSOT
  - output: exact rules for `SpanI64` / `SpanMutI64`
  - no return, field store, capture, publish, provider boundary crossing, or
    owner resize/free inside the span lifetime

- [ ] LANG-DM-005: SpanI64 / SpanMutI64 minimal pilot
  - output: borrow only from `DirectArrayI64`
  - access plans reuse DirectArray proof vocabulary
  - no NativePtr / Bytes yet

- [ ] LANG-DM-006: `direct {}` contract
  - output: direct block requires `FastPathPlan`
  - direct is not unsafe and does not imply unchecked

### Parked

- [ ] LANG-DM-007: unsafe memory / Bytes parked design
  - output: NativePtr remains opaque; Bytes owns byte-offset load/store methods

- [ ] LANG-DM-008: LayoutSpan / bulk memory pattern parking lot
  - output: future layout view and fill/zero/iota/copy recognition remain
    separate from existing enum record terminology

## Decision Log

- 2026-06-01: The active task focus moves from mimalloc source-level
  micro-optimization to direct-memory language substrate cleanup. Mimalloc is
  paused after MIM-054, not abandoned. The next language tasks should make
  DirectArray / Span / Bytes planning explicit without adding C-style pointers.
- 2026-06-01: LANG-DM-001 reference policy is locked in
  `docs/reference/language/low-level-capabilities.md`. Next active task is
  LANG-DM-002: make `DirectArrayAccessPlan` carry the element/proof shape that
  later Span/Bytes work can reuse.
- 2026-06-01: LANG-DM-002 is implemented without widening `.hako` syntax.
  `DirectArrayAccessPlan` now carries `proof_ids` beside the existing
  `element_type`, and Rust JSON / Python / C shim consumers all validate the
  proof carrier. `region_id` / `view_id` remain deferred metadata until
  LANG-DM-003 decides the fact vocabulary.
