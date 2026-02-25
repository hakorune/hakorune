# Phase 237-EX: ExprLowerer 条件パターンカタログ（JsonParser / selfhost）

目的: JsonParser/selfhost に存在するループ条件・break/continue 条件を棚卸しし、ExprLowerer/ScopeManager で必ず扱いたいパターンと後回しにしてよいパターンを整理する。コード変更は行わず、設計 SSOT としてのカタログを作るフェーズだよ。

---

## 1. 対象範囲

- **JsonParser**: `tools/hako_shared/json_parser.hako` の主要ループ（_parse_number / _parse_string / _parse_array / _parse_object / _skip_whitespace / _trim / _match_literal / _atoi）。
- **selfhost 代表ループ**: `apps/tests/phase190_atoi_impl.hako`, `apps/tests/phase190_parse_number_impl.hako`（Stage-3/コンパイラ本体に類似する最小パターン）。
- 既存設計メモ: `phase230-expr-lowerer-design.md`, `phase230-expr-lowering-inventory.md`, `phase236-exprlowerer-integration.md`, `joinir-architecture-overview.md`。

---

## 2. 条件パターン一覧（サマリ）

| ID | Source | Function/Loop | Pattern (P1–P5) | Position | AST Pattern | Uses | ExprLowerer Support (P236時点) | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| JP-01 | json_parser.hako | _parse_number | P2 | header | `p < s.length()` | LoopParam + MethodCall(length) | PARTIAL | MethodCall(length)はBoolExprLowererで許容済み。ExprLowererではMethodCall未対応のため NO に近い扱い。 |
| JP-02 | json_parser.hako | _parse_number | P2 | break | `digit_pos < 0` | LoopBodyLocal(digit_pos) + OuterLocal(digits) | YES | Phase 223–236 で昇格済み (ConditionOnly carrier経由)。 |
| JP-03 | json_parser.hako | _parse_string | P2-ish (early return) | break/return | `p < s.length()` / `ch == "\"" -> return` | LoopParam + MethodCall(substring/length) | PARTIAL | returnで抜ける構造。ExprLowererはMethodCall未対応なので当面 ConditionEnv 経由。 |
| JP-04 | json_parser.hako | _parse_array | P4 | header | `p < s.length()` | LoopParam + MethodCall(length) | PARTIAL | continue/returnを含む。MethodCall未対応。 |
| JP-05 | json_parser.hako | _parse_array | P4 | break/return | `ch == "]"` / `ch == ","` | LoopBodyLocal(ch) + MethodCall(substring) | PARTIAL | substring + equality。ExprLowererにMethodCall対応が必要。 |
| JP-06 | json_parser.hako | _parse_object | P4 | header | `p < s.length()` | LoopParam + MethodCall(length) | PARTIAL | 上と同様。 |
| JP-07 | json_parser.hako | _skip_whitespace | P4 | continue | `p < s.length()` + whitespace 判定 | LoopParam + MethodCall(substring) | PARTIAL | 文字比較のみ。MethodCall(substring)が鍵。 |
| JP-08 | json_parser.hako | _trim (leading) | P4 | continue | `start < end` + whitespace 判定 | LoopBodyLocal(start/end) + MethodCall(substring) | PARTIAL | 先頭/末尾トリム。 |
| JP-09 | json_parser.hako | _match_literal | P2 | header | `i < len` | LoopParam + OuterLocal(len) | YES | 純スカラ比較のみ。 |
| JP-10 | json_parser.hako | _atoi | P2 | header | `i < n` | LoopParam + OuterLocal(n) | YES | 純スカラ比較。 |
| SH-01 | phase190_atoi_impl.hako | main loop | P2 | header | `i < 10` | LoopParam | YES | ExprLowerer 条件対応済み。 |
| SH-02 | phase190_atoi_impl.hako | break | `i >= 3` | LoopParam | YES | 比較のみ。 |
| SH-03 | phase190_parse_number_impl.hako | header | `i < 10` | LoopParam | YES | 比較のみ。 |
| SH-04 | phase190_parse_number_impl.hako | break | `i > 3` | LoopParam | YES | 比較のみ。 |

※ Support 列の解釈: YES = Phase 236 までの ExprLowerer/condition_to_joinir で扱えるか実績あり、PARTIAL = BoolExprLowerer/ConditionEnv では対応しているが ExprLowerer で MethodCall 等が未実装、NO = まだ扱っていない。

---

## 3. パターン詳細（JsonParser）

