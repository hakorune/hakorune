# JoinIR Route Policies - ルーティング箱の責務

## 概要
このディレクトリには、route 認識とルーティング（policy決定）を行う「箱」が格納されています。

## Policy箱の責務

### ルーティング決定
- 入力: LoopSkeleton、break条件、carrier情報
- 出力: 適用可能な route / policy decision と LoweringResult
- 判断基準: route マッチング条件（Trim, ConditionalStep, Escape, etc.）

### Lowering Result生成
- route 固有の情報（ConditionOnlyRecipe, ConditionalStepInfo, etc.）を生成
- CarrierInfo拡張（promoted variables, trim_helper, etc.）
- ConditionBinding設定

## Policy箱の候補（将来の整理対象）

### trim_loop_lowering.rs (Phase 180/92/93)
**現在の場所**: `plan/trim_loop_lowering.rs`

**責務**: Trimパターン認識とConditionOnlyルーティング

**判断基準**:
- LoopBodyLocal変数がTrim条件（whitespace check）
- break条件がConditionOnly（毎イテレーション再計算）

**出力**:
- `TrimLoweringResult` - condition, carrier_info, condition_only_recipe

**将来的な整理**:
- Trim route 判断ロジックは `cleanup/policies/trim_policy.rs` へ移設済み
- current location は `plan/trim_loop_lowering.rs`（old `patterns/*` path は historical token のみ）

---

### p5b_escape_derived_policy.rs (Phase 94)
**現在の場所**: compat at `plan/policies/keep_plan/p5b_escape_derived_policy.rs`

**owner surface**: `cleanup/policies/p5b_escape_derived_policy.rs`

**責務**: P5b escapeパターン認識とBodyLocalDerivedルーティング

**判断基準**:
- body-local変数の再代入（例: `ch = s.substring(...)`）
- escape skip条件（例: `if ch == "\\" { i = i + 2 }`）
- loop変数の条件付き更新

**出力**:
- `P5bEscapeDerivedDecision::UseDerived(BodyLocalDerivedRecipe)` - escape recipe
- `P5bEscapeDerivedDecision::Reject(String)` - 検出失敗理由
- `P5bEscapeDerivedDecision::None` - 該当なし

**将来的な整理**:
- `cleanup/policies/p5b_escape_derived_policy.rs` に移設済み
- keep-plan 側は compat re-export のみ

---

### loop_true_read_digits_policy.rs (Phase 104/105)
**現在の場所**: retired from this compat shelf

**owner surface**: `cleanup/policies/loop_true_read_digits_policy.rs`

**責務**: `loop(true)` + break-only digits（`read_digits_from` family）の認識とルーティング

**判断基準**:
- `loop(true)` である
- read-digits(loop(true)) detector に一致する
- `ReadDigitsBreakConditionBox` が eos 条件 + digit 条件を抽出できる

**出力**:
- `PolicyDecision::Use(LoopTrueReadDigitsPolicyResult)` - `break_when_true := (ch == \"\") || !(digit_cond)` と `ch` allow-list
- `PolicyDecision::Reject(String)` - 検出失敗理由（Fail-Fast）
- `PolicyDecision::None` - 該当なし

**将来的な整理**:
- `cleanup/policies/loop_true_read_digits_policy.rs` に移設済み
- keep-plan 側の compat re-export は 291x-566 で撤去済み
- 追加の呼び出しは cleanup owner path を直接使う

---

### body_local_policy.rs
**現在の場所**: `plan/body_local_policy.rs`

**責務**: Body-local変数の昇格判断

**判断基準**:
- body-local変数がloop carrier候補か
- 昇格可能なパターンか（Trim, Escape, etc.）

**将来的な整理**:
- policies/へ移動してpolicy箱として統一

---

## 設計原則

### 単一判断の原則
- 各policy箱は1つの route 判断のみ
- 複数 route の判断は別のpolicy箱に委譲

### 非破壊的判断
- 入力を変更しない
- 判断結果をResultで返す（`Ok(Some(result))` / `Ok(None)` / `Err(msg)`）

### Fail-Fast
- route マッチング失敗は即座に`Ok(None)`を返す
- エラーは明示的に`Err(msg)`
- Reject理由は`error_tags::freeze()`でタグ付与

### Decision型の統一
- `PolicyDecision<T>`（Use / Reject / None）をSSOTにする
- 例: `P5bEscapeDerivedDecision = PolicyDecision<BodyLocalDerivedRecipe>`, `TrimPolicyResult`

## 使用例

