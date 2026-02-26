# Phase 115: If-Only Call Result Merge Parity

**Status**: ✅ DONE
**Date**: 2025-12-18

## 背景

LLVMバックエンドで壊れやすいパターン - if分岐内での関数呼び出し結果をマージするケース。

### 問題パターン

```hako
fn f(x) { return x + 1 }
fn g(flag) {
    local v = 0
    if flag == 1 { v = f(1) } else { v = f(2) }
    print(v)
}
```

このパターンでは：
1. if/else両分岐で関数呼び出し `f()` が発生
2. その結果を変数 `v` に代入
3. if後にマージされた `v` を使用

LLVM EXEモードでは、PHI node生成やSSA変換が正しく行われない可能性がある。

## 実装内容

### 1. テストフィクスチャ

- **ファイル**: `/home/tomoaki/git/hakorune-selfhost/apps/tests/phase115_if_only_call_merge_min.hako`
- **期待出力**: `2\n3`
  - `g(1)` → `f(1)` → `1+1` → `2`
  - `g(0)` → `f(2)` → `2+1` → `3`

### 2. VM Smoke Test

- **ファイル**: `/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/archive/phase115_if_only_call_merge_vm.sh`
- **環境変数**: `NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1`
- **検証**: 数値行抽出して `2\n3` と比較

### 3. LLVM EXE Smoke Test

- **ファイル**: `/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/archive/phase115_if_only_call_merge_llvm_exe.sh`
- **利用**: `tools/smokes/v2/lib/llvm_exe_runner.sh`
- **検証**: `EXPECTED='2\n3'`, `EXPECTED_LINES=2`

## 検証コマンド

```bash
# VM smoke test
bash tools/smokes/v2/profiles/integration/apps/archive/phase115_if_only_call_merge_vm.sh

# LLVM EXE smoke test
bash tools/smokes/v2/profiles/integration/apps/archive/phase115_if_only_call_merge_llvm_exe.sh

# Phase 114回帰テスト（推奨）
bash tools/smokes/v2/profiles/integration/apps/archive/phase114_if_only_return_then_post_llvm_exe.sh
```

## 関連Phase

- **Phase 103**: If-Only基本パリティ（制御フロー基礎）
- **Phase 113**: If-Only部分代入パリティ（変数マージ）
- **Phase 114**: If-Only return+post パリティ（early returnとpost-if文）
- **Phase 115**: If-Only call result merge パリティ（関数呼び出し結果マージ） ← 今回

## 技術的詳細

### JoinIR → MIR変換

If-Only Pattern 1では：
1. Then/Else分岐それぞれで関数呼び出し
2. 各分岐の終端で変数への代入
3. Exit block入口でPHI node生成（then_value vs else_value）
4. Post-if文でマージされた値を使用

### LLVM SSA変換

LLVM EXEモードでは：
1. 関数呼び出し結果を一時レジスタに保持
2. 各分岐終端でstore
3. Exit block入口でload + phi
4. Post-if文でphi結果を使用

## Fail-Fast原則

- プラグイン無効化（`NYASH_DISABLE_PLUGINS=1`）でコア機能のみテスト
- JoinIR厳格モード（`HAKO_JOINIR_STRICT=1`）で不正な制御フローを即座に検出
- 数値行のみ抽出して検証（余計なログを排除）
