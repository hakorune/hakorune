---
Status: Ready
Scope: docs+verification (Pattern9 legacy table removal decision)
Related:
  - docs/development/current/main/phases/phase-29ap/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - src/mir/builder/control_flow/joinir/patterns/router.rs
---

# Phase 29ap P7: Pattern9 legacy table removal decision (and removal if safe)

Date: 2025-12-31  
Status: Ready for execution  
Goal: Pattern9 を JoinIR legacy table から外しても SSOT gate が壊れないかを確認し、OKなら撤去する。

## 非目的

- Pattern9 の新規実装/拡張
- 既定挙動・恒常ログの変更
- 新しい env var 追加

## Step 0: P6 の着地確認 (必須)

- `git status -sb` が clean
- Gate が緑:
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Step 1: Pattern9 使用状況の監査 (撤去可否の判断材料)

列挙して「stdlib/quick/gate が依存しているか」を確認する:

- 参照箇所:
  - `rg -n "Pattern9|pattern9|AccumConstLoop" src/ apps/ tools/ docs/`
- legacy table への掲載有無:
  - `rg -n "legacy.*Pattern9|Pattern9" src/mir/builder/control_flow/joinir/patterns/router.rs`
- smoke/gate の対象:
  - `rg -n "pattern9" tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh tools/smokes/v2/profiles/integration/joinir/* || true`

判断基準 (SSOT):

- stdlib / quick / gate が Pattern9 依存 → P7 は「撤去保留」として理由を docs に固定
- 上記に該当しない → 撤去 OK

## Step 2A: 撤去 OK の場合

- `src/mir/builder/control_flow/joinir/patterns/router.rs` から Pattern9 entry を削除
- Pattern9 wrapper/module が未使用なら削除 or plan への委譲に縮退
- legacy pack があれば「legacy扱い」の注記だけ残す (gate には入れない)

## Step 2B: 撤去 NG の場合

- `docs/development/current/main/phases/phase-29ap/README.md` に保留理由を 1 段落で固定
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` の Next を P8 に進める

## Step 3: 検証 (必須)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Step 4: docs 更新 (最小)

- `docs/development/current/main/phases/phase-29ap/README.md` に P7 結果を反映
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## コミット

- `git commit -m "phase29ap(p7): decide pattern9 legacy removal"`
