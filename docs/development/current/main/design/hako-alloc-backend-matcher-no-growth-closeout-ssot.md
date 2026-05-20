---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-354A
Scope: backend matcher no-growth closeout after provider inactive boundary.
Related:
  - docs/development/current/main/phases/phase-293x/293x-970-MIMAP-354A-BACKEND-MATCHER-NO-GROWTH-CLOSEOUT.md
  - tools/checks/k2_wide_hako_alloc_backend_matcher_no_growth_closeout_guard.sh
---

# Hako Alloc Backend Matcher No-Growth Closeout

## Decision

MIMAP-354A closes the worker/TLS → provider inactive boundary seam by proving
that no backend `.inc` owner-name matcher was added for the allocator proof
apps or owner boxes in this chain.

This row does not add allocator behavior. It protects the boundary that backend
lowering must consume MIR route metadata instead of recognizing app, box,
owner, or row names.

## Checked Names

The no-growth guard checks the current first-real-seam chain:

- no-escape pointer residence pilot
- arena backing handle pilot
- pointer-derived lookup execution pilot
- segment-map mutation pilot
- atomic bitmap pilot
- OSVM/page-source pilot
- worker/TLS pilot
- provider inactive boundary inventory

## Stop Lines

- No backend `.inc` matcher by app, box, owner, or row name.
- No provider activation.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No source-level worker-local or concurrency surface.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_backend_matcher_no_growth_closeout_guard.sh
```
