# Phase 200-B: FunctionScopeCaptureAnalyzer 実装 & ConditionEnv 統合

**Date**: 2025-12-09
**Status**: Ready for Implementation
**Prerequisite**: Phase 200-A complete

---

## ゴール

1. **CapturedEnv に「安全にキャプチャできる関数ローカル」を実際に埋める**
2. **ConditionEnv / JoinInlineBoundaryBuilder に統合して、`digits` みたいな変数を JoinIR 側から参照できるようにする**
3. **影響範囲は `_parse_number` / `_atoi` の最小ケースに限定、挙動は Fail-Fast を維持**

**スコープ制限**:
- ✅ ConditionEnv に digits を見せられるようにする
- ❌ `digits.indexOf(ch)` の E2E 動作は Phase 200-C（ComplexAddendNormalizer 連携）

---

## Task 200-B-1: capture 判定ロジック実装

### 対象ファイル
`src/mir/loop_pattern_detection/function_scope_capture.rs`（Phase 200-A で作ったスケルトン）

### 実装内容

`analyze_captured_vars(fn_body, loop_ast, scope) -> CapturedEnv` を実装する。

**判定条件（全部満たしたものだけ許可）**:

1. **関数トップレベルで `local name = <expr>;` として 1 回だけ定義されている**
   - ループより前の位置で定義
   - 複数回定義されていない

2. **その変数 `name` はループ本体（条件含む）で読み取りのみ（再代入なし）**
   - ループ内で `name = ...` が存在しない

3. **`<expr>` は「安全な初期式」だけ**:
   - 文字列リテラル `"0123456789"`
   - 整数リテラル `123`
   - 将来拡張を見越して Const 系だけにしておく（MethodCall 等はまだ対象外）

### 実装アルゴリズム

```rust
pub fn analyze_captured_vars(
    fn_body: &[Stmt],
    loop_ast: &Stmt,
    scope: &LoopScopeShape,
) -> CapturedEnv {
    let mut env = CapturedEnv::new();

    // Step 1: Find loop position in fn_body
    let loop_index = find_stmt_index(fn_body, loop_ast);

    // Step 2: Collect local declarations BEFORE the loop
    let pre_loop_locals = collect_local_declarations(&fn_body[..loop_index]);

    // Step 3: For each pre-loop local, check:
    for local in pre_loop_locals {
        // 3a: Is init expression a safe constant?
        if !is_safe_const_init(&local.init) {
            continue;
        }

        // 3b: Is this variable reassigned anywhere in fn_body?
        if is_reassigned_in_fn(fn_body, &local.name) {
            continue;
        }

        // 3c: Is this variable used in loop (condition or body)?
        if !is_used_in_loop(loop_ast, &local.name) {
            continue;
        }

        // 3d: Skip if already in LoopParam or LoopBodyLocal
        if scope.loop_params.contains(&local.name) || scope.body_locals.contains(&local.name) {
            continue;
        }

        // All checks passed: add to CapturedEnv
        env.add_var(CapturedVar {
            name: local.name.clone(),
            host_id: local.value_id,  // From scope/variable_map
            is_immutable: true,
        });
    }

    env
}

/// Check if expression is a safe constant (string/integer literal)
fn is_safe_const_init(expr: &Option<Expr>) -> bool {
    match expr {
        Some(Expr::StringLiteral(_)) => true,
        Some(Expr::IntegerLiteral(_)) => true,
        _ => false,
    }
}

/// Check if variable is reassigned anywhere in function body
fn is_reassigned_in_fn(fn_body: &[Stmt], name: &str) -> bool {
    // Walk all statements, check for `name = ...` (excluding initial declaration)
    // Implementation uses AST visitor pattern
}

/// Check if variable is referenced in loop condition or body
fn is_used_in_loop(loop_ast: &Stmt, name: &str) -> bool {
    // Walk loop AST, check for Identifier(name) references
}
```

### ユニットテスト

