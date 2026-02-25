# Phase 238-EX: ExprLowerer/ScopeManager Scope Boundaries

目的: ExprLowerer / ScopeManager / ConditionEnv / LoopBodyLocalEnv / UpdateEnv の責務と参照範囲を文章で固定し、「誰がどこまで見てよいか」を SSOT 化する。コード変更は行わず、ガイドラインと境界ルールを整備するフェーズだよ。

---

## 1. 対象コンポーネント

- ExprLowerer（条件式 lowering 箱）
- ScopeManager（名前解決の窓口）
- ConditionEnv（条件式用の名前→ValueId マップ）
- LoopBodyLocalEnv（body-local init 専用の環境）
- UpdateEnv（carrier 更新専用の環境）

---

## 2. 境界ルール（原則）

- **名前解決は ScopeManager 経由のみ**: ExprLowerer は ConditionEnv / LoopBodyLocalEnv / CapturedEnv / CarrierInfo に直接触らない。ScopeManager の lookup/scope_of を唯一の窓口とする。
- **条件式から UpdateEnv へは触らない**: header/break/continue 条件は carrier 更新専用の UpdateEnv にアクセスしない。更新式は UpdateEnv / LoopBodyLocalInitLowerer に限定。
- **ConditionEnv は「条件で参照する値のみ」**: loop var / carriers / ConditionOnly / captured const など、条件式で参照する JoinIR ValueId のみを持ち、body-local を直接含めない（昇格する場合は ScopeManager+CarrierInfo 経由）。
- **LoopBodyLocalEnv は init 専用**: body 内 local 定義の init 式だけを扱い、条件式の参照は ScopeManager が仲介する（条件側から直接参照しない）。
- **Fail-Fast / by-name 禁止**: ExprLowerer/ScopeManager は「名前ヒューリスティック」や silent fallback を行わず、Unsupported/NotFound は明示エラー（上位で Fail-Fast ポリシーに従う）。

---

## 3. JsonParser/selfhost 条件パターンへの対応（237 カタログとの橋渡し）

- **優先対応（YES/PARTIAL で拾いたいもの）**
  - P2/P4 header の単純比較 (`i < n`, `p < s.length()`): LoopParam + OuterLocal/Captured/MethodCall(length)。ScopeManager で loop var / captured const を解決し、MethodCall 対応は将来 ExprLowerer 拡張で吸収。
  - P2 break 条件 `digit_pos < 0`: body-local 昇格済み (ConditionOnly carrier)。ScopeManager が `digit_pos`→`is_digit_pos` を解決。
  - substring + equality の単純分岐（`ch == "]"` など）: LoopBodyLocal + MethodCall(substring)。MethodCallLowerer 経由で ExprLowerer へ委譲する拡張が必要。
- **当面後回し**
  - return ベースの終了（_parse_string のように return で抜ける）: LoopPattern 側の設計が先。ExprLowerer は条件式評価に限定。
  - 複合的な文字列操作（escape 処理など）: legacy ConditionEnv +専用 lowerer 維持。

ScopeManager が解決する名前の例:
- LoopParam: `i`, `p`, `start`, `end`
- Captured/OuterLocal: `len`, `n`, `digits`
- Promoted LoopBodyLocal: `digit_pos` → `is_digit_pos`
- BodyLocal（条件で参照する場合）: `ch`, `temp` など（LoopBodyLocalEnv → ScopeManager 経由）

---

## 4. LAYER_GUARD / ガード実装案（構造メモ）

- 将来的に `src/mir/join_ir/lowering/LAYER_GUARD.rs` 相当で、以下を静的に検知する案を検討:
  - ExprLowerer が ConditionEnv / LoopBodyLocalEnv を直接 import していないか。
  - ScopeManager が UpdateEnv を参照していないか。
  - condition_to_joinir から ScopeManager をバイパスするパスが残っていないか。
- 既存の AGENTS 原則（by-name ハードコード禁止 / Fail-Fast / 構造優先）を lint 的に守る仕組みの叩き台にする。

---

## 5. 次フェーズ候補（実装 TODO のメモ）

- ExprLowerer に MethodCall(length/substring/indexOf) の条件用 lowering を追加（カタログ JP-01/03/04/06/08/07 対応）。
- ScopeManager のガード強化（条件式から UpdateEnv へのアクセス禁止を型/モジュール境界で表現）。
- ConditionEnv 構築を ScopeManager 中心に巻き直す（ConditionEnvBuilder v2 を ScopeManager-front に寄せる）。
- LAYER_GUARD 的な静的チェックの導入検討。***
Status: Active  
Scope: ExprLowerer スコープ境界（JoinIR/ExprLowerer ライン）
