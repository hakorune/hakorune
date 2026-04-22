/*!
 * UnifiedCallEmitterBox - 統一Call発行専用箱
 *
 * 箱理論の実践:
 * - 箱にする: 統一Call発行ロジックを1箱に集約
 * - 境界を作る: Legacy/Unifiedの明確な分離
 * - 状態最小: MirBuilderを引数として受け取る（所有しない）
 *
 * 責務:
 * - emit_unified_call: 統一Call発行の公開API
 * - emit_unified_call_impl: コア実装（CallTarget → MirCall変換）
 * - emit_global_unified: Global関数呼び出し
 * - emit_value_unified: 第一級関数呼び出し
 */

use super::call_unified;
use super::CallTarget;
use crate::mir::builder::{Effect, EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::mir::definitions::call_unified::Callee;

/// 統一Call発行専用箱
///
/// 箱理論:
/// - 単一責務: 統一Call発行のみ（Legacy Callは別モジュール）
/// - 状態レス: MirBuilderを引数で受け取る設計
/// - ピュア関数的: 入力CallTarget → 解決・発行 → MirCall命令
pub struct UnifiedCallEmitterBox;

impl UnifiedCallEmitterBox {
    /// Unified call emission - replaces all emit_*_call methods
    /// ChatGPT5 Pro A++ design for complete call unification
    pub fn emit_unified_call(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        // Debug: Check recursion depth
        const MAX_EMIT_DEPTH: usize = 100;
        builder.recursion_depth += 1;
        if builder.recursion_depth > MAX_EMIT_DEPTH {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.error(&format!(
                "[FATAL] emit_unified_call recursion depth exceeded {}",
                MAX_EMIT_DEPTH
            ));
            ring0.log.error(&format!(
                "[FATAL] Current depth: {}",
                builder.recursion_depth
            ));
            ring0.log.error(&format!("[FATAL] Target: {:?}", target));
            return Err(format!(
                "emit_unified_call recursion depth exceeded: {}",
                builder.recursion_depth
            ));
        }

        // Check environment variable for unified call usage
        let result = if !call_unified::is_unified_call_enabled() {
            // Fall back to legacy implementation
            builder.emit_legacy_call(dst, target, args)
        } else {
            Self::emit_unified_call_impl(builder, dst, target, args)
        };
        builder.recursion_depth -= 1;
        result
    }

    fn emit_unified_call_impl(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        // Phase 287 P4: Debug trace to see what CallTarget is passed
        if crate::config::env::builder_static_call_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[P287-TRACE] emit_unified_call_impl: target={:?}, dst={:?}, args={:?}",
                target, dst, args
            ));
        }

        // Emit resolve.try for method targets (dev-only; default OFF)
        let arity_for_try = args.len();
        if let CallTarget::Method {
            ref box_type,
            ref method,
            receiver,
        } = target
        {
            let recv_cls = box_type
                .clone()
                .or_else(|| builder.type_ctx.value_origin_newbox.get(&receiver).cloned())
                .or_else(|| {
                    builder.type_ctx.value_types.get(&receiver).and_then(|t| {
                        if matches!(t, crate::mir::MirType::String) {
                            Some("StringBox".to_string())
                        } else {
                            None
                        }
                    })
                })
                .unwrap_or_default();
            // Use indexed candidate lookup (tail → names)
            let candidates: Vec<String> = builder.method_candidates(method, arity_for_try);
            let meta = serde_json::json!({
                "recv_cls": recv_cls,
                "method": method,
                "arity": arity_for_try,
                "candidates": candidates,
            });
            crate::mir::builder::observe::resolve::emit_try(builder, meta);
        }

        // Centralized user-box rewrite for method targets (toString/stringify, equals/1, Known→unique)
        if let CallTarget::Method {
            ref box_type,
            ref method,
            receiver,
        } = target
        {
            let class_name_opt = box_type
                .clone()
                .or_else(|| builder.type_ctx.value_origin_newbox.get(&receiver).cloned())
                .or_else(|| {
                    builder.type_ctx.value_types.get(&receiver).and_then(|t| {
                        if let crate::mir::MirType::Box(b) = t {
                            Some(b.clone())
                        } else if matches!(t, crate::mir::MirType::String) {
                            Some("StringBox".to_string())
                        } else {
                            None
                        }
                    })
                });
            // Early str-like
            if let Some(res) = crate::mir::builder::rewrite::special::try_early_str_like_to_dst(
                builder,
                dst,
                receiver,
                &class_name_opt,
                method,
                args.len(),
            ) {
                res?;
                return Ok(());
            }
            // equals/1
            if let Some(res) = crate::mir::builder::rewrite::special::try_special_equals_to_dst(
                builder,
                dst,
                receiver,
                &class_name_opt,
                method,
                args.clone(),
            ) {
                res?;
                return Ok(());
            }
            // Known or unique
            if let Some(res) = crate::mir::builder::rewrite::known::try_known_or_unique_to_dst(
                builder,
                dst,
                receiver,
                &class_name_opt,
                method,
                args.clone(),
            ) {
                res?;
                return Ok(());
            }
        }

        // Convert CallTarget to Callee using CalleeResolverBox
        if let CallTarget::Global(ref _n) = target { /* dev trace removed */ }
        // Fallback: if Global target is unknown, try unique static-method mapping (name/arity)
        let resolver = super::resolver::CalleeResolverBox::new(
            &builder.type_ctx.value_origin_newbox,
            &builder.type_ctx.value_types,
            Some(&builder.comp_ctx.type_registry), // 🎯 TypeRegistry を渡す
        );
        let mut callee = match resolver.resolve(target.clone()) {
            Ok(c) => c,
            Err(e) => {
                if let CallTarget::Global(ref name) = target {
                    // Try fallback handlers (via CallMaterializerBox)
                    if let Some(result) =
                        super::materializer::CallMaterializerBox::try_global_fallback_handlers(
                            builder, dst, name, &args,
                        )?
                    {
                        return Ok(result);
                    }
                }
                return Err(e);
            }
        };

        // 🎯 Phase 21.7: Methodization (HAKO_MIR_BUILDER_METHODIZE=1)
        // Convert Global("BoxName.method/arity") → Method{receiver=static singleton}
        let methodize_on = match crate::config::env::builder_methodize_mode().as_deref() {
            // 明示的に "0" が指定されたときだけ無効化。
            Some("0") => false,
            _ => true,
        };
        if methodize_on {
            if let Callee::Global(ref name) = callee {
                let name_clone = name.clone(); // Clone to avoid borrow checker issues
                                               // 🎯 Phase 21.7++ Phase 3: StaticMethodId SSOT 実装
                if let Some(id) = crate::mir::naming::StaticMethodId::parse(&name_clone) {
                    // Check if arity matches provided args (arity may be None if not specified)
                    let arity_matches = id.arity.map_or(true, |a| a == args.len());
                    if arity_matches {
                        let box_name = &id.box_name;
                        let method = &id.method;
                        let box_kind = resolver.classify_box_kind(box_name);

                        if box_kind
                            != crate::mir::definitions::call_unified::CalleeBoxKind::StaticCompiler
                        {
                            callee = Callee::Method {
                                box_name: box_name.to_string(),
                                method: method.to_string(),
                                receiver: None,
                                certainty:
                                    crate::mir::definitions::call_unified::TypeCertainty::Known,
                                box_kind,
                            };

                            if crate::config::env::builder_methodize_trace() {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!(
                                    "[methodize] Global({}) → Method{{{}.{}, recv=None}} kind={:?}",
                                    name_clone, box_name, method, box_kind
                                ));
                            }
                        } else if crate::config::env::builder_methodize_trace() {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[methodize] keep Global({}) for StaticCompiler {}.{}",
                                name_clone, box_name, method
                            ));
                        }
                    }
                }
            }
        }

        // Structural guard FIRST: prevent static compiler boxes from being called with runtime receivers
        // 箱理論: CalleeGuardBox による構造的分離
        // (Guard may convert Method → Global, so we check BEFORE materializing receiver)
        let guard = super::guard::CalleeGuardBox::new(&builder.type_ctx.value_types);
        callee = guard.apply_static_runtime_guard(callee)?;

        // Safety: ensure receiver is materialized ONLY for Method calls
        // (Global calls don't have receivers, so skip materialization)
        if matches!(callee, Callee::Method { .. }) {
            callee = super::materializer::CallMaterializerBox::materialize_receiver_in_callee(
                builder, callee,
            )?;
        }

        // Emit resolve.choose for method callee (dev-only; default OFF)
        if let Callee::Method {
            box_name,
            method,
            certainty,
            ..
        } = &callee
        {
            let chosen = format!("{}.{}{}", box_name, method, format!("/{}", arity_for_try));
            let meta = serde_json::json!({
                "recv_cls": box_name,
                "method": method,
                "arity": arity_for_try,
                "chosen": chosen,
                "certainty": format!("{:?}", certainty),
                "reason": "unified",
            });
            crate::mir::builder::observe::resolve::emit_choose(builder, meta);
        }

        // Validate call arguments
        // 箱理論: CalleeResolverBox で引数検証
        let resolver = super::resolver::CalleeResolverBox::new(
            &builder.type_ctx.value_origin_newbox,
            &builder.type_ctx.value_types,
            Some(&builder.comp_ctx.type_registry),
        );
        resolver.validate_args(&callee, &args)?;

        // Dev trace: resolved callee (static vs instance) and receiver origin
        if crate::config::env::builder_call_resolve_trace() {
            use crate::mir::definitions::call_unified::Callee;
            match &callee {
                Callee::Method {
                    box_name,
                    method,
                    receiver,
                    ..
                } => {
                    // Try to retrieve origin info for receiver
                    let recv_meta = receiver.and_then(|r| {
                        builder
                            .type_ctx
                            .value_origin_newbox
                            .get(&r)
                            .cloned()
                            .map(|cls| (r, cls))
                    });
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[call-resolve] Method box='{}' method='{}' recv={:?} recv_origin={:?} args={:?}",
                        box_name, method, receiver, recv_meta, args
                    ));
                }
                Callee::Global(name) => {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[call-resolve] Global name='{}' args={:?}",
                        name, args
                    ));
                }
                Callee::Constructor { box_type, .. } => {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[call-resolve] Constructor box='{}' args={:?}",
                        box_type, args
                    ));
                }
                Callee::Closure { .. } => {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0
                        .log
                        .debug(&format!("[call-resolve] Closure args={:?}", args));
                }
                Callee::Value(v) => {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[call-resolve] Value callee=%{:?} args={:?}",
                        v.0, args
                    ));
                }
                Callee::Extern(name) => {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[call-resolve] Extern name='{}' args={:?}",
                        name, args
                    ));
                }
            }
        }

        // Stability guard: decide route via RouterPolicyBox (behavior-preserving rules)
        if let Callee::Method {
            box_name,
            method,
            receiver: Some(r),
            certainty,
            ..
        } = &callee
        {
            let route = crate::mir::builder::router::policy::choose_route(
                box_name,
                method,
                *certainty,
                arity_for_try,
            );
            if let crate::mir::builder::router::policy::Route::BoxCall = route {
                if crate::mir::builder::utils::builder_debug_enabled()
                    || crate::config::env::builder_local_ssa_trace()
                {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[router-guard] {}.{} → BoxCall fallback (recv=%{})",
                        box_name, method, r.0
                    ));
                }
                let effects = EffectMask::READ.add(Effect::ReadHeap);
                // Prevent BoxCall helper from bouncing back into emit_unified_call
                // for the same call. RouterPolicyBox has already decided on
                // Route::BoxCall for this callee, so emit_box_or_plugin_call
                // must not re-enter the unified path even if its own heuristics
                // would otherwise choose Unified.
                let prev_flag = builder.in_unified_boxcall_fallback;
                builder.in_unified_boxcall_fallback = true;
                let res =
                    builder.emit_box_or_plugin_call(dst, *r, method.clone(), None, args, effects);
                builder.in_unified_boxcall_fallback = prev_flag;
                return res;
            }
        }

        // Finalize operands in current block (EmitGuardBox wrapper)
        let mut callee = callee;
        let mut args_local: Vec<ValueId> = args;
        crate::mir::builder::emit_guard::finalize_call_operands(
            builder,
            &mut callee,
            &mut args_local,
        )?;

        // 📦 Hotfix 7 (Phase 21.7 fixed): Include receiver in args for instance methods ONLY
        // VM's exec_function_inner expects receiver as the first parameter (ValueId(0))
        // but finalize_call_operands keeps receiver in Callee, not in args.
        // We must add it to args_local here so VM can bind it correctly.
        //
        // 🎯 Phase 21.7: static box method の receiver 追加を防止
        // - StaticCompiler box kind: コンパイル時 static box（ParserBox, StageBArgsBox等）
        // - これらは lowered function として定義され、receiver を期待しない
        // - instance method（RuntimeData/UserDefined）のみ receiver を追加
        if let Callee::Method {
            receiver: Some(recv),
            box_kind,
            box_name,
            method,
            ..
        } = &callee
        {
            use crate::mir::definitions::call_unified::CalleeBoxKind;

            // 🎯 Phase 21.7++ Phase 3: StaticMethodId による static box method 判定
            let is_static_box_method = if *box_kind == CalleeBoxKind::StaticCompiler {
                // StaticCompiler の場合、StaticMethodId でパース可能か確認
                let func_name = format!("{}.{}", box_name, method); // arity なしで試行
                crate::mir::naming::StaticMethodId::parse(&func_name).is_some()
            } else {
                false
            };

            // instance method のみ receiver を追加（static box method は追加しない）
            if !is_static_box_method {
                args_local.insert(0, *recv);
            } else if crate::config::env::builder_static_method_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[hotfix7] skipped receiver for static box method: {}.{}",
                    box_name, method
                ));
            }
        }

        // Create MirCall instruction using the new module (pure data composition)
        let mir_call = call_unified::create_mir_call(dst, callee.clone(), args_local.clone());

        // Dev trace: show final callee/recv right before emission (guarded)
        if crate::config::env::builder_local_ssa_trace()
            || crate::mir::builder::utils::builder_debug_enabled()
        {
            if let Callee::Method {
                method,
                receiver,
                box_name,
                ..
            } = &callee
            {
                if let Some(r) = receiver {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[vm-call-final] bb={:?} method={} recv=%{} class={}",
                        builder.current_block, method, r.0, box_name
                    ));
                }
            }
        }

        // Prepare annotation BEFORE moving values into instruction
        let annotation_info = if let Some(dst) = mir_call.dst {
            use super::annotation::callee_sig_name;
            let arity = match &callee {
                Callee::Method {
                    receiver: Some(recv),
                    ..
                } if args_local.first() == Some(recv) => args_local.len().saturating_sub(1),
                _ => args_local.len(),
            };
            callee_sig_name(&callee, arity).map(|func_name| (dst, func_name))
        } else {
            None
        };

        // For Phase 2: Convert to legacy Call instruction with new callee field (use finalized operands)
        let legacy_call = MirInstruction::Call {
            dst: mir_call.dst,
            func: ValueId::INVALID, // Dummy value for legacy compatibility (not a real SSA use)
            callee: Some(callee),
            args: args_local,
            effects: mir_call.effects,
        };

        let res = builder.emit_instruction(legacy_call);

        // Annotate call result with return type from module signature
        if let Some((dst, func_name)) = annotation_info {
            if crate::config::env::builder_debug_annotation() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[annotation] dst=%{} func_name={}",
                    dst.0, func_name
                ));
            }
            super::annotation::annotate_call_result_from_func_name(builder, dst, &func_name);
        }

        // Dev-only: verify block schedule invariants after emitting call
        crate::mir::builder::emit_guard::verify_after_call(builder);
        res
    }

    /// Emit global call with name constant (public for legacy compatibility)
    pub fn emit_global_unified(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        name: String,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        // Create a string constant for the function name via NameConstBox
        let name_const = crate::mir::builder::name_const::make_name_const_result(builder, &name)?;
        // Allocate a destination if not provided so we can annotate it
        let actual_dst = if let Some(d) = dst {
            d
        } else {
            builder.next_value_id()
        };
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(builder, &mut args)?;
        builder.emit_instruction(MirInstruction::Call {
            dst: Some(actual_dst),
            func: name_const,
            callee: Some(Callee::Global(name.clone())),
            args,
            effects: EffectMask::IO,
        })?;
        // Annotate from module signature (if present)
        builder.annotate_call_result_from_func_name(actual_dst, name);
        Ok(())
    }

    /// Emit value call (first-class function, public for legacy compatibility)
    pub fn emit_value_unified(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        func_val: ValueId,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(builder, &mut args)?;
        builder.emit_instruction(MirInstruction::Call {
            dst,
            func: func_val,
            callee: Some(Callee::Value(func_val)),
            args,
            effects: EffectMask::IO,
        })
    }
}
