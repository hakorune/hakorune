---
Status: Landed
Date: 2026-05-27
Scope: close the .hako-derived provider package pilot with descriptor/API-bind evidence.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-30-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - src/cli/provider_package_hako_derived_build.rs
  - apps/provider-package/hako-derived-allocator-fixture/main.hako
---

# 296x-31 Provider Package .hako-Derived Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001
```

The `.hako`-derived provider package lane now proves the full package-read
surface for the selected fixture:

```text
selected .hako source
  -> MIR JSON emission
  -> source/MIR hashes in manifest contract
  -> shared-library provider wrapper artifact
  -> metadata preflight
  -> descriptor smoke
  -> provider API bind smoke
```

This closes the first honest `.hako` to provider package artifact step. The
package is derived from real `.hako` source and emitted MIR evidence, and the
resulting artifact exposes the provider ABI descriptor/API table expected by the
runtime smoke tools.

## Evidence

Required output evidence:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
package_mode=hako-derived-provider-package
build_mode=hako-derived-selected-fixture
hako_source_checked=1
hako_mir_json_emitted=1
hako_semantic_provider_codegen=0
shared_library_artifact_generated=1
summary=ok
```

Required smoke evidence:

```text
metadata-preflight=ok
descriptor-smoke=ok
provider-api-bind=ok
descriptor_ready=1
provider_api_bound=1
provider_call_executed=0
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001
```

The next row should decide the smallest semantic provider-codegen boundary:
which `.hako` declarations are allowed to map into native provider entrypoints,
which ABI wrapper responsibilities remain in Rust/C, and which evidence proves
that `hako_semantic_provider_codegen` can move from `0` to a specific nonzero
mode without opening activation or replacement.

## Stop Line

This closeout does not yet make `.hako` allocator semantics executable as
native provider entrypoints. It keeps semantic provider codegen,
activation, process replacement, hooks, global allocator integration, explicit
provider calls, allocator entrypoint measurement, and winner claims closed.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
