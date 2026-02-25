# Phase 29aj P6: JoinIR regression SSOT gate + phase143_* isolation

Date: 2025-12-29  
Status: Ready for execution  
Scope: docs + smoke guard only（挙動コードは触らない）  
Goal: JoinIR 回帰の受け入れを phase29ae pack に固定し、phase143_* を隔離する

## Objective

- JoinIR 回帰の integration gate を `phase29ae_regression_pack_vm.sh` に一本化
- phase143_* は対象外と明記（LoopBuilder 撤去 / plugin disable / LLVM exe 期待の乖離）
- phase143_* は legacy pack として隔離し、SKIP で明示

## Non-goals

- phase143_* の修正
- 新 env var 追加
- JoinIR 本体の挙動変更

## Steps

1) docs 更新
   - `docs/development/current/main/phases/phase-29aj/README.md`
   - `docs/development/current/main/phases/phase-29ae/README.md`
   - `docs/development/current/main/10-Now.md`
   - `docs/development/current/main/30-Backlog.md`
   - `CURRENT_TASK.md`
2) phase143 legacy pack 追加
   - `tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` を SKIP で用意

## Verification

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase143_legacy_pack.sh` (SKIP)

## Commit

- `git add -A && git commit -m "docs(phase29aj): define joinir regression gate; isolate phase143 legacy"`
