---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M74 native system allocator provider proof boundary.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-debug-guarded-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-system-proof-v0.toml
---

# Allocator Provider Native System Proof (SSOT)

## Goal

Define the reserved proof boundary for `native_system_malloc` before any
active provider registry, provider selection, `#[global_allocator]`, or process
allocator replacement row exists.

M74 is a proof-boundary row. It documents the native system allocator contract
and adds a reserved fixture, but it does not activate native allocation.

## Decision

`native_system_malloc` remains a native system provider boundary only. It may
describe the ABI contract for a future system allocator provider, but it must
not become the process allocator, install a runtime hook, add environment
toggles, or widen allocator activation routes.

The reserved proof fixture is:

```text
docs/development/current/main/design/allocator-provider-native-system-proof-v0.toml
```

That fixture is not consumed by runtime or backend code in M74.

## Native System Contract

The provider manifest already reserves:

```text
provider_id = "native_system_malloc"
provider_kind = "native_system_allocator"
operations = ["alloc", "realloc", "free"]
```

M74 adds the proof vocabulary that a later validator must require before this
provider can participate in any selection or activation row. The proof still
reports:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Required Future Proof Bundle

Any future active native system provider row must prove all of these in one
row:

```text
explicit_provider_manifest_fact
provider_readiness_preflight_ready
combined_dry_run_ready
system_allocator_abi_surface_named
malloc_realloc_free_contract_named
oom_failure_policy_named
bootstrap_allocation_path_named
no_global_allocator_attribute
no_process_allocator_replacement
no_runtime_hook_activation
no_hidden_environment_toggle
no_app_or_facade_name_matching
fail_fast_diagnostic_named
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/native-system-proof-missing]
```

## Stop Line

M74 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- native metal provider activation;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