- **_parse_number**
  - header: `loop(p < s.length())` — MethodCall(length) を含む。現在は ConditionEnv + condition_to_joinir で処理、ExprLowererは MethodCall 未対応なので PARTIAL。
  - break: `if digit_pos < 0 { break }` — digit_pos は body-local → ConditionOnly carrier として昇格済み。ExprLowerer 対応可。
  - continue path: digit が見つかったら `p = p + 1` で次イテレーション。
- **_parse_string**
  - header: `loop(p < s.length())` — MethodCall(length)。
  - break/return: `if ch == "\"" { return ... }` / escape 判定など。MethodCall(substring) + equality の組み合わせ。
- **_parse_array**
  - header: `loop(p < s.length())` — MethodCall(length)。
  - 内部: `if ch == "]" { return ... }`, `if ch == "," { ...; continue }` — substring + equality。
- **_parse_object**
  - header: `loop(p < s.length())` — MethodCall(length)。
  - 内部: `if ch == "}" { return ... }` / `if ch == "," { continue }` — substring + equality。
- **_skip_whitespace**
  - header: `loop(p < s.length())`
  - continue: whitespace 判定 (`" " || "\t" || "\n" || "\r"`) → continue、そうでなければ break 相当。
- **_trim**
  - leading trim: `loop(start < end)` + whitespace 判定 → continue/break。
  - trailing trim: `loop(end > start)` + whitespace 判定 → continue/break。
- **_match_literal**
  - header: `loop(i < len)`、body: 文字比較 + break。
  - ExprLowerer で既に扱える純スカラ比較。
- **_atoi**
  - header: `loop(i < n)`
  - break: 非 digit を見つけたら break。
  - 文字比較 + indexOf(digit) などが絡むが、条件自体は比較のみ。

---

## 4. パターン詳細（selfhost 代表）

- **phase190_atoi_impl.hako**
  - header: `loop(i < 10)`、break: `if i >= 3 { break }`
  - 純スカラ比較のみ。ExprLowerer 本番経路で扱える。
- **phase190_parse_number_impl.hako**
  - header: `loop(i < 10)`、break: `if i > 3 { break }`
  - 純スカラ比較のみ。ExprLowerer 本番経路で扱える。
- （selfhost コンパイラ本体の Stage-3 ループは JsonParser と同型/サブセットであることが多く、上記 JP パターンに準拠する想定。必要に応じて SH-xx を追加する。）

---

## 5. ExprLowerer/ScopeManager へのマッピング

- **YES（Phase 236 時点で処理可能）**
  - スカラ比較のみの header/break: `i < 10`, `i > 3`, `digit_pos < 0`（昇格済み ConditionOnly carrier）。
  - `_match_literal`, `_atoi` のループ条件・break 条件のような純スカラ比較。
- **PARTIAL**
  - MethodCall を含む header 条件: `p < s.length()`（length 呼び出し）、`ch == s.substring(...)` など。
  - substring/indexOf を含む break/continue 条件: `_parse_array/_object/_string` 内の `ch == "]"` / `ch == ","` 等。
  - → ExprLowerer で MethodCall (length/substring/indexOf) を許可する拡張が必要。
- **NO（未対応）**
  - return ベースでループを抜けるパターン（_parse_string のように return で終了するケース）は、ExprLowerer 以前に LoopPattern 側での扱いを要検討。
  - 文字列操作が複合的に絡む条件（escape 処理など）も当面は ConditionEnv + legacy lowerer 維持を推奨。

---

## 6. 次フェーズ候補（実装案のメモ）

- 候補 A: Pattern2 header 条件の MethodCall(length) を ExprLowerer で扱う（JP-01/03/04/06 の header を統一）。
- 候補 B: substring + equality のシンプル条件を ExprLowerer に許可し、_parse_array/_object の `ch == "]"` / `ch == ","` を扱う。
- 候補 C: selfhost の P2 ループ（header/break）を ExprLowerer 本番経路に順次寄せて、実戦ループのカバレッジを測定する。
- 候補 D: return ベースの終了条件を LoopPattern 側でどこまで扱うか設計し、ExprLowerer への影響範囲を決める。

---

## 7. 完了の定義（Phase 237-EX）

- JsonParser 11 ループの主要条件（header/break/continue/if-guard）が JP-xx 行としてカタログに掲載されている。
- selfhost 代表ループ 3〜5 例（SH-xx）が掲載されている。
- 各行に ExprLowerer Support (YES/PARTIAL/NO) と簡単な Notes が入っており、今後どの箱を触ればよいか判断できる。***
Status: Active  
Scope: ExprLowerer 条件カタログ（JoinIR/ExprLowerer ライン）
