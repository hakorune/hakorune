# Phase 200-C: digits.indexOf E2E 連携

**Date**: 2025-12-09
**Status**: Ready for Implementation
**Prerequisite**: Phase 200-A/B complete

---

## ゴール

1. **PatternPipelineContext / LoopPatternContext に fn_body（関数全体 AST）を通す**
2. **Pattern 2 で FunctionScopeCaptureAnalyzer を実際に呼び出す**
3. **digits.indexOf(ch) を含む最小ループを JoinIR 経由で最後まで動かす**

**成功基準**:
- `phase200_digits_atoi_min.hako` が正しい結果（123）を出力
- `phase200_digits_parse_number_min.hako` が正しい結果（"42"）を出力

---

## Task 200-C-1: LoopPatternContext に fn_body を追加

### 対象ファイル
- `src/mir/builder/control_flow/joinir/patterns/router.rs`
- `src/mir/builder/control_flow/joinir/routing.rs`

### 実装内容

#### 1. LoopPatternContext 拡張

```rust
// router.rs
pub struct LoopPatternContext<'a> {
    // 既存フィールド
    pub condition: &'a ASTNode,
    pub body: &'a [ASTNode],
    pub func_name: &'a str,
    pub debug: bool,
    pub has_continue: bool,
    pub has_break: bool,
    pub features: LoopFeatures,
    pub pattern_kind: LoopPatternKind,

    // Phase 200-C: NEW - 関数全体の AST
    pub fn_body: Option<&'a [ASTNode]>,
}

impl<'a> LoopPatternContext<'a> {
    pub fn new(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
    ) -> Self {
        // 既存コード...
        Self {
            // ...
            fn_body: None,  // Phase 200-C: Default to None
        }
    }

    /// Phase 200-C: Create context with fn_body for capture analysis
    pub fn with_fn_body(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        func_name: &'a str,
        debug: bool,
        fn_body: &'a [ASTNode],
    ) -> Self {
        let mut ctx = Self::new(condition, body, func_name, debug);
        ctx.fn_body = Some(fn_body);
        ctx
    }
}
```

#### 2. routing.rs から fn_body を渡す

```rust
// routing.rs - cf_loop_joinir_impl()

pub(in crate::mir::builder) fn cf_loop_joinir_impl(
    &mut self,
    condition: &ASTNode,
    body: &[ASTNode],
    func_name: &str,
    debug: bool,
) -> Result<Option<ValueId>, String> {
    use super::patterns::{route_loop_pattern, LoopPatternContext};

    // Phase 200-C: Get fn_body from current_function if available
    let fn_body_opt = self.current_function.as_ref()
        .map(|f| f.body.as_slice());

    let ctx = if let Some(fn_body) = fn_body_opt {
        LoopPatternContext::with_fn_body(condition, body, func_name, debug, fn_body)
    } else {
        LoopPatternContext::new(condition, body, func_name, debug)
    };

    if let Some(result) = route_loop_pattern(self, &ctx)? {
        // ...
    }
    // ...
}
```

### 制約

- P1/P3/P4 は `fn_body` を使わなくても動く（`None` を無視）
- `fn_body` が取得できない場合も動作する（空の CapturedEnv になる）

---

## Task 200-C-2: Pattern 2 で FunctionScopeCaptureAnalyzer を呼ぶ

### 対象ファイル
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

### 実装内容

Pattern 2 lowerer の `lower()` 関数内で capture 解析を呼び出す:

```rust
// pattern2_with_break.rs

use crate::mir::loop_pattern_detection::function_scope_capture::{
    analyze_captured_vars, CapturedEnv
};

pub fn lower(
    builder: &mut MirBuilder,
    ctx: &LoopPatternContext,
) -> Result<Option<ValueId>, String> {
    // 既存のループスコープ分析...
    let scope = /* ... */;

    // Phase 200-C: Capture analysis
    let captured_env = if let Some(fn_body) = ctx.fn_body {
        // fn_body が利用可能 - capture 解析を実行
        let loop_ast = /* ループ AST を構築 or ctx から取得 */;
        analyze_captured_vars(fn_body, &loop_ast, &scope)
    } else {
        // fn_body なし - 空の CapturedEnv
        CapturedEnv::new()
    };

    // 既存の ConditionEnv 構築を v2 に置き換え
    let cond_env = build_with_captures(
        &loop_var_name,
        &captured_env,
        &builder.variable_map,
        loop_var_id,
    );

    // 以降は既存のフロー...
}
```

### 注意点

1. **ループ AST の構築**: `analyze_captured_vars` は `loop_ast: &ASTNode` を必要とする
   - `ctx.condition` と `ctx.body` から Loop ノードを構築する必要がある
   - または `fn_body` 内でループ位置を見つける

2. **既存フローとの互換性**: `captured_env` が空の場合は既存の動作と同じ

---

## Task 200-C-3: ConditionEnvBuilder v2 の統合

### 対象
Pattern 2 lowerer 内の ConditionEnv 構築箇所

### 実装内容

```rust
// 既存コード (Phase 200-B まで)
let cond_env = build_loop_param_only(&loop_var_name, &boundary)?;

// Phase 200-C: v2 に置き換え
let cond_env = build_with_captures(
    &loop_var_name,
    &captured_env,       // 200-C-2 で取得
    &builder.variable_map,
    loop_var_id,
);
```

