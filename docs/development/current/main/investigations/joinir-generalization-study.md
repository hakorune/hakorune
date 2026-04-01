Status: Historical (context)  
Scope: ChatGPT Pro 相談用に、Phase 131–138 の “当時点” の状況と一般化案をまとめたメモ。  
Related:
- SSOT (roadmap): `docs/development/current/main/10-Now.md`
- SSOT (decisions): `docs/development/current/main/20-Decisions.md`
- SSOT (design): `docs/development/current/main/design/normalized-expr-lowering.md`
- phases: `docs/development/current/main/phases/archive/phase-139/README.md`, `docs/development/current/main/phases/archive/phase-140/README.md`

# Hakorune JoinIR 設計 - ChatGPT Pro 用コンテキスト

> 注意: この文書は “相談の前提” であり SSOT ではありません。コード断片は擬似コードを含みます。
> 正本（収束方針）は `docs/development/current/main/design/normalized-expr-lowering.md` を参照してください。

## 概要

Hakorune（セルフホスティングコンパイラ）の JoinIR（Join Intermediate Representation）設計に関する一般化戦略の相談。

### 現状の問題

- Phase 131-138 で「パターン追加」方式で拡張中
- このままでは無限に Phase が必要になる可能性
- 目標: パターンマッチング → 式の一般化（AST walker）への移行

### 提案中の方向

```
制御フロー形（loop/if）: 段階的拡張（正規化）OK
式と return: 早期に一般化（ExprLowererBox）へ移行
PHI: Normalized 内で避ける → 後段（SSA/PHI）に押し出す
```

---

## ファイル構成（主要部分抜粋）

### 1. LoopTrueBreakOnceBuilderBox (loop_true_break_once.rs - 主要実装)

```rust
//! Phase 131-138: loop(true) break-once 正規化
//! 
//! ## 責務
//! - loop(true) { <assign>* ; break } パターンを JoinModule に変換
//! - PHI-free: env パラメータ + 継続で値を渡す
//! - Return lowering は ReturnValueLowererBox に委譲
//!
//! ## 生成される構造
//! main(env) 
//!   → TailCall(loop_step, env)
//! loop_step(env)
//!   → TailCall(loop_body, env)
//! loop_body(env)
//!   → <assign statements> → TailCall(k_exit, env)
//! k_exit(env)
//!   → Ret(env[x]) or TailCall(post_k, env)
//! post_k(env)  [Phase 132-P4+]
//!   → <post assign> → Ret(env[x])

pub struct LoopTrueBreakOnceBuilderBox;

impl LoopTrueBreakOnceBuilderBox {
    pub fn lower(
        step_tree: &StepTree,
        env_layout: &EnvLayout,
    ) -> Result<Option<(JoinModule, JoinFragmentMeta)>, String> {
        // 1. Extract loop(true) { body ; break } [; <post>] pattern
        let (prefix_nodes, loop_node, post_nodes) = 
            Self::extract_loop_true_pattern(&step_tree.root)?;
        
        // 2. Verify condition is Bool(true)
        let is_loop_true = Self::is_bool_true_literal(&cond_ast.0);
        if !is_loop_true {
            return Ok(None);  // Fallback
        }
        
        // 3. Generate JoinModule with continuations
        // - main → loop_step → loop_body → k_exit [→ post_k]
        // - All parameters: env (BTreeMap<String, ValueId>)
        
        // 4. Return lowering via ReturnValueLowererBox
        match ReturnValueLowererBox::lower_to_value_id(
            return_value_ast,
            &mut func.body,
            &mut next_value_id,
            &env,
        )? {
            Some(vid) => func.body.push(JoinInst::Ret { value: Some(vid) }),
            None => return Ok(None),  // Out of scope
        }
    }
}
```

**Key Points**:
- env は BTreeMap<String, ValueId> - 変数 → 値の対応
- 継続関数は全て env をパラメータとして受け取る
- Return lowering は ReturnValueLowererBox に委譲（SSOT）

---

### 2. ReturnValueLowererBox (common/return_value_lowerer_box.rs - SSOT)

