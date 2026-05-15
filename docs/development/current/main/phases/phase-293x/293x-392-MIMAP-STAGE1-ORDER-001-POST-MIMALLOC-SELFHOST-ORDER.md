# 293x-392 MIMAP-STAGE1-ORDER-001 Post-Mimalloc Selfhost Order

Status: landed
Date: 2026-05-15

## Decision

Broad Stage1 `.hako` compiler migration is not a prerequisite for the mimalloc
port.

The mimalloc lane should continue as `.hako` / `hako_alloc` allocator
completeness work. Stage1/selfhost routes remain monitor/proof routes while
allocator rows advance. Only narrow Stage1 semantics, MIR facts, substrate
routes, or compatibility repairs required by allocator rows may be pulled
forward.

## Future Row

```text
SELFHOST-POST-MIMAP-001:
  parked
  reopen broad Stage1 .hako owner reduction after mimalloc completeness evidence
```

That future row must remain separate from allocator behavior rows.

## Stop Lines

- Do not force broad `.hako` parser migration before mimalloc closeout.
- Do not force broad `.hako` mirbuilder rewrite before mimalloc closeout.
- Do not remove Rust bootstrap / bridge keeps just to claim mimalloc completion.
- Do not mix selfhost owner-reduction commits with allocator behavior rows.
- Keep the existing mirbuilder-first / parser-after selfhost order when the
  post-mimalloc row is reopened.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
