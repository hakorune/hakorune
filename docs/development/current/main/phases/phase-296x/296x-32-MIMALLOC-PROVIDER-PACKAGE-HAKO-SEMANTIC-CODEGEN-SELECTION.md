---
Status: Landed
Date: 2026-05-27
Scope: select the smallest .hako semantic provider-codegen boundary.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-31-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-CLOSEOUT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - src/cli/provider_package_hako_derived_build.rs
---

# 296x-32 Provider Package .hako Semantic Codegen Selection

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-CODEGEN-SELECTION-296X-001
```

Select `ping-literal-v0` as the first semantic provider-codegen mode.

The accepted shape is intentionally narrow:

```text
.hako static box HakoProvider {
  ping() { return <i64 literal> }
}
  -> MIR JSON function HakoProvider.ping/0
  -> const i64 return literal extraction
  -> generated provider hako_ping() returns that literal
  -> provider noop-call smoke observes the same value
```

This opens exactly one semantic entrypoint and still does not open allocator
entrypoints, activation, replacement, hooks, global allocator integration, or
winner claims.

## Accepted Output Vocabulary

```text
--provider-package-hako-semantic-codegen ping-literal-v0
hako_semantic_provider_codegen=ping-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=<i64>
provider_noop_call_executed=1
provider_noop_call_result=<same i64>
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-PING-PILOT-296X-001
```

The next row should implement the CLI option, update the selected fixture with
`HakoProvider.ping/0`, extract the literal from emitted MIR JSON, generate the
provider wrapper `hako_ping()` from that value, and prove the value through
`provider_package_noop_call_smoke.py`.

## Stop Line

This selection does not codegen `alloc`, `free`, `owns`, `realloc`, aligned
allocation, process allocator replacement, hooks, global allocator integration,
or benchmark winner claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_codegen_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
