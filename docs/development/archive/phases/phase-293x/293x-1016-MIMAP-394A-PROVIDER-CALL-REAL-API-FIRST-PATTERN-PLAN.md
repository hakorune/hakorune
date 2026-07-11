# 293x-1016 MIMAP-394A Provider Call Real API First-Pattern Plan

Status: landed
Date: 2026-05-21

## Decision

Plan the provider-call real API first-pattern boundary after the real API
execution preflight. The next behavior row may open a stubbed real-provider-call
execution seam, but it must not replace the host allocator, install hooks, add
backend matchers, run worker/TLS behavior, or install `#[global_allocator]`.

## Scope

- Record the provider-call real API first-pattern gate.
- Require MIMAP-392A real API execution preflight as prerequisite evidence.
- Allow the next behavior row to model a stub provider API call result.
- Keep host allocator replacement / hooks / backend matcher additions /
  `#[global_allocator]` as separate optional ladders.

## Stop Lines

- No actual host allocator replacement, hooks, backend matcher additions, or
  `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-395A is selected as the next row-selection card.
