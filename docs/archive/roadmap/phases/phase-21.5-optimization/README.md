# Phase 21.5 — Optimization (AotPrep‑First)

目的
- .hako 側（AotPrep）で前処理最適化（構造のみ）を行い、LLVM/AOT に渡すIRを軽量にする。
- 既定は挙動不変（opt‑in）。Return 純化ガードで安全性を担保。

チェックリスト（21.5 時点の着地）
- [x] パス分割（StrlenFold / LoopHoist / ConstDedup / CollectionsHot / BinopCSE）
- [x] CollectionsHot（Array/Map）導入（既定OFF）
- [x] Map key モード `NYASH_AOT_MAP_KEY_MODE={h|i64|hh|auto}`
- [x] LoopHoist v1 / BinopCSE v1（最小）
- [x] ベンチ `linidx`/`maplin` 追加
- [ ] LoopHoist v2（+/* 右項 const の連鎖前出し/fix‑point）
- [ ] BinopCSE v2（線形 `i*n` 共通化の強化）
- [ ] CollectionsHot v2（array index の共通SSA利用）
- [ ] Map auto 精緻化（_is_const_or_linear の再帰判定）
- [ ] Idempotence（置換済みタグで再実行時も不変）
- [ ] `arraymap`/`matmul` ≤ 125%（C基準）

メモ（21.5 クロージング）
- linidx/maplin など「線形インデックス＋Array/Map」系は CollectionsHot + hoist/CSE で C≒100% 近辺まで到達。
- arraymap は Array/Map 部分の externcall 化は進んだものの、文字列キー生成（toString/`\"k\"+idx`）と hash パスが支配的なため、C の単純 int[] とは根本的に前提が異なる状態で終了。
- matmul は CollectionsHot 自体は単体では効いているが、行列積そのものが ArrayBox ベースであり、Core 数値箱不在のまま 80% 目標には届かず。これは 21.6 以降の「Core 数値箱＋行列箱」導入で扱う。

トグル
- `NYASH_MIR_LOOP_HOIST=1` … StrlenFold/LoopHoist/ConstDedup/BinopCSE を有効化
- `NYASH_AOT_COLLECTIONS_HOT=1` … CollectionsHot（Array/Map）
- `NYASH_AOT_MAP_KEY_MODE` … `h|i64|hh|auto`（推奨: `auto`）
- `NYASH_VERIFY_RET_PURITY=1` … Return 純化ガード（開発時ON）

ベンチ（例）
```bash
export NYASH_SKIP_TOML_ENV=1 NYASH_DISABLE_PLUGINS=1 \
       NYASH_LLVM_SKIP_BUILD=1 NYASH_LLVM_FAST=1 NYASH_LLVM_FAST_INT=1 \
       NYASH_MIR_LOOP_HOIST=1 NYASH_AOT_COLLECTIONS_HOT=1 NYASH_VERIFY_RET_PURITY=1
for c in arraymap matmul sieve linidx maplin; do \
  tools/perf/microbench.sh --case $c --exe --runs 3; echo; done
```
