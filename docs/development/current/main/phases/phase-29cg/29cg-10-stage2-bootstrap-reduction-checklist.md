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

## 4) Done judgment

- [ ] `tools/selfhost_identity_check.sh` no longer needs the current default-bootstrap note for the reduced case
- [ ] `selfhost-bootstrap-route-ssot.md` can reclassify one `future retire target`
- [ ] reduced case can describe the bridge as `temporary bootstrap boundary`, not as current route authority

## 5) Current contract note

- `stage1-cli` reduction means `bridge-first Stage2 build`, not raw `NYASH_BIN=$STAGE1_BIN`
- proof sources:
  - raw direct contract returns `97`
  - `stage1_contract_exec_mode` emits Program(JSON), and `stage1_cli_env.hako` now carries helper defs (`defs_len=20`)
  - `stage1_contract_exec_mode ... emit-mir ...` currently fails with `96`
  - `STAGE1_CLI_DEBUG=1` shows the exact blocker: `MirBuilderBox.emit_from_program_json_v0 returned null`
  - `HAKO_STAGE1_MODULE_DISPATCH_TRACE=1` shows the MirBuilder module-dispatch route is hit, but no success/error payload is returned before the child receives `null`
  - direct kernel/plugin proof accepts the same `stage1_cli_env.hako` Program(JSON v0) and returns MIR(JSON)
  - experimental `build_stage1.sh` bridge-first path still exits non-zero, but helper-def absence is no longer the asserted blocker
