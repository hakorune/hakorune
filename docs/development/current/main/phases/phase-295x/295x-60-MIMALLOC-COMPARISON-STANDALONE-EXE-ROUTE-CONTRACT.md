---
Status: Landed
Date: 2026-05-25
Scope: phase-295x standalone EXE route contract.
Related:
  - docs/development/current/main/design/standalone-exe-route-contract-ssot.md
  - docs/development/current/main/design/plugin-loadset-linking-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-59-MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT.md
---

# 295x-60 Standalone EXE Route Contract

## Blocker

```text
MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT-295X-001
```

## Decision

Define standalone EXE as a future build/run route family, not as an immediate
packaging backend.

The route vocabulary is:

```text
standalone-minimal
standalone-root
standalone-diagnostic
```

Each route must expose:

```text
runtime_config_profile
selected_loadset
plugin_load_policy
link_policy
standalone_packaging_generated
```

The current comparison lane can continue using exact-MIR EXE artifacts while
the standalone contract remains parked as a visible future route.

## Follow-On

```text
MIMALLOC-COMPARISON-POST-STANDALONE-ROUTE-SELECTION-295X-001
```

The follow-on should decide whether to wire standalone-route fields into
comparison-runner evidence next, or return to workload/measurement rows.

## Stop Line

This row does not implement `hakorune build --kind standalone`, generate
standalone packages, change exact-MIR EXE behavior, make `empty` the default,
compute RSS winners, require RSS parity, or open provider/DLL/replacement/hook
/ global allocator seams.
