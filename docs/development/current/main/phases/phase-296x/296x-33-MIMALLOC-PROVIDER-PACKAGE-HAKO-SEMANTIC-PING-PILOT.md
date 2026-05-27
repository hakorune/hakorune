---
Status: Landed
Date: 2026-05-27
Scope: implement the first .hako semantic provider-codegen pilot for provider ping.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-32-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION.md
  - src/cli/provider_package_hako_derived_build.rs
  - apps/provider-package/hako-derived-allocator-fixture/main.hako
  - tools/allocator/provider_package_noop_call_smoke.py
---

# 296x-33 Provider Package .hako Semantic Ping Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001
```

Add the first `.hako` semantic provider-codegen mode:

```bash
--provider-package-hako-semantic-codegen ping-literal-v0
```

The selected fixture now defines:

```text
HakoProvider.ping/0 -> i64 literal 7
```

The package build emits MIR JSON, extracts that literal, and generates the
provider wrapper `hako_ping()` so runtime no-op smoke observes the same value.

## Evidence

Required build evidence:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
hako_semantic_provider_codegen=ping-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=7
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
output_contract=hakorune-provider-package-noop-call-smoke-v0
provider_call_executed=1
provider_noop_call_executed=1
provider_noop_call_result=7
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
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT-296X-001
```

The closeout row should re-run the semantic ping package through metadata,
descriptor, API-bind, and no-op call evidence, then choose the next semantic
entrypoint boundary.

## Stop Line

This pilot opens only `HakoProvider.ping/0` literal codegen. It does not
codegen allocator entrypoints, activate providers, replace allocators, install
hooks, use global allocator integration, or make winner claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_ping_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
