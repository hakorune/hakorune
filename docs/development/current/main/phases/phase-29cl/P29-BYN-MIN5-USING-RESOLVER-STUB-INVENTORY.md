---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P28` after confirming `module_string_dispatch.rs` is still a live parent router; inventory the narrow `resolve_for_source` stub route before reopening any broader module-string route family.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/tests.rs
  - lang/src/runner/stage1_cli.hako
  - lang/src/runner/stage1_cli_env.hako
---

# P29: BYN-min5 Using-Resolver Stub Inventory

## Purpose

- inventory the narrow `Stage1UsingResolverBox.resolve_for_source` module-string route inside `module_string_dispatch.rs`
- keep the live `MirBuilderBox.emit_from_source_v0` compat seam out of this slice
- decide whether the resolver route is still a live keep surface or only a frozen stub bucket

## Current Truth

1. `module_string_dispatch.rs` remains the live parent router surface after `P28`
2. `handle_using_resolver_resolve_for_source(...)` is still an intentionally empty-string stub
3. kernel tests still pin both direct `dispatch_stage1_module(...)` and exported `nyash_plugin_invoke_by_name_i64(...)` proof for `resolve_for_source`
4. `Stage1UsingResolverBox.resolve_for_source(...)` already has direct-lowered proof on the LLVM Python side, so this bucket is about the remaining kernel/module-string stub only
5. `MirBuilderBox.emit_from_source_v0` remains a separate live compat seam and must not be mixed into this inventory

## Next Exact Front

1. inventory `resolve_for_source` caller-proof and decide whether the stub can move from live keep to frozen residue without touching `emit_from_source_v0`
