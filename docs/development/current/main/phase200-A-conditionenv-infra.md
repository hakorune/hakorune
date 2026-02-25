# Phase 200-A: ConditionEnv 拡張インフラ（型と箱のみ）

**Date**: 2025-12-09
**Status**: Ready for Implementation
**Prerequisite**: Phase 197 complete

---

## 目的

**ConditionEnv を壊さずに、関数スコープの "実質定数" ローカル（例: `local digits = "0123456789"`）を安全に扱う基盤を作る。**

**このフェーズでは**:
- ✅ 型と解析箱だけを導入
- ✅ 既存ループの挙動は変えない
- ❌ 具体ループ（`_parse_number` / `_atoi` 等）にはまだ手を出さない

**設計原則**:
- ConditionEnv の「芯」（= boundary に載ったものだけを見る）を保持
- 2-tier 設計（ConditionEnv / LoopBodyLocalEnv）は維持
- 新しい箱は「関数スコープの実質定数」を捕捉するレイヤーとして追加

---

## アーキテクチャ概要

### 現在の設計（Phase 193 まで）

```
┌─────────────────────────────────────────────────────────┐
│ ConditionEnv                                            │
│   - LoopParam (i, len, etc.)                           │
│   - OuterLocal (function params: s, pos)               │
│   - LoopBodyLocal (body-local vars: ch, digit)         │
└─────────────────────────────────────────────────────────┘
          ↓
┌─────────────────────────────────────────────────────────┐
│ JoinInlineBoundary                                      │
│   - join_inputs / host_inputs                          │
│   - condition_bindings                                  │
│   - exit_bindings                                       │
└─────────────────────────────────────────────────────────┘
```

**問題**: `digits` のような「関数スコープで宣言され、ループ内で不変なローカル」が捕捉できない

### 新設計（Phase 200+）

```
┌─────────────────────────────────────────────────────────┐
│ FunctionScopeCaptureAnalyzer                            │
│   - 関数スコープの "実質定数" を検出                      │
│   - CapturedEnv を生成                                   │
└─────────────────────────────────────────────────────────┘
          ↓
┌─────────────────────────────────────────────────────────┐
│ CapturedEnv                                             │
│   - CapturedVar[] { name, host_id, is_immutable }      │
└─────────────────────────────────────────────────────────┘
          ↓
┌─────────────────────────────────────────────────────────┐
│ ConditionEnvBuilder v2                                  │
│   - LoopParam + CapturedEnv → ConditionEnv に合成       │
└─────────────────────────────────────────────────────────┘
          ↓
┌─────────────────────────────────────────────────────────┐
│ JoinInlineBoundary (拡張)                               │
│   - ParamRole: LoopParam / Condition / Carrier / Expr  │
│   - CapturedEnv の変数も condition_bindings に載せる    │
└─────────────────────────────────────────────────────────┘
```

---

## Task 200-A-1: CapturedVar / CapturedEnv 型の導入

### 目標
関数スコープの「実質定数」を表現する型を定義する。

### ファイル
**新規**: `src/mir/loop_pattern_detection/function_scope_capture.rs`

### 実装内容

```rust
//! Phase 200-A: Function scope capture infrastructure
//!
//! This module provides types for capturing function-scoped variables
//! that are effectively immutable within a loop context.

/// A variable captured from function scope for use in loop conditions.
///
/// Example: `local digits = "0123456789"` in JsonParser._atoi()
#[derive(Debug, Clone)]
pub struct CapturedVar {
    /// Variable name (e.g., "digits", "table")
    pub name: String,

    /// MIR ValueId of the original definition
    pub host_id: ValueId,

    /// Whether this variable is never reassigned in the function
    pub is_immutable: bool,
}

/// Environment containing function-scoped captured variables.
///
/// Phase 200-A: Type definition only, not yet integrated with ConditionEnv.
#[derive(Debug, Clone, Default)]
pub struct CapturedEnv {
    pub vars: Vec<CapturedVar>,
}

impl CapturedEnv {
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }

    pub fn add_var(&mut self, var: CapturedVar) {
        self.vars.push(var);
    }

    /// Look up a captured variable by name
    pub fn get(&self, name: &str) -> Option<&CapturedVar> {
        self.vars.iter().find(|v| v.name == name)
    }
}
```

### テスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    #[test]
    fn test_captured_env_empty() {
        let env = CapturedEnv::new();
        assert!(env.is_empty());
        assert!(env.get("digits").is_none());
    }

    #[test]
    fn test_captured_env_add_and_get() {
        let mut env = CapturedEnv::new();
        env.add_var(CapturedVar {
            name: "digits".to_string(),
            host_id: ValueId(42),
            is_immutable: true,
        });

        assert!(!env.is_empty());
        let var = env.get("digits").unwrap();
        assert_eq!(var.name, "digits");
        assert_eq!(var.host_id, ValueId(42));
        assert!(var.is_immutable);
    }
}
```

### 成果物
- [x] `CapturedVar` 構造体定義
- [x] `CapturedEnv` 構造体定義
- [x] 基本的なアクセサメソッド
- [x] Unit tests (2件)

---

## Task 200-A-2: FunctionScopeCaptureAnalyzer スケルトン

### 目標
関数スコープの実質定数を検出する解析関数の枠を作る。

### ファイル
同じ `function_scope_capture.rs` に追加

### 実装内容

```rust
/// Analyzes function-scoped variables that can be safely captured for loop conditions.
///
/// # Phase 200-A Status
/// Currently returns empty CapturedEnv (skeleton implementation).
/// Actual capture detection will be implemented in Phase 200-B.
///
/// # Future Detection Criteria (Phase 200-B+)
/// - Variable is declared before the loop
/// - Variable is never reassigned within the function
/// - Variable is referenced in loop condition or body
pub fn analyze_captured_vars(
    _fn_body: &[Stmt],
    _loop_ast: &Stmt,
    _scope: &LoopScopeShape,
) -> CapturedEnv {
    // Phase 200-A: Skeleton implementation
    // TODO(Phase 200-B): Implement actual capture detection
    //
    // Detection algorithm:
    // 1. Find all `local` declarations before the loop
    // 2. Check if each is never reassigned (is_immutable = true)
    // 3. Check if referenced in loop condition/body
    // 4. Exclude loop parameters and body-local variables

    CapturedEnv::new()
}
```

### 設計メモ

**Phase 200-B で実装する検出アルゴリズム**:
1. ループの前にある `local` 宣言を全て収集
2. 関数全体で再代入されていないかチェック（`is_immutable`）
3. ループ条件/本体で参照されているかチェック
4. LoopParam / LoopBodyLocal は除外（既に ConditionEnv で扱われている）

### 成果物
- [x] `analyze_captured_vars` 関数シグネチャ
- [x] ドキュメントコメント（将来の検出基準を記載）
- [x] 空実装（Phase 200-B で中身を実装）

---

## Task 200-A-3: ConditionEnvBuilder v2 入口

### 目標
将来 CapturedEnv を受け取るフックだけ用意する。

### ファイル
`src/mir/join_ir/lowering/condition_env_builder.rs` または既存の ConditionEnv 関連ファイル

### 実装内容

```rust
/// Build ConditionEnv with optional captured variables.
///
/// # Phase 200-A Status
/// Currently ignores `captured` parameter and calls existing implementation.
/// Integration with CapturedEnv will be implemented in Phase 200-B.
///
/// # Future Behavior (Phase 200-B+)
/// - Add captured variables to condition_bindings
/// - Generate Copy instructions for captured vars in entry block
/// - Track captured vars separately from loop params
pub fn build_condition_env_with_captures(
    loop_params: &[ParamInfo],
    _captured: &CapturedEnv,
    boundary: &mut JoinInlineBoundaryBuilder,
) -> ConditionEnv {
    // Phase 200-A: Delegate to existing implementation
    // TODO(Phase 200-B): Integrate captured vars into ConditionEnv
    //
    // Integration steps:
    // 1. For each captured var, add to boundary.condition_bindings
    // 2. Mark as ParamRole::Condition (not Carrier or LoopParam)
    // 3. Ensure captured vars are NOT included in exit_bindings

    build_condition_env_from_params(loop_params, boundary)
}
```

### 成果物
- [x] `build_condition_env_with_captures` 関数（既存実装に委譲）
- [x] ドキュメントコメント（Phase 200-B の統合手順を記載）

---

## Task 200-A-4: ParamRole enum 追加

### 目標
JoinInlineBoundary のパラメータ役割を明示的に区別する。

### ファイル
`src/mir/join_ir/lowering/inline_boundary.rs` または関連ファイル

### 実装内容

```rust
/// Role of a parameter in JoinIR lowering.
///
/// # Invariants
/// - **LoopParam**: Participates in header PHI, updated in loop body
/// - **Condition**: Used in condition only, NOT in header PHI, NOT in ExitLine
/// - **Carrier**: Updated in loop body, participates in header PHI and ExitLine
/// - **ExprResult**: Return value of the loop expression, handled by exit_phi_builder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamRole {
    /// Loop iteration variable (e.g., `i` in `loop(i < len)`)
    LoopParam,

    /// Condition-only parameter (e.g., `digits` in `digits.indexOf(ch)`)
    /// NOT included in header PHI or ExitLine
    Condition,

    /// State carried across iterations (e.g., `sum`, `count`)
    Carrier,

    /// Expression result returned by the loop
    ExprResult,
}

