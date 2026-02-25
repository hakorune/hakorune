# selfhost/shared/mir — JSON emit / LoopForm helpers

責務
- MIR(JSON v0) の構築・出力（Gate C 互換）。
- ループ構造（LoopForm: Header/Latch/Exit）の最小組み立て。

方針（型依存ロジックの統一）
- Array の長さ: `ArrayBox.size/1` → `StringHelpers.to_i64` で整数化。
- Map の取得: `MapBox.get/2`（`type`/`value` 等）でアクセス。
- 整数化: `StringHelpers.to_i64` を使用。文字列ヒューリスティックは禁止。
- JSON 文字列化: `StringHelpers.json_quote` を使用（安全なエスケープ）。

禁止（Fail-Fast 防止のため）
- `.getField` 相当のフィールド直参照の混入。
- 文字列化結果からの数値抽出（`"MapBox("` 判定など）。

関連
- `json_emit_box.hako`: Gate C JSON 出力（numbers unwrapped）。
- `mir_schema_box.hako`: MIR(JSON) 構築ヘルパー（v0 スキーマ）。
- `loop_form_box.hako`: LoopForm 構造の最小組み立て。

