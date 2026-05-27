---
Status: Landed
Date: 2026-05-27
Scope: implement the first .hako semantic allocator-entrypoint provider-codegen pilot.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-35-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION.md
  - src/cli/provider_package_hako_derived_build.rs
  - apps/provider-package/hako-derived-allocator-fixture/main.hako
  - tools/allocator/provider_package_alloc_free_smoke.py
---

# 296x-36 Provider Package .hako Semantic Alloc/Free Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001
```

Add the first `.hako` semantic allocator-entrypoint mode:

```bash
--provider-package-hako-semantic-codegen alloc-free-owns-literal-v0
```

The selected fixture now defines:

```text
HakoProvider.ping/0 -> i64 literal 7
HakoProvider.ownsAllocated/0 -> i64 literal 1
```

The package build emits MIR JSON, extracts both literals, generates
`hako_ping()` from the ping value, and generates `hako_owns(non_null_ptr)` from
the ownership value. Runtime explicit alloc/free smoke calls provider
`alloc`, `owns`, and `free`, then observes the `.hako` ownership value.

## Evidence

Required build evidence:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
hako_semantic_provider_codegen=alloc-free-owns-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=7
hako_provider_owns_codegen=1
hako_provider_owns_value=1
provider_call_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

Required runtime smoke evidence:

```text
output_contract=hakorune-provider-package-alloc-free-smoke-v0
dll_mode=provider-alloc-free
provider_call_executed=1
allocator_entrypoint_called=1
provider_alloc_executed=1
provider_free_executed=1
provider_owns_result=1
allocated_pointer_nonzero=1
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
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-CLOSEOUT-296X-001
```

The closeout row should rerun the `alloc-free-owns-literal-v0` package through
metadata, descriptor, API-bind, no-op, and alloc/free smoke evidence, then
select the next semantic allocator boundary.

## Stop Line

This pilot opens explicit provider allocator entrypoint smoke and `.hako`
ownership policy lowering only. It does not make `.hako` responsible for native
pointer allocation/free mechanics, activate providers, replace allocators,
install hooks, use global allocator integration, or make benchmark winner
claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_alloc_free_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
