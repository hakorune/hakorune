//! Parameter setup and binding (static/instance methods)
//!
//! 責務:
//! - static method のパラメータ設定（setup_function_params）
//! - instance method のパラメータ設定（setup_method_params）
//! - "me" パラメータの特別処理
//! - SlotRegistry への登録

use crate::mir::builder::MirBuilder;
use crate::mir::builder::MirType;
use hakorune_mir_core::{MirValueKind, ValueId};

impl MirBuilder {
    /// 🎯 箱理論: Step 3 - パラメータ設定
    /// Phase 269 P1.2: static call 正規化により、static box では "me" の擬似初期化を行わない（receiver は compile-time で確定）
    #[allow(deprecated)]
    pub(super) fn setup_function_params(&mut self, params: &[String]) {
        // Phase 136 Step 3/7: Clear scope_ctx (SSOT)
        self.scope_ctx.function_param_names.clear();
        // SlotRegistry 更新は borrow 競合を避けるため、まずローカルに集約してから反映するよ。
        let mut slot_regs: Vec<(String, Option<MirType>)> = Vec::new();
        // Phase 26-A-3: パラメータ型情報も後で一括登録（借用競合回避）
        let mut param_kinds: Vec<(ValueId, u32)> = Vec::new();

        if let Some(ref mut f) = self.scope_ctx.current_function {
            // 📦 Hotfix 5: Use pre-populated params from MirFunction::new()
            // Static methods have implicit receiver at params[0], so actual parameters start at offset
            let receiver_offset = if f.params.is_empty() {
                0
            } else {
                // If params already populated (by Hotfix 4+5), use them
                if f.params.len() > params.len() {
                    1
                } else {
                    0
                }
            };

            let param_types = f.signature.params.clone();

            for (idx, p) in params.iter().enumerate() {
                let param_idx = receiver_offset + idx;
                let pid = if param_idx < f.params.len() {
                    // Use pre-allocated ValueId from MirFunction::new()
                    f.params[param_idx]
                } else {
                    // Allocate new ValueId (fallback for non-static methods)
                    let new_pid = f.next_value_id();
                    f.params.push(new_pid);
                    new_pid
                };
                self.variable_ctx.variable_map.insert(p.clone(), pid);
                // Phase 136 Step 3/7: Insert into scope_ctx (SSOT)
                self.scope_ctx.function_param_names.insert(p.clone());

                // Phase 26-A-3: パラメータ型情報を収集（後で一括登録）
                // param_idx: receiver offset を考慮した実際のパラメータインデックス
                param_kinds.push((pid, param_idx as u32));

                let ty = param_types.get(param_idx).cloned();
                slot_regs.push((p.clone(), ty));
            }
        }

        // Phase 26-A-3: パラメータ型情報を一括登録（GUARD Bug Prevention）
        for (pid, param_idx) in param_kinds {
            self.register_value_kind(pid, MirValueKind::Parameter(param_idx));
        }

        if let Some(reg) = self.comp_ctx.current_slot_registry.as_mut() {
            for (name, ty) in slot_regs {
                reg.ensure_slot(&name, ty);
            }
        }
    }

    /// 🎯 箱理論: Step 3b - パラメータ設定（instance method版: me + params）
    pub(super) fn setup_method_params(&mut self, box_name: &str, params: &[String]) {
        // SlotRegistry 更新はローカルバッファに集約してから反映するよ。
        let mut slot_regs: Vec<(String, Option<MirType>)> = Vec::new();
        let mut param_kinds: Vec<(ValueId, u32)> = Vec::new();
        let me_type = MirType::Box(box_name.to_string());

        if let Some(ref mut f) = self.scope_ctx.current_function {
            // 📦 Hotfix 6 改訂版:
            // MirFunction::new() が既に 0..N の ValueId を params 用に予約しているので、
            // ここではそれを「上書き使用」するだけにして、push で二重定義しないようにするよ。
            //
            // params レイアウト:
            //   index 0: me (box<MyBox>)
            //   index 1..: 通常パラメータ
            if f.params.is_empty() {
                // 安全弁: 何らかの理由で pre-populate されていない場合は従来どおり new する
                let me_id = ValueId(0);
                f.params.push(me_id);
                for i in 0..params.len() {
                    f.params.push(ValueId((i + 1) as u32));
                }
            }

            // me
            let me_id = f.params[0];
            self.variable_ctx
                .variable_map
                .insert("me".to_string(), me_id);
            param_kinds.push((me_id, 0));
            self.type_ctx.value_types.insert(me_id, me_type.clone());
            self.type_ctx
                .value_origin_newbox
                .insert(me_id, box_name.to_string());
            slot_regs.push(("me".to_string(), Some(me_type.clone())));

            // 通常パラメータ
            for (idx, p) in params.iter().enumerate() {
                let param_idx = idx + 1;
                if param_idx < f.params.len() {
                    let pid = f.params[param_idx];
                    self.variable_ctx.variable_map.insert(p.clone(), pid);
                    param_kinds.push((pid, param_idx as u32));
                    slot_regs.push((p.clone(), None));
                } else {
                    // 念のため足りない場合は新規に確保（互換用）
                    let pid = f.next_value_id();
                    f.params.push(pid);
                    self.variable_ctx.variable_map.insert(p.clone(), pid);
                    param_kinds.push((pid, param_idx as u32));
                    slot_regs.push((p.clone(), None));
                }
            }
        }

        for (pid, param_idx) in param_kinds {
            self.register_value_kind(pid, MirValueKind::Parameter(param_idx));
        }

        if let Some(reg) = self.comp_ctx.current_slot_registry.as_mut() {
            for (name, ty) in slot_regs {
                reg.ensure_slot(&name, ty);
            }
        }
    }
}
