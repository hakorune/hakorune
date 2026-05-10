---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M73 debug guarded allocator provider proof fixture.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-hako-model-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-debug-guarded-proof-v0.toml
---

# Allocator Provider Debug Guarded Proof (SSOT)

## Goal

Define the reserved proof fixture for `debug_guarded_allocator` before any
active provider registry, provider selection, hook activation, or process
allocator replacement row exists.

M73 is a proof-fixture row. It validates the guarded-provider diagnostic proof
shape without activating a replacement allocator.

## Decision

`debug_guarded_allocator` remains a diagnostic provider only. It may describe
guarded allocation lifecycle checks and leak-check observation, but it must not
replace the process allocator, install a runtime hook, add environment toggles,
or widen allocator activation routes.

The reserved proof fixture is:

```text
docs/development/current/main/design/allocator-provider-debug-guarded-proof-v0.toml
```

That fixture is not consumed by runtime or backend code in M73.

## Guarded Contract

The provider manifest already reserves:

```text
provider_id = "debug_guarded_allocator"
provider_kind = "debug_guarded_provider"
operations = ["alloc", "realloc", "free", "guard_check", "leak_check"]
```

M73 adds the proof vocabulary that a later validator must require before this
provider can participate in any selection or activation row. The proof still
reports:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Required Future Proof Bundle

Any future active debug guarded provider row must prove all of these in one
row:

```text
explicit_provider_manifest_fact
provider_readiness_preflight_ready
combined_dry_run_ready
guard_check_lifecycle_bounds_named
leak_check_observation_named
allocation_api_guard_surface_named
no_process_allocator_replacement
no_native_metal_activation
no_hidden_environment_toggle
no_app_or_facade_name_matching
fail_fast_diagnostic_named
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/debug-guarded-proof-missing]
```

## Stop Line

M73 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- native metal provider activation;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
