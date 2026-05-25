---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-240A segment arena backing scalar requirement matrix guard root into an impl-backed wrapper and keep the memory README owner note plus proof manifest entry in sync.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh
  - tools/checks/impl/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh
  - lang/src/hako_alloc/memory/README.md
  - tools/checks/proof_apps.toml
---

# 295x-112 Segment Arena Backing Requirement Matrix Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-240A segment arena backing scalar requirement matrix
guard. The batch keeps the same validation semantics, but moves the real shell
body into `tools/checks/impl/` and keeps the memory README owner note plus
proof manifest entry in sync.

Selected root:

- `k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the MIMAP-240A owner note visible in `lang/src/hako_alloc/memory/README.md`.
- Keep the MIMAP-240A proof-app manifest entry on `scalar-mir` / deferred closeout.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-240A segment arena backing scalar requirement matrix guard is now
easier to scan at the root level, and the memory README plus proof manifest
explicitly document the owner note that the guard expects.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_requirement_matrix_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
