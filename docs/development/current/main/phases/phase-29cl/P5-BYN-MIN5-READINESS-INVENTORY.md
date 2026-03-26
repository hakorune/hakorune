---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` hard-retire readiness を判定する前後の blocker inventory を固定し、positive judgment へ進むために何を exact frozen residue と見なしたかを docs で明示する。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P4-BYN-MIN4-HOOK-REGISTRY-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P11-BYN-MIN5-METHOD-DISPATCH-SHRINK.md
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

- `BYN-min5` の readiness judgment に入るために何を blocker と見なしたかを owner bucket で固定する。
- current reading では、この inventory は hard-retire execution 前の frozen residue map として残る。
- これは delete でも code widening でもない。readiness / reopen boundary inventory だよ。

## Blockers

### 1. Daily caller / caller-shrink residue remains as explicit execution residue

- `src/llvm_py/instructions/direct_box_method.py`
  - now delegates the last FileBox compat leaf into `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`
  - direct-route helper itself is thinner, but the compat-only residue still exists in the explicit helper
- `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`
  - owns the remaining `nyash.plugin.invoke_by_name_i64` emission for FileBox
- `src/backend/mir_interpreter/handlers/calls/method.rs`
- `src/runtime/type_registry.rs`
- `src/backend/wasm_v2/unified_dispatch.rs`
  - these remain name-resolution dependent migration targets

### 2. Compiled-stage1 proof residue is frozen exact archive-only

- `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
  - these are frozen exact proof residues and are no longer treated as live readiness blockers

### 3. Compat keep owners are explicit frozen exact keeps

- `crates/nyash_kernel/src/hako_forward_bridge.rs`
- `crates/nyash_kernel/src/hako_forward.rs`
- `crates/nyash_kernel/src/hako_forward_registry.c`
- `lang/c-abi/shims/hako_forward_registry_shared_impl.inc`
- `lang/c-abi/shims/hako_kernel.c`
  - hook registry and fallback policy remain explicit compat-only keeps, but no longer block readiness by themselves

## Current Truth

1. `BYN-min3` is closed.
2. `BYN-min4` is closed.
3. `P9` is now a positive readiness judgment.
4. no new daily caller is allowed to appear while this inventory remains the frozen residue map.
5. compiled-stage1 surrogate residue is no longer treated as a live readiness blocker.
6. compat keep residue is no longer treated as an ambiguous live readiness blocker.
7. `P10`, `P11`, `P12`, and `P16` narrowed the visible FileBox / method residue before the positive re-check.
8. `P13`, `P17`, and `P18` classify the surrogate cluster as archive-only proof residue.
9. `P14`, `P19`, and `P20` classify the compat keep cluster as a frozen exact keep set.
10. next exact front is `P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md`.

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
3. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
4. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
5. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`

## Reopen Rule

Reopen this inventory only when one of these becomes true.

1. a new daily caller appears on `by_name`
2. a compiled-stage1 surrogate becomes the only green proof path again
3. compat keep owners stop being a clearly frozen exact set
4. the phase-29cl docs can no longer explain hard-retire readiness without caveats

## Non-Goals

1. deleting `by_name.rs`
2. removing hook/registry compat keeps
3. widening module-string dispatch
4. reintroducing daily by_name callers while this inventory is open
