# 293x-1047 MIMAP-425A Optional Process Allocator Replacement Proposal

Status: selected current
Date: 2026-05-21

## Purpose

Draft the optional process allocator replacement proposal after backend
matcher no-growth has been reconfirmed. This row must stay proposal-only and
must not replace the process allocator, install hooks, add backend matchers, or
install a global allocator.

## Scope

- Name the explicit proposal boundary for optional process allocator
  replacement.
- Keep `hako_alloc` as a comparable allocator implementation, not the default
  process allocator.
- Preserve rollback/no-growth requirements for any future execution row.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planning validation is L0:

```text
current state pointer guard
git diff --check
```