```rust
//! Phase 136-138: Return 値 lowering SSOT
//! 
//! ## 責務
//! - return value (変数/リテラル/式) → ValueId に変換
//! - 任意の式に対応する準備（現在は pure expression のみ）

pub struct ReturnValueLowererBox;

impl ReturnValueLowererBox {
    pub fn lower_to_value_id(
        value_ast: &Option<AstNodeHandle>,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
        env: &BTreeMap<String, ValueId>,
    ) -> Result<Option<ValueId>, String> {
        match value_ast {
            None => Ok(Some(ValueId(0))),  // void return
            
            Some(ast_handle) => {
                match ast_handle.0.as_ref() {
                    // Variable: env lookup
                    ASTNode::Variable { name, .. } => {
                        env.get(name).copied().ok_or_else(|| {
                            // Variable not in env - out of scope
                            Ok(None)
                        })
                    }
                    
                    // Integer literal: Const generation
                    ASTNode::Literal { value: LiteralValue::Integer(i), .. } => {
                        let const_vid = ValueId(*next_value_id);
                        *next_value_id += 1;
                        body.push(JoinInst::Compute(MirLikeInst::Const {
                            dst: const_vid,
                            value: ConstValue::Integer(*i),
                        }));
                        Ok(Some(const_vid))
                    }
                    
                    // BinaryOp (Phase 137 P0: Add のみ)
                    ASTNode::BinaryOp { operator, left, right, .. } => {
                        if !matches!(operator, BinaryOperator::Add) {
                            return Ok(None);  // Out of scope
                        }
                        
                        // Lower LHS and RHS recursively
                        let lhs_vid = Self::lower_operand(left, body, next_value_id, env)?;
                        let rhs_vid = Self::lower_operand(right, body, next_value_id, env)?;
                        
                        // Generate BinOp
                        let result_vid = ValueId(*next_value_id);
                        *next_value_id += 1;
                        body.push(JoinInst::Compute(MirLikeInst::BinOp {
                            dst: result_vid,
                            op: BinOpKind::Add,
                            lhs: lhs_vid,
                            rhs: rhs_vid,
                        }));
                        
                        Ok(Some(result_vid))
                    }
                    
                    _ => Ok(None)  // Other types - out of scope
                }
            }
        }
    }
}
```

**問題点**:
- 現在は Add のみ対応（Phase 137）
- Sub, Mul, Div を追加するたびに Phase が必要
- → 一般化（Phase 140+）で `lower_operand()` を再帰的に拡張したい

---

### 3. NormalizationPlanBox (plan_box.rs - パターン検出)

```rust
//! Phase 134-138: Pattern detection SSOT
//!
//! ## 責務
//! - Block suffix が "正規化可能なパターン" か判定
//! - loop(true) のみ受け入れ（loop(i<n) は Ok(None)）
//! - post-loop statements の形を検出

pub struct NormalizationPlanBox;

impl NormalizationPlanBox {
    pub fn plan_block_suffix(
        remaining: &[ASTNode],
        ...,
    ) -> Result<Option<NormalizationPlan>, String> {
        // 1. First statement must be loop(true)
        let is_loop_true = match &remaining[0] {
            ASTNode::Loop { condition, .. } => {
                matches!(
                    condition.as_ref(),
                    ASTNode::Literal { value: LiteralValue::Bool(true), .. }
                )
            }
            _ => false,
        };
        
        if !is_loop_true {
            return Ok(None);  // Fallback: loop(i<n) など
        }
        
        // 2. Count post-loop assignments
        let mut post_assign_count = 0;
        for i in 1..remaining.len() {
            if matches!(&remaining[i], ASTNode::Assignment { .. }) {
                post_assign_count += 1;
            } else {
                break;
            }
        }
        
        // 3. After assignments, must have return
        let return_index = 1 + post_assign_count;
        if return_index >= remaining.len() {
            return Ok(None);
        }
        
        let has_return = matches!(&remaining[return_index], ASTNode::Return { .. });
        if !has_return {
            return Ok(None);
        }
        
        // 4. Return plan
        Ok(Some(NormalizationPlan {
            consumed: return_index + 1,
            kind: if post_assign_count == 0 {
                PlanKind::LoopOnly
            } else {
                PlanKind::LoopWithPost { post_assign_count }
            },
            requires_return: true,
        }))
    }
}
```

