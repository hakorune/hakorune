---
Status: Complete
Date: 2026-05-12
Scope: backend capability fail-fast for exact numeric storage and op routes.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/runtime/substrate-capabilities.md
---

# 294x-13 Backend Exact Numeric Capability Fail-Fast

## Decision

Non-VM backend routes must not silently lower exact numeric typed-object storage
or exact numeric operation route facts through the legacy `Integer(i64)` path.

This row adds a MIR-owned backend capability gate:

- dynamic exact numeric runtime-check contracts still fail fast with the
  existing runtime-check contract tag;
- typed-object fields whose storage needs a native exact numeric slot fail fast
  before backend emission;
- exact numeric operation route facts fail fast before backend emission.

Legacy `i64` typed-object storage and `handle` storage remain accepted by this
gate because they do not claim new exact-width native slots.

## Stable Diagnostics

```text
[freeze:backend][exact-numeric/storage-unsupported]
[freeze:backend][exact-numeric/route-unsupported]
```

The existing runtime-check contract tag remains:

```text
[freeze:contract][exact-numeric/runtime-check-unsupported-backend]
```

## Ownership

Code owner:

```text
src/mir/exact_numeric_backend_capability.rs
```

Backend entry points consume only this capability gate. They do not each own
separate exact numeric compatibility checks.

## In Scope

- One backend capability inspection surface.
- Fail-fast for exact numeric typed-object storage that requires native slots.
- Fail-fast for exact numeric arithmetic, compare, and logical-shift route
  facts.
- Keep MIR JSON diagnostic export available.

## Out Of Scope

- Native typed-object exact numeric slot lowering.
- LLVM/native unsigned compare or logical-shift lowering.
- Exact runtime value representation beyond the current VM reference executor.
- hako_alloc live field migration.
- Mimalloc `.hako` algorithm rows.

## Verification

```text
cargo test -q --lib exact_numeric_backend_capability
cargo check -q --lib
```
