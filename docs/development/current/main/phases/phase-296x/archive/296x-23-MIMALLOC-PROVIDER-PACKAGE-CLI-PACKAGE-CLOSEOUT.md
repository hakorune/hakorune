---
Status: Landed
Date: 2026-05-27
Scope: close the Hakorune CLI provider package entry and select stable v0 usage docs.
Blocker: MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-22-MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT.md
  - src/cli/provider_package_existing_binary.rs
---

# 296x-23 Provider Package CLI Package Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001
```

The Hakorune CLI now has a Phase A provider package entry for existing
provider shared libraries. The accepted surface is:

```text
--provider-package-existing-binary
--provider-package-out-dir
--provider-package-id
--provider-package-name
--provider-package-target-triple
--provider-package-platform
```

The CLI emits:

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

Generated packages contain `hakorune_provider.json`,
`hakorune_provider.sha256`, and the selected shared-library artifact. The
generated manifest passes metadata preflight without loading the provider.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001
```

The next row should document the v0 command, output layout, stop line, and
preflight verification path as the stable user-facing provider packaging
entry. Selected provider binary build remains a later lane.

## Stop Line

This closeout does not compile `.hako` to a shared library, load provider
binaries, resolve exports, read descriptors, bind provider APIs, call
allocator entrypoints, activate providers, replace the process allocator,
install hooks, use global allocator integration, or compute winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