```rust
#[test]
fn test_capture_simple_digits() {
    // local digits = "0123456789"
    // loop(i < 10) { digits.indexOf(ch) }
    // → 1 var captured (digits)
}

#[test]
fn test_capture_reassigned_rejected() {
    // local digits = "0123456789"
    // digits = "abc"  // reassignment
    // loop(i < 10) { digits.indexOf(ch) }
    // → 0 vars captured
}

#[test]
fn test_capture_after_loop_rejected() {
    // loop(i < 10) { ... }
    // local digits = "0123456789"  // defined AFTER loop
    // → 0 vars captured
}

#[test]
fn test_capture_method_call_init_rejected() {
    // local result = someBox.getValue()  // MethodCall init
    // loop(i < 10) { result.indexOf(ch) }
    // → 0 vars captured (not safe const)
}
```

### 成果物
- [x] `analyze_captured_vars` 本実装
- [x] ヘルパ関数（`is_safe_const_init`, `is_reassigned_in_fn`, `is_used_in_loop`）
- [x] 4+ unit tests

---

## Task 200-B-2: ConditionEnvBuilder v2 実装

### 対象ファイル
`src/mir/builder/control_flow/joinir/patterns/condition_env_builder.rs`

### 実装内容

`build_with_captures(loop_var_name, captured, boundary) -> ConditionEnv` を本実装にする。

```rust
pub fn build_with_captures(
    loop_var_name: &str,
    captured: &CapturedEnv,
    boundary: &mut JoinInlineBoundaryBuilder,
) -> ConditionEnv {
    // Step 1: Build base ConditionEnv with loop params (existing logic)
    let mut env = build_loop_param_only(loop_var_name, boundary);

    // Step 2: Add captured vars as ParamRole::Condition
    for var in &captured.vars {
        // 2a: Add to boundary with Condition role
        boundary.add_param_with_role(&var.name, var.host_id, ParamRole::Condition);

        // 2b: Add to ConditionEnv.captured map
        // Need JoinIR ValueId from boundary/remapper
        let join_id = boundary.get_condition_binding(&var.name)
            .expect("captured var should be in boundary");
        env.captured.insert(var.name.clone(), join_id);
    }

    // Step 3: Debug guard - Condition params must NOT be in PHI candidates
    #[cfg(debug_assertions)]
    for var in &captured.vars {
        assert!(
            !env.params.contains_key(&var.name),
            "Captured var '{}' must not be in loop params (ParamRole conflict)",
            var.name
        );
    }

    env
}
```

### ConditionEnv 拡張

```rust
pub struct ConditionEnv {
    pub params: BTreeMap<String, ValueId>,    // LoopParam (existing)
    pub captured: BTreeMap<String, ValueId>,  // NEW: Captured vars (ParamRole::Condition)
}

impl ConditionEnv {
    /// Look up a variable (params first, then captured)
    pub fn get(&self, name: &str) -> Option<ValueId> {
        self.params.get(name).copied()
            .or_else(|| self.captured.get(name).copied())
    }

    /// Check if variable is a captured (Condition role) var
    pub fn is_captured(&self, name: &str) -> bool {
        self.captured.contains_key(name)
    }
}
```

### JoinInlineBoundaryBuilder 更新

```rust
impl JoinInlineBoundaryBuilder {
    pub fn add_param_with_role(&mut self, name: &str, host_id: ValueId, role: ParamRole) {
        match role {
            ParamRole::LoopParam | ParamRole::Carrier => {
                // Existing: add to join_inputs
                self.add_input(name, host_id);
            }
            ParamRole::Condition => {
                // NEW: Add to condition_bindings only (no PHI, no ExitLine)
                let join_id = self.alloc_value();  // Allocate JoinIR ValueId
                self.condition_bindings.push(ConditionBinding {
                    name: name.to_string(),
                    host_id,
                    join_id,
                    role: ParamRole::Condition,
                });
            }
            ParamRole::ExprResult => {
                // Handled by set_expr_result
            }
        }
    }

    pub fn get_condition_binding(&self, name: &str) -> Option<ValueId> {
        self.condition_bindings.iter()
            .find(|b| b.name == name)
            .map(|b| b.join_id)
    }
}
```

### ユニットテスト

