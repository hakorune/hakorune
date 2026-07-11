# Phase 287 P8: Rewriter README / guard（docs-first, 意味論不変）

**Date**: 2025-12-27  
**Status**: Ready（next）  
**Scope**: `src/mir/builder/control_flow/joinir/merge/rewriter/` に README（責務境界）を追加し、SSOT（Plan→Apply）と “この層でやらないこと” を明文化する。  
**Non-goals**: コードの挙動変更、エラータグ変更、ログ増加、Box の再増殖

---

## 目的

- `rewriter/` の責務を 1 枚で説明できるようにする（迷子防止）。
- `rewriter/stages/*` を SSOT として固定し、今後の refactor でも入口がブレないようにする。

---

## 追加するファイル

- `src/mir/builder/control_flow/joinir/merge/rewriter/README.md`

内容（最小）:
- 役割: JoinIR merge の “rewriting pipeline” 実装（Plan→Apply）
- SSOT:
  - `rewriter/stages/mod.rs`（入口）
  - `rewriter/stages/plan/`（純粋変換）
  - `rewriter/stages/apply.rs`（builder mutation）
- やらないこと:
  - contract checks の追加/変更（`merge/contract_checks/*` が担当）
  - boundary の設計変更（`JoinInlineBoundary` の契約は別）
  - silent fallback の追加（Fail-Fast 原則）

---

## 検証

docs-only なので `cargo check` と quick が通ることだけ確認:

```bash
cargo check -p nyash-rust --lib
./tools/smokes/v2/run.sh --profile quick
```
