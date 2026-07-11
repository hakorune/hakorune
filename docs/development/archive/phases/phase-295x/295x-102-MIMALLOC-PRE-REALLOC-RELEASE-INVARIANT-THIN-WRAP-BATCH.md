---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the M173 pre-realloc release invariant guard root into an impl-backed wrapper and add the missing memory README owner note.
Related:
  - tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
  - tools/checks/impl/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
  - lang/src/hako_alloc/memory/README.md
---

# 295x-102 Mimalloc Pre-Realloc Release Invariant Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the `mimalloc-pre-realloc-release-invariant` guard root. The batch
keeps the same validation semantics, but moves the real shell body into
`tools/checks/impl/` and documents the observer module in
`lang/src/hako_alloc/memory/README.md`.

Selected root:

- `k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the M173 observer only as an invariant observer, not an execution owner.
- Add the M173 observer module note to the memory-layer README.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The M173 release invariant guard is now easier to scan at the root level, and
the memory README explicitly documents the observer module it owns.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
