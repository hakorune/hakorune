---
Status: Instructions
Scope: Phase 29ba P1
---

# P1: Converge fallback observability to FlowBox freeze (strict/dev only)

目的: strict/dev の fallback 可視化を FlowBox schema（`[flowbox/freeze ...]`）へ収束させ、補助タグ（例: `[plan/fallback:*]`）
を “出さない” 方向へ引き締める。

## 非目的

- release 既定の挙動/恒常ログを変える
- 新しい環境変数の追加
- 「とりあえず通す」ハードコード

## 現状

- fallback は `flowbox/freeze` が SSOT（`docs/development/current/main/design/flowbox-fallback-observability-ssot.md`）
- code 側に `[plan/fallback:*]` が残っている場合、それは “移行中の補助タグ” であり、最終 SSOT ではない

## 作業内容

1. code 側の `[plan/fallback:*]` を撤去する（strict/dev only の補助タグ）
   - 例: `eprintln!("[plan/fallback:planner_none]")` など
2. strict/dev の fallback 可視化は `flowbox/freeze` のみで行う
   - code は `emit_flowbox_freeze_tag_from_facts(...)` を唯一の入口として使う
3. gate smoke は “fallback タグ不在” を維持する
   - gate-target 形状で fallback が出るのはバグなので、`[plan/fallback:` が消えても gate の意味は不変
4. ドキュメントを更新する
   - `docs/development/current/main/phases/phase-29ae/README.md` の説明が FlowBox schema SSOT になっていることを確認

## 受け入れ

- `rg -n "\\[plan/fallback:" src/mir` が 0 件
- strict/dev の raw output に `[plan/fallback:` が出ない（回帰 pack で確認）
- Gate（SSOT）が緑

## 検証（SSOT）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
