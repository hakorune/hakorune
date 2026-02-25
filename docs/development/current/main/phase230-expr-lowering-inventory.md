# Phase 230: Expr Lowering Inventory

このメモは、既存の式 lowering がどこに散らばっているかを棚卸しするためのインベントリだよ。  
Phase 230 では「コードをいじらずに把握だけする」のが目的。

---

## 1. condition_to_joinir.rs / condition_lowerer.rs

- 役割:
  - ループの条件式（header 条件 / break 条件）を AST → JoinIR の Compare/BinOp/UnaryOp 列に落とす高レベル入口。
  - `ConditionEnv` を使って「変数名 → JoinIR ValueId」の解決を行う。
- 主な機能:
  - `lower_condition_to_joinir(ast, alloc_value, &ConditionEnv)`:
    - 二項比較（`var < literal`, `var == var`）を JoinIR Compare に lowering。
    - `ConditionPatternBox` による正規化後の単純条件を対象。
  - `lower_value_expression(ast, alloc_value, &ConditionEnv, &mut instructions)`:
    - 条件式の内部で出てくるサブ式（`i+1`, `s.length()`, `digits.indexOf(ch)` など）を JoinIR 値に潰す。
    - `MethodCallLowerer` を経由してメソッド呼び出しを BoxCall に変換。
- 制約:
  - JoinIR 専用（MirBuilder には触らない）。
  - 変数解決は ConditionEnv に限定（LoopBodyLocalEnv や UpdateEnv には直接アクセスしない）。
  - support 対象外の AST ノードは Fail-Fast（`Result::Err`）で返す。

---

## 2. bool_expr_lowerer.rs

- 役割:
  - 「MIR 向け」の boolean 式 lowering（AST → MirBuilder / SSA）を行う旧来の箱。
  - OR チェーンや `&&`/`||`/`!` を MIR の Compare / BinOp / UnaryOp に展開する。
- 主な機能:
  - `BoolExprLowerer::lower_condition(&ASTNode) -> Result<ValueId, String>`:
    - BinaryOp（比較演算子 + `&&`/`||`）を再帰的に潰し、MirInstruction を emit。
    - 変数・リテラル・メソッド呼び出しなどは MirBuilder の `build_expression` に委譲。
- 制約:
  - MirBuilder 前提の API で、JoinIR condition_to_joinir とは別ライン。
  - 現時点では「ほぼ未使用（テストもコメントアウト）」扱いの歴史的モジュール。
  - condition_to_joinir 側と直接の接点はなく、将来 ExprLowerer に統合する際の候補。

---

## 3. loop_body_local_init.rs（LoopBodyLocalInitLowerer）

- 役割:
  - ループ本体の `local` 宣言の初期化式を AST → JoinIR に落として `LoopBodyLocalEnv` に格納する。
  - 「body-local 変数の定義側（init）」専用の lowering。
- 主な機能:
  - `lower_inits_for_loop(body_ast, &mut LoopBodyLocalEnv)`:
    - ループ本体 AST から `ASTNode::Local` をスキャンし、各変数の init 式を順番に処理。
  - `lower_init_expr(expr, &LoopBodyLocalEnv) -> Result<ValueId, String>`:
    - リテラル（整数/文字列）→ Const
    - 変数参照 → ConditionEnv 経由で解決
    - 二項演算（`+ - * /`）→ BinOp
    - MethodCall（`s.substring`, `digits.indexOf`）→ `emit_method_call_init` 経由で MethodCallLowerer に委譲
- 制約:
  - 変数解決は ConditionEnv + 既存の LoopBodyLocalEnv（cascading）のみ。
  - サポート外のリテラル種別・演算子・複雑な式は Fail-Fast。
  - lowering 対象は「init 式」だけで、更新式（UpdateExpr）は別の箱（CarrierUpdateEmitter）が担当。

---

## 4. method_call_lowerer.rs（MethodCallLowerer）

