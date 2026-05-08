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
  - docs/development/current/main/design/static-const-table-syntax-ssot.md
  - docs/development/current/main/design/inline-plan-ssot.md
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
| `M0a numeric type-name storage lock` | `live-narrow` | language + MIR + typed-object storage | `usize/isize` and fixed-width integer type-name classifier; typed-object inline i64 storage hints; exact width/range/overflow deferred |
| `M0b numeric arithmetic semantics lock` | `live-narrow` | language + MIR + backends | current `>>` is signed i64 arithmetic shift; logical shift and wrapping/checked arithmetic remain explicit future rows |
| `M1 raw layout vocabulary` | `live-narrow` | language + MIR layout facts | MIR-owned `repr_c_v0` vocabulary for fixed-width numeric fields; source syntax, pointer-sized fields, and backend-active native layout remain future rows |
| `M2 hako.mem/buf/ptr widening` | `live-narrow` | capability substrate | restricted memory/buffer/pointer facades; `BufCoreBox.cap_i64` routes through `PtrCoreBox.slot_cap_i64`; no unrestricted unsafe |
| `M3 RawBuf + RawArray allocator fixture` | `live-narrow` | algorithm substrate | allocator-shaped fixture using RawBuf/RawArray only; RawArray has len/cap/reserve/grow shape; no TLS/atomic/OSVM dependency |
| `M4 minimum verifier hardening` | `live-narrow` | verifier substrate | RawArray remove/insert now pass bounds/initialized-range gates; slice, double-free, and use-after-free remain follow-up splits |
| `M5 rune contract verifier` | `live-narrow` | rune metadata + verifier | `@rune Contract(no_alloc)` / `@rune Contract(no_safepoint)` are checked by the MIR verifier before backend use; backend export/use remains disabled |
| `M6 hako.atomic useful rows` | `live-narrow` | capability substrate | memory-order vocabulary plus ordered fence row are live; load/store/CAS/fetch_add remain future splits; no allocator policy inside atomic module |
| `M7 hako.tls useful rows` | `live-narrow` | capability substrate | diagnostics TLS status helpers are live; generic thread/task-local slot and cache-slot primitive remain future splits; no helper-local cache exposure as final API |
| `M8 hako.osvm allocator rows` | `live-narrow` | capability substrate + native keep | page_size/reserve/commit/decommit facades are live with native metal leaf below; allocator policy remains outside osvm |
| `M9 intrinsic rows` | `live-narrow` | intrinsic metadata + LLVM/VM | `clz_i64`, `ctz_i64`, and `popcnt_i64` current-lane non-negative i64 rows are live; `prefetch`, `assume`, `unreachable`, unsigned-width semantics, and backend optimization use remain future splits |
| `M10a export attrs consistency gate` | `live-narrow` | optimization export service | guard locks current weak export attrs and rejects strong attr names in active LLVM/runtime-decl export points; no backend fact widening |
| `M10b runtime-decl readonly fact guard` | `live-narrow` | optimization export service | manifest `readonly` attrs must match `memory = "read"`; missing readonly remains allowed for conservative rows |
| `M10c-pre pointer/handle return proof vocabulary` | `reserved` | optimization export proof | separates handle return classes from native pointer return classes before any strong LLVM pointer attrs |
| `M10c LLVM export attrs widening` | `blocked` | optimization export | `noalias`, `nonnull`, `dereferenceable`, alignment, stronger `nocapture` only after pointer/native-ptr proof and verifier/export consistency proof |
| `M11a static readonly data segment` | `live-narrow` | backend-private const data | backend-private static data manifest emits a readonly u16 size-class fixture as LLVM data; no source syntax or const eval |
| `M11b const eval/static table syntax` | `live-narrow` | language + MIR const data | `M11b-decl` source `u16` static const table declarations, `M11b-load` static table reads, and `M11b-eval` narrow integer initializer expressions are live; const fn remains future |
| `M11c InlinePlan rows` | `live-narrow` | rune metadata + MIR optimizer | `M11c-preserve` keeps `Hint(inline/noinline/hot/cold)` as MIR `inline_plans` metadata with no backend use; soft same-module leaf inline and substrate-only `Lowering(inline_required)` remain future rows |
| `M12 mimalloc raw-page proof` | `blocked` | allocator substrate consumer | page/free-list fixture on raw substrate with `no_alloc` / `no_safepoint` proof gates |
| `M13 allocator fast-path EXE proof` | `blocked` | EXE backend + substrate | direct EXE proof for allocator fast path; helper calls only where capability route says so |

