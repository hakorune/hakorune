---
Status: Landed
Date: 2026-05-27
Scope: close the selected provider binary build/package pilot before opening .hako-derived provider package work.
Blocker: MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-27-MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - src/cli/provider_package_selected_binary_build.rs
---

# 296x-28 Provider Package Selected Binary Build Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001
```

The Phase B1 selected fixture build now produces a real provider package
artifact:

```text
output_contract=hakorune-provider-package-selected-binary-build-v0
package_mode=selected-binary-build-package
build_mode=selected-fixture
build_command_executed=1
hako_shared_library_generation=0
arbitrary_shell_build_executed=0
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

The generated descriptor now reports the same `contract_hash` as the generated
manifest, and `hakorune_provider_get_api_v1` exposes the API table shape used
by the runtime smoke tools.

## Evidence

The selected build package was verified through the existing ladder:

```text
metadata-preflight=ok
descriptor-smoke=ok
provider-api-bind=ok
provider-explicit-repeated-measurement=ok
provider-explicit-comparison-adapter=ok
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

Representative repeated provider evidence:

```text
sample_count=3
warmup_count=1
operation_repeat=128
request_size=32
request_align=8
provider_alloc_executed=1
provider_free_executed=1
summary=ok
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-BUILD-SELECTION-296X-001
```

The next row should define Phase C: `.hako`-derived provider package build
selection. It should choose the smallest `.hako` source-to-provider-package
boundary and keep activation, process replacement, hooks, global allocator
integration, and winner claims closed.

## Stop Line

This closeout does not turn arbitrary `.hako` code into a shared library yet.
It does not activate providers, replace the process allocator, install hooks,
use global allocator integration, or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_selected_binary_build_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
