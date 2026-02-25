# Hako Check — Rule Tests (MVP)

構成（1 ルール = 1 ディレクトリ）
- tools/hako_check/tests/<rule_name>/
  - ok.hako    … 検出なし
  - ng.hako    … 最低 1 件の検出
  - edge.hako  … 端境（任意）
  - expected.json … `--format json-lsp` の期待ダイアグノスティクス

実行（MVP）
- `bash tools/hako_check/run_tests.sh` で全テストを走査
- 差分があれば終了コード 1、詳細を提示

注意
- 21.4 は AST JSON 優先。Text fallback の差異は expected に反映
- ルール名は HCxxx を推奨（例: HC002）
