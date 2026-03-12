//! BoxCall emission utilities
//!
//! Phase 87: CoreMethodId-based type inference (reduced 75 lines to 25 lines)
//! Phase 84-4-B: Builtin Box method return type inference fallback
//!
//! Complex routing logic:
//! - RouterPolicyBox: Centralized route decision (Unified vs BoxCall)
//! - Type inference: plugin_method_sigs → CoreMethodId → Unknown fallback
//! - LocalSSA: Ensures receiver and args have in-block definitions

impl super::super::MirBuilder {
    /// Phase 87: BoxCall のメソッド戻り値型を推論（CoreMethodId ベース）
    ///
    /// 責務: ビルトイン Box のメソッド戻り値型を型安全に返す
    /// - Phase 84-4-B のハードコード (75行) を CoreMethodId で統合 (25行に削減)
    /// - plugin_method_sigs に登録されていないメソッドの型推論
    /// - PhiTypeResolver が依存する base 定義の型情報を提供
    fn infer_boxcall_return_type(
        &self,
        box_val: super::super::ValueId,
        method: &str,
    ) -> Option<super::super::MirType> {
        use crate::runtime::{CoreBoxId, CoreMethodId};

        // 1. box_val の型を取得
        let box_ty = self.type_ctx.value_types.get(&box_val)?;

        // 2. Box 型名を取得
        let box_name = match box_ty {
            super::super::MirType::Box(name) => name.as_str(),
            super::super::MirType::String => "StringBox", // String → StringBox として扱う
            _ => return None,
        };

        // 3. Phase 87: CoreBoxId/CoreMethodId による型安全な型推論
        let box_id = CoreBoxId::from_name(box_name)?;
        let method_id = CoreMethodId::from_box_and_method(box_id, method);

        if let Some(method_id) = method_id {
            // CoreMethodId で定義されたメソッドの戻り値型
            let type_name = method_id.return_type_name();
            return Some(match type_name {
                "StringBox" => super::super::MirType::Box("StringBox".to_string()),
                "IntegerBox" => super::super::MirType::Box("IntegerBox".to_string()),
                "BoolBox" => super::super::MirType::Box("BoolBox".to_string()),
                "ArrayBox" => super::super::MirType::Box("ArrayBox".to_string()),
                "FileBox" => super::super::MirType::Box("FileBox".to_string()),
                "Void" => super::super::MirType::Void,
                "Unknown" => super::super::MirType::Unknown,
                _ => super::super::MirType::Unknown,
            });
        }

        // 4. CoreMethodId で未定義のメソッド（Stage1Cli 等の特殊 Box）
        if box_name == "Stage1CliBox" && matches!(method, "parse" | "compile" | "execute") {
            return Some(super::super::MirType::Unknown);
        }

        // 5. Result-like Box の汎用メソッド（QMark 用）
        if method == "isOk" {
            return Some(super::super::MirType::Box("BoolBox".to_string()));
        }
        if method == "getValue" {
            return Some(super::super::MirType::Unknown); // Result<T> の T
        }

        // 6. 未知のメソッド → Unknown として登録（None を返すとPhiTypeResolverが使えない）
        if crate::config::env::builder_boxcall_type_debug() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[boxcall_type] unknown method {}.{} → Unknown",
                box_name, method
            ));
        }
        Some(super::super::MirType::Unknown)
    }

    /// Emit a Box method call or plugin call (unified BoxCall)
    pub(in crate::mir::builder) fn emit_box_or_plugin_call(
        &mut self,
        dst: Option<super::super::ValueId>,
        box_val: super::super::ValueId,
        method: String,
        _method_id: Option<u16>,
        args: Vec<super::super::ValueId>,
        effects: super::super::EffectMask,
    ) -> Result<(), String> {
        // Ensure receiver has a definition in the current block to avoid undefined use across
        // block boundaries (LoopForm/header, if-joins, etc.).
        // LocalSSA: ensure receiver has an in-block definition (kind=0 = recv)
        let box_val = self.local_recv(box_val);
        // LocalSSA: finalize args (strict/dev+planner_required uses try_ensure; release is unchanged)
        let mut args = args;
        crate::mir::builder::ssa::local::finalize_args(self, &mut args)?;
        // Check environment variable for unified call usage, with safe overrides for core/user boxes
        let use_unified_env = super::super::calls::call_unified::is_unified_call_enabled();
        // First, try to determine the box type
        let mut box_type: Option<String> = self.type_ctx.value_origin_newbox.get(&box_val).cloned();
        if box_type.is_none() {
            if let Some(t) = self.type_ctx.value_types.get(&box_val) {
                match t {
                    super::super::MirType::String => box_type = Some("StringBox".to_string()),
                    super::super::MirType::Box(name) => box_type = Some(name.clone()),
                    _ => {}
                }
            }
        }
        // Route decision is centralized in RouterPolicyBox（仕様不変）。
        let bx_name = box_type.clone().unwrap_or_else(|| "UnknownBox".to_string());
        let route = crate::mir::builder::router::policy::choose_route(
            &bx_name,
            &method,
            crate::mir::definitions::call_unified::TypeCertainty::Union,
            args.len(),
        );
        if super::builder_debug_enabled() || crate::config::env::builder_local_ssa_trace() {
            if matches!(
                method.as_str(),
                "parse" | "substring" | "has_errors" | "length"
            ) {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[boxcall-decision] method={} bb={:?} recv=%{} class_hint={:?} prefer_legacy={}",
                    method,
                    self.current_block,
                    box_val.0,
                    box_type,
                    matches!(route, crate::mir::builder::router::policy::Route::BoxCall)
                ));
            }
        }
        // Unified path from BoxCall helper is only allowed when we are not
        // already in a BoxCall fallback originating from emit_unified_call.
        // in_unified_boxcall_fallback is set by emit_unified_call's RouterPolicy
        // guard when it has already decided that this call must be a BoxCall.
        if use_unified_env
            && matches!(route, crate::mir::builder::router::policy::Route::Unified)
            && !self.in_unified_boxcall_fallback
        {
            let target = super::super::builder_calls::CallTarget::Method {
                box_type,
                method: method.clone(),
                receiver: box_val,
            };
            return self.emit_unified_call(dst, target, args);
        }

        // Canonical implementation (RCL-3-min3): emit Call(callee=Method)
        let certainty = if box_type.is_some() {
            crate::mir::definitions::call_unified::TypeCertainty::Known
        } else {
            crate::mir::definitions::call_unified::TypeCertainty::Union
        };
        let box_name_for_call = box_type
            .clone()
            .unwrap_or_else(|| "RuntimeDataBox".to_string());
        let box_kind =
            crate::mir::builder::calls::call_unified::classify_box_kind(&box_name_for_call);
        self.emit_instruction(super::super::MirInstruction::Call {
            dst,
            func: super::super::ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: box_name_for_call,
                method: method.clone(),
                receiver: Some(box_val),
                certainty,
                box_kind,
            }),
            args,
            effects,
        })?;
        if let Some(d) = dst {
            let mut recv_box: Option<String> =
                self.type_ctx.value_origin_newbox.get(&box_val).cloned();
            if recv_box.is_none() {
                if let Some(t) = self.type_ctx.value_types.get(&box_val) {
                    match t {
                        super::super::MirType::String => recv_box = Some("StringBox".to_string()),
                        super::super::MirType::Box(name) => recv_box = Some(name.clone()),
                        _ => {}
                    }
                }
            }
            if let Some(bt) = recv_box {
                if let Some(mt) = self
                    .comp_ctx
                    .plugin_method_sigs
                    .get(&(bt.clone(), method.clone()))
                {
                    self.type_ctx.value_types.insert(d, mt.clone());
                } else {
                    // Phase 84-4-B: ビルトイン Box のメソッド戻り値型推論
                    // plugin_method_sigs に登録されていない場合のフォールバック
                    if let Some(ret_ty) = self.infer_boxcall_return_type(box_val, &method) {
                        self.type_ctx.value_types.insert(d, ret_ty.clone());

                        if crate::config::env::builder_boxcall_type_trace() {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[boxcall_type] registered %{} = BoxCall(%{}, {}) → {:?}",
                                d.0, box_val.0, method, ret_ty
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
