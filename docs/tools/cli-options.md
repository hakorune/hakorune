# Hakorune CLI Options Quick Reference

最終更新: 2026-04-02

## 現在のバックエンド読み

- `--backend llvm`: product main。`ny-llvmc` 経由の object / EXE 出力ライン
- `--backend vm`: compat/proof keep。selfhost、recovery、tooling の explicit keep
- `--backend vm-hako`: reference / conformance
- `--compile-wasm`: experimental / monitor-only
- `--backend interpreter` / `--compile-native` / `--aot`: historical / non-primary route

注意:
- raw CLI default はまだ `vm` ですが、これは legacy ingress の都合であり product ownership を意味しません。
- user-facing main narrative は `llvm/exe` first です。default flip は別 gate (`phase-30x / 30xF`) で最後に判断します。

## 基本
- `file`: 実行するNyashファイル（位置引数）
- `--backend {vm|vm-hako|llvm|interpreter}`: 実行バックエンド選択（current raw default: `vm`, legacy ingress only）
- `--debug-fuel {N|unlimited}`: パーサーのデバッグ燃料（無限ループ対策）
- `--parser`: removed（M4 tail cleanup で削除済み）。指定時は CLI で reject される。

## MIR関連
- `--dump-mir`: MIRを出力（実行はしない / compile-only。実行経路SSOTの確認は `NYASH_VM_DUMP_MIR=1 --backend vm` の compat/proof route を優先）
- `--verify`: MIR検証を実施
- `--mir-verbose`: 詳細MIR出力（統計など）

## VM関連
- `--vm-stats`: VM命令統計を有効化（`NYASH_VM_STATS=1`）
- `--vm-stats-json`: VM統計をJSONで出力（`NYASH_VM_STATS_JSON=1`）

## GC
- `--gc {auto|rc+cycle|off}`: GCモード（既定: `auto` → rc+cycle）
  - 運用SSOTで固定されているのは `rc+cycle` と `off`（ON/OFF 意味論不変ゲート対象）
  - `rc+cycle`: 参照カウント + 循環回収（通常運用）
  - `off`: GC hook無効（expert/検証用、循環はリークし得る）
  - `minorgen` / `stw` / `rc` は非対応（指定時は fail-fast）
- 関連ENV
  - `NYASH_GC_MODE`（CLIが優先）
  - `NYASH_GC_METRICS`
  - 詳細: `docs/reference/runtime/gc.md`

## Product / Experimental Output
- `--emit-exe FILE`: `ny-llvmc` 経由で product main のネイティブ EXE を生成
- `--emit-exe-nyrt DIR`: `--emit-exe` で使う `libnyash_kernel.a` の場所を指定
- `--emit-exe-libs FLAGS`: `--emit-exe` 時の追加リンカフラグ
- `--compile-wasm`: WASMバイナリ（`.wasm`）を出力（experimental / monitor-only）
- `--compile-native` / `--aot`: historical AOT実行ファイル出力（要wasm-backend）
- `--output, -o FILE`: 出力先を指定

## ベンチマーク
- `--benchmark`: バックエンド比較ベンチを実行
- `--iterations N`: ベンチ実行回数（既定: 10）

## 使用例
```bash
# Product main: LLVM EXE を生成
nyash --backend llvm --emit-exe myapp program.hako

# Compat/proof: Rust VM で実行 + 統計をJSON出力
nyash --backend vm --vm-stats --vm-stats-json program.hako

# Reference/conformance: vm-hako
nyash --backend vm-hako program.hako

# MIRを出力
nyash --dump-mir --mir-verbose program.hako

# Experimental: WASM
nyash --compile-wasm -o program.wasm program.hako

# ベンチマーク
nyash --benchmark --iterations 100
```

詳細は `docs/reference/architecture/execution-backends.md` も参照してください。

## Retired/Removed Flags (M4 cleanup)
- `--parser ny` は mainline 入口から削除済み。
- 旧 ENV `NYASH_USE_NY_PARSER=1` は mainline では無効（direct-v0 route を起動しない）。

## 参考: `nyash --help` スナップショット
- `docs/tools/nyash-help.md`
- これは historical capture です。current source truth は `src/cli/args.rs` を優先してください。
