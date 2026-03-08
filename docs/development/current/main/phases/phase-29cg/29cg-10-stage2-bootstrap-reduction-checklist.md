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

## 5) Current contract note

- `stage1-cli` reduction means `bridge-first Stage2 build`, not raw `NYASH_BIN=$STAGE1_BIN`
- proof sources:
  - raw direct contract returns `97`
  - `stage1_contract_exec_mode` emits Program/MIR successfully
