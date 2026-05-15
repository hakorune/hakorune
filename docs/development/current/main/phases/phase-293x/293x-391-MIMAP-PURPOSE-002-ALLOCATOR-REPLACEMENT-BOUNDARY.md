# 293x-391 MIMAP-PURPOSE-002 Allocator Replacement Boundary

Status: landed
Date: 2026-05-15

## Decision

Clarify that the mimalloc port is a `.hako` / `hako_alloc` completeness lane,
not default process allocator replacement.

Completing the port may create a future explicit provider candidate, similar in
spirit to Rust's explicit global-allocator selection, but that provider option
is a separate ladder and remains inactive until reopened.

## Scope

- Clarify `port` vs `provider option` vs `process allocator replacement`.
- Keep ordinary host/process malloc as the current default runtime allocation
  path.
- Keep provider activation, hook install, and `#[global_allocator]` out of the
  current mimalloc lane.

## Stop Lines

- No behavior changes.
- No provider activation.
- No hook installation.
- No default malloc/free replacement.
- No `.inc` or Stage0 mimalloc/provider matchers.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
