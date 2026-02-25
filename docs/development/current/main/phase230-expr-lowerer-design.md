# Phase 230: ExprLowerer / ScopeManager Design

このドキュメントは、Phase 230 で検討する「ExprLowerer / ScopeManager」設計のメモだよ。  
**コード変更は行わず、将来の統合先インターフェースだけを先に固める**ことが目的。

---

## 1. ExprLowerer の役割と API スケッチ

### 1.1 役割

- ExprLowerer は「AST の式ノード → JoinIR ValueId」の SSOT として振る舞う箱。
- 条件式 / init 式 / Update 式（UpdateExpr に入る前の生 AST）など、式まわりの lowering を一本化する。
- 変数解決やスコープ情報は ScopeManager に委譲し、ExprLowerer 自身は「式構造」を見ることに専念する。

### 1.2 API スケッチ

```rust
pub struct ExprLowerer<'env> {
    /// 名前解決とスコープ情報を提供する窓口
    scope: &'env dyn ScopeManager,

    /// 型情報の参照（将来用。現時点では Option でもよい想定）
    types: Option<&'env TypeContext>,

    /// JoinIR 命令バッファ
    instructions: &'env mut Vec<JoinInst>,

    /// JoinIR ValueId アロケータ
    alloc_value: &'env mut dyn FnMut() -> ValueId,
}

impl<'env> ExprLowerer<'env> {
    /// 任意の式 AST を JoinIR ValueId に lowering する入口
    pub fn lower_expr(
        &mut self,
        ast: &ASTNode,
        ctx: ExprContext,
    ) -> Result<ValueId, ExprLoweringError> {
        // ctx: Condition / Init / Update / Misc などの文脈ヒント
        unimplemented!()
    }
}
```

```rust
/// 式の文脈（どこから呼ばれているか）を表す軽量フラグ
pub enum ExprContext {
    Condition,         // ループ条件 / if 条件
    InitBodyLocal,     // body-local init
    UpdateCarrier,     // carrier update (UpdateExpr 相当)
    Misc,              // その他（将来用）
}
```

### 1.3 呼び出し元の想定

- 条件式:
  - Pattern1–4 lowerer や `condition_to_joinir` から
  - `lower_expr(ast, ExprContext::Condition)` として利用。
- init 式（body-local）:
  - `LoopBodyLocalInitLowerer` 相当の責務を段階的に ExprLowerer に寄せる。
  - `lower_expr(init_ast, ExprContext::InitBodyLocal)` を呼んで ValueId を受け取り、LoopBodyLocalEnv に名前を紐づけるだけの薄い箱にする。
- Update 式:
  - いまは `LoopUpdateAnalyzer` → `UpdateExpr` → `CarrierUpdateEmitter` だが、
    将来は UpdateExpr 生成の一部を ExprLowerer に委譲することも視野に入れる。
  - 初期段階では `ExprContext::UpdateCarrier` だけ定義しておき、実際の統合は後続フェーズに回す。

### 1.4 既存箱からの委譲イメージ

- BoolExprLowerer:
  - 現状ほぼ未使用の MIR 向け lowering だが、「条件式の構造を落とす」という責務は ExprLowerer と重なる。
  - 将来は ExprLowerer の内部実装（もしくは Condition/MIR プロファイル）として吸収する候補。
- MethodCallLowerer:
  - CoreMethodId メタデータと BoxCall emission ロジックはそのままユーティリティとして残す。
  - ExprLowerer 側から `MethodCallLowerer::lower_*` を呼ぶ形で再利用。
- condition_to_joinir:
  - 入口/オーケストレーターとして残しつつ、中身の AST → JoinIR 値の部分を ExprLowerer に差し替える方針。

---

## 2. ScopeManager の設計

### 2.1 目的

- 変数名 → ValueId の解決と、その変数がどのスコープに属しているかを一箇所で扱う。
- ConditionEnv / LoopBodyLocalEnv / CapturedEnv / CarrierInfo(promoted_loopbodylocals など) を覆う「ビュー」を提供する。

### 2.2 インターフェース案

```rust
/// 変数スコープの種類
pub enum VarScopeKind {
    LoopParam,           // ループ変数（i, p など）
    OuterLocal,          // 関数ローカルだがループ外で定義されたもの
    CapturedConst,       // CapturedEnv に載っている実質定数
    ConditionOnly,       // 条件専用（digits, len など）
    BodyLocal,           // ループ本体の local（ch, digit_pos など）
    PromotedLoopBody,    // 昇格済み LoopBodyLocal（is_ws, is_digit_pos など）
    CarrierLoopState,    // キャリア（sum, num_str など）
}

/// ScopeManager は ExprLowerer に対して「名前解決 + スコープ情報」を提供する。
pub trait ScopeManager {
    /// 名前から JoinIR ValueId を取得（なければ None）
    fn lookup(&self, name: &str) -> Option<ValueId>;

    /// 変数のスコープ種別を問い合わせる
    fn scope_of(&self, name: &str) -> Option<VarScopeKind>;
}
```

### 2.3 既存構造との対応づけ

- ConditionEnv:
  - loop_param + condition-only + body-only carrier を持っている。
  - ScopeManager 実装の中で「LoopParam / ConditionOnly / CarrierLoopState」などに振り分ける役割。