## Fixed Implementation Order

1. `M0a numeric type-name storage lock`
2. `M0b numeric arithmetic semantics lock`
3. `M1 raw layout vocabulary`
4. `M4 minimum verifier hardening`
5. `M2 hako.mem/buf/ptr widening`
6. `M3 RawBuf + RawArray allocator fixture`
7. `M5 rune contract verifier`
8. `M6 hako.atomic useful rows`
9. `M7 hako.tls useful rows`
10. `M8 hako.osvm allocator rows`
11. `M9 intrinsic rows`
12. `M10a export attrs consistency gate`
13. `M10b runtime-decl readonly fact guard`
14. `M11a static readonly data segment`
15. `M11b-decl source static const table declaration`
16. `M11b-load static table read route`
17. `M11c-docs InlinePlan boundary lock`
18. `M11b-eval const integer expression table generation`
19. `M11c-preserve Hint inline/noinline/hot/cold into MIR InlinePlan`
20. `M11c-soft-leaf best-effort same-module MIR inline`
21. `M10c-pre pointer/handle return proof vocabulary`
22. `M10c LLVM export attrs widening`
23. `M11c-required-vocab substrate-only Lowering(inline_required)`
24. `M11c-required-verify verifier-backed required inline acceptance`
25. `M12 mimalloc raw-page proof`
26. `M13 allocator fast-path EXE proof`

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

The first mimalloc-grade substrate card landed as `M0a numeric type-name
storage lock`. `M0b numeric arithmetic semantics lock` now fixes the current
`>>` behavior as signed i64 arithmetic shift, while logical shift and
wrapping/checked arithmetic remain future explicit rows. `M1 raw layout
vocabulary` now gives future allocator rows a MIR-owned `repr_c_v0` layout
target for fixed-width numeric fields only. `M4 minimum verifier hardening`
now covers RawArray remove/insert verifier gates. `M2` is narrowing the current
memory/buffer/pointer capability split so buffer shape facades do not own direct
backend slot ABI names. `M3` now has a readable RawArray capacity observer over
the buffer facade. `M5` now checks `Contract(no_alloc)` and
`Contract(no_safepoint)` in the MIR verifier, but those facts are not exported
to backend optimization yet. `M9a` now exposes `hako.intrin` bit-count rows for
current-lane non-negative i64 values; this does not activate
`@rune IntrinsicCandidate` or backend optimization use. `M11a` now proves the
backend-private static readonly data seam with a generated u16 size-class
fixture. `M11b-decl` now accepts source-level
`static const NAME: u16[] = [...]` declarations into MIR `static_data_plans`.
`M11b-load` now accepts `NAME[index]` as a MIR-owned static-data load for those
tables. `M11c-docs` now reserves the InlinePlan boundary so allocator fast-path
work does not grow `.inc` or backend-local inliners. `M11b-eval` now evaluates
narrow integer const expressions in source static `u16` table initializers.
`M11c-preserve` now keeps existing `Hint(inline/noinline/hot/cold)` as MIR
InlinePlan metadata with no backend use. The next code implementation target is
`M11c-soft-leaf`: best-effort same-module MIR inline. Implement that before any
required-inline or
allocator fast-path proof. Do not jump to allocator fast-path lowering before
the remaining verifier facts make raw access auditable.
