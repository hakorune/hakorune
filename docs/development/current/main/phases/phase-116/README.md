# Phase 116: if-only keep+call merge parity

**Status**: ✅ DONE
**Date**: 2025-12-18

## 背景

LLVMバックエンドで壊れやすいパターンの固定: 片側が元値保持、片側がcall結果のmerge。

### 問題のパターン

```hako
fn f(x) { return x + 1 }
fn g(flag) {
    local v = 10
    if flag == 1 { v = f(1) }
    // else側は暗黙的にv=10を保持
    print(v)
}
g(0)  // → 10 (元値保持)
g(1)  // → 2 (f(1) = 1+1 = 2)
```

- **then側**: call結果でvを更新 (`v = f(1)`)
- **else側**: 元の値を保持 (`v = 10`)
- **merge地点**: 2つの異なるソース（元値 vs call結果）からのPHI

## 実装内容

### 1. テストフィクスチャ

**ファイル**: `apps/tests/phase116_if_only_keep_plus_call_min.hako`

最小限のケース:
- `g(0)` → `10` (元値保持)
- `g(1)` → `2` (call結果)

### 2. VM smoke test

**ファイル**: `tools/smokes/v2/profiles/integration/apps/phase116_if_only_keep_plus_call_vm.sh`

- `output_validator.sh` を使用して数値2行 `10\n2` を検証
- 実行条件: `NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1`

### 3. LLVM EXE smoke test

**ファイル**: `tools/smokes/v2/profiles/integration/apps/phase116_if_only_keep_plus_call_llvm_exe.sh`

- `llvm_exe_runner.sh` を利用（plugin dlopen/cache/build-all SSOT）
- `llvm_exe_build_and_run_numeric_smoke` で出力検証（`10\n2`）

## 検証コマンド

```bash
# VM smoke test
bash tools/smokes/v2/profiles/integration/apps/phase116_if_only_keep_plus_call_vm.sh

# LLVM EXE smoke test
bash tools/smokes/v2/profiles/integration/apps/phase116_if_only_keep_plus_call_llvm_exe.sh

# 回帰テスト (Phase 115)
bash tools/smokes/v2/profiles/integration/apps/phase115_if_only_call_merge_llvm_exe.sh
```

## 技術的詳細

### JoinIR Pattern

このケースは **Pattern 1 (Simple If)** として処理される:

```
entry_block:
  v = 10
  if flag == 1 goto then_block else exit_block

then_block:
  v = f(1)
  goto exit_block

exit_block:
  v_merged = PHI [v=10 from entry, v=f(1) from then]
  print(v_merged)
```

### PHI接続の重要性

- **entry → exit**: 元値 (`10`) を直接伝播
- **then → exit**: call結果 (`f(1)`) を伝播
- **PHI**: 2つの異なる型のソース（変数 vs call結果）を正しくmerge

LLVM IRでは、これらが適切な型で統一される必要がある。

## 期待される効果

1. **LLVM安定性向上**: keep+call mergeパターンの回帰を防止
2. **PHI生成品質**: 異なるソースタイプのmerge処理の検証
3. **パリティ保証**: VM/LLVM両方で同じ動作を保証

## 関連Phase

- **Phase 115**: if-only call result merge (両側がcall)
- **Phase 114**: if-only PHI minimal (基本的なPHI)
- **Phase 33**: Box Theory Modularization (JoinIR architecture)

## Lessons Learned

### Box-First原則の適用

このPhaseでは、既存の箱化されたコンポーネントを活用:

- ✅ `output_validator.sh` による出力検証の統一
- ✅ `llvm_exe_runner.sh` によるLLVM実行の標準化
- ✅ テストインフラの再利用（no reinvention）

### Fail-Fast原則

- VM/LLVM両方でエラーを即座に検出
- `HAKO_JOINIR_STRICT=1` で厳密な検証を有効化
- フォールバック処理なし（エラーは明示的に失敗）

## Future Work

- **Phase 117+**: より複雑なmergeパターン（両側keep、ネストしたcall等）
- **最適化**: 元値保持の場合のPHI削減の可能性検討