- LoopBodyLocalEnv:
  - `local ch`, `local digit_pos` などの body-local を保持。
  - ScopeManager からは `VarScopeKind::BodyLocal` として見える。
- CapturedEnv:
  - function_scope_capture.rs で検出された「関数スコープの実質定数」（digits, base, limit 等）。
  - ScopeManager 上では `VarScopeKind::CapturedConst` として扱う。
- CarrierInfo:
  - carrier 名と join_id、ConditionOnly role、promoted_loopbodylocals 情報を持つ。
  - ScopeManager 側からは、「PromotedLoopBody / CarrierLoopState」などの分類情報を取り出す。

ここではあくまでインターフェースレベルの設計に留めておき、  
実際の `ScopeManagerImpl`（ConditionEnv + LoopBodyLocalEnv + CapturedEnv + CarrierInfo を束ねる構造体）は後続フェーズで実装する想定だよ。

---

## 3. 既存箱とのマッピング表

Phase 230 の時点では「どの箱をどこに収めるか」をざっくり決めておくところまでにするよ。

| Box / モジュール名            | 現在の責務                                               | ExprLowerer との関係                                | ScopeManager / TypeContext との関係                      |
|------------------------------|----------------------------------------------------------|----------------------------------------------------|----------------------------------------------------------|
| BoolExprLowerer              | AST → MIR boolean 式 lowering（ほぼ未使用）             | 将来 `ExprLowerer` の内部ロジックとして統合候補     | 直接は関与しない（MirBuilder 向けの歴史的遺産）         |
| condition_to_joinir          | 条件式 lowering のオーケストレーター                    | 入口モジュールとして残し、中身を ExprLowerer 呼びに | ScopeManager を使って ConditionEnv 系を内側に隠す       |
| condition_lowerer            | AST → JoinIR 値（条件用）のコアロジック                 | ExprLowerer に徐々に移管し、将来は内部実装に        | 変数解決部分を ScopeManager に差し替える                |
| LoopBodyLocalInitLowerer     | body-local init 式の lowering + LoopBodyLocalEnv 更新    | lowering 部分は ExprLowerer に寄せ、将来は「宣言スキャン + env 更新」の薄い箱に | LoopBodyLocalEnv の管理は ScopeManager 実装が引き取る   |
| MethodCallLowerer            | MethodCall AST → BoxCall（CoreMethodId メタ駆動）       | ExprLowerer から utility として呼び出す             | 引数の変数解決に ScopeManager を利用するよう将来変更    |
| CarrierUpdateEmitter         | UpdateExpr（構造化済み）→ JoinIR 更新命令               | UpdateExpr 生成側が ExprLowerer と噛み合うよう設計する方向 | UpdateEnv を ScopeManager ベースの実装に置き換える候補  |
| ConditionEnv                 | 条件用の名前解決レイヤ                                  | ScopeManager の内部実装の一部                       | TypeContext と組み合わせて「型付き環境」にしていく余地  |
| LoopBodyLocalEnv             | ループ本体 local 変数の JoinIR ValueId マップ           | ScopeManager によって一段抽象化される               | 将来的に型付き body-local として TypeContext と連携     |
| CapturedEnv                  | 関数スコープ実質定数の環境                              | ScopeManager 実装が `CapturedConst` として吸収       | TypeContext から「不変 + 型」の情報を共有できると理想   |

---

## 4. Phase 230 のスコープと非スコープ

- スコープ（やること）:
  - 既存の式 lowering の散らばり方を inventory に落とし込む。
  - ExprLowerer / ScopeManager / ExprContext / VarScopeKind のインターフェース案を決める。
  - 既存箱がどこに入りそうか（or 外に残りそうか）を表で整理する。
- 非スコープ（やらないこと）:
  - 実際のコードの統合・リファクタリング。
  - condition_to_joinir / LoopBodyLocalInitLowerer などの実装変更。
  - TypeContext 自体の設計・実装（ここでは「将来ここにくっつける」レベルの言及に留める）。

結論として Phase 230 は、JoinIR ラインにおける「式」と「スコープ」の SSOT を見据えた  
**設計フェーズ（ドキュメントのみ）** として完了させるイメージだよ。

補足（Phase 231 時点の実装状況メモ）:

- `src/mir/join_ir/lowering/expr_lowerer.rs` / `scope_manager.rs` には、Phase 231 のパイロット実装が入っている。
  - コンテキストは `ExprContext::Condition` のみサポート。
  - 実装は ScopeManager → ConditionEnv を構築してから、既存の `lower_condition_to_joinir` に委譲する薄いラッパー。
  - `pattern2_with_break.rs` では **バリデーション専用（結果は捨てて、実際の lowering は従来経路）の呼び出し**になっている。
- 本ドキュメントは「最終的に目指す ExprLowerer/ScopeManager 像」の設計であり、Phase 231 の実装はあくまで  
  その前段階のパイロットという位置づけだよ（実装が完全にこの API 図のとおりになっているわけではない点に注意）。
Status: Active  
Scope: Expr Lowerer 設計（JoinIR/ExprLowerer ライン）
