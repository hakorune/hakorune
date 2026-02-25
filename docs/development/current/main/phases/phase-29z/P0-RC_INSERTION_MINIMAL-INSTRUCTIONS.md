# Phase 29z P0: RC insertion minimal（意味論不変ガード付き）

**Date**: 2025-12-27  
**Status**: Ready（next）  
**Scope**: `src/mir/passes/rc_insertion.rs` を no-op から “最小の1ケース” だけ動作する pass にする。  
**Non-goals**: 大規模な所有モデル導入、全ケース対応、既定挙動変更、env var 新設

---

## 目的（SSOT）

- Phase 29y の RC insertion SSOT を “実装で1回証明” する。
- 既定挙動は変えず、**最小ケースだけ**を opt-in で通す（Fail-Fast/rollback容易）。

SSOT:
- `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`

---

## 実装方針（ガード）

既定OFFで導入する（互換維持）。

推奨ガード（env var を増やさない）:
- Cargo feature で opt-in（例: `--features rc-insertion-minimal`）
- どうしてもトグルが必要なら `src/config/env` 集約＆ docs に登録（撤去計画つき）

---

## 最小でやるケース（1つだけ）

### Case: 上書き release（explicit overwrite）

`x = <new>` の直前に、`x` が保持していた “旧値” を release する。

制約:
- PHI/loop/early-exit は扱わない（out-of-scope は今は no-op のまま）
- 影響は “明示的な上書き” のみ

---

## 手順

1. `src/mir/passes/rc_insertion.rs` の入口を整理し、`RcInsertionStats` を維持したまま実装領域を作る
2. “上書き release” だけを挿入（他は触らない）
3. 最小の conformance fixture を 1 本追加（VMのみでも可、LLVMはSKIP可）
4. 既定OFFで quick が緑のままを確認

---

## 検証（受け入れ基準）

```bash
cargo build --release
./tools/smokes/v2/run.sh --profile quick
```

受け入れ:
- Build: 0 errors
- quick: 154/154 PASS
- 既定挙動不変（トグルOFFで完全に影響なし）

追加（opt-in の動作確認）:

```bash
# RC insertion minimal を有効化して、passの最小ケースが動くことを自己診断バイナリで確認
cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal
```
