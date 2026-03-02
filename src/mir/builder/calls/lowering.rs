//! 🎯 箱理論: 関数lowering処理 - オーケストレーター
//!
//! ## 責務
//! このモジュールは **オーケストレーター** として、関数lowering処理全体を統括する：
//! - static/instance method を MIR function に lowering する統合エントリーポイント提供
//! - 以下の専門モジュールへの処理委譲と調整を行う：
//!   - `context_lifecycle`: Context準備・復元ライフサイクル管理
//!   - `skeleton_builder`: 関数スケルトン（signature + entry block）生成
//!   - `parameter_setup`: パラメータ変数マッピング設定
//!
//! ## アーキテクチャ
//! ```text
//! lowering.rs (オーケストレーター)
//!   ├─ Step 1: context_lifecycle::prepare_lowering_context()
//!   ├─ Step 2: skeleton_builder::create_function_skeleton()
//!   ├─ Step 3: parameter_setup::setup_function_params()
//!   ├─ Step 4: lower_function_body() (本体lowering - このファイルで実装)
//!   ├─ Step 5: finalize_function() (Void return追加・型推論 - このファイルで実装)
//!   └─ Step 6: context_lifecycle::restore_lowering_context()
//! ```
//!
//! ## 設計原則
//! - **単一責任**: 各ステップは専門モジュールまたはこのファイル内の専門関数が担当
//! - **明確な境界**: Context管理・スケルトン生成・パラメータ設定は外部モジュール、本体lowering・finalize処理はこのファイル
//! - **Box理論**: BoxCompilationContext による完全独立化、型情報・変数マッピングの適切な管理

use super::function_lowering;
use crate::ast::ASTNode;
use crate::mir::builder::{MirBuilder, MirInstruction, MirType};

fn parse_declared_method_arity(func_name: &str) -> Option<usize> {
    let (_, tail) = func_name.rsplit_once('/')?;
    tail.parse::<usize>().ok()
}

fn is_constructor_name(func_name: &str) -> bool {
    func_name.contains(".birth/") || func_name.contains(".init/") || func_name.contains(".pack/")
}

fn normalize_instance_method_params(func_name: &str, mut params: Vec<String>) -> Vec<String> {
    let Some(declared_arity) = parse_declared_method_arity(func_name) else {
        return params;
    };

    if params.len() == declared_arity {
        return params;
    }

    // Defensive normalization is constructor-only.
    // Instance methods need receiver + declared args, so normalizing them here
    // can create arity regressions (e.g. run/1 declared=1 call=2).
    if is_constructor_name(func_name) && params.len() == declared_arity + 1 {
        params.remove(0);
        return params;
    }

    params
}

impl MirBuilder {
    // ============================================================================
    // Step 4: 本体lowering (Body Lowering)
    // ============================================================================

