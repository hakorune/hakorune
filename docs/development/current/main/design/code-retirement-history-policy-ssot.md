# Code Retirement / History Policy (SSOT)

Status: SSOT  
Scope: Rust/.hako 実装の縮退・置換時に「どこに残すか」を固定する。  
Related:
- `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- `CURRENT_TASK.md`

## Purpose

- 退役コードを repo 内に複製保存して散らさない。
- 「保存は Git 履歴、現行は最小コード」を原則化する。
- rollback 可能性は tag/commit 境界で担保する。

## Policy

1. 正本は Git 履歴
- 旧実装の別ファイル退避（`*_old.rs`, `archive/*.rs`）は禁止。
- 退役前の状態は小粒コミットと注釈タグで保存する。

2. 現行コードは最小
- 退役後は削除、または fail-fast stub だけ残す。
- 暫定 compat は既定OFF + 撤去条件つきでのみ許可。

3. 理由は docs に残す
- 退役理由・境界・検証コマンドは `CURRENT_TASK.md` と phase/design SSOT に記録する。
- 「コード本文」を docs に複製しない。必要なら commit hash を記録する。

4. rollback 単位を固定
- 1 blocker = 1 受理形 = 1 commit を基本とする。
- 構造整理で series が必要な場合も、目的1つ・2〜5コミットに限定する。

## Allowed / Forbidden

Allowed:
- `git tag -a <name> -m "<note>"` で節目固定
- `git show <commit>:<path>` で旧実装参照
- docs に commit hash / gate 結果を記録

Forbidden:
- `archive/` や `docs/` への Rust 実装コピー保存
- mainline に無効化されただけの dead code を長期残置
- rollback 用の常時ON fallback 実装

## Minimal Workflow

1. docs-first で「何を retire するか」を SSOT に追記  
2. 退役実装 + gate 緑化  
3. commit（必要なら節目 tag）  
4. `CURRENT_TASK.md` に結果と commit hash を記録
