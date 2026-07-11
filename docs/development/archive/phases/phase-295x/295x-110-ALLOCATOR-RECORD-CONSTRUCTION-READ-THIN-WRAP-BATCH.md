---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the C205b allocator record construction/read lowering guard root into an impl-backed wrapper without changing the current mimalloc comparison blocker.
Related:
  - tools/checks/k2_wide_allocator_record_construction_read_guard.sh
  - tools/checks/impl/k2_wide_allocator_record_construction_read_guard.sh
---

# 295x-110 Allocator Record Construction Read Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the C205b allocator record construction/read lowering guard root. The
batch keeps the same validation semantics, but moves the real shell body into
`tools/checks/impl/` so the root is a thin wrapper.

Selected root:

- `k2_wide_allocator_record_construction_read_guard.sh`

## Cleanup

- Keep the root script as a thin wrapper that execs its impl body.
- Keep the C205b record construction/read lowering guard on the current design
  and implementation SSOTs.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The C205b allocator record construction/read lowering guard is now easier to
scan at the root level and its implementation lives under `tools/checks/impl/`.

## Stop Line

This batch does not open provider activation, provider/DLL packaging, process
allocator replacement, hooks, `#[global_allocator]`, worker/TLS, atomics,
remote-free stress, abandoned-heap stress, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
