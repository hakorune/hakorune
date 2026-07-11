---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the huge/OSVM comparison slice guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker.
Related:
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
  - tools/checks/impl/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 295x-101 Mimalloc Comparison Huge/OSVM Slice Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the `huge-osvm-slice` comparison guard root. The selected batch keeps
the same guard paths and validation semantics, but moves the shell body into
`tools/checks/impl/` and keeps the root as a thin exec wrapper.

Selected root:

- `k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Preserve the current huge/OSVM comparison semantics and stop lines.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The huge/OSVM comparison root is now easier to scan, while the real validation
logic lives under `tools/checks/impl/`.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