**Problem**:
- パターンが複雑になると判定ロジックが肥大化
- ネストした if などには対応困難
- → 一般化で「制御フロー構造」と「式」を分離したい

---

### 4. NormalizationExecuteBox (execute_box.rs - 実行ロジック)

```rust
//! Phase 134-138: Plan の実行

pub struct NormalizationExecuteBox;

impl NormalizationExecuteBox {
    pub fn execute(
        builder: &mut MirBuilder,
        plan: &NormalizationPlan,
        remaining: &[ASTNode],
        ...,
    ) -> Result<ValueId, String> {
        match &plan.kind {
            PlanKind::LoopOnly => {
                Self::execute_loop_only(builder, remaining, ...)
            }
            PlanKind::LoopWithPost { post_assign_count } => {
                Self::execute_loop_with_post(builder, plan, remaining, ...)
            }
        }
    }
    
    fn execute_loop_only(...) -> Result<ValueId, String> {
        // 1. Build StepTree from loop AST
        let tree = StepTreeBuilderBox::build_from_ast(&loop_ast);
        
        // 2. Lower to JoinModule (PHI-free)
        let (join_module, join_meta) =
            match StepTreeNormalizedShadowLowererBox::try_lower_if_only(
                &tree,
                &available_inputs
            )? {
                Ok(Some(result)) => result,
                Ok(None) => return Err("Out of scope".to_string()),
                Err(e) => return Err(e),
            };
        
        // 3. Merge into MIR (DirectValue mode)
        Self::merge_normalized_joinir(
            builder,
            join_module,
            join_meta,
            ...,
        )?;
        
        Ok(void_id)
    }
    
    fn merge_normalized_joinir(...) -> Result<(), String> {
        // DirectValue mode: No PHI generation
        // Exit values from join_meta → variable_map reconnection
        let boundary = JoinInlineBoundary::new_with_exit_bindings(
            vec![],
            vec![],
            exit_bindings,
        );
        boundary.exit_reconnect_mode = ExitReconnectMode::DirectValue;
        
        // Bridge JoinIR → MIR
        let mir_module = bridge_joinir_to_mir_with_meta(...)?;
        
        // Merge with boundary
        merge::merge_joinir_mir_blocks(builder, &mir_module, Some(&boundary), ...)?;
    }
}
```

**Problem**:
- Plan と Execute の責務が曖昧
- パターン検出と実行ロジックが 1:1 対応
- → Plan を「どこまで拡張するか」の判断基準が不明確

---

## 式の一般化案（Phase 140+ 向け）

### 提案: NormalizedExprLowererBox

