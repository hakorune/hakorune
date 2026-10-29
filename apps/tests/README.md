# Nyash Test Programs

このディレクトリには、CI/CDやデバッグ用のテストプログラムが含まれています。
実用的なアプリケーションは親ディレクトリ（`apps/`）にあります。

## テストプログラム一覧

### LLVMバックエンドテスト
- **ny-llvm-smoke/** - ArrayBox基本操作テスト
- **ny-array-llvm-ret/** - ArrayBox戻り値テスト
- **ny-echo-lite/** - 最小echo実装（I/Oテスト）
- **ny-map-llvm-smoke/** - MapBoxプラグインテスト
- **ny-vinvoke-smoke/** - 可変長引数（5引数）テスト
- **ny-vinvoke-llvm-ret/** - 可変長引数戻り値テスト
- **ny-vinvoke-llvm-ret-size/** - 固定長引数（size()）テスト

### archive/
- 退役した compare/proof wrapper は `archive/` に置く
- active gate 用のテストはここに追加しない

## 実行方法

旧LLVM手動smokeは退役済みです。現行EXE受け入れは次を使用します。

```bash
tools/smokes/v2/run.sh --profile integration --owner-profile integration --suite real-apps-exe-boundary
```

## 注意事項

- これらは最小限の機能テストであり、実用的なアプリケーションではありません
- CIでの自動テストを前提に設計されています
- エラー時の切り分けが容易になるよう、各テストは単一の機能に焦点を当てています
- phase29aq_*_diag_min.hako は調査用の最小再現（diagnostics, dev-only）。通常の gate/CI 対象外で、必要時に手動実行する