impl JoinInlineBoundaryBuilder {
    /// Add a parameter with explicit role.
    ///
    /// # Phase 200-A Status
    /// Currently stores role but does not use it for routing.
    /// Role-based routing will be implemented in Phase 200-B.
    pub fn add_param_with_role(&mut self, name: &str, host_id: ValueId, role: ParamRole) {
        // Phase 200-A: Store role for future use
        // TODO(Phase 200-B): Route based on role
        //
        // Routing rules:
        // - LoopParam: join_inputs + host_inputs
        // - Condition: condition_bindings only (no PHI, no ExitLine)
        // - Carrier: join_inputs + host_inputs + exit_bindings
        // - ExprResult: handled by exit_phi_builder

        match role {
            ParamRole::LoopParam | ParamRole::Carrier => {
                // Existing behavior: add to join_inputs
                self.add_input(name, host_id);
            }
            ParamRole::Condition => {
                // Phase 200-A: Just log for now
                // TODO(Phase 200-B): Add to condition_bindings without PHI
            }
            ParamRole::ExprResult => {
                // Handled separately by set_expr_result
            }
        }
    }
}
```

### 不変条件（joinir-architecture-overview.md に追記）

```markdown
9. **ParamRole の不変条件**
   - **Condition 役のパラメータは「PHI dst にしてはいけない」**
     - 理由: 条件専用変数はループ内で更新されない
     - 例: `digits` は header PHI に参加しない
   - **Condition 役のパラメータは「ExitLine の対象にも入れない」**
     - 理由: 条件専用変数はループ外で使われない（ループ内でのみ参照）
     - 例: `digits.indexOf(ch)` の結果は exit_bindings に載らない
```

### 成果物
- [x] `ParamRole` enum 定義
- [x] `add_param_with_role` メソッド（スケルトン）
- [x] 不変条件のドキュメント

---

## Task 200-A-5: ドキュメント更新

### 1. joinir-architecture-overview.md

**Section 2.3 (Boundary / Carrier ライン) に追記**:

```markdown
- **FunctionScopeCaptureAnalyzer / CapturedEnv** (Phase 200-A)
  - ファイル: `src/mir/loop_pattern_detection/function_scope_capture.rs`
  - 責務:
    - 関数スコープで宣言され、ループ内で不変な変数を検出
    - CapturedVar: { name, host_id, is_immutable }
    - CapturedEnv: 検出された変数のコレクション
  - **Phase 200-A**: 型と空実装のみ（中身は Phase 200-B で実装）

- **ParamRole enum** (Phase 200-A)
  - ファイル: `src/mir/join_ir/lowering/inline_boundary.rs`
  - 責務:
    - JoinInlineBoundary のパラメータ役割を明示的に区別
    - LoopParam / Condition / Carrier / ExprResult
  - **Phase 200-A**: enum 定義のみ（ルーティングは Phase 200-B で実装）
```

**Section 1 (不変条件) に追記**:

```markdown
9. **ParamRole の不変条件**
   - Condition 役のパラメータは「PHI dst にしてはいけない」
   - Condition 役のパラメータは「ExitLine の対象にも入れない」
   - 理由: 条件専用変数はループ内で更新されず、ループ外で使われない
```

### 2. CURRENT_TASK.md

**Phase 200-A セクション追加**:

```markdown
  - [x] **Phase 200-A: ConditionEnv 拡張インフラ（型と箱のみ）** ✅ (完了: 2025-12-XX)
        - **目的**: ConditionEnv を壊さずに関数スコープ "実質定数" を扱う基盤
        - **実装内容**:
          - 200-A-1: CapturedVar / CapturedEnv 型導入 ✅
          - 200-A-2: FunctionScopeCaptureAnalyzer スケルトン ✅
          - 200-A-3: ConditionEnvBuilder v2 入口 ✅
          - 200-A-4: ParamRole enum 追加 ✅
          - 200-A-5: ドキュメント更新 ✅
        - **スコープ**: Infra only / Integration pending
        - **成果**:
          - 型と箱の定義完了 ✅
          - 既存ループの挙動変更なし ✅
          - Phase 200-B への準備完了 ✅
        - **次フェーズ**: Phase 200-B（digits 系ループへの適用）
```

---

## 成功基準

- [x] `CapturedVar` / `CapturedEnv` 型が定義されている
- [x] `analyze_captured_vars` 関数スケルトンが存在する
- [x] `build_condition_env_with_captures` 入口が存在する
- [x] `ParamRole` enum が定義されている
- [x] 既存テストが全て PASS（退行なし）
- [x] ドキュメント更新（overview + CURRENT_TASK）

---

## 設計原則（Phase 200-A）

1. **Infra only**: 型と箱の定義のみ、具体ループへの適用は Phase 200-B
2. **既存挙動維持**: 現在のループは全く同じように動作する
3. **箱化原則**: 新しい責務は新しい箱（FunctionScopeCaptureAnalyzer）に集約
4. **ドキュメント駆動**: 将来の実装方針をコメントとして残す

---

## 関連ファイル

### 新規作成
- `src/mir/loop_pattern_detection/function_scope_capture.rs`

### 修正対象
- `src/mir/loop_pattern_detection/mod.rs`（モジュール追加）
- `src/mir/join_ir/lowering/condition_env_builder.rs`（v2 入口）
- `src/mir/join_ir/lowering/inline_boundary.rs`（ParamRole）

### ドキュメント
- `docs/development/current/main/joinir-architecture-overview.md`
- `CURRENT_TASK.md`
Status: Active  
Scope: ConditionEnv インフラ設計（JoinIR v2 / selfhost 深度2 用）
