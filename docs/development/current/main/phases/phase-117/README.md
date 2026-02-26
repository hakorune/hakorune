# Phase 117: if-only nested-if + call merge parity

**Status**: ✅ DONE (2025-12-18)

## 背景

Phase 116で「keep + call merge」を固定した後、**ネストされたif内部でのcall merge**が正しく動作するか検証が必要。

特に以下のパターンを検証:
- 外側if (a == 1) の中に内側if (b == 1)
- 内側if の両分岐で異なる引数でf()を呼び出し
- 外側else でも別引数でf()を呼び出し
- すべてのcall結果をvに集約してprint

これはJoinIR Pattern3（if-only）+ call mergeの**ネスト版**であり、VM/LLVM EXE両方で同じ動作をすることが品質保証の要件。

## 実装

### Fixture
- **ファイル**: `apps/tests/phase117_if_only_nested_if_call_merge_min.hako`
- **構造**: static box Main パターン
  - `f(x)`: 単純な関数（x + 1を返す）
  - `g(a, b)`: ネストif + call merge
    - a == 1 かつ b == 1 → f(1) → 2
    - a == 1 かつ b != 1 → f(2) → 3
    - a != 1 → f(3) → 4
  - `main()`: 3パターンを順次実行
- **期待出力**: `2\n3\n4`

### VM Smoke Test
- **ファイル**: `tools/smokes/v2/profiles/integration/apps/archive/phase117_if_only_nested_if_call_merge_vm.sh`
- **実行条件**: `NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1`
- **検証**: `validate_numeric_output 3 "2\n3\n4"`

### LLVM EXE Smoke Test
- **ファイル**: `tools/smokes/v2/profiles/integration/apps/archive/phase117_if_only_nested_if_call_merge_llvm_exe.sh`
- **Required plugins**: FileBox, MapBox, StringBox, ConsoleBox, IntegerBox (Phase 115/116と同じ)
- **検証**: numeric output `2\n3\n4` (3 lines)

## 検証コマンド

```bash
# VM smoke test
bash tools/smokes/v2/profiles/integration/apps/archive/phase117_if_only_nested_if_call_merge_vm.sh

# LLVM EXE smoke test
bash tools/smokes/v2/profiles/integration/apps/archive/phase117_if_only_nested_if_call_merge_llvm_exe.sh

# 回帰確認（Phase 116）
bash tools/smokes/v2/profiles/integration/apps/archive/phase116_if_only_keep_plus_call_llvm_exe.sh
```

## 技術的詳細

### JoinIR Pattern3（if-only）のネスト
- **外側if**: a == 1 の分岐
- **内側if**: b == 1 の分岐（外側then内部にネスト）
- **Call merge**: 3箇所のf()呼び出しが最終的に1つのvに集約

### MIR生成の観点
- ネストしたif-onlyが正しくPattern3として認識される
- 各分岐でのcall命令が適切にマージされる
- PHI nodeが正しく生成される（ネスト構造を反映）

### LLVM EXE固有の検証ポイント
- プラグイン（StringBox, IntegerBox, ConsoleBox）の正しいリンク
- ネストしたcall mergeのLLVM IR生成
- 実行時のメモリ安全性（segfault無し）

## 成果

✅ VM/LLVM EXE両方でparity達成
✅ ネストif + call mergeパターンの品質固定
✅ Phase 115/116の退行なし

## Next Steps

Phase 118以降でさらに複雑なネストパターン（if-else nested, loop内if等）を段階的に固定していく。
