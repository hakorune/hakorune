---
Status: Landed
Date: 2026-05-27
Scope: select the first .hako-derived provider package build boundary after the selected-binary build closeout.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001
Related:
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-28-MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT.md
  - src/cli/provider_package_selected_binary_build.rs
---

# 296x-29 Provider Package .hako-Derived Build Selection

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001
```

Select Phase C0/C1 as a staged `.hako`-derived provider package lane:

```text
selected .hako provider fixture
  -> source existence + .hako extension preflight
  -> MIR JSON emission preflight
  -> source_hash + mir_json_hash included in package contract/build metadata
  -> Hakorune-owned provider ABI wrapper artifact
  -> manifest v1 package
  -> metadata preflight
```

This is the smallest honest step from `.hako` toward provider packages. The
first implementation row must prove the package was derived from a selected
`.hako` source by including both source and MIR JSON hashes in the generated
package metadata. It must not claim that arbitrary `.hako` allocator semantics
are already lowered into native provider entrypoints.

## Accepted Output Vocabulary

The next implementation row should introduce a new package contract instead of
overloading the selected-binary fixture contract:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
package_mode=hako-derived-provider-package
build_mode=hako-derived-selected-fixture
hako_source_path=<path>
hako_source_checked=1
hako_source_hash=<sha256>
hako_mir_json_emitted=1
hako_mir_json_hash=<sha256>
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

`hako_semantic_provider_codegen=0` is intentional for the first row. It keeps
the boundary honest: the shared library may still be a Hakorune-owned ABI
wrapper, but the package contract must already be tied to a real `.hako`
source/MIR artifact.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-MINIMAL-FIXTURE-PILOT-296X-001
```

The next row should add the minimal CLI surface and guard for a selected `.hako`
provider fixture. It should reuse manifest v1 and the existing metadata
preflight path. Descriptor/API bind and explicit provider calls stay out of the
first `.hako`-derived build row unless the row explicitly opens them after the
metadata package contract is green.

## Stop Line

This selection does not activate providers, replace the process allocator,
install hooks, use global allocator integration, make winner claims, or promise
arbitrary `.hako` to native provider semantic lowering.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_build_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
