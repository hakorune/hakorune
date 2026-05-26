---
Status: Landed
Date: 2026-05-27
Scope: select the first load-only DLL metadata smoke after benchmark contracts stabilized.
Blocker: MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-07-MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-08 DLL Load-Only Selection

## Decision

Close:

```text
MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001
```

Select the first DLL/provider-package row after the benchmark contract lane
has a real exact-EXE repeated measurement.

The selected row is metadata-first and no-load:

```text
MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001
```

Reason: the provider package ABI SSOT defines a package as a manifest,
descriptor, hash, and host-side preflight before a host loads a shared library.
The next row should validate the package metadata shape before any `dlopen`,
`LoadLibrary`, provider API call, allocator replacement, hook, or
`#[global_allocator]` work opens.

## Selected Next

```text
MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001
```

The row should accept only a fixture-level metadata preflight contract:

```text
dll_mode=metadata-preflight
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Stop Line

This row does not generate a DLL/shared library, load one, call exported
symbols, activate a provider, replace the process allocator, install hooks,
or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_dll_load_only_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
