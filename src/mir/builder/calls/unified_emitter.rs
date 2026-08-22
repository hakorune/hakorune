/*!
 * UnifiedCallEmitterBox - 統一Call発行専用箱
 *
 * 責務:
 * - emit_unified_call: 統一Call発行の公開API
 * - emit_unified_call_impl: コア実装（CallTarget → MirCall変換）
 * - emit_global_unified: Global関数呼び出し
 * - emit_value_unified: 第一級関数呼び出し
 */

use super::call_unified;
use super::CallTarget;
use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::{Effect, EffectMask, MirBuilder, ValueId};
use crate::mir::definitions::call_unified::Callee;
use crate::mir::policies::callee_box_kind::{
    classify_callee_box_kind_v1, CalleeBoxKindPolicyContextV1,
};

/// 統一Call発行専用箱
///
/// 箱理論:
/// - 単一責務: 統一Call発行のみ（Legacy Callは別モジュール）
/// - 状態レス: MirBuilderを引数で受け取る設計
/// - ピュア関数的: 入力CallTarget → 解決・発行 → MirCall命令
pub struct UnifiedCallEmitterBox;

#[cfg(test)]
mod array_write_timing_tests;
mod compat_entrypoints;
#[cfg(test)]
mod map_write_timing_tests;
#[cfg(test)]
mod physical_receipt_tests;
mod physical_terminal;
mod post_success;
mod request_boundary;
#[cfg(test)]
mod temporal_witness_tests;

use physical_terminal::UnifiedCallEmissionOutcomeV1;
pub(in crate::mir::builder) use physical_terminal::{
    CompletedUnifiedValueCallEmissionV1, UnifiedCallAlternateRouteV1,
};
use post_success::UnifiedCallSignaturePublicationV1;
pub(in crate::mir::builder) use request_boundary::UnifiedValueCallReceiptErrorV1;
use request_boundary::{UnifiedCallAttemptErrorV1, UnifiedCompatibilityDispositionV1};

impl UnifiedCallEmitterBox {
    /// Unified call emission - replaces all emit_*_call methods
    /// ChatGPT5 Pro A++ design for complete call unification
    pub fn emit_unified_call(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        Self::emit_unified_call_with_lookup_and_map_replay(builder, dst, target, args, None, None)
    }

    /// Header-aware sibling used by invocation-owned terminals.  The lookup
    /// is never stored and the legacy facade continues to pass `None`.
    pub(in crate::mir::builder) fn emit_unified_call_with_lookup(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
    ) -> Result<(), String> {
        Self::emit_unified_call_with_lookup_and_map_replay(builder, dst, target, args, lookup, None)
    }

    /// Private BoxCall-to-Unified handoff retaining an already prepared Map
    /// semantic-source replay. No public call API gains receipt state.
    pub(in crate::mir::builder) fn emit_unified_call_with_map_replay(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        map_write_replay: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
    ) -> Result<(), String> {
        Self::emit_unified_call_with_lookup_and_map_replay(
            builder,
            dst,
            target,
            args,
            None,
            map_write_replay,
        )
    }

    fn emit_unified_call_with_lookup_and_map_replay(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
        map_write_replay: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
    ) -> Result<(), String> {
        Self::emit_unified_call_outcome_with_lookup_and_map_replay(
            builder,
            dst,
            target,
            args,
            lookup,
            map_write_replay,
            UnifiedCompatibilityDispositionV1::PermitLegacy,
            UnifiedCallSignaturePublicationV1::Existing,
        )
        .map(drop)
        .map_err(UnifiedCallAttemptErrorV1::into_ordinary_string)
    }

    fn emit_unified_call_outcome_with_lookup_and_map_replay(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
        map_write_replay: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
        compatibility: UnifiedCompatibilityDispositionV1,
        signature_publication: UnifiedCallSignaturePublicationV1,
    ) -> Result<UnifiedCallEmissionOutcomeV1, UnifiedCallAttemptErrorV1> {
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
            return Err(UnifiedCallAttemptErrorV1::Emission(format!(
                "emit_unified_call recursion depth exceeded: {}",
                builder.recursion_depth
            )));
        }

