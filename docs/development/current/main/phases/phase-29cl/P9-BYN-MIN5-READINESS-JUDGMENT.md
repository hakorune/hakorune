---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` hard-retire readiness judgment を、P6/P7/P8 closeout と P10/P12/P17/P18/P19/P20 の evidence 後に positive judgment として固定し、next exact front を execution pack へ進める。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md
  - docs/development/current/main/phases/phase-29cl/P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md
  - docs/development/current/main/phases/phase-29cl/P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P11-BYN-MIN5-METHOD-DISPATCH-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P12-BYN-MIN5-FILEBOX-WRITE-COMPAT-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P13-BYN-MIN5-COMPILED-STAGE1-PROOF-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P14-BYN-MIN5-COMPAT-KEEP-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P17-BYN-MIN5-BUILD-SURROGATE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P18-BYN-MIN5-LLVM-BACKEND-SURROGATE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P19-BYN-MIN5-HAKO-FORWARD-BRIDGE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P20-BYN-MIN5-HAKO-FORWARD-REGISTRY-SHARED-IMPL-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/hako_forward.rs
  - crates/nyash_kernel/src/hako_forward_registry.c
  - lang/c-abi/shims/hako_forward_registry_shared_impl.inc
  - lang/c-abi/shims/hako_kernel.c
  - src/llvm_py/instructions/direct_box_method.py
  - src/backend/mir_interpreter/handlers/calls/method.rs
  - src/runtime/type_registry.rs
  - src/backend/wasm_v2/unified_dispatch.rs
---

# P9: BYN-min5 Readiness Judgment

## Purpose

- Decide whether `BYN-min5` may now enter hard-retire readiness.
- Treat this as a judgment step, not a widening step and not a delete step.
- The judgment can only be made after P6/P7/P8 are all closed.

## Input State

1. `P6-BYN-MIN5-DAILY-CALLER-SHRINK.md` is closed
   - the last FileBox compat leaf is isolated in the explicit compat helper
2. `P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md` is closed
   - compiled-stage1 surrogate owners remain frozen exact owners only
3. `P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md` is closed
   - hook / registry / fallback keep cluster is archive-only residue

## Current Truth

1. the acceptance set is green, and no new mainline `by_name` owner has appeared
2. daily caller residue is confined to explicit FileBox compat execution residue and larger name-resolution migration targets; it no longer makes readiness ambiguous
3. compiled-stage1 surrogate residue is frozen archive-only proof residue, not a live proof owner
4. direct BuildBox and LlvmBackendBox caller proof stays green without reopening generic module-string `by_name`
5. compat keep owners are an explicit frozen exact keep set with a single shared C body owner
6. phase-29cl docs can now describe hard-retire readiness without live-owner caveats
7. this judgment is therefore positive today
8. the next exact front is `P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md`

## Judgment Criteria

1. no new daily caller remains on `by_name`
2. no compiled-stage1 proof owner is still required as a live owner
3. compat keep owners are explicit archive-only or a smaller frozen set with no ambiguity
4. phase-29cl docs can say hard-retire readiness without caveats

## Output

1. this judgment is positive today; `BYN-min5` readiness opens
2. the next exact front is `P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md`

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/checks/phase29cl_by_name_surrogate_archive_guard.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
5. `cargo test -p nyash_kernel build_surrogate_route_contract_is_stable -- --nocapture`
6. `cargo test -p nyash_kernel llvm_backend_surrogate_ -- --nocapture`
7. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
8. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
9. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`

## Reopen Rule

Reopen this judgment only if one of these becomes true.

1. a new daily caller appears on `by_name`
2. a compiled-stage1 surrogate becomes the only green proof path again
3. compat keep owners stop being a clearly frozen exact set
4. the docs can no longer explain why hard-retire readiness is or is not open

## Non-Goals

1. widening hook/registry behavior
2. pretending hard-retire execution is already complete
3. changing `by_name` into a final architecture
4. mixing this judgment with multiple execution slices at once

## Next Exact Front

1. `P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md`
