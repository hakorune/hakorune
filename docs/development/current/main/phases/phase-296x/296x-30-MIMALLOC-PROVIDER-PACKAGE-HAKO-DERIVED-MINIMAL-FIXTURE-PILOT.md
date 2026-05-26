---
Status: Landed
Date: 2026-05-27
Scope: add the minimal selected .hako-derived provider package build pilot.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-29-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - src/cli/provider_package_hako_derived_build.rs
  - apps/provider-package/hako-derived-allocator-fixture/main.hako
---

# 296x-30 Provider Package .hako-Derived Minimal Fixture Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001
```

Add the first `.hako`-derived provider package CLI surface:

```bash
hakorune \
  --provider-package-hako-derived-build-fixture apps/provider-package/hako-derived-allocator-fixture/main.hako \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.hako.fixture \
  --provider-package-name hako-derived-fixture-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux
```

The command consumes a selected `.hako` source, emits MIR JSON as a preflight
artifact, records `hako_source_hash` and `hako_mir_json_hash`, builds a
Hakorune-owned provider ABI wrapper shared library, and writes a manifest v1
provider package.

## Output Contract

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
package_mode=hako-derived-provider-package
build_mode=hako-derived-selected-fixture
hako_source_checked=1
hako_mir_json_emitted=1
hako_semantic_provider_codegen=0
shared_library_artifact_generated=1
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

The first pilot intentionally keeps `hako_semantic_provider_codegen=0`. This
means the package is derived from real `.hako` source and emitted MIR evidence,
but the native provider entrypoint wrapper is still owned by Hakorune until a
later semantic provider-codegen row opens.

## Evidence

The generated package passes metadata preflight:

```text
output_contract=hakorune-provider-package-metadata-preflight-v0
manifest_ready=1
binary_hash_ready=1
descriptor_ready=0
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT-296X-001
```

The closeout row should run the `.hako`-derived package through descriptor
smoke and API bind evidence while keeping semantic provider codegen,
activation, replacement, hooks, globals, and winner claims closed.

## Stop Line

This pilot does not make arbitrary `.hako` allocator semantics executable as
native provider entrypoints. It does not activate providers, replace the
process allocator, install hooks, use global allocator integration, or make
winner claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_minimal_fixture_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
