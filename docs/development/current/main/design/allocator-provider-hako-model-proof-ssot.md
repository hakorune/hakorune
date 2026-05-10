---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M72 hako model allocator provider proof fixture.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md
  - docs/development/current/main/design/allocator-provider-hako-model-proof-v0.toml
---

# Allocator Provider Hako Model Proof (SSOT)

## Goal

Define the reserved proof fixture for `hako_model_allocator` before any active
provider registry, provider selection, native metal provider, or allocator
replacement row exists.

M72 is a proof-fixture row. It validates the shape of the `.hako`
policy/model provider contract without activating it.

## Decision

`hako_model_allocator` remains a model provider only. It is allowed to describe
policy/state validation for Hakorune allocator behavior, but it must not own
native pointers, OS VM metal, runtime provider selection, hook activation, or
process allocator replacement.

The reserved proof fixture is:

```text
docs/development/current/main/design/allocator-provider-hako-model-proof-v0.toml
```

That fixture is not consumed by runtime or backend code in M72.

## Model Contract

The provider manifest already reserves:

```text
provider_id = "hako_model_allocator"
provider_kind = "hako_policy_model"
operations = ["model_alloc", "model_free", "stats", "stress_validate"]
```

M72 adds the proof vocabulary that a later validator must require before this
provider can participate in selection or activation. The proof still reports:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Required Future Proof Bundle

Any future active hako model provider row must prove all of these in one row:

```text
explicit_provider_manifest_fact
provider_readiness_preflight_ready
combined_dry_run_ready
hako_alloc_policy_state_named
model_alloc_free_state_transition_named
model_stats_observation_named
stress_validate_fixture_named
no_native_pointer_or_metal_activation
no_process_allocator_replacement
no_hidden_environment_toggle
no_app_or_facade_name_matching
fail_fast_diagnostic_named
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/hako-model-proof-missing]
```

## Stop Line

M72 keeps these inactive:

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
bash tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