        // Check environment variable for unified call usage
        let result = if !call_unified::is_unified_call_enabled() {
            if compatibility == UnifiedCompatibilityDispositionV1::RequireGenericReceipt {
                Err(UnifiedCallAttemptErrorV1::UnifiedDisabledForReceipt)
            } else if lookup.is_some() {
                Err(UnifiedCallAttemptErrorV1::Emission(
                    "[freeze:contract][headerport/unified_call_disabled] explicit header lookup cannot retry through legacy emission"
                        .to_owned(),
                ))
            } else {
                // Use the compatibility call entry when unified calls are disabled.
                builder
                    .emit_legacy_call(dst, target, args)
                    .map(|()| {
                        UnifiedCallEmissionOutcomeV1::Alternate(
                            UnifiedCallAlternateRouteV1::LegacyCompatibility,
                        )
                    })
                    .map_err(UnifiedCallAttemptErrorV1::Emission)
            }
        } else {
            Self::emit_unified_call_outcome_impl_with_lookup_and_map_replay(
                builder,
                dst,
                target,
                args,
                lookup,
                map_write_replay,
                signature_publication,
            )
            .map_err(UnifiedCallAttemptErrorV1::Emission)
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
        Self::emit_unified_call_impl_with_lookup_and_map_replay(
            builder, dst, target, args, None, None,
        )
    }

    fn emit_unified_call_impl_with_map_replay(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        map_write_replay: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
    ) -> Result<(), String> {
        Self::emit_unified_call_impl_with_lookup_and_map_replay(
            builder,
            dst,
            target,
            args,
            None,
            map_write_replay,
        )
    }

    fn emit_unified_call_impl_with_lookup_and_map_replay(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
        map_write_replay: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
    ) -> Result<(), String> {
        Self::emit_unified_call_outcome_impl_with_lookup_and_map_replay(
            builder,
            dst,
            target,
            args,
            lookup,
            map_write_replay,
            UnifiedCallSignaturePublicationV1::Existing,
        )
        .map(drop)
    }

