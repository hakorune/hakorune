# Phase 287 P7: Rewriter Box scaffolding cleanup（意味論不変）

**Date**: 2025-12-27  
**Status**: Completed ✅  
**Scope**: `src/mir/builder/control_flow/joinir/merge/rewriter/` 配下に残っている “旧 pipeline の箱（Box）雛形” を、現状の SSOT（Plan→Apply）に合わせて整理する。  
**Non-goals**: 仕様変更、エラータグ/ヒント文の変更、ログ恒常増加、silent fallback 追加、実行経路の追加

---

## 背景

Phase 287 P6 で Scan stage を削除し、pipeline は 2-stage（Plan→Apply）へ収束した。

一方で、`apply_box.rs` など “箱雛形” が残っており、現在の SSOT（`rewriter/stages/*`）と責務が二重に見える。

---

## 目的

- 入口（`rewriter/stages/*`）を SSOT として明確にし、雛形を整理して迷子を防ぐ。
- “実際に使っていない雛形” は削除し、残す場合は理由を docs に固定する。

---

## 手順（安全な順序）

1. `rg` で参照を確認し、未使用の Box 雛形を特定する
   - 例: `ApplyBox`, `apply_box.rs` など
2. 未使用なら削除（または `#[cfg(test)]` 専用なら test-only へ隔離）
3. `rewriter/mod.rs` の module 宣言と doc を SSOT に合わせて更新
4. `cargo check -p nyash-rust --lib` が通ることを確認

---

## 検証（受け入れ基準）

```bash
cargo build --release
./tools/smokes/v2/run.sh --profile quick
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
```

受け入れ:
- Build: 0 errors
- quick: 154/154 PASS
- Pattern6: RC=9 維持
- 恒常ログ増加なし