    /// 🎯 箱理論: Step 4 - 本体lowering
    ///
    /// 責務: 関数本体（static method）を MIR に lowering
    /// - StepTree capability guard 実行（strict-only）
    /// - build_expression() 経由で本体処理
    fn lower_function_body(&mut self, body: Vec<ASTNode>) -> Result<(), String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        // Phase 112: StepTree capability guard (strict-only) + dev shadow lowering
        let strict = crate::config::env::joinir_dev::strict_enabled();
        let dev = crate::config::env::joinir_dev_enabled();
        let func_name = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_string());

        struct JoinLoopTraceDevAdapter<'a> {
            trace: &'a crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace,
        }
        impl crate::mir::control_tree::normalized_shadow::dev_pipeline::DevTrace
            for JoinLoopTraceDevAdapter<'_>
        {
            fn dev(&self, tag: &str, msg: &str) {
                self.trace.dev(tag, msg)
            }
        }
        let trace_adapter = JoinLoopTraceDevAdapter { trace: &trace };

        crate::mir::control_tree::normalized_shadow::dev_pipeline::StepTreeDevPipelineBox::run(
            self,
            &body,
            &func_name,
            strict,
            dev,
            &trace_adapter,
        )?;

        trace.emit_if(
            "debug",
            "lower_function_body",
            &format!("body.len() = {}", body.len()),
            trace.is_enabled(),
        );

        let program_ast = function_lowering::wrap_in_program(body);
        trace.emit_if(
            "debug",
            "lower_function_body",
            "About to call build_expression",
            trace.is_enabled(),
        );
        let _last = self.build_expression(program_ast)?;
        trace.emit_if(
            "debug",
            "lower_function_body",
            "build_expression completed",
            trace.is_enabled(),
        );
        Ok(())
    }

    // ============================================================================
    // Step 5: 関数finalize (Function Finalization)
    // ============================================================================

    /// 🎯 箱理論: Step 5 - 関数finalize
    ///
    /// 責務: 関数の最終処理
    /// - Void return 追加（必要な場合）
    /// - 型推論（return 型が不明な場合）
    /// - Module への関数追加
    #[allow(deprecated)]
    fn finalize_function(&mut self, returns_value: bool) -> Result<(), String> {
        // Void return追加（必要な場合）
        if !returns_value {
            if let Some(ref mut f) = self.scope_ctx.current_function {
                    if let Some(block) = f.get_block(self.current_block.unwrap()) {
                        if !block.is_terminated() {
                            let void_val = crate::mir::builder::emission::constant::emit_void(self)?;
                            self.emit_instruction(MirInstruction::Return {
                                value: Some(void_val),
                            })?;
                        }
                    }
            }
        }

        // 型推論
        if let Some(ref mut f) = self.scope_ctx.current_function {
            if returns_value && matches!(f.signature.return_type, MirType::Void | MirType::Unknown)
            {
                let mut inferred: Option<MirType> = None;
                'search: for (_bid, bb) in f.blocks.iter() {
                    for inst in bb.instructions.iter() {
                        if let MirInstruction::Return { value: Some(v) } = inst {
                            if let Some(mt) = self.type_ctx.value_types.get(v).cloned() {
                                inferred = Some(mt);
                                break 'search;
                            }
                        }
                    }
                    if let Some(MirInstruction::Return { value: Some(v) }) = &bb.terminator {
                        if let Some(mt) = self.type_ctx.value_types.get(v).cloned() {
                            inferred = Some(mt);
                            break;
                        }
                    }
                }
                if let Some(mt) = inferred {
                    f.signature.return_type = mt;
                }
            }
        }

        // Moduleに追加
        // Phase 136 Step 3/7: Take from scope_ctx (SSOT)
        crate::mir::builder::emission::value_lifecycle::verify_typed_values_are_defined(
            self,
            "finalize_function",
        )?;
        let finalized = self.scope_ctx.current_function.take().unwrap();
        if let Some(ref mut module) = self.current_module {
            module.add_function(finalized);
        }

        Ok(())
    }

    // ============================================================================
    // Step 4b: 本体lowering (Method Body Lowering)
    // ============================================================================

    /// 🎯 箱理論: Step 4b - 本体lowering（instance method版: cf_block）
    ///
    /// 責務: メソッド本体（instance method）を MIR に lowering
    /// - StepTree capability guard 実行（strict-only）
    /// - cf_block() 経由で本体処理（method専用）
    fn lower_method_body(&mut self, body: Vec<ASTNode>) -> Result<(), String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let strict = crate::config::env::joinir_dev::strict_enabled();
        let dev = crate::config::env::joinir_dev_enabled();
        let func_name = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.clone())
            .unwrap_or_else(|| "<unknown>".to_string());

        struct JoinLoopTraceDevAdapter<'a> {
            trace: &'a crate::mir::builder::control_flow::joinir::trace::JoinLoopTrace,
        }
        impl crate::mir::control_tree::normalized_shadow::dev_pipeline::DevTrace
            for JoinLoopTraceDevAdapter<'_>
        {
            fn dev(&self, tag: &str, msg: &str) {
                self.trace.dev(tag, msg)
            }
        }
        let trace_adapter = JoinLoopTraceDevAdapter { trace: &trace };

        crate::mir::control_tree::normalized_shadow::dev_pipeline::StepTreeDevPipelineBox::run(
            self,
            &body,
            &func_name,
            strict,
            dev,
            &trace_adapter,
        )?;

        let _last = self.cf_block(body)?;
        Ok(())
    }

    // ============================================================================
    // 統合エントリーポイント (Unified Entry Points)
    // ============================================================================

    /// 🎯 箱理論: 統合エントリーポイント - static method lowering
    ///
    /// 6-Step オーケストレーション:
    /// 1. Context準備 (context_lifecycle)
    /// 2. 関数スケルトン作成 (skeleton_builder)
    /// 3. パラメータ設定 (parameter_setup)
    /// 4. 本体lowering (lower_function_body)
    /// 5. 関数finalize (finalize_function)
    /// 6. Context復元 (context_lifecycle)
    pub(in crate::mir::builder) fn lower_static_method_as_function(
        &mut self,
        func_name: String,
        params: Vec<String>,
        body: Vec<ASTNode>,
    ) -> Result<(), String> {
        // Phase 200-C: Store fn_body for capture analysis
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[lower_static_method_as_function] Storing fn_body with {} nodes for '{}'",
                body.len(),
                func_name
            ));
        }
        let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
            || crate::config::env::joinir_dev_enabled();
        let planner_required =
            strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
        if planner_required && !has_any_loop(&body) {
            let msg = format!("[joinir/no_plan reason=no_loop] func={}", func_name);
            if crate::config::env::joinir_dev::strict_planner_required_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
            } else if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&msg);
            }
        }
        self.comp_ctx.fn_body_ast = Some(body.clone());

        // ========================================
        // Step 1: Context準備 (context_lifecycle へ委譲)
        // ========================================
        let mut ctx = self.prepare_lowering_context(&func_name);

        // ========================================
        // Step 2: 関数スケルトン作成 (skeleton_builder へ委譲)
        // ========================================
        self.create_function_skeleton(func_name, &params, &body, &mut ctx)?;

        // ========================================
        // Step 3: パラメータ設定 (parameter_setup へ委譲)
        // ========================================
        self.setup_function_params(&params);

        // ========================================
        // Step 4: 本体lowering (このファイルで実装)
        // ========================================
        self.lower_function_body(body)?;

        // ========================================
        // Step 5: 関数finalize (このファイルで実装)
        // ========================================
        let returns_value = if let Some(ref f) = self.scope_ctx.current_function {
            !matches!(f.signature.return_type, MirType::Void)
        } else {
            false
        };
        self.finalize_function(returns_value)?;

        // FunctionRegion を 1 段ポップして元の関数コンテキストに戻るよ。
        crate::mir::region::observer::pop_function_region(self);

        // ========================================
        // Step 6: Context復元 (context_lifecycle へ委譲)
        // ========================================
        self.restore_lowering_context(ctx);

        // Phase 200-C: Clear fn_body_ast after function lowering
        self.comp_ctx.fn_body_ast = None;

        Ok(())
    }

    /// 🎯 箱理論: 統合エントリーポイント - instance method lowering
    ///
    /// 6-Step オーケストレーション (method版):
    /// 1. Context準備 (インラインで実装)
    /// 2b. メソッドスケルトン作成 (skeleton_builder)
    /// 3b. パラメータ設定 (parameter_setup - me + params)
    /// 4b. 本体lowering (lower_method_body)
    /// 5. 関数finalize (インラインで実装)
    /// 6. Context復元 (インラインで実装)
    pub(in crate::mir::builder) fn lower_method_as_function(
        &mut self,
        func_name: String,
        box_name: String,
        params: Vec<String>,
        body: Vec<ASTNode>,
    ) -> Result<(), String> {
        let params = normalize_instance_method_params(&func_name, params);
        if crate::config::env::joinir_dev::debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[lower_method_as_function] Storing fn_body with {} nodes for '{}' (box={})",
                body.len(),
                func_name,
                box_name
            ));
        }
        let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
            || crate::config::env::joinir_dev_enabled();
        let planner_required =
            strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled();
        if planner_required && !has_any_loop(&body) {
            let msg = format!("[joinir/no_plan reason=no_loop] func={}", func_name);
            if crate::config::env::joinir_dev::strict_planner_required_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
            } else if crate::config::env::joinir_dev::debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&msg);
            }
        }
        // Phase 200-C: Store fn_body for capture analysis
        self.comp_ctx.fn_body_ast = Some(body.clone());

        // ========================================
        // Step 1: Context準備 (context_lifecycle へ委譲)
        // ========================================
        let mut ctx = self.prepare_lowering_context(&func_name);

        // ========================================
        // Step 2b: メソッドスケルトン作成 (skeleton_builder へ委譲)
        // ========================================
        self.create_method_skeleton(func_name, &box_name, &params, &body, &mut ctx)?;

        // ========================================
        // Step 3b: パラメータ設定 (parameter_setup へ委譲 - me + params)
        // ========================================
        self.setup_method_params(&box_name, &params);

        // ========================================
        // Step 4b: 本体lowering (このファイルで実装 - cf_block版)
        // ========================================
        self.lower_method_body(body)?;

        // ========================================
        // Step 5: 関数finalize (このファイルで実装)
        // ========================================
        let returns_value = if let Some(ref f) = self.scope_ctx.current_function {
            !matches!(f.signature.return_type, MirType::Void)
        } else {
            false
        };

        // Void return追加（必要な場合）
        if !returns_value && !self.is_current_block_terminated() {
            let void_val = crate::mir::builder::emission::constant::emit_void(self)?;
            self.emit_instruction(MirInstruction::Return {
                value: Some(void_val),
            })?;
        }

        // 型推論（Step 5の一部として）
        if let Some(ref mut f) = self.scope_ctx.current_function {
            if returns_value && matches!(f.signature.return_type, MirType::Void | MirType::Unknown)
            {
                let mut inferred: Option<MirType> = None;
                'search: for (_bid, bb) in f.blocks.iter() {
                    for inst in bb.instructions.iter() {
                        if let MirInstruction::Return { value: Some(v) } = inst {
                            if let Some(mt) = self.type_ctx.value_types.get(v).cloned() {
                                inferred = Some(mt);
                                break 'search;
                            }
                        }
                    }
                    if let Some(MirInstruction::Return { value: Some(v) }) = &bb.terminator {
                        if let Some(mt) = self.type_ctx.value_types.get(v).cloned() {
                            inferred = Some(mt);
                            break;
                        }
                    }
                }
                if let Some(mt) = inferred {
                    f.signature.return_type = mt;
                }
            }
        }

        // Moduleに追加
        crate::mir::builder::emission::value_lifecycle::verify_typed_values_are_defined(
            self,
            "lower_method_as_function",
        )?;
        let finalized_function = self.scope_ctx.current_function.take().unwrap();
        if let Some(ref mut module) = self.current_module {
            module.add_function(finalized_function);
        }

        // FunctionRegion を 1 段ポップして元の関数コンテキストに戻るよ。
        crate::mir::region::observer::pop_function_region(self);

        // ========================================
        // Step 6: Context復元 (context_lifecycle へ委譲)
        // ========================================
        self.restore_lowering_context(ctx);

        // Phase 200-C: Clear fn_body_ast after function lowering
        self.comp_ctx.fn_body_ast = None;

        Ok(())
    }
}

// ============================================================================
// Helper Functions (ヘルパー関数)
// ============================================================================

/// ループ検出ヘルパー: 関数本体にループが含まれるか判定
///
/// planner_required モード時に「ループがない → JoinIR plan 不要」の判断に使用
fn has_any_loop(body: &[ASTNode]) -> bool {
    for stmt in body {
        if node_has_loop(stmt) {
            return true;
        }
    }
    false
}

/// ノード単位のループ検出: 再帰的にASTノードをトラバースしてループを検索
fn node_has_loop(node: &ASTNode) -> bool {
    match node {
        ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => true,
        ASTNode::ScopeBox { body, .. } => has_any_loop(body),
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            if has_any_loop(then_body) {
                return true;
            }
            if let Some(else_body) = else_body {
                return has_any_loop(else_body);
            }
            false
        }
        _ => false,
    }
}
