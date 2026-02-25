Status: Active → Close-out  
Scope: JsonParser `_parse_number` のループを Pattern2（break 付き）で JoinIR 経路に載せるための設計決定メモ。

# Phase 245-EX: JsonParser `_parse_number` の JoinIR 統合（1 本目）

## 1. 目的 / スコープ
- 目的: `_parse_number` のループを、既存の Pattern2 + ExprLowerer/ConditionEnv/CarrierUpdateEmitter で JoinIR 経路に載せる。
- スコープ: このフェーズでは `_parse_number` のみ。`_atoi` / `_atof_loop` / `_parse_array` など他ループは後続フェーズ。
- 明示: **扱うのは `p` の header / break / 更新のみ。`num_str` には手を出さない**（文字列連結キャリアは Phase 245B 以降で検討）。

## 2. 現状の挙動と既存フェーズの整理
- ループ概要（tools/hako_shared/json_parser.hako より）:
  - ループ変数: `p`（文字走査位置）
  - ヘッダ条件: `p < s.length()`
  - body 計算: `ch = s[p]`; `digit_pos = digits.indexOf(ch)`
  - break 条件: `digit_pos < 0`（非 digit で脱出）
  - 更新: `p = p + 1`; 数値文字列の累積（`num_str = num_str + ch` 相当）あり
- 参考フェーズ:
  - ループ全体の設計/在庫: `phase181-jsonparser-loop-roadmap.md`, `phase174-jsonparser-loop-inventory-2.md`
  - digit_pos / ConditionEnv 系: `phase200-A/B/C`, `phase224-digitpos-condition-normalizer.md`, `phase224-digitpos-promoter-design.md`
  - ExprLowerer/ScopeManager: `phase230-expr-lowerer-design.md`, `phase236-exprlowerer-integration.md`, `phase237-exprlowerer-condition-catalog.md`, `phase238-exprlowerer-scope-boundaries.md`
- 既にカバーされている要素:
  - header 条件 `p < len` は ExprLowerer/ConditionEnv で扱える想定（Phase 230/236 系）
  - break 条件 `digit_pos < 0` は digitpos 正規化経路で扱う前提（Phase 224 系）
  - キャリア更新 `p = p + 1` は Pattern2/CarrierUpdateEmitter で許容
- まだ本番経路に載っていない部分:
  - 数値文字列の累積（`num_str`）の扱いを今回どうするか（キャリアに入れるか、今回は p 更新のみに絞るか）を決める必要あり。

## 3. ターゲット JoinIR パターン / 箱構成
- パターン: **Pattern2 (Break)** を基本とし、必要なら LoopBodyLocal 昇格（P5 相当の body-local 扱い）を併用。
- ループ変数・キャリア・body-local・captured の対応表:
  - loop var: `p`
  - carriers: `p` は必須。`num_str` は今回の Phase では **任意**（下記の許可範囲で決める）。
  - condition inputs: `p`, `s.length()`, `digit_pos`
  - break 条件: `digit_pos < 0`
  - body-local/captured: `s`, `digits` は captured 扱いで読み取りのみ。
- 経由させる箱:
  - ConditionEnv + ExprLowerer（header 条件 / break 条件）
  - MethodCallLowerer（`digits.indexOf(ch)`）
  - CarrierUpdateEmitter（`p = p + 1`、必要なら `num_str` 更新）

## 4. 条件式・更新式パターンの許可範囲
- ヘッダ条件: `p < s.length()` は ExprLowerer/ConditionEnv の既存カバー範囲で扱う（YES 前提）。
- break 条件: `digit_pos < 0` を digitpos 正規化経路（Phase 224 系）に乗せる。Compare/Jump で Pattern2 に合流すること。
- 更新式:
  - 必須: `p = p + 1` を CarrierUpdateEmitter で扱う。
  - 任意: `num_str = num_str + ch`  
    - もし ExprLowerer/CarrierUpdate が文字列連結キャリアを安全に扱えるなら、キャリアとして含める。  
    - 難しければ本フェーズは `p` の更新と break 条件の JoinIR 化に限定し、`num_str` は後続フェーズで扱うと明示。
- 線引き:
  - **今回扱う**: header 条件、break 条件、`p` 更新。`num_str` 更新は「可能なら扱う、無理なら後続」と書き分ける。原則として **Phase 245-EX では `num_str` をキャリアに載せない**。
  - **後続に回す**: `_parse_array` / `_parse_object` / `_unescape_string` / if-sum/continue を含む Pattern3/4 の適用。

## 5. 期待する検証方法（テスト観点）
- 既存テストで固定したいもの:
  - JsonParser の数値解析系スモーク（ファイル名/ケース名があれば列挙）。  
  - 例: `"123"` → 数値として成功 / `"123a"` → 非 digit で break して期待どおりのパース失敗/戻り値になること。
- 必要なら追加する最小ケース（例）:
  - 入力: `"42"` → 正常に数値化（num_str が "42"）し、p が len に一致。
  - 入力: `"7z"` → `z` で break、num_str が "7" で止まり、エラー/戻り値が従来と一致。
- JoinIR レベル確認ポイント:
  - header 条件が Compare + Jump で Pattern2 のヘッダに乗っていること。
  - break 条件 `digit_pos < 0` が ConditionEnv/ExprLowerer 経由で JoinIR の break ブロックに接続していること。
  - `p` の更新が CarrierUpdateEmitter で扱われ、LoopHeader PHI / ExitLine と矛盾しないこと。

## 6. 非目標 / 今回はやらないこと
- `_parse_array` / `_parse_object` / `_unescape_string` など他ループへの展開は本フェーズ外。
- continue/if-sum を含む Pattern3/4 への適用は別フェーズ。
- JsonParser 全体の設計変更や API 変更は行わない。ループ部分の JoinIR 経路追加/切り替えに限定。

## 7. コード側 Phase 245-EX への引き継ぎメモ
- 対象ループ: `_parse_number`
- パターン: Pattern2 (Break) + 必要に応じて body-local 昇格（P5 相当）
- 変数の役割:
  - loop var: `p`
  - carriers: `p`（必須）、`num_str`（可能なら含める/後続に回すかをここで決める）
  - condition inputs: `p`, `s.length()`, `digit_pos`
  - break 条件: `digit_pos < 0`
  - captured: `s`, `digits`
- 許可された式:
  - header: `p < s.length()`
  - break: `digit_pos < 0`
  - 更新: `p = p + 1`（必須）、`num_str = num_str + ch`（扱うかどうかを本メモで明記）
- 検証:
  - 使うテストケース（既存/追加）と期待する挙動（RC/ログ）を本メモに列挙しておく。

## 8. 完了メモ（Phase 245-EX 締め）
- `_parse_number` の p ヘッダ条件（`p < s.length()`）・break 条件（`digit_pos < 0`）・更新（`p = p + 1`）を Pattern2 + ExprLowerer/CarrierUpdateEmitter 経路に載せた。
- 既存の挙動確認: `cargo test --release phase245_json_parse_number -- --nocapture` を実行し、RC/ログともに従来からの差分なし（num_str 未導入のため外部挙動不変）。
- 次フェーズ（245B）で扱うもの: `num_str` をキャリアに載せるかどうか、更新式の許容範囲、固定すべきテストを設計する。
