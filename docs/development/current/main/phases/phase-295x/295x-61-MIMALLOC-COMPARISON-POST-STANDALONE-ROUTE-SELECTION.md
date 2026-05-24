---
Status: Landed
Date: 2026-05-25
Scope: phase-295x post-standalone route selection.
Related:
  - docs/development/current/main/design/standalone-exe-route-contract-ssot.md
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-60-MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT.md
---

# 295x-61 Post Standalone Route Selection

## Blocker

```text
MIMALLOC-COMPARISON-POST-STANDALONE-ROUTE-SELECTION-295X-001
```

## Decision

Select a reference-doc alignment row before implementing standalone packaging or
adding more comparison workloads.

The plugin loadset and standalone EXE route contracts are durable enough to be
visible from `docs/reference/runtime/`, not only phase-local design docs.

## Follow-On

```text
MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-295X-001
```

## Stop Line

This row does not change runtime behavior, generate standalone packages,
compute RSS winners, require RSS parity, or open provider/DLL/replacement/hook
/ global allocator seams.
