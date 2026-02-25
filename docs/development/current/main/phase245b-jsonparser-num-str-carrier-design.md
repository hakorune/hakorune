Status: Implemented (Phase 245B-IMPL)  
Scope: `_parse_number` で `num_str` を Pattern2/P5 のキャリアとして扱うかどうかを決める設計フェーズ（実装は Structured→Normalized dev フィクスチャで完了）。
Notes: jsonparser_parse_number_real フィクスチャで `num_str = num_str + ch` を LoopState キャリアとして実装し、dev Normalized 比較テストで固定済み。

# Phase 245B: JsonParser `_parse_number` の `num_str` キャリア設計

## 1. 目的
- `_parse_number` で数値文字列を蓄積する `num_str` を、JoinIR Pattern2/P5 のキャリアとして扱うかを決める。
- UpdateExpr の許容範囲（例: `num_str = num_str + ch`）と、どのテストで意味論を固定するかを先に書き下す。

## 2. 論点
- キャリア化するか:
  - Option A: `num_str` をキャリアとして Pattern2 に含める（LoopHeader PHI/ExitLine まで通す）。
  - Option B: 今フェーズは `p` のみ、`num_str` は後続（言語仕様/意味論決定後）に回す。
- 許可する UpdateExpr:
  - 文字連結パターン（`num_str = num_str + ch`）のみを許容するか。
  - それ以外の文字列操作（substring/indexOf 等）は当面禁止するか。
- 依存する箱:
  - CarrierUpdateEmitter が文字列連結を安全に扱えるか（型/ValueId の整合）。
  - ExprLowerer/MethodCallLowerer で文字列メソッドが必要か。

## 3. テストで固定したいこと（候補）
- 正常系: `"42"` → `num_str == "42"`, `p == len`, RC/ログ従来通り。
- 非digit混在: `"7z"` → break で `num_str == "7"`, RC/ログ従来通り。
- 既存の JsonParser スモークがあればそれを JoinIR 経路で回して差分が出ないことを確認。

## 4. 進め方（小タスク案）
1) UpdateExpr の whitelist を決める（文字連結のみ/その他禁止）。
2) CarrierInfo に `num_str` を入れるかどうかを設計メモに明記。
3) どのテストで意味論を固定するかを列挙（既存/新規）。
4) これらを決めてからコード側 Phase 245B（小変更）に着手する。

## 5. メモ
- Phase 245-EX では `p` のみ JoinIR Pattern2 に載せた。`num_str` の扱いは未決。
- 文字列キャリアは ValueId/ExitLine との整合が崩れやすいので、Fail-Fast 原則を崩さずに小さく導入すること。
