---
Status: Landed
Date: 2026-05-27
Scope: close the first .hako semantic provider-codegen pilot for provider ping.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-33-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/reference/runtime/provider-package-v0.md
  - tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_ping_closeout_guard.sh
---

# 296x-34 Provider Package .hako Semantic Ping Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001
```

The `.hako` semantic ping pilot is accepted across the staged provider package
ladder:

```text
.hako source + MIR JSON
  -> generated shared-library provider artifact
  -> metadata preflight
  -> descriptor-read smoke
  -> API-bind smoke
  -> no-op provider call smoke
```

The semantic value remains visible at the final no-op provider call:

```text
HakoProvider.ping/0 -> 7
provider_noop_call_result=7
```

## Evidence Contract

The closeout row requires all of these contracts over the same generated
package manifest:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
hako_semantic_provider_codegen=ping-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=7
shared_library_artifact_generated=1
provider_call_executed=0
summary=ok
```

```text
output_contract=hakorune-provider-package-metadata-preflight-v0
dll_mode=metadata-preflight
shared_library_load_executed=0
descriptor_ready=0
provider_active=0
replacement_active=0
summary=ok
```

```text
output_contract=hakorune-provider-package-descriptor-smoke-v0
dll_mode=descriptor-smoke
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_call_executed=0
allocator_entrypoint_called=0
summary=ok
```

```text
output_contract=hakorune-provider-package-api-bind-smoke-v0
dll_mode=provider-api-bind
provider_api_bound=1
provider_call_executed=0
allocator_entrypoint_called=0
summary=ok
```

```text
output_contract=hakorune-provider-package-noop-call-smoke-v0
dll_mode=provider-noop-call
provider_call_executed=1
provider_noop_call_executed=1
provider_noop_call_result=7
allocator_entrypoint_called=0
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
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001
```

The next row should define the smallest honest allocator semantic boundary for
`.hako`-derived provider packages. It may select an explicit `alloc/free` pilot,
but it must keep process allocator replacement, hooks, global allocator
integration, provider activation, and winner claims closed.

## Stop Line

This closeout proves only package generation plus `HakoProvider.ping/0`
semantic codegen. It does not codegen allocator entrypoints, activate providers,
replace allocators, install hooks, use global allocator integration, or make
benchmark winner claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_ping_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
