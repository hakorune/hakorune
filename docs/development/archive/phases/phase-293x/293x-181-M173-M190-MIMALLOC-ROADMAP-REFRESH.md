---
Status: Complete
Date: 2026-05-12
Scope: readable M173-M190 mimalloc `.hako` port task order.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/phases/phase-293x/README.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 293x-181 M173-M190 Mimalloc Roadmap Refresh

## Goal

Make the post-M172 task order easy to read before implementation resumes.

The accepted order is:

```text
M173: pre-realloc release invariant freeze
M174: realloc same-class / no-move path
M175: realloc alloc-copy-release fallback
M176: realloc negative matrix / failure contract
M177: alignment policy object
M178: aligned allocation small path
M179: huge threshold and routing
M180: huge page model
M181: huge release seam
M182: secure free-list policy inventory
M183: secure-list diagnostics-only
M184: secure-list encode/decode small path
M185-M190: remaining usize field groups and allocator API parity
```

## Correction

The ChatGPT Pro order is correct through `M184`.

Rows `M185-M190` need one repo-specific correction: broad hako_alloc numeric
field inventory already landed as `294x-16`, and facade-local exact `usize`
stats already landed as `294x-19e`. Future `M185+` rows must not redo those
completed slices. They should focus on newly introduced fields, size-class and
request-path `usize` migration, object-return EXE parity, and explicit
failure-handle shape.

## Stop Line

Do not mix these work streams:

- realloc body before release invariants are frozen;
- alignment with huge-page routing;
- secure-list implementation before diagnostics;
- production `usize` migration for sentinel-bearing fields;
- VM-only green as allocator API completion;
- provider activation, hooks, process allocator replacement, or `.inc`
  allocator-name matching.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
