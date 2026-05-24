---
Status: Landed
Date: 2026-05-25
Scope: phase-295x runtime reference docs for plugin loadsets and standalone routes.
Related:
  - docs/reference/runtime/plugin-loadsets.md
  - docs/reference/runtime/standalone-exe-routes.md
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/design/standalone-exe-route-contract-ssot.md
---

# 295x-62 Runtime Reference Loadset / Standalone Docs

## Blocker

```text
MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-295X-001
```

## Decision

Add user-facing runtime references for:

```text
docs/reference/runtime/plugin-loadsets.md
docs/reference/runtime/standalone-exe-routes.md
```

The references document:

- loadset vocabulary (`root`, `empty`, `all`, reserved `app`, reserved `core`);
- `plugin_load_policy=eager_selected`;
- no implicit lazy loading;
- no-dlopen preflight plan fields;
- standalone route names (`standalone-minimal`, `standalone-root`,
  `standalone-diagnostic`);
- standalone link policy vocabulary;
- closed provider/DLL/replacement/hook/global allocator seams.

## Follow-On

```text
MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT-295X-001
```

## Stop Line

This row does not change runtime behavior, implement standalone packaging,
teach NyRT to read `hako.toml` directly, compute RSS winners, require RSS
parity, or open provider/DLL/replacement/hook/global allocator seams.
