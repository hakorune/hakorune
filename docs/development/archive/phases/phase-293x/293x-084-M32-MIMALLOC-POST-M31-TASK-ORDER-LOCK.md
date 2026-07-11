---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M32 mimalloc post-M31 task-order lock
---

# 293x-084 M32 Mimalloc Post-M31 Task-Order Lock

## Decision

`M32 mimalloc post-M31 task-order lock` is live-docs.

M31 proved the remote-free push pattern over fixed-slot i64 atomics without
adding a backend route. The next work must not jump directly to production
allocator policy or native pointer attrs. The remaining order is split so each
future row owns exactly one new seam.

## Fixed Next Rows

```text
M33 atomic memory-order args docs/route vocabulary lock
M34 pointer atomic vocabulary docs lock
M35 native pointer atomic route proof
M36 TLS + pointer remote-free composition proof
M37 allocator remote-free policy integration proof
M38 mimalloc allocator app closeout guard
```

## Ordering Rationale

- Memory-order arguments must be named before pointer atomics use them.
- Pointer atomic vocabulary must exist before native pointer CAS/load/store
  routes are accepted.
- Native pointer atomic proof must land before a pointer-shaped remote-free
  list proof.
- TLS coupling stays after pointer remote-free so TLS does not become the owner
  of atomic semantics.
- Production allocator policy stays last; earlier rows are substrate proof rows.

## Non-Goals

- No new parser syntax.
- No new MIR route rows.
- No `.inc` lowering changes.
- No NyRT exports.
- No allocator policy changes.
- No LLVM native pointer attrs or noalias/nonnull widening.

## Cleanup

This card also syncs stale taskboard wording for:

- `M6 hako.atomic useful rows`
- `M7 hako.tls useful rows`

The pure-first helper expansion to `hako_mem`, `RawBuf`, RawArray, and
allocator fast-path guards was already complete before this card, so it remains
unchanged.

## Gate

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Result on 2026-05-10: docs-only order lock landed.