    fn emit_unified_call_outcome_impl_with_lookup_and_map_replay(
        builder: &mut MirBuilder,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
        map_write_replay: Option<
            crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
        >,
        signature_publication: UnifiedCallSignaturePublicationV1,
    ) -> Result<UnifiedCallEmissionOutcomeV1, String> {
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
                .or_else(|| {
                    builder
                        .function_state
                        .type_ctx
                        .value_origin_newbox
                        .get(&receiver)
                        .cloned()
                })
                .or_else(|| {
                    builder
                        .function_state
                        .type_ctx
                        .value_types
                        .get(&receiver)
                        .and_then(|t| {
                            if matches!(t, crate::mir::MirType::String) {
                                Some("StringBox".to_string())
                            } else {
                                None
                            }
                        })
                })
                .unwrap_or_default();
            // Use indexed candidate lookup (tail → names)
            let candidates: Vec<String> = lookup
                .map(|headers| {
                    crate::mir::builder::builder_method_index::method_candidates_from_headers(
                        headers,
                        method,
                        arity_for_try,
                    )
                })
                .unwrap_or_else(|| builder.method_candidates(method, arity_for_try));
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
                .or_else(|| {
                    builder
                        .function_state
                        .type_ctx
                        .value_origin_newbox
                        .get(&receiver)
                        .cloned()
                })
                .or_else(|| {
                    builder
                        .function_state
                        .type_ctx
                        .value_types
                        .get(&receiver)
                        .and_then(|t| {
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
                return Ok(UnifiedCallEmissionOutcomeV1::Alternate(
                    UnifiedCallAlternateRouteV1::EarlyStringLikeRewrite,
                ));
            }
            // equals/1
            if let Some(res) =
                crate::mir::builder::rewrite::special::try_special_equals_to_dst_with_lookup(
                    builder,
                    dst,
                    receiver,
                    &class_name_opt,
                    method,
                    args.clone(),
                    lookup,
                )
            {
                res?;
                return Ok(UnifiedCallEmissionOutcomeV1::Alternate(
                    UnifiedCallAlternateRouteV1::SpecialEqualsRewrite,
                ));
            }
            // Known or unique
            if let Some(res) =
                crate::mir::builder::rewrite::known::try_known_or_unique_to_dst_with_lookup(
                    builder,
                    dst,
                    receiver,
                    &class_name_opt,
                    method,
                    args.clone(),
                    lookup,
                )
            {
                res?;
                return Ok(UnifiedCallEmissionOutcomeV1::Alternate(
                    UnifiedCallAlternateRouteV1::KnownOrUniqueRewrite,
                ));
            }
        }

        // Convert CallTarget to Callee using CalleeResolverBox
        if let CallTarget::Global(ref _n) = target { /* dev trace removed */ }
        // If a Global target is unresolved, try the additional global resolvers.
        let resolver = super::resolver::CalleeResolverBox::new(
            &builder.function_state.type_ctx.value_origin_newbox,
            &builder.function_state.type_ctx.value_types,
            Some(&builder.comp_ctx.type_registry), // 🎯 TypeRegistry を渡す
        );
        let mut callee = match resolver.resolve(target.clone()) {
            Ok(c) => c,
            Err(e) => {
                if let CallTarget::Global(ref name) = target {
                    // Try additional resolvers (via CallMaterializerBox)
                    let authority = lookup.map_or(
                        super::materializer::GlobalPresenceAuthorityV1::LegacyCompatibility {
                            present: false,
                        },
                        super::materializer::GlobalPresenceAuthorityV1::InvocationHeader,
                    );
                    if let Some(_result) =
                        super::materializer::CallMaterializerBox::try_global_additional_resolvers_with_authority(
                            builder, dst, name, &args, authority,
                        )?
                    {
                        return Ok(UnifiedCallEmissionOutcomeV1::Alternate(
                            UnifiedCallAlternateRouteV1::AdditionalGlobalResolver,
                        ));
                    }
                }
                return Err(e);
            }
        };

        // 🎯 Phase 21.7: Methodization (HAKO_MIR_BUILDER_METHODIZE=1)
        // Convert lowered static-method globals to Method calls only for
        // runtime data boxes. User-defined/static helper boxes must stay
        // Global("Box.method/arity"), otherwise they become Method{recv=None}
        // and break both VM and LLVM routes.
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
                        let box_kind = classify_callee_box_kind_v1(
                            CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler,
                            box_name,
                        );

                        if box_kind
                            == crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData
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
                                "[methodize] keep Global({}) for non-runtime static method {}.{} kind={:?}",
                                name_clone, box_name, method, box_kind
                            ));
                        }
                    }
                }
            }
        }

        // Structural guard FIRST: prevent static compiler boxes from being called with runtime receivers
        // 箱理論: CalleeGuardBox による構造的分離
        // (Guard may convert Method → Global, so we check BEFORE materializing receiver)
        let guard = super::guard::CalleeGuardBox::new(&builder.function_state.type_ctx.value_types);
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
            receiver: Some(receiver),
            ..
        } = &callee
        {
            if box_name == "ArrayBox" {
                if builder.try_emit_known_array_method_write(dst, *receiver, method, &args)? {
                    super::super::types::array_element::observe_array_write_call(
                        builder, &callee, &args,
                    );
                    return Ok(UnifiedCallEmissionOutcomeV1::Alternate(
                        UnifiedCallAlternateRouteV1::KnownArrayWrite,
                    ));
                }
            }
        }

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
            &builder.function_state.type_ctx.value_origin_newbox,
            &builder.function_state.type_ctx.value_types,
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
                            .function_state
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
                        "[router-guard] {}.{} -> BoxCall route (recv=%{})",
                        box_name, method, r.0
                    ));
                }
                let effects = EffectMask::READ.add(Effect::ReadHeap);
                // Prevent BoxCall helper from bouncing back into emit_unified_call
                // for the same call. RouterPolicyBox has already decided on
                // Route::BoxCall for this callee, so emit_box_or_plugin_call
                // must not re-enter the unified path even if its own heuristics
                // would otherwise choose Unified.
                let prev_flag = builder.function_state.in_unified_boxcall_fallback;
                builder.function_state.in_unified_boxcall_fallback = true;
                let res =
                    builder.emit_box_or_plugin_call(dst, *r, method.clone(), None, args, effects);
                builder.function_state.in_unified_boxcall_fallback = prev_flag;
                return res.map(|()| {
                    UnifiedCallEmissionOutcomeV1::Alternate(UnifiedCallAlternateRouteV1::BoxCall)
                });
            }
        }

        // Array element facts belong to the semantic receiver, not to the
        // LocalSSA receiver copy created below. Record writes before
        // finalization so later calls on the same source value see Array<T>.
        super::super::types::array_element::observe_array_write_call(builder, &callee, &args);
        let mut map_write_replay = match map_write_replay {
            Some(mut replay) => {
                replay
                    .append_if_distinct_receiver(&callee, &args)
                    .map_err(|error| format!("[freeze:contract][map_write/replay_handoff] {error:?}"))?;
                Some(replay)
            }
            None => crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1::prepare(&callee, &args),
        };

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

        if let Some(replay) = &mut map_write_replay {
            replay
                .append_if_distinct_receiver(&callee, &args_local)
                .map_err(|error| format!("[freeze:contract][map_write/replay_final] {error:?}"))?;
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
                        builder.function_state.current_block, method, r.0, box_name
                    ));
                }
            }
        }

        physical_terminal::emit_finalized_generic_call_v1(
            builder,
            mir_call,
            map_write_replay,
            lookup,
            signature_publication,
        )
        .map(UnifiedCallEmissionOutcomeV1::Generic)
    }
}
