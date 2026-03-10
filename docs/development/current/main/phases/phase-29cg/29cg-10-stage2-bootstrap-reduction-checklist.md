---
Status: Accepted
Decision: accepted
Date: 2026-03-09
Scope: `phase-29cg` の reduction checklist。
Related:
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29cg/P0-STAGE2-BOOTSTRAP-REDUCTION-INVENTORY.md
  - tools/selfhost_identity_check.sh
  - tools/selfhost/build_stage1.sh
---

# 29cg-10 Stage2 Bootstrap Reduction Checklist

## 1) Inventory lock

- [x] `SBR-01` exact owner fixed
- [x] `SBR-02` exact condition fixed
- [x] `SBR-03` reduction target fixed

## 2) Contract definition

- [x] `SBR-04` define stage1-first Stage2 build contract for `stage1-cli`
- [x] `SBR-05` define acceptance proof for removing one default-bootstrap dependency

## 3) Execution rule

- [ ] `SBR-06` reduce exactly one Stage2 default-bootstrap dependency
- [ ] `SBR-07` keep `phase-29cf` inventory unchanged while `phase-29cg` executes reduction
- [x] `SBR-08` review mixed worker diff and adopt only minimal Rust-side safe-keep patches

## 4) Done judgment

- [ ] `tools/selfhost_identity_check.sh` no longer needs the current default-bootstrap note for the reduced case
- [ ] `selfhost-bootstrap-route-ssot.md` can reclassify one `future retire target`
- [ ] reduced case can describe the bridge as `temporary bootstrap boundary`, not as current route authority

## 5) Current contract note

- `stage1-cli` reduction means `bridge-first Stage2 build`, not raw `NYASH_BIN=$STAGE1_BIN`
- proof sources:
  - raw direct contract returns `97`
  - `stage1_contract_exec_mode` emits Program(JSON), and `stage1_cli_env.hako` now carries helper defs (`defs_len=22`) for entry-local `Main` helpers
  - `stage1_contract_exec_mode ... emit-mir ...` now returns MIR(JSON)
  - `HAKO_STAGE1_MODULE_DISPATCH_TRACE=1` shows the MirBuilder module-dispatch route is hit and returns `output_bytes=213003` / `output_handle=97`
  - direct kernel/plugin proof accepts the same `stage1_cli_env.hako` Program(JSON v0) and returns MIR(JSON)
  - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` passes with `verify_rc=0`
  - bridge/runtime extern-like names no longer depend on `HAKO_MIR_BUILDER_CALL_RESOLVE` for `Callee::Extern`
  - mixed worker stash review adopted only Rust-side minimal arity canonicalization in `callsite_canonicalize.rs` and `json_v0_bridge/lowering/program.rs`; broader `lang/src/mir/builder/**`, `tools/selfhost/lib/stage1_contract.sh`, and `src/llvm_py/**` stash lanes remain deferred
  - experimental `build_stage1.sh` bridge-first path still exits non-zero because the reduced Stage2 object materializes only entry-local defs while helper/import calls and `env.console.log` remain `Global`
  - exact next blocker is helper/source closure plus selfhost MIR call classification, not bridge return-path or current LLVM PHI wiring
