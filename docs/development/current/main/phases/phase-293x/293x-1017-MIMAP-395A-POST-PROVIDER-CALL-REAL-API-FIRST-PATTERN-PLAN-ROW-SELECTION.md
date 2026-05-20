# 293x-1017 MIMAP-395A Post Provider Call Real API First-Pattern Plan Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after the provider-call real API
first-pattern plan. The first-pattern boundary is fixed; the next behavior row
may open a stubbed provider API execution seam without host allocator
replacement, hooks, backend matcher additions, or global allocator install.

## Candidate Next Rows

- provider-call real API call stub execution pilot
- provider-call first-pattern closeout
- provider-call host replacement optional ladder selection

## Stop Lines

- No host allocator replacement, hooks, backend matcher additions, or
  `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
