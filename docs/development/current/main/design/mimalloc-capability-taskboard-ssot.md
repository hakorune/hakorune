---
Status: SSOT
Decision: accepted
Date: 2026-05-08
Scope: mimalloc-grade allocator substrate task order, capability-module boundary, and required manual-update contract.
Related:
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/reference/runtime/substrate-capabilities.md
---

# Mimalloc Capability Taskboard (SSOT)

## Decision

The clean path for mimalloc-grade allocator work is not a broad C-style
`unsafe` surface in `.hako`.

The accepted path is:

```text
hako.mem
hako.buf
hako.ptr
hako.atomic
hako.tls
hako.osvm
@rune Contract(...)
minimum verifier
```

This keeps low-level power explicit, staged, and auditable. Capability modules
provide the substrate vocabulary. `@rune Contract(...)` states obligations such
as `no_alloc` / `no_safepoint`. Verifiers must prove those obligations before a
backend may trust them for lowering or optimization.

## Non-Goals

- Do not add unrestricted C-like pointer arithmetic as a language-wide feature.
- Do not add a monolithic `hako.sys` unsafe shelf.
- Do not make allocator-specific C shim branches for `Mi*`, `HakoAlloc*`, or
  any app-specific box name.
- Do not treat syntax acceptance as implementation acceptance.
- Do not make `@rune Contract(...)` backend-active without verifier proof.
- Do not mix real-app EXE parity cards with broad substrate widening in the
  same commit.

## Status Legend

| Status | Meaning |
| --- | --- |
| `live-narrow` | A first row exists, but it is intentionally small. |
| `reserved` | Vocabulary is named, but not live. |
| `next-card` | Ready to turn into one implementation card when the lane opens. |
| `blocked` | Must wait for an earlier row. |

## Task Rows

| Row | Status | Owner | Required output |
| --- | --- | --- | --- |
| `M0 numeric substrate lock` | `next-card` | language + MIR + backends | `usize/isize`, fixed-width integers, logical/arithmetic shift distinction, wrapping/checked arithmetic decision, fixtures, manual update |
| `M1 raw layout vocabulary` | `reserved` | language + MIR layout facts | raw layout distinct from `box`, alignment, `sizeof`, `offsetof`, repr-like contract, fail-fast unsupported backends |
| `M2 hako.mem/buf/ptr widening` | `live-narrow` | capability substrate | restricted memory/buffer/pointer facades, no unrestricted unsafe, verifier hooks named |
| `M3 RawBuf + RawArray allocator fixture` | `live-narrow` | algorithm substrate | allocator-shaped fixture using RawBuf/RawArray only; no TLS/atomic/OSVM dependency |
| `M4 minimum verifier hardening` | `next-card` | verifier substrate | bounds, initialized range, ownership, double-free/use-after-free follow-up split |
| `M5 rune contract verifier` | `reserved` | rune metadata + verifier | `@rune Contract(no_alloc)` / `@rune Contract(no_safepoint)` parsed facts become verifier-checked before backend use |
| `M6 hako.atomic useful rows` | `blocked` | capability substrate | load/store/CAS/fetch_add/fence with memory order; no allocator policy inside atomic module |
| `M7 hako.tls useful rows` | `blocked` | capability substrate | thread/task-local slot and cache-slot primitive; no helper-local cache exposure as final API |
| `M8 hako.osvm allocator rows` | `reserved` | capability substrate + native keep | page reserve/commit/decommit/page-size facade with native metal leaf below |
| `M9 intrinsic rows` | `reserved` | intrinsic metadata + LLVM/VM | `clz`, `ctz`, `popcnt`, `prefetch`, `assume`, `unreachable`, fail-fast unsupported backends |
| `M10 LLVM export attrs` | `blocked` | optimization export | `noalias`, `nonnull`, `dereferenceable`, alignment, stronger `nocapture` only after verifier/export consistency gate |
| `M11 const/static table rows` | `reserved` | language + MIR const data | static const tables for size classes; no runtime Array/Map construction for fixed tables |
| `M12 mimalloc raw-page proof` | `blocked` | allocator substrate consumer | page/free-list fixture on raw substrate with `no_alloc` / `no_safepoint` proof gates |
| `M13 allocator fast-path EXE proof` | `blocked` | EXE backend + substrate | direct EXE proof for allocator fast path; helper calls only where capability route says so |

## Fixed Implementation Order

1. `M0 numeric substrate lock`
2. `M1 raw layout vocabulary`
3. `M4 minimum verifier hardening`
4. `M2 hako.mem/buf/ptr widening`
5. `M3 RawBuf + RawArray allocator fixture`
6. `M5 rune contract verifier`
7. `M6 hako.atomic useful rows`
8. `M7 hako.tls useful rows`
9. `M8 hako.osvm allocator rows`
10. `M9 intrinsic rows`
11. `M10 LLVM export attrs`
12. `M11 const/static table rows`
13. `M12 mimalloc raw-page proof`
14. `M13 allocator fast-path EXE proof`

This order may be split further, but it must not be inverted unless a new SSOT
card explains the dependency change.

## Per-Row Acceptance Contract

Each implementation row must land as:

```text
one row
one fixture/gate
one manual update
one commit
```

Required acceptance fields:

- owner layer
- syntax or API surface, if any
- MIR/value representation row
- VM behavior
- LLVM/EXE behavior
- Stage0 / MIR JSON behavior when exposed across that boundary
- fail-fast diagnostic for unsupported consumers
- smoke or unit gate
- manual update paths

## Manual Update Contract

Every implementation row must update user-facing or reference documentation in
the same commit.

Default manual targets:

- substrate capability manual:
  - `docs/reference/runtime/substrate-capabilities.md`
- numeric/language syntax rows:
  - `docs/reference/language/types.md`
  - `docs/reference/language/EBNF.md`
- rune contract rows:
  - `docs/reference/mir/hints.md`
  - `docs/reference/runtime/substrate-capabilities.md`
- ABI/export rows:
  - `docs/reference/abi/ABI_INDEX.md`
  - `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- MIR metadata / route facts:
  - `docs/reference/mir/metadata-facts-ssot.md`

If a row does not update a manual, the implementation card must state why the
row is internal-only and name the future manual update trigger.

## Parser Update Rule

When a row adds syntax, both parser fronts must be considered:

- Rust parser / tokenizer path
- `.hako` selfhost parser path

If the row can be expressed as a capability box call without new syntax, prefer
that first. If new syntax is required, the implementation card must include the
Rust parser, selfhost parser, EBNF, and MIR JSON acceptance plan.

## Rune Contract Rule

`@rune Contract(...)` is not a comment and not a hint once a row is backend
active.

Before backend use:

1. parse and preserve the contract
2. lower it into MIR-owned metadata
3. verify it structurally
4. fail-fast on violation
5. export only the proven fact

Examples:

```hako
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
method alloc_fast(size: usize) -> Ptr<u8> {
  ...
}
```

The backend may not infer `no_alloc` from the method name, app name, or helper
choice.

## Current Reading

Current real-app work may continue on VM/EXE parity using typed-object planning.
That is separate from this taskboard.

The first mimalloc-grade substrate card should be `M0 numeric substrate lock`,
not raw pointers and not allocator fast-path lowering.
