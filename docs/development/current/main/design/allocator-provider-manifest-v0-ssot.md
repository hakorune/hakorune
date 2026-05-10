---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: reserved allocator provider manifest vocabulary.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0.toml
---

# Allocator Provider Manifest v0 (SSOT)

## Goal

Define a reserved manifest fixture for allocator provider rows before runtime
provider parsing, provider selection, or allocator replacement exists.

## Decision

`allocator-provider-manifest-v0.toml` is a docs fixture only in M65. Runtime,
CLI, `.inc`, and backend code must not consume it yet.

The manifest is reserved and inactive:

```text
schema_version = "allocator_provider_manifest_v0"
status = "reserved"
active = false
provider_selection = "inactive"
activation = "future_row_required"
```

## Provider Rows

The fixture reserves the M64 provider ids:

- `native_system_malloc`
- `native_mimalloc`
- `hako_model_allocator`
- `debug_guarded_allocator`

Every provider row must include:

- `provider_id`;
- `provider_kind`;
- `role`;
- `state = "reserved"`;
- `operations`;
- `activation = "future_row_required"`.

## Stop Line

M65 keeps these inactive:

- runtime provider manifest parser;
- runtime provider registry;
- provider selection CLI;
- provider environment toggles;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- implicit runtime file-system manifest discovery;
- `.inc` provider/hook/facade/policy name matching;
- route widening for allocator activation.

## Next Allowed Row

The next row may add a diagnostic-only provider manifest parser that accepts
caller-provided TOML text. It must not read this fixture from production code,
select a provider, or activate allocator replacement.

## Gate

```bash
bash tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh
bash tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
