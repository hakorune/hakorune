---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` readiness の次 blocker bucket として、runtime method dispatch の name-resolution dependent residue を `method.rs` 先頭で縮める。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md
  - src/backend/mir_interpreter/handlers/calls/method.rs
  - src/runtime/type_registry.rs
  - src/backend/wasm_v2/unified_dispatch.rs
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P11: BYN-min5 Method Dispatch Shrink

## Purpose

- Shrink the next narrowest blocker after `P10` at the runtime dispatch entrypoint.
- Treat `method.rs` as the first live consumer to tighten, while `type_registry.rs` remains the shared slot SSOT and `unified_dispatch.rs` stays a later mirror consumer.
- Do not mix this wave with compiled-stage1 surrogate or hook/registry demotion.

## Fixed Targets

1. `src/runtime/type_registry.rs`
   - shared slot/name SSOT referenced by the runtime consumers
2. `src/backend/mir_interpreter/handlers/calls/method.rs`
   - first exact runtime consumer to shrink
3. `src/backend/wasm_v2/unified_dispatch.rs`
   - later mirror consumer; not the first edit target

## Current Truth

1. `method.rs` still resolves dynamic calls through `resolve_slot_by_name(...)` and then carries local special-method fallback after slot lookup misses.
2. this makes `method.rs` the narrowest runtime blocker that still materially affects `P9` readiness.
3. `type_registry.rs` remains the shared slot owner and should not gain consumer-specific fallback policy.
4. `unified_dispatch.rs` is a mirror consumer and is not the first next edit target.
5. this is the next exact blocker bucket under the still-negative `P9` readiness judgment.

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
3. targeted Rust tests covering `method.rs` dispatch and `resolve_slot_by_name(...)` users stay green

## Reopen Rule

Reopen this wave only when one of these is true.

1. `method.rs` starts carrying new local fallback truth that should instead move into shared slot ownership
2. `unified_dispatch.rs` becomes the only place that knows a migrated method family
3. docs stop making it clear that `method.rs` is the first next slice and `type_registry.rs` remains shared SSOT

## Non-Goals

1. modifying `module_string_dispatch.rs`
2. modifying `hako_forward_bridge.rs`
3. deleting `by_name.rs`
4. mixing this wave with hard-retire execution
