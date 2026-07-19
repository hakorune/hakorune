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
//!   ├─ function_session::with_function_lowering_session()
//!   ├─ Step 2: skeleton_builder::create_function_skeleton()
//!   ├─ Step 3: parameter_setup::setup_function_params()
//!   ├─ Step 4: lower_function_body() (本体lowering - このファイルで実装)
//!   ├─ Step 5: finalize_function_draft() (未公開 draft を返す)
//!   └─ session close: caller restore → module commit
//! ```
//!
//! ## 設計原則
//! - **単一責任**: 各ステップは専門モジュールまたはこのファイル内の専門関数が担当
//! - **明確な境界**: Context管理・スケルトン生成・パラメータ設定は外部モジュール、本体lowering・finalize処理はこのファイル
//! - **Box理論**: BoxCompilationContext による完全独立化、型情報・変数マッピングの適切な管理

use super::function_lowering;
use crate::ast::{ASTNode, ParamDecl};
use crate::mir::builder::{MirBuilder, MirFunction, MirInstruction, MirType};
use crate::mir::function::MirParamDecl;

#[cfg(test)]
use crate::mir::builder::stmts::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};

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

fn normalize_instance_method_param_decls(
    func_name: &str,
    mut param_decls: Vec<ParamDecl>,
) -> Vec<ParamDecl> {
    let Some(declared_arity) = parse_declared_method_arity(func_name) else {
        return param_decls;
    };

    if param_decls.len() == declared_arity {
        return param_decls;
    }

    if is_constructor_name(func_name) && param_decls.len() == declared_arity + 1 {
        param_decls.remove(0);
        return param_decls;
    }

    param_decls
}

fn mir_param_decls_from_source(params: &[String], param_decls: &[ParamDecl]) -> Vec<MirParamDecl> {
    ParamDecl::with_name_fallback(param_decls, params)
        .iter()
        .map(|decl| MirParamDecl {
            name: decl.name.clone(),
            declared_type_name: decl.declared_type_name.clone(),
            implicit_receiver: false,
        })
        .collect()
}