### LoopBreakルーティング（現在の route family）
```rust
// Step 1: P5b escape route判断
match classify_p5b_escape_derived(body, loop_var_name) {
    P5bEscapeDerivedDecision::UseDerived(recipe) => {
        // P5b escape適用
        return lower_with_p5b_escape(recipe, ...);
    }
    P5bEscapeDerivedDecision::Reject(reason) => {
        return Err(reason);
    }
    P5bEscapeDerivedDecision::None => {
        // 次の policy へ
    }
}

// Step 2: Trim route判断
if let Some(trim_result) = TrimLoopLowerer::try_lower_trim_pattern(...)? {
    // Trim適用
    return Ok(trim_result);
}

// Step 3: デフォルト lowering
Ok(default_loop_break_lowering(...))
```

### 将来の統一ルーティング（policies/移動後）
```rust
use plan::policies::{P5bEscapePolicy, TrimPolicy, ConditionalStepPolicy};

// Policy boxの統一インターフェース
trait RoutePolicy {
    type Decision;
    fn classify(&self, context: &RouteContext) -> Self::Decision;
}

// ルーティングパイプライン
let decision = P5bEscapePolicy.classify(&ctx)
    .or_else(|| TrimPolicy.classify(&ctx))
    .or_else(|| ConditionalStepPolicy.classify(&ctx))
    .unwrap_or(DefaultDecision);
```

## デバッグ

### ログprefixの統一
- `[policy/p5b-escape]` - P5b escape判断
- `[policy/trim]` - Trim判断
- `[policy/conditional-step]` - ConditionalStep判断

### 環境変数
- `HAKO_JOINIR_DEBUG=1` - JoinIR全般のデバッグログ
- `joinir_dev_enabled()` - 既存の制御機構

### デバッグ出力例
```rust
if joinir_dev_enabled() {
    eprintln!("[policy/p5b-escape] Detected escape pattern: {:?}", info);
}
```

## 命名規則

### ファイル命名
- `<role>_policy.rs` - route 判断policy箱
- 例: `trim_policy.rs`, `escape_policy.rs`, `conditional_step_policy.rs`

### 型命名
- `<role>Decision` - 判断結果型
- `<role>Policy` - policy箱の構造体（将来）

### 列挙型パターン
```rust
pub enum Decision {
    None,           // 該当なし
    Use(Recipe),    // 適用可能
    Reject(String), // 検出失敗
}
```

## 将来の拡張

### Phase 95以降の候補
- `escape_policy.rs` - P5b escapeを統一インターフェースに
- `array_loop_policy.rs` - 配列ループパターン判断
- `map_loop_policy.rs` - Mapループパターン判断
- `conditional_step_policy.rs` - ConditionalStep独立箱化

### 段階的な移行計画

#### Phase 1: ディレクトリ準備（今回）
- policies/ディレクトリ作成 ✅
- README.md作成 ✅
- mod.rs作成 ✅

#### Phase 2: 既存policy箱の移動（将来）
- `loop_true_read_digits_policy.rs` → landed at `cleanup/policies/loop_true_read_digits_policy.rs`; keep-plan compat shelf retired in 291x-566
- `p5b_escape_derived_policy.rs` → landed at `cleanup/policies/p5b_escape_derived_policy.rs`
- `body_local_policy.rs` → `policies/body_local_policy.rs`
- `trim_policy.rs` → landed at `cleanup/policies/trim_policy.rs`

#### Phase 3: インターフェース統一（将来）
- `PatternPolicy` trait定義
- Decision型の統一
- ルーティングパイプラインの実装

### 拡張時の注意
- README.mdにpolicy箱の責務を追加
- Decision型を統一パターンに従う
- 既存のpolicy箱との一貫性を保つ
- Fail-Fastタグを明記

## 参考資料

### 関連ドキュメント
- [JoinIR アーキテクチャ概要](../../../../development/current/main/joinir-architecture-overview.md)
- [JoinIR 設計マップ](../../../../development/current/main/design/joinir-design-map.md)
- [Route Prep Pipeline](../../../plan/route_prep_pipeline.rs) - パターン判断の統合処理
- [Escape Route-Shape Recognizer](../escape_shape_recognizer.rs) - escape検出ロジック

### Phase Log
- [Phase 92 Log](../../../../development/current/main/phases/phase-92/) - ConditionalStep
- [Phase 93 Log](../../../../development/current/main/phases/phase-93/) - ConditionOnly
- [Phase 94 Log](../../../../development/current/main/phases/phase-94/) - BodyLocalDerived

## 設計哲学（Box Theory）

### 箱理論の適用
- **箱にする**: Policy判断を独立した箱に分離
- **境界を作る**: Decisionを統一インターフェースに
- **戻せる**: 段階的移行で既存コードを壊さない
- **見える化**: ログprefixで判断過程を可視化

### Fail-Fast原則の徹底
- フォールバック処理は原則禁止
- Reject理由を明示的に返す
- エラータグで根本原因を追跡可能に

### 単一責任の箱
- 1つのpolicy箱 = 1つのパターン判断
- 複数パターンの組み合わせは上位層で制御
- policy箱同士は独立・疎結合