```rust
#[test]
fn test_build_with_empty_captures() {
    // CapturedEnv empty → same as existing build
    let captured = CapturedEnv::new();
    let env = build_with_captures("i", &captured, &mut builder);
    assert!(env.captured.is_empty());
}

#[test]
fn test_build_with_digits_capture() {
    // CapturedEnv with "digits"
    let mut captured = CapturedEnv::new();
    captured.add_var(CapturedVar {
        name: "digits".to_string(),
        host_id: ValueId(42),
        is_immutable: true,
    });

    let env = build_with_captures("i", &captured, &mut builder);

    // Verify captured var is in ConditionEnv
    assert!(env.captured.contains_key("digits"));

    // Verify boundary has Condition role
    let binding = builder.get_condition_binding("digits").unwrap();
    // binding should exist with ParamRole::Condition
}
```

### 成果物
- [x] `build_with_captures` 本実装
- [x] `ConditionEnv.captured` フィールド追加
- [x] `add_param_with_role` の Condition ブランチ実装
- [x] 2+ unit tests

---

## Task 200-B-3: パイプライン組み込み

### 対象
PatternPipelineContext / Pattern lowerer の「前処理パス」

### 実装内容

Pattern 決定後、JoinIR lowering に入る前の箇所で capture 解析を挿入。

```rust
// In pattern lowerer (e.g., pattern2_with_break.rs)

// Step 1: Existing - build PatternPipelineContext
let pipeline_ctx = PatternPipelineContext::new(/* ... */);

// Step 2: NEW - Analyze captured vars
let captured = analyze_captured_vars(
    &fn_body,      // Function body statements
    &loop_ast,     // Loop AST
    &pipeline_ctx.loop_scope,
);

// Step 3: Build ConditionEnv with captures
let cond_env = build_with_captures(
    &pipeline_ctx.loop_var_name,
    &captured,
    &mut boundary_builder,
);

// Step 4: Proceed with JoinIR lowering using cond_env
```

### 段階適用（今フェーズ）

- **Pattern 2 のみに適用**（`_parse_number` / `_atoi` 向け）
- 他パターン（P1/P3/P4）は既存 ConditionEnv のまま（影響なし）

### テストファイル whitelist

```rust
// routing.rs に追加（必要な場合）
// Phase 200-B: digits capture test cases
"phase200_digits_atoi_min",
"phase200_digits_parse_number_min",
```

### 成果物
- [x] Pattern 2 に capture 解析パス追加
- [x] 必要に応じて whitelist 更新

---

## Task 200-B-4: digits ケース検証

### テストファイル作成

#### `apps/tests/phase200_digits_atoi_min.hako`

```nyash
// Phase 200-B: Minimal atoi with digits capture
static box Main {
    main() {
        local s = "123"
        local digits = "0123456789"  // ← Captured var

        local i = 0
        local v = 0
        local n = s.length()

        loop(i < n) {
            local ch = s.substring(i, i+1)
            local pos = digits.indexOf(ch)  // ← Uses captured digits

            if pos < 0 {
                break
            }

            v = v * 10 + pos
            i = i + 1
        }

        print(v)  // Expected: 123
    }
}
```

#### `apps/tests/phase200_digits_parse_number_min.hako`

```nyash
// Phase 200-B: Minimal parse_number with digits capture
static box Main {
    main() {
        local s = "42abc"
        local digits = "0123456789"  // ← Captured var

        local p = 0
        local num_str = ""
        local n = s.length()

        loop(p < n) {
            local ch = s.substring(p, p+1)
            local digit_pos = digits.indexOf(ch)  // ← Uses captured digits

            if digit_pos < 0 {
                break
            }

            num_str = num_str + ch
            p = p + 1
        }

        print(num_str)  // Expected: "42"
    }
}
```

### 検証手順

```bash
# Step 1: 構造トレース（Pattern 選択確認）
NYASH_JOINIR_CORE=1 NYASH_JOINIR_STRUCTURE_ONLY=1 ./target/release/hakorune \
  apps/tests/phase200_digits_atoi_min.hako 2>&1 | head -30

# Expected: Pattern 2 selected, NO [joinir/freeze]

# Step 2: Capture trace（digits が捕捉されているか）
NYASH_JOINIR_CORE=1 NYASH_CAPTURE_DEBUG=1 ./target/release/hakorune \
  apps/tests/phase200_digits_atoi_min.hako 2>&1 | grep -i "capture"

# Expected: [capture] Found: digits (host_id=XX, is_immutable=true)

# Step 3: E2E 実行
NYASH_JOINIR_CORE=1 ./target/release/hakorune apps/tests/phase200_digits_atoi_min.hako

# Phase 200-B Goal: digits がConditionEnv に見えていることを確認
# E2E 動作は Phase 200-C（ComplexAddendNormalizer + digits.indexOf 連携）
```