### 不変条件

- `captured_env` が空の場合、既存の `build_loop_param_only` と同じ結果
- `captured_env` に変数がある場合:
  - `ConditionEnv.captured` に追加される
  - `ParamRole::Condition` として boundary に登録される
  - Header PHI や ExitLine の対象にはならない

---

## Task 200-C-4: digits ループの E2E 検証

### テストファイル

- `apps/tests/phase200_digits_atoi_min.hako` (Phase 200-B で作成済み)
- `apps/tests/phase200_digits_parse_number_min.hako` (Phase 200-B で作成済み)

### 検証手順

```bash
# Step 1: 構造トレース - Pattern 2 がマッチすること確認
NYASH_JOINIR_STRUCTURE_ONLY=1 ./target/release/hakorune \
  apps/tests/phase200_digits_atoi_min.hako 2>&1 | head -30

# 確認:
# - Pattern2_WithBreak がマッチ
# - [joinir/freeze] や UnsupportedPattern が出ていない

# Step 2: Capture debug - digits が捕捉されていること確認
NYASH_CAPTURE_DEBUG=1 ./target/release/hakorune \
  apps/tests/phase200_digits_atoi_min.hako 2>&1 | grep -i "capture"

# 期待出力:
# [capture] Found: digits (host_id=XX, is_immutable=true)

# Step 3: E2E 実行
./target/release/hakorune apps/tests/phase200_digits_atoi_min.hako
# 期待: 123

./target/release/hakorune apps/tests/phase200_digits_parse_number_min.hako
# 期待: "42"
```

### トラブルシューティング

異常があれば:

```bash
# PHI トレース
NYASH_TRACE_PHI=1 NYASH_TRACE_VARMAP=1 ./target/release/hakorune \
  apps/tests/phase200_digits_atoi_min.hako 2>&1 | tail -50

# 確認ポイント:
# - digits が ConditionEnv.captured に入っているか
# - digits の ValueId が未定義になっていないか
```

### 期待される結果

| テスト | 期待値 | 確認内容 |
|--------|--------|----------|
| `phase200_digits_atoi_min.hako` | 123 | print(v) の出力 |
| `phase200_digits_parse_number_min.hako` | "42" | print(num_str) の出力 |

---

## Task 200-C-5: ドキュメント更新

### 1. joinir-architecture-overview.md

**追記内容**:

```markdown
### Phase 200-C: digits.indexOf E2E 連携 (完了)

- **LoopPatternContext 拡張**
  - `fn_body: Option<&[ASTNode]>` フィールド追加
  - `with_fn_body()` コンストラクタ追加
  - 関数全体の AST を Pattern 2 lowerer に渡す

- **Pattern 2 キャプチャ統合**
  - `analyze_captured_vars()` を Pattern 2 で呼び出し
  - `build_with_captures()` で ConditionEnv 構築
  - digits のような関数ローカルが JoinIR 経由で参照可能に

- **JsonParser 対応状況** (更新)
  | メソッド | Pattern | ConditionEnv | Status |
  |----------|---------|--------------|--------|
  | _parse_number | P2 | digits capture | ✅ JoinIR |
  | _atoi | P2 | digits capture | ✅ JoinIR |
```

### 2. CURRENT_TASK.md

**追記内容**:

```markdown
  - [x] **Phase 200-C: digits.indexOf E2E 連携** ✅ (完了: 2025-12-09)
        - **目的**: 200-A/B インフラを実際に Pattern 2 経路に統合
        - **実装内容**:
          - 200-C-1: LoopPatternContext に fn_body 追加 ✅
          - 200-C-2: Pattern 2 で capture 解析呼び出し ✅
          - 200-C-3: ConditionEnvBuilder v2 統合 ✅
          - 200-C-4: digits E2E 検証 ✅
          - 200-C-5: ドキュメント更新 ✅
        - **成果**:
          - phase200_digits_atoi_min.hako → 123 ✅
          - phase200_digits_parse_number_min.hako → "42" ✅
        - **次フェーズ**: Phase 200-D（ComplexAddendNormalizer 拡張 - 必要なら）
```

---

## 成功基準

- [x] LoopPatternContext に fn_body が追加されている
- [x] Pattern 2 で analyze_captured_vars() が呼ばれる
- [x] digits が CapturedEnv に捕捉される
- [x] ConditionEnv.captured に digits が存在する
- [x] phase200_digits_atoi_min.hako → 123 出力
- [x] phase200_digits_parse_number_min.hako → "42" 出力
- [x] 既存テストに退行なし

---

## 関連ファイル

### 修正対象
- `src/mir/builder/control_flow/joinir/patterns/router.rs`
- `src/mir/builder/control_flow/joinir/routing.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

### ドキュメント
- `docs/development/current/main/joinir-architecture-overview.md`
- `CURRENT_TASK.md`

---

## 設計原則

1. **後方互換**: fn_body が取得できない場合も動作（空 CapturedEnv）
2. **段階適用**: Pattern 2 のみに統合、他パターンは影響なし
3. **Fail-Fast 維持**: 安全でないパターンは無視（エラーにしない）
4. **最小変更**: 既存の routing/lowering フローを大幅に変えない
Status: Active  
Scope: digits ケースの end-to-end 収束メモ（ConditionEnv ライン）
