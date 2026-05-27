---
Status: Landed
Date: 2026-05-27
Scope: close the .hako-derived provider package v0 as a functional package artifact.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/reference/runtime/provider-package-v0.md
  - src/cli/provider_package_hako_derived_build.rs
  - tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_functional_closeout_guard.sh
---

# 296x-38 Provider Package .hako-Derived Functional Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT-296X-001
```

The `.hako`-derived provider package v0 is functional as a package artifact:

```text
selected .hako source
  -> source existence and .hako extension preflight
  -> MIR JSON emission
  -> source/MIR hashes in package metadata
  -> generated shared-library provider artifact
  -> manifest v1 and sha256 sidecar
  -> metadata preflight
  -> descriptor-read smoke
  -> API-bind smoke
  -> no-op provider call smoke
  -> explicit alloc/free provider smoke
```

The accepted semantic codegen for v0 remains deliberately staged:

```text
HakoProvider.ping/0 -> hako_ping()
HakoProvider.ownsAllocated/0 -> hako_owns(non_null_ptr)
```

Native pointer allocation/free mechanics are still owned by the generated
wrapper in v0. Provider activation, process allocator replacement, hooks,
global allocator integration, and winner claims remain separate lanes.

## Evidence Contract

The closeout requires one generated package to satisfy:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
package_mode=hako-derived-provider-package
build_mode=hako-derived-selected-fixture
hako_source_checked=1
hako_mir_json_emitted=1
hako_semantic_provider_codegen=alloc-free-owns-literal-v0
hako_provider_ping_value=7
hako_provider_owns_value=1
shared_library_artifact_generated=1
shared_library_load_executed=0
provider_call_executed=0
summary=ok
```

and then pass:

```text
output_contract=hakorune-provider-package-metadata-preflight-v0
output_contract=hakorune-provider-package-descriptor-smoke-v0
output_contract=hakorune-provider-package-api-bind-smoke-v0
output_contract=hakorune-provider-package-noop-call-smoke-v0
output_contract=hakorune-provider-package-alloc-free-smoke-v0
provider_noop_call_result=7
provider_owns_result=1
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
MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001
```

The next row should return to benchmark integration using the now-functional
`.hako`-derived provider package artifact. It must continue to keep activation,
process replacement, hooks, global allocator integration, and winner claims
closed unless a later explicit decision row opens them.

## Stop Line

This closeout does not claim arbitrary `.hako` to native allocator lowering. It
closes the selected v0 package artifact lane only: selected source, MIR hash,
generated provider wrapper, manifest, descriptor/API surface, ping semantic,
owns semantic, and explicit allocator smoke.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_derived_functional_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