- 役割:
  - MethodCall AST ノードを CoreMethodId メタデータに基づいて JoinIR BoxCall に lowering する箱。
  - 同じメソッド呼び出しでも「条件文から使うか」「init から使うか」でホワイトリストを分けている。
- 主な機能:
  - `lower_for_condition(recv_val, method_name, args, alloc, &ConditionEnv, &mut instructions)`:
    - `allowed_in_condition()` に通る CoreMethodId だけ許可（例: `length`, 一部の `indexOf`）。
    - 引数は `condition_lowerer::lower_value_expression` で JoinIR 値に lowering。
  - `lower_for_init(recv_val, method_name, args, alloc, &ConditionEnv, &LoopBodyLocalEnv, &mut instructions)`:
    - `allowed_in_init()` に通るメソッド（`substring`, `indexOf` など）を body-local init 用に lowering。
    - 引数の変数は LoopBodyLocalEnv → ConditionEnv の優先順で解決（cascading local をサポート）。
  - `lower_arg_with_cascading`:
    - 引数用の小さなヘルパー。変数なら LoopBodyLocalEnv/ConditionEnv を見て、それ以外は condition_lowerer に委譲。
- 制約:
  - CoreMethodId メタデータが前提（メソッド名や Box 名のハードコード禁止）。
  - 文脈（condition/init）ごとに別 API で呼び分ける必要がある。
  - 型情報は暗黙的（CoreMethodId 側に埋め込まれており、TypeContext のような統一ビューはまだない）。

---

## 5. carrier_update_emitter（CarrierUpdateEmitter）

- 役割:
  - LoopUpdateAnalyzer で分類された UpdateExpr（`sum = sum + digit` など）を JoinIR 命令列に変換する。
  - 「キャリア更新の右辺式」を安全なパターンだけ受理して lowering するホワイトリスト箱。
- 主な機能:
  - `emit_carrier_update_with_env(carrier, &UpdateExpr, alloc, &UpdateEnv, &mut instructions)`:
    - `UpdateRhs::Const` → Const + BinOp(Add/Mul)。
    - `UpdateRhs::Variable` → UpdateEnv.resolve(name) 経由で条件変数・body-local を横断解決。
    - `UpdateRhs::StringLiteral` → Const(String)。
    - `UpdateRhs::NumberAccumulation` → base/digit 組み合わせを Mul + Add の2段構成で emit。
  - `emit_carrier_update`（レガシー）:
    - ConditionEnv ベースの旧 API（body-local 非対応）を後方互換のために残している。
- 制約:
  - lowering 対象は「UpdateExpr に正規化済みの式」に限る（生 AST は扱わない）。
  - `UpdateRhs::Other` や method call を含む複雑な更新は can_lower() 側で reject される前提。
  - 型は整数/String の一部パターンに限定（TypeContext への統合は未実施）。

---

## 6. 小まとめ（Phase 230 時点の散らばり方）

- 「条件式」と「init/body-local」と「UpdateExpr」が、それぞれ別の箱で AST / 中間表現を潰している：
  - 条件式: `condition_to_joinir` + `condition_lowerer`（+ 一部 BoolExprLowerer/MIR 側）  
  - body-local init: `LoopBodyLocalInitLowerer` + `MethodCallLowerer(lower_for_init)`  
  - carrier 更新: `CarrierUpdateEmitter` + `UpdateEnv`（UpdateExpr ベース）
- 変数解決も 3 系統に分かれている：
  - `ConditionEnv`（loop param / captured / condition-only）
  - `LoopBodyLocalEnv`（body 内 `local`）
  - `UpdateEnv`（ConditionEnv + LoopBodyLocalEnv の合成ビュー）
- 将来の ExprLowerer/ScopeManager では、これらを
  - 「式 lowering の SSOT」として ExprLowerer
  - 「名前解決の SSOT」として ScopeManager
  に段階統合していくのがターゲット、という整理になっているよ。
Status: Active  
Scope: Expr Lowering 在庫（JoinIR/ExprLowerer ライン）
