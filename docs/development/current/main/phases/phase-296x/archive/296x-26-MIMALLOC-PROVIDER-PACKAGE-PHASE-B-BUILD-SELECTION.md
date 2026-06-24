---
Status: Landed
Date: 2026-05-27
Scope: select the next provider package Phase B build lane after v0 existing-binary packaging.
Blocker: MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001
Related:
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/reference/runtime/provider-package-v0.md
---

# 296x-26 Provider Package Phase B Build Selection

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001
```

Select Phase B as a selected-provider-binary build/package lane:

```text
repo-selected provider source or build fixture
  -> explicit Hakorune-owned build step
  -> shared-library artifact
  -> existing provider package manifest path
  -> metadata preflight
```

Phase B does not mean arbitrary shell build execution and does not mean full
`.hako` to shared-library generation. The build owner must be a Hakorune-owned
entry with a stable report contract, not hidden discovery or ambient loader
search.

## Phase Split

```text
Phase A:
  package an existing shared library and emit manifest v1

Phase B:
  build a selected provider binary through a Hakorune-owned build entry, then
  reuse the existing manifest/preflight package path

Phase C:
  build a full .hako-derived provider package
```

The next implementation row is Phase B1, not Phase C.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001
```

The next row should define the stable Phase B1 report contract and pilot the
smallest selected provider binary build/package path. It may add code only for
that selected build entry.

## Required Boundaries

Phase B1 must preserve the v0 package and runtime boundaries:

```text
package_mode=selected-binary-build-package
schema_version=hakorune-provider-package-v1
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

No Phase B row may:

```text
run an arbitrary user shell command
search PATH / LD_LIBRARY_PATH for provider binaries
compile .hako into a shared library
resolve exports as part of package build
read descriptors as part of package build
bind provider APIs as part of package build
call allocator entrypoints as part of package build
activate a provider
replace the process allocator
install hooks
enable global allocator integration
claim benchmark winners
```

## Implementation Direction

The Phase B1 pilot should reuse the existing provider package manifest contract
instead of creating a second manifest shape. The new surface should be a build
producer in front of the existing package path:

```text
selected build producer
  produces artifact bytes
  records build_mode / build_profile / artifact hash
  delegates manifest layout to provider package v1
```

Keep the first build producer intentionally narrow. Widening to platform
matrix builds, external provider families, and `.hako`-derived provider
packages are later rows.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_phase_b_build_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
