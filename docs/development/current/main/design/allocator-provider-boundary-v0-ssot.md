---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator provider boundary vocabulary before allocator replacement.
Related:
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
---

# Allocator Provider Boundary v0 (SSOT)

## Goal

Name the provider boundary below `hako_alloc` policy/state without activating
process allocator replacement.

## Decision

`hako_alloc` owns allocator policy/state. Provider rows are only metal-facing
backends that may satisfy raw allocation or page-source requests after a later
activation row proves safety.

The v0 provider vocabulary is:

```text
native_system_malloc
native_mimalloc
hako_model_allocator
debug_guarded_allocator
```

These names are documentation vocabulary only in M64. They are not runtime
implementation symbols, CLI options, environment variables, `.inc` matchers, or
process allocator hooks.

## Layer Split

```text
language lifecycle:
  cleanup / fini / ownership / keepalive / weak / GC trigger

hako_alloc policy/state:
  size class / page policy / free-list / reuse / stats / stress proof

provider boundary:
  explicit provider id / capability facts / activation preflight facts

native metal keep:
  platform allocation, OS VM calls, platform atomics/TLS glue
```

## Provider Rows

| Provider id | Role | M64 status |
| --- | --- | --- |
| `native_system_malloc` | system allocator provider for diagnostics and fallback proof work | reserved |
| `native_mimalloc` | mimalloc-backed metal provider for future production proof | reserved |
| `hako_model_allocator` | `.hako` policy/model provider for validation and stress fixtures | reserved |
| `debug_guarded_allocator` | guarded debug provider for leak/bounds/lifecycle checks | reserved |

## Required Future Facts

A future active provider row must provide:

- provider id;
- provider kind;
- supported operations;
- ownership of raw allocation/free/realloc or page-source operations;
- bootstrap allocation strategy;
- reentrancy behavior;
- rollback behavior;
- activation preflight proof id.

## Stop Line

M64 keeps these inactive:

- runtime provider registry;
- provider selection CLI;
- provider environment toggles;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- implicit runtime file-system manifest discovery;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Next Allowed Row

The next row may add a reserved provider manifest fixture or a diagnostic-only
provider parser. It must still return `would_activate = false` and must not
install or select a process allocator provider.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