```rust
//! Phase 140 P0: Pure expression 一般化
//!
//! ## 責務
//! - 任意の pure expression → ValueId に変換
//! - 再帰的に operand を lower

pub struct NormalizedExprLowererBox;

impl NormalizedExprLowererBox {
    pub fn lower_expr(
        ast: &ASTNode,
        body: &mut Vec<JoinInst>,
        next_value_id: &mut u32,
        env: &BTreeMap<String, ValueId>,
    ) -> Result<Option<ValueId>, String> {
        match ast {
            // Variable
            ASTNode::Variable { name, .. } => {
                env.get(name).copied().ok_or_else(|| Ok(None))
            }
            
            // Literals
            ASTNode::Literal { value: LiteralValue::Integer(i), .. } => {
                let vid = ValueId(*next_value_id);
                *next_value_id += 1;
                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: vid,
                    value: ConstValue::Integer(*i),
                }));
                Ok(Some(vid))
            }
            
            ASTNode::Literal { value: LiteralValue::Bool(b), .. } => {
                let vid = ValueId(*next_value_id);
                *next_value_id += 1;
                body.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: vid,
                    value: ConstValue::Bool(*b),
                }));
                Ok(Some(vid))
            }
            
            // BinaryOp - recursive
            ASTNode::BinaryOp { operator, left, right, .. } => {
                let lhs = Self::lower_expr(left, body, next_value_id, env)?
                    .ok_or_else(|| Ok(None))?;
                let rhs = Self::lower_expr(right, body, next_value_id, env)?
                    .ok_or_else(|| Ok(None))?;
                
                let result = ValueId(*next_value_id);
                *next_value_id += 1;
                
                let op = match operator {
                    BinaryOperator::Add => BinOpKind::Add,
                    BinaryOperator::Sub => BinOpKind::Sub,
                    BinaryOperator::Mul => BinOpKind::Mul,
                    BinaryOperator::Div => BinOpKind::Div,
                    BinaryOperator::Equal => BinOpKind::Eq,
                    BinaryOperator::Less => BinOpKind::Lt,
                    // ... other operators
                    _ => return Ok(None),  // Out of scope
                };
                
                body.push(JoinInst::Compute(MirLikeInst::BinOp {
                    dst: result,
                    op,
                    lhs,
                    rhs,
                }));
                
                Ok(Some(result))
            }
            
            // UnaryOp - recursive
            ASTNode::UnaryOp { operator, operand, .. } => {
                let operand_vid = Self::lower_expr(
                    operand,
                    body,
                    next_value_id,
                    env,
                )?
                .ok_or_else(|| Ok(None))?;
                
                let result = ValueId(*next_value_id);
                *next_value_id += 1;
                
                let op = match operator {
                    UnaryOperator::Not => UnaryOpKind::Not,
                    UnaryOperator::Minus => UnaryOpKind::Neg,
                    _ => return Ok(None),
                };
                
                body.push(JoinInst::Compute(MirLikeInst::UnaryOp {
                    dst: result,
                    op,
                    operand: operand_vid,
                }));
                
                Ok(Some(result))
            }
            
            // Call/MethodCall - Phase 141+ で対応
            ASTNode::Call { .. } | ASTNode::MethodCall { .. } => {
                Ok(None)  // Out of scope (Phase 141+)
            }
            
            _ => Ok(None)  // Out of scope
        }
    }
}
```

**利点**:
- 新しい operator を追加する際、1箇所だけ修正（総当たりにならない）
- 再帰的なので、ネストした式も自動的に対応
- Phase 141+ で Call/MethodCall を追加しても、Return lowering は変更不要

---

## 現在の Phase スケジュール

| Phase | 内容 | Status |
|-------|------|--------|
| 131 | loop(true) break-once | ✅ DONE |
| 132-133 | + post assigns | ✅ DONE |
| 134 | NormalizationPlan 統一 | ✅ DONE |
| 135 | post assigns 0個対応 | ✅ DONE |
| 136 | return literal | ✅ DONE |
| 137 | return add expr (x+2) | ✅ DONE |
| 138 | ReturnValueLowererBox SSOT | ✅ DONE |
| 139 | post_if_post_k 統一（計画中） | ⏳ |
| **140** | **ExprLowererBox 初版（pure のみ）** | 📋 提案 |
| 141+ | Call/MethodCall 対応 | 📋 提案 |

---

## 質問（ChatGPT Pro 向け）

1. **式の一般化と責務分離**
   - ExprLowererBox と MirBuilder の責務の境界は？
   - JoinIR 層でやるべきか MIR 層に押し出すべきか？

2. **PHI-free の限界**
   - PHI-free は無限に維持可能か？
   - ネストした条件式（return if(a) {x} else {y}）で PHI-free は可能か？

3. **Pattern2 との統合**
   - loop(true) break-once と loop(i < n) をどう統一するか？
   - 「canonical」戦略は正しいか？

4. **Call/MethodCall の分離**
   - pure expression と impure expression の分離は正しいか？
   - 混在する式（x + f(y)）の扱いは？

5. **セルフホスティングとの整合性**
   - JoinIR 設計は .hako 側でも再現可能か？
   - 二重実装の問題をどう解決すべきか？

---

## 参考: 現在の環境

- **Rust コンパイラ**: `src/mir/control_tree/normalized_shadow/`
- **テスト**: Phase 131-138 全て green（VM + LLVM EXE）
- **セルフホスト**: 開発中（.hako で Nyash コンパイラを実装予定）
