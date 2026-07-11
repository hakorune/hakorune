---
Status: Instructions
Scope: Phase 29az P2
---

# P2: Remove legacy `coreplan/shadow_adopt` tags

目的: 旧 `[coreplan/shadow_adopt:*]` タグの生成と参照を撤去し、strict/dev の観測を FlowBox schema のみに収束させる。

## 非目的

- release 既定の挙動/恒常ログを変える
- 新しい環境変数の追加
- “とりあえず通す” ハードコード

## 作業内容

1. 旧タグの emit を撤去する
   - `eprintln!(\"[coreplan/shadow_adopt:...\"] )` の削除
2. strict/dev gate smoke の “旧タグ必須” を撤去する
   - 代替: raw output の `[flowbox/adopt ... via=shadow]` を必須化
3. `filter_noise` から旧タグフィルタ行を撤去する（残っていれば）
4. SSOT を更新する
   - `docs/development/current/main/phases/phase-29ae/README.md` のタグ説明を FlowBox schema へ
   - 旧タグ SSOT は `Deprecated` 扱いにし、現行 SSOT を FlowBox coverage へ

## 受け入れ

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が緑
- strict/dev の raw output に `[coreplan/shadow_adopt:` が出ない
- strict/dev の gate は FlowBox schema で検証される

## 検証（SSOT）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
