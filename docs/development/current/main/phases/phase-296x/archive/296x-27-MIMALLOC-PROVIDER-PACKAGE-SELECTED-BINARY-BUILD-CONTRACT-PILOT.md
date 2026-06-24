---
Status: Landed
Date: 2026-05-27
Scope: pilot the smallest selected provider binary build/package contract.
Blocker: MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-26-MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/reference/runtime/provider-package-v0.md
  - src/cli/provider_package_selected_binary_build.rs
---

# 296x-27 Provider Package Selected Binary Build Contract Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CONTRACT-PILOT-296X-001
```

Add a Phase B1 CLI producer:

```text
--provider-package-selected-binary-build-fixture
```

This producer builds one Hakorune-selected fixture provider binary, then emits
the same manifest v1 package layout used by Phase A. The build step is fixed
and owned by Hakorune; it does not execute an arbitrary user shell command and
does not discover provider binaries from ambient search paths.

## Accepted Contract

```text
output_contract=hakorune-provider-package-selected-binary-build-v0
package_mode=selected-binary-build-package
build_mode=selected-fixture
build_command_executed=1
hako_shared_library_generation=0
arbitrary_shell_build_executed=0
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

## Boundary

The fixture provider exists only to prove the build/package lane. It may export
the provider descriptor and API table symbols, but the package command must not
load the resulting shared library, resolve exports, read the descriptor, bind
the API, call allocator entrypoints, activate the provider, replace the process
allocator, install hooks, enable global allocator integration, or claim
benchmark winners.

Full `.hako` to provider package generation remains Phase C.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-SELECTED-BINARY-BUILD-CLOSEOUT-296X-001
```

The next row should close Phase B1 by collecting the selected build CLI, the
generated manifest preflight evidence, and reference docs. It should not widen
the selected fixture into `.hako` generation or external provider families.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_selected_binary_build_contract_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