### 期待される結果

**Phase 200-B のゴール達成**:
- ✅ `digits` が CapturedEnv に捕捉される
- ✅ `digits` が ConditionEnv.captured に存在する
- ✅ boundary に ParamRole::Condition として登録される

**Phase 200-C への引き継ぎ**:
- ⚠️ `digits.indexOf(ch)` の E2E 動作はまだ Fail-Fast の可能性あり
- → ComplexAddendNormalizer が MethodCall を扱えるようにする必要あり

### 成果物
- [x] `phase200_digits_atoi_min.hako` テストファイル
- [x] `phase200_digits_parse_number_min.hako` テストファイル
- [x] 構造トレース確認
- [x] Capture debug 確認

---

## Task 200-B-5: ドキュメント更新

### 1. joinir-architecture-overview.md

**Section 2.3 に追記**:

```markdown
- **FunctionScopeCaptureAnalyzer** (Phase 200-B 実装完了)
  - 責務: 関数スコープの「実質定数」を検出
  - 判定条件:
    1. 関数トップレベルで 1 回だけ定義
    2. ループ内で再代入なし
    3. 安全な初期式（文字列/整数リテラル）のみ
  - 結果: CapturedEnv に name, host_id, is_immutable を格納

- **ConditionEnvBuilder v2** (Phase 200-B 実装完了)
  - 責務: CapturedEnv から ParamRole::Condition として ConditionEnv に追加
  - 経路: analyze_captured_vars → build_with_captures → ConditionEnv.captured
  - 不変条件: Condition role は Header PHI / ExitLine の対象にならない
```

### 2. CURRENT_TASK.md

```markdown
  - [x] **Phase 200-B: FunctionScopeCaptureAnalyzer 実装 & ConditionEnv 統合** ✅ (完了: 2025-12-XX)
        - **目的**: digits 等の関数ローカルを ConditionEnv から参照可能に
        - **実装内容**:
          - 200-B-1: capture 判定ロジック実装 ✅
          - 200-B-2: ConditionEnvBuilder v2 実装 ✅
          - 200-B-3: パイプライン組み込み（Pattern 2）✅
          - 200-B-4: digits ケース検証 ✅
          - 200-B-5: ドキュメント更新 ✅
        - **成果**:
          - digits が CapturedEnv に捕捉される ✅
          - ConditionEnv.captured に登録される ✅
          - ParamRole::Condition として boundary に追加される ✅
        - **制約**:
          - digits.indexOf(ch) の E2E 動作は Phase 200-C
          - ComplexAddendNormalizer の MethodCall 対応が必要
        - **次フェーズ**: Phase 200-C（digits.indexOf E2E 連携）
```

---

## 成功基準

- [x] `analyze_captured_vars` が digits を正しく検出
- [x] `build_with_captures` が ConditionEnv.captured に追加
- [x] boundary に ParamRole::Condition として登録
- [x] 既存テストが退行しない
- [x] Unit tests (6+ 件) が PASS
- [x] phase200_digits_*.hako で capture が確認できる

---

## 設計原則（Phase 200-B）

1. **スコープ限定**: digits 系の最小ケースのみ
2. **Fail-Fast 維持**: 安全でないパターンは即座に拒否
3. **段階適用**: Pattern 2 のみに適用、他パターンは影響なし
4. **E2E 分離**: ConditionEnv への統合と、MethodCall 連携は別フェーズ

---

## 関連ファイル

### 修正対象
- `src/mir/loop_pattern_detection/function_scope_capture.rs`
- `src/mir/builder/control_flow/joinir/patterns/condition_env_builder.rs`
- `src/mir/join_ir/lowering/inline_boundary_builder.rs`
- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

### 新規作成
- `apps/tests/phase200_digits_atoi_min.hako`
- `apps/tests/phase200_digits_parse_number_min.hako`

### ドキュメント
- `docs/development/current/main/joinir-architecture-overview.md`
- `CURRENT_TASK.md`
Status: Active  
Scope: ConditionEnv capture 実装メモ（JoinIR v2 / selfhost 深度2 用）
