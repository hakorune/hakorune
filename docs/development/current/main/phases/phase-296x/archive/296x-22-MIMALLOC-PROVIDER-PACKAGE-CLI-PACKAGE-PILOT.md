---
Status: Landed
Date: 2026-05-27
Scope: expose existing-binary provider package creation through the Hakorune CLI.
Blocker: MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-21-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT.md
  - src/cli/provider_package_existing_binary.rs
---

# 296x-22 Provider Package CLI Package Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001
```

Expose existing-binary package creation through the Hakorune CLI:

```bash
hakorune \
  --provider-package-existing-binary libhakorune_provider.so \
  --provider-package-out-dir dist/hakorune-provider \
  --provider-package-id org.hakorune.provider.example \
  --provider-package-name example-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux
```

The CLI writes:

```text
hakorune_provider.json
hakorune_provider.sha256
libhakorune_provider.so | hakorune_provider.dll | libhakorune_provider.dylib
```

The generated manifest remains compatible with metadata preflight and does not
load or activate the provider.

## Contract

```text
output_contract=hakorune-provider-package-existing-binary-manifest-v0
package_mode=existing-binary-manifest
schema_version=hakorune-provider-package-v1
abi_version=hakorune-provider-abi-v1
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
MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-CLOSEOUT-296X-001
```

The next row should close the CLI package entry and decide whether to document
the command as the stable v0 interface or start selected provider binary build.

## Stop Line

This row does not compile `.hako` to a shared library, load provider binaries,
resolve exports, read descriptors, bind provider APIs, call allocator
entrypoints, activate providers, replace the process allocator, install hooks,
use global allocator integration, or compute winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_cli_package_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
