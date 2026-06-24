---
Status: Landed
Date: 2026-05-27
Scope: document the stable v0 provider package command, output layout, and preflight verification path.
Blocker: MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001
Related:
  - docs/reference/runtime/provider-package-v0.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - src/cli/provider_package_existing_binary.rs
---

# 296x-24 Provider Package v0 Usage Docs

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-V0-USAGE-DOCS-296X-001
```

The stable v0 user-facing entry is now documented at:

```text
docs/reference/runtime/provider-package-v0.md
```

The documented v0 lane is:

```text
existing shared library
  -> Hakorune CLI package command
  -> hakorune_provider.json + sha256 + shared-library artifact
  -> no-load metadata preflight
```

The docs fix the accepted command surface, output layout, generated manifest
contract, and verification command. They also keep the v0 stop line explicit:
the package command does not load providers, resolve exports, read descriptors,
bind provider APIs, call allocator entrypoints, activate providers, replace the
process allocator, install hooks, use global allocator integration, or claim
benchmark winners.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-V0-FUNCTIONAL-CLOSEOUT-296X-001
```

The next row should close the provider package v0 lane by collecting the CLI
package guard, metadata preflight evidence, usage docs guard, and quick gate
status into one final feature-ready card.

## Stop Line

This row is documentation-only. It does not add a new package mode, compile
`.hako` to a shared library, load provider binaries, resolve exports, read
descriptors, bind provider APIs, call allocator entrypoints, activate
providers, replace the process allocator, install hooks, use global allocator
integration, or compute winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_v0_usage_docs_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
