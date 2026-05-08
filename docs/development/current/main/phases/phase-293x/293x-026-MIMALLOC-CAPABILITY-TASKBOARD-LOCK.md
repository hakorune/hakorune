# 293x-026: mimalloc capability taskboard lock

- Status: Landed
- Date: 2026-05-08
- Lane: `phase-293x real-app bringup / allocator substrate planning`

## Summary

This card clarifies the task order for future mimalloc-grade allocator work.
It is docs-only and does not change compiler acceptance.

The decision is that `.hako` should not gain a broad C-style unsafe surface.
Allocator substrate work proceeds through capability modules plus
`@rune Contract(...)` verifier rows.

## Added SSOT

- `docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md`
- `docs/reference/runtime/substrate-capabilities.md`

## Task Lock

Future mimalloc-grade implementation must follow the taskboard order:

1. numeric substrate lock
2. raw layout vocabulary
3. minimum verifier hardening
4. `hako.mem` / `hako.buf` / `hako.ptr` widening
5. `RawBuf` + `RawArray` allocator fixture
6. `@rune Contract(no_alloc/no_safepoint)` verifier
7. `hako.atomic` useful rows
8. `hako.tls` useful rows
9. `hako.osvm` allocator rows
10. intrinsic rows
11. LLVM export attrs
12. const/static table rows
13. mimalloc raw-page proof
14. allocator fast-path EXE proof

## Manual Rule

Every implementation row must update the reference manual in the same commit.
Default manual target:

- `docs/reference/runtime/substrate-capabilities.md`

Syntax rows must also update language reference / EBNF. ABI or metadata rows
must update ABI and MIR reference docs.

## Boundary

- No new parser syntax was added.
- No new compiler acceptance was added.
- No C shim allocator special case was added.
- `phase-293x` real-app EXE parity remains separate from the broader substrate
  widening lane.

## Gates

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
