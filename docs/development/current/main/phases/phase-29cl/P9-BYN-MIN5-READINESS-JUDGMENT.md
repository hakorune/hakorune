---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` hard-retire readiness judgment を、P6/P7/P8 closeout 後の next exact front として固定し、現時点では negative であることを明示する。
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

1. the acceptance set is green, so the existing compat/proof surfaces are stable
2. daily caller residue still remains in the explicit FileBox compat helper and larger name-resolution migration targets
3. compiled-stage1 surrogate owners are still required as frozen proof owners
4. compat keep owners are explicit residue, but hard-retire readiness still has caveats
5. this judgment is therefore negative today
6. `P11-BYN-MIN5-METHOD-DISPATCH-SHRINK.md` is now landed, so the next exact step is a readiness re-check rather than another pre-opened shrink bucket
7. `P12-BYN-MIN5-FILEBOX-WRITE-COMPAT-SHRINK.md` is now landed
8. readiness still has compiled-stage1 proof and compat-keep caveats, so the judgment remains negative
9. `P13-BYN-MIN5-COMPILED-STAGE1-PROOF-READINESS-INVENTORY.md` confirms the surrogate cluster is still live proof owner
10. the next exact blocker bucket is compat keep readiness, not another caller-shrink slice
11. `P14-BYN-MIN5-COMPAT-KEEP-READINESS-INVENTORY.md` confirms the compat keep cluster is still live keep owner
12. `P15-BYN-MIN5-FILEBOX-BUILTIN-KEEP-INVENTORY.md` confirms the built-in `FileBox` keep surface is the next narrowest compat bucket
13. the next exact blocker bucket is `FileBox.writeBytes` only
14. `P16-BYN-MIN5-FILEBOX-WRITEBYTES-COMPAT-SHRINK.md` is now landed
15. readiness still remains negative because compiled-stage1 proof owners and the remaining compat keep cluster are still live

## Judgment Criteria

1. no new daily caller remains on `by_name`
2. no compiled-stage1 proof owner is still required as a live owner
3. compat keep owners are explicit archive-only or a smaller frozen set with no ambiguity
4. phase-29cl docs can say hard-retire readiness without caveats

## Output

1. this judgment is negative today; `BYN-min5` readiness stays closed
2. after `P17` closed, the next exact front is `P18-BYN-MIN5-LLVM-BACKEND-SURROGATE-READINESS-INVENTORY.md`

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
3. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
4. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
5. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`

## Reopen Rule

Reopen this judgment only if one of these becomes true.

1. a new daily caller appears on `by_name`
2. a compiled-stage1 surrogate becomes the only green proof path again
3. compat keep owners stop being clearly archive-only
4. the docs can no longer explain why hard-retire readiness is or is not open

## Non-Goals

1. widening hook/registry behavior
2. deleting compat keep owners
3. changing `by_name` into a final architecture
4. mixing this judgment with new caller-shrink work

## Next Exact Front

1. `P18-BYN-MIN5-LLVM-BACKEND-SURROGATE-READINESS-INVENTORY.md`
