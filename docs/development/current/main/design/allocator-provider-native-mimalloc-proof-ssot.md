---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: M75 native mimalloc allocator provider proof boundary.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-native-system-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-mimalloc-proof-v0.toml
---

# Allocator Provider Native Mimalloc Proof (SSOT)

## Goal

Define the reserved proof boundary for `native_mimalloc` before any active
provider registry, provider selection, production activation, or process
allocator replacement row exists.

M75 is a proof-boundary row. It documents the native mimalloc provider contract
and adds a reserved fixture, but it does not activate mimalloc as the process
allocator.

## Decision

`native_mimalloc` remains a native provider boundary only. It may describe the
mimalloc ABI/page lifecycle contract for a future provider row, but it must not
be selected, installed, or used for process allocator replacement.

The reserved proof fixture is:

```text
docs/development/current/main/design/allocator-provider-native-mimalloc-proof-v0.toml
```

That fixture is not consumed by runtime or backend code in M75.

## Native Mimalloc Contract

The provider manifest already reserves:

```text
provider_id = "native_mimalloc"
provider_kind = "native_mimalloc_allocator"
operations = ["alloc", "realloc", "free", "page_reserve", "page_commit", "page_decommit"]
```

M75 adds the proof vocabulary that a later validator must require before this
provider can participate in any selection or production activation row. The
proof still reports:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Required Future Proof Bundle

Any future active native mimalloc provider row must prove all of these in one
row:

```text
explicit_provider_manifest_fact
provider_readiness_preflight_ready
combined_dry_run_ready
mimalloc_allocator_abi_surface_named
mimalloc_page_lifecycle_contract_named
mimalloc_size_class_policy_named
mimalloc_remote_free_policy_named
mimalloc_tls_cache_policy_named
no_production_activation_without_later_row
no_global_allocator_attribute
no_process_allocator_replacement
no_runtime_hook_activation
no_hidden_environment_toggle
no_app_or_facade_name_matching
fail_fast_diagnostic_named
```

The reserved fail-fast diagnostic root is:

```text
[allocator-provider/native-mimalloc-proof-missing]
```

## Stop Line

M75 keeps these inactive:

- runtime provider registry implementation;
- provider selection implementation;
- provider selection CLI;
- provider selection environment toggles;
- implicit runtime file-system manifest discovery;
- production mimalloc activation;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- `GlobalAlloc`;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