fn mir_method_param_decls_from_source(
    _box_name: &str,
    params: &[String],
    param_decls: &[ParamDecl],
) -> Vec<MirParamDecl> {
    let mut decls = Vec::with_capacity(params.len() + 1);
    decls.push(MirParamDecl {
        name: "me".to_string(),
        declared_type_name: None,
        implicit_receiver: true,
    });
    decls.extend(mir_param_decls_from_source(params, param_decls));
    decls
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
    pub(super) fn lower_function_body(&mut self, body: Vec<ASTNode>) -> Result<(), String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        // Phase 112: StepTree capability guard (strict-only) + dev shadow lowering
        let strict = crate::config::env::joinir_dev::strict_enabled();
        let dev = crate::config::env::joinir_dev_enabled();
        let func_name = self
            .function_state
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
    /// - session へ未公開 `MirFunction` draft を返す
    #[allow(deprecated)]
    pub(in crate::mir::builder) fn finalize_function_draft(
        &mut self,
        returns_value: bool,
    ) -> Result<MirFunction, String> {
        // Void return追加（必要な場合）
        if !returns_value {
            if let Some(ref mut f) = self.function_state.current_function {
                if let Some(block) = f.get_block(self.function_state.current_block.unwrap()) {
                    if !block.is_terminated() {
                        let void_val = crate::mir::builder::emission::constant::emit_void(self)?;
                        self.emit_instruction(MirInstruction::Return {
                            value: Some(void_val),
                        })?;
                    }
                }
            }
        }

        if let Some(ref mut f) = self.function_state.current_function {
            use crate::mir::type_propagation::TypePropagationPipeline;
            TypePropagationPipeline::run(f, &mut self.function_state.type_ctx.value_types)?;
        }

        if let (Some(function), Some(module)) = (
            self.function_state.current_function.as_ref(),
            self.current_module.as_ref(),
        ) {
            crate::mir::builder::type_hint_providers::annotate_missing_result_types_from_calls_and_await(
                &mut self.function_state.type_ctx, function, module,
            );
        }

        let origin_caller_rows = self.value_origin_caller_rows();

        // 型推論
        if let Some(ref mut f) = self.function_state.current_function {
            if returns_value && matches!(f.signature.return_type, MirType::Void | MirType::Unknown)
            {
                let mut inferred: Option<MirType> = None;
                'search: for (_bid, bb) in f.blocks.iter() {
                    for inst in bb.instructions.iter() {
                        if let MirInstruction::Return { value: Some(v) } = inst {
                            if let Some(mt) =
                                self.function_state.type_ctx.value_types.get(v).cloned()
                            {
                                inferred = Some(mt);
                                break 'search;
                            }
                        }
                    }
                    if let Some(MirInstruction::Return { value: Some(v) }) = &bb.terminator {
                        if let Some(mt) = self.function_state.type_ctx.value_types.get(v).cloned() {
                            inferred = Some(mt);
                            break;
                        }
                    }
                }
                if let Some(mt) = inferred {
                    f.signature.return_type = mt;
                }
            }

            // Keep per-function metadata complete before the function enters the
            // module so later canonicalization sees the same receiver facts on
            // direct-lowered instance methods as it does on main.
            f.metadata.value_types = self.function_state.type_ctx.value_types.clone();
            let mut origin_callers = f.metadata.value_origin_callers.clone();
            for (k, v) in &origin_caller_rows {
                origin_callers.insert(*k, v.clone());
            }
            f.metadata.value_origin_callers = origin_callers;
        }

        // Keep the draft unpublished until the function session restores and
        // verifies the caller context.
        crate::mir::builder::emission::value_lifecycle::verify_typed_values_are_defined(
            self,
            "finalize_function_draft",
        )?;
        self.function_state.current_function.take().ok_or_else(|| {
            "[freeze:contract][canonical_function_session/finalize_without_draft]".to_string()
        })
    }

    // ============================================================================
    // Step 4b: 本体lowering (Method Body Lowering)
    // ============================================================================

    /// 🎯 箱理論: Step 4b - 本体lowering（instance method版: cf_block）
    ///
    /// 責務: メソッド本体（instance method）を MIR に lowering
    /// - StepTree capability guard 実行（strict-only）
    /// - cf_block() 経由で本体処理（method専用）
    pub(super) fn lower_method_body(&mut self, body: Vec<ASTNode>) -> Result<(), String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        let strict = crate::config::env::joinir_dev::strict_enabled();
        let dev = crate::config::env::joinir_dev_enabled();
        let func_name = self
            .function_state
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
    /// Function session owns prepare, rollback, and post-cleanup publication.
    pub(in crate::mir::builder) fn lower_static_method_as_function(
        &mut self,
        func_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: crate::ast::DeclarationAttrs,
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
        let session_name = func_name.clone();
        self.with_function_lowering_session(&session_name, body.clone(), move |builder| {
            builder.create_function_skeleton(func_name, &params, &body)?;
            builder.set_current_function_declared_signature(
                mir_param_decls_from_source(&params, &param_decls),
                return_type_name,
            );
            builder.set_current_function_runes(&attrs);
            builder.set_current_function_declared_capability_uses(&uses);
            builder.setup_function_params(&params)?;
            builder.lower_function_body(body)?;

            let returns_value = builder
                .function_state
                .current_function
                .as_ref()
                .is_some_and(|function| !matches!(function.signature.return_type, MirType::Void));
            builder.finalize_function_draft(returns_value)
        })
    }

    /// 🎯 箱理論: 統合エントリーポイント - instance method lowering
    ///
    /// The same function session owns instance-method cleanup and publication.
    pub(in crate::mir::builder) fn lower_method_as_function(
        &mut self,
        func_name: String,
        box_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: crate::ast::DeclarationAttrs,
    ) -> Result<(), String> {
        let params = normalize_instance_method_params(&func_name, params);
        let param_decls = normalize_instance_method_param_decls(&func_name, param_decls);
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
        let session_name = func_name.clone();
        self.with_function_lowering_session(&session_name, body.clone(), move |builder| {
            builder.create_method_skeleton(func_name, &box_name, &params, &body)?;
            builder.set_current_function_declared_signature(
                mir_method_param_decls_from_source(&box_name, &params, &param_decls),
                return_type_name,
            );
            builder.set_current_function_runes(&attrs);
            builder.set_current_function_declared_capability_uses(&uses);
            builder.setup_method_params(&box_name, &params)?;
            builder.lower_method_body(body)?;

            let returns_value = builder
                .function_state
                .current_function
                .as_ref()
                .is_some_and(|function| !matches!(function.signature.return_type, MirType::Void));
            builder.finalize_function_draft(returns_value)
        })
    }

    /// Replays one exact instance-method entry and hands off its live root
    /// block before one body suffix. This is test-only support for raw/located
    /// parity: it deliberately reuses the production skeleton, signature, and
    /// parameter publishers instead of synthesizing parameter facts.
    #[cfg(test)]
    pub(in crate::mir) fn lower_instance_method_prefix_for_test<R>(
        &mut self,
        box_name: &str,
        declaration: ASTNode,
        prefix_len: usize,
        continuation: impl for<'suffix> FnOnce(
            &mut MirBuilder,
            &'suffix [ASTNode],
        ) -> Result<(crate::mir::ValueId, R), String>,
    ) -> Result<R, String> {
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            is_static,
            ..
        } = declaration
        else {
            return Err(
                "[freeze:contract][raw_prefix_harness/not_function_declaration]".to_string(),
            );
        };
        if is_static {
            return Err("[freeze:contract][raw_prefix_harness/static_method]".to_string());
        }
        if prefix_len >= body.len() {
            return Err(format!(
                "[freeze:contract][raw_prefix_harness/invalid_prefix] prefix_len={} body_len={}",
                prefix_len,
                body.len()
            ));
        }

        let func_name = format!("{box_name}.{name}/{}", params.len());
        let params = normalize_instance_method_params(&func_name, params);
        let param_decls = normalize_instance_method_param_decls(&func_name, param_decls);
        let session_name = func_name.clone();
        let body_snapshot = body.clone();
        let mut observed = None;

        self.with_function_lowering_session(&session_name, body_snapshot, |builder| {
            builder.create_method_skeleton(func_name, box_name, &params, &body)?;
            builder.set_current_function_declared_signature(
                mir_method_param_decls_from_source(box_name, &params, &param_decls),
                return_type_name,
            );
            builder.set_current_function_runes(&attrs);
            builder.set_current_function_declared_capability_uses(&uses);
            builder.setup_method_params(box_name, &params)?;

            let mut continuation = Some(continuation);
            let mut handoff = |builder: &mut MirBuilder, suffix: &[ASTNode]| {
                let continuation = continuation.take().ok_or_else(|| {
                    "[freeze:contract][raw_prefix_harness/continuation_reentered]".to_string()
                })?;
                let (last_value, result) = continuation(builder, suffix)?;
                observed = Some(result);
                Ok(last_value)
            };
            let mut port = InstanceMethodPrefixPortV1 {
                body: &body,
                prefix_len,
                continuation: &mut handoff,
            };
            drive_legacy_block_v1(builder, &mut port)?;

            let returns_value = builder
                .function_state
                .current_function
                .as_ref()
                .is_some_and(|function| !matches!(function.signature.return_type, MirType::Void));
            builder.finalize_function_draft(returns_value)
        })?;

        observed.ok_or_else(|| {
            "[freeze:contract][raw_prefix_harness/continuation_not_called]".to_string()
        })
    }
}

#[cfg(test)]
struct InstanceMethodPrefixPortV1<'body, 'callback> {
    body: &'body [ASTNode],
    prefix_len: usize,
    continuation: &'callback mut dyn FnMut(
        &mut MirBuilder,
        &[ASTNode],
    ) -> Result<crate::mir::ValueId, String>,
}

#[cfg(test)]
impl LegacyBlockDescentPortV1 for InstanceMethodPrefixPortV1<'_, '_> {
    type SuffixInput<'a>
        = &'a [ASTNode]
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.prefix_len + 1
    }

    fn suffix_route_input(&self, index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        Ok((index < self.prefix_len).then_some(&self.body[index..]))
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<crate::mir::ValueId, String> {
        if index < self.prefix_len {
            return crate::mir::builder::stmts::block_stmt::build_statement(
                builder,
                self.body[index].clone(),
            );
        }
        (self.continuation)(builder, &self.body[self.prefix_len..])
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
        ASTNode::Loop { .. } | ASTNode::LoopRange { .. } => true,
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
