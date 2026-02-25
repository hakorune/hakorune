---
Status: Instructions
Scope: Phase 29az P1
---

# P1: Migrate strict/dev smokes to FlowBox schema

目的: strict/dev の採用点検証を FlowBox schema（`[flowbox/*]`）へ寄せ、旧 `[coreplan/shadow_adopt:*]` 依存を減らす。

## 非目的

- release の恒常ログを増やすこと（既定挙動は不変）
- 新しい環境変数の追加
- by-name での分岐や “とりあえず通す” ハードコード

## 作業内容

1. Gate 対象の strict/dev smoke を棚卸しし、FlowBox schema の期待を SSOT 化する
   - 参照: `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md`
2. 各 strict/dev smoke で raw output を用いて FlowBox adopt を必須化する
   - 例: `grep -F \"[flowbox/adopt\"`
   - 必須項目: `box_kind` / `features` / `via=shadow`
3. 旧 `coreplan/shadow_adopt` タグは “必須” をやめる（移行中は残ってもよい）
4. “非strict でタグが出ない” を保証する smoke は維持する
   - non-strict は `filter_noise` の影響を受けやすいので raw output を優先する

## 受け入れ

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が緑
- FlowBox タグ coverage gate が緑（strict/dev のみ）
- 旧 `coreplan/shadow_adopt` タグを参照する smoke が gate から消える（最終的に P2 で撤去可能になる）

## 検証（SSOT）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
