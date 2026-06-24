---
Status: Landed
Date: 2026-05-27
Scope: select the first .hako semantic allocator-entrypoint boundary.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-34-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-CLOSEOUT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - tools/allocator/provider_package_alloc_free_smoke.py
---

# 296x-35 Provider Package .hako Semantic Alloc/Free Selection

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-SELECTION-296X-001
```

Select `alloc-free-owns-literal-v0` as the first allocator-entrypoint semantic
provider-codegen mode.

The accepted shape is intentionally narrow:

```text
.hako static box HakoProvider {
  ping() { return <i64 literal> }
  ownsAllocated() { return <0|1 literal> }
}
  -> MIR JSON functions HakoProvider.ping/0 and HakoProvider.ownsAllocated/0
  -> generated provider hako_ping() returns ping literal
  -> generated provider hako_owns(non_null_ptr) returns owns literal
  -> provider alloc/free smoke calls explicit alloc/free and observes owns result
```

This opens explicit allocator entrypoint calls through the provider API, but
does not claim `.hako` owns native pointer allocation mechanics yet. The
allocation/free mechanics remain in the Hakorune-owned wrapper for this pilot;
the `.hako` semantic value is the observable ownership policy.

## Accepted Output Vocabulary

```text
--provider-package-hako-semantic-codegen alloc-free-owns-literal-v0
hako_semantic_provider_codegen=alloc-free-owns-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=<i64>
hako_provider_owns_codegen=1
hako_provider_owns_value=<0|1>
provider_alloc_executed=1
provider_free_executed=1
provider_owns_result=<same owns literal for non-null pointer>
allocator_entrypoint_called=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT-296X-001
```

The next row should implement the `alloc-free-owns-literal-v0` mode, extend the
selected fixture with `HakoProvider.ownsAllocated/0`, extract the literal from
MIR JSON, generate `hako_owns()` from that value, and prove it through
`provider_package_alloc_free_smoke.py`.

## Stop Line

This selection does not make `.hako` responsible for native pointer allocation,
reallocation, aligned allocation, process allocator replacement, hooks, global
allocator integration, provider activation, or benchmark winner claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_alloc_free_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
