---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` hard-retire readiness を判定する前の blocker inventory を固定し、いまは開始条件未充足であることを docs で明示する。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P4-BYN-MIN4-HOOK-REGISTRY-CLOSEOUT.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/hako_forward_registry.c
  - lang/c-abi/shims/hako_forward_registry_shared_impl.inc
  - lang/c-abi/shims/hako_kernel.c
  - src/llvm_py/instructions/direct_box_method.py
  - src/backend/mir_interpreter/handlers/calls/method.rs
  - src/runtime/type_registry.rs
  - src/backend/wasm_v2/unified_dispatch.rs
  - src/llvm_py/tests/
---

# P5: BYN-min5 Readiness Inventory

## Purpose

- `BYN-min5` は hard-retire readiness judgment の段階だが、現時点では開始条件が揃っていない。
- ここでは "なぜまだ開始できないか" を owner bucket で固定し、早開きや曖昧な reopen を防ぐ。
- これは delete でも code widening でもない。readiness blocker inventory only だよ。

## Blockers

### 1. Daily caller / caller-shrink residue is still present

- `src/llvm_py/instructions/direct_box_method.py`
  - current direct-miss fallback leaf still emits `nyash.plugin.invoke_by_name_i64`
  - compat-only residue ではあるが、caller shrink wave の最後に残る evidence leaf だよ
- `src/backend/mir_interpreter/handlers/calls/method.rs`
- `src/runtime/type_registry.rs`
- `src/backend/wasm_v2/unified_dispatch.rs`
  - these remain name-resolution dependent migration targets

### 2. Compiled-stage1 proof owners are still required

- `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
  - these are still frozen exact proof owners and are not yet removable

### 3. Compat keep owners still carry bootstrap/module-string evidence

- `crates/nyash_kernel/src/hako_forward_bridge.rs`
- `crates/nyash_kernel/src/hako_forward.rs`
- `crates/nyash_kernel/src/hako_forward_registry.c`
- `lang/c-abi/shims/hako_forward_registry_shared_impl.inc`
- `lang/c-abi/shims/hako_kernel.c`
  - hook registry and fallback policy are still explicit compat-only keeps

## Current Truth

1. `BYN-min3` is closed.
2. `BYN-min4` is closed.
3. `BYN-min5` is not open yet because the current inventory still contains proof/compat residues.
4. no new daily caller is allowed to appear while this inventory stays pending.
5. readiness judgment can only happen after these blocker buckets stop owning live proof.

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
3. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
4. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
5. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`

## Reopen Rule

`BYN-min5` can move from readiness inventory into actual readiness judgment only when all of these are true.

1. no daily caller remains
2. no compiled-stage1 proof owner is still required
3. compat keep owners are explicit archive-only or demoted to a smaller frozen set
4. the phase-29cl docs can say hard-retire readiness without caveats

## Non-Goals

1. deleting `by_name.rs`
2. removing hook/registry compat keeps
3. widening module-string dispatch
4. reintroducing daily by_name callers while this inventory is open
