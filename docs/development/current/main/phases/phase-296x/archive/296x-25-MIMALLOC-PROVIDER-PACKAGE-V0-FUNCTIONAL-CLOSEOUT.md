---
Status: Landed
Date: 2026-05-27
Scope: close provider package v0 as a functional existing-binary package lane.
Blocker: MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-22-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-24-MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS.md
  - docs/reference/runtime/provider-package-v0.md
  - src/cli/provider_package_existing_binary.rs
---

# 296x-25 Provider Package v0 Functional Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001
```

Provider package v0 is functional for the accepted Phase A scope:

```text
existing shared library
  -> Hakorune CLI package command
  -> provider package artifact
  -> generated manifest
  -> no-load metadata preflight
  -> stable user-facing docs
```

The functional evidence is:

```text
CLI package guard:
  tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_pilot_guard.sh

Usage docs guard:
  tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_usage_docs_guard.sh

Reference docs:
  docs/reference/runtime/provider-package-v0.md
```

The CLI guard builds a fixture shared library, packages it through the
Hakorune CLI, verifies `hakorune_provider.json`,
`hakorune_provider.sha256`, and the copied artifact, then runs metadata
preflight on the generated manifest.

## Accepted Contract

```text
output_contract=hakorune-provider-package-existing-binary-manifest-v0
package_mode=existing-binary-manifest
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

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-PHASE-B-BUILD-SELECTION-296X-001
```

The next lane may decide how Hakorune should build selected provider binaries.
It must not silently expand v0 into `.hako` shared-library generation,
provider activation, process allocator replacement, hooks, or global allocator
integration.

## Stop Line

v0 closes existing-binary packaging only. Full `.hako` to shared-library
generation, descriptor-read promotion into the package command, provider API
activation, allocator replacement, hooks, global allocator integration, and
benchmark winner claims remain later decisions.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_pilot_guard.sh
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_usage_docs_guard.sh
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_functional_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
