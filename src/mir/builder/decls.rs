// Declarations lowering: static boxes and box declarations
use super::{declaration_order::sorted_method_entries, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::slot_registry::{get_or_assign_type_id, reserve_method_slot};
use serde_json;
use std::collections::HashSet;

impl super::MirBuilder {
    /// Build static box (e.g., Main) - extracts main() method body and converts to Program
    /// Also lowers other static methods into standalone MIR functions: BoxName.method/N
    pub(super) fn build_static_main_box(
        &mut self,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, String> {
        // Lower other static methods (except main) to standalone MIR functions so JIT can see them
        for (mname, mast) in sorted_method_entries(&methods) {
            if mname == "main" {
                continue;
            }
            if let ASTNode::FunctionDeclaration {
                params,
                body,
                attrs,
                ..
            } = mast
            {
                // NamingBox 経由で static メソッド名を一元管理する
                let func_name =
                    crate::mir::naming::encode_static_method(&box_name, mname, params.len());
                self.lower_static_method_as_function(
                    func_name,
                    params.clone(),
                    body.clone(),
                    attrs.clone(),
                )?;
            }
        }
        // Within this lowering, treat `me` receiver as this static box
        let saved_static = self.comp_ctx.current_static_box.clone();
        self.comp_ctx.current_static_box = Some(box_name.clone());
        // Look for the main() method
        let out = if let Some(main_method) = methods.get("main") {
            if let ASTNode::FunctionDeclaration {
                params,
                body,
                attrs,
                ..
            } = main_method
            {
                // Optional: materialize a callable function entry "BoxName.main/N" for harness/PyVM.
                // This static entryは通常の VM 実行では使用されず、過去の Hotfix 4 絡みの loop/control-flow
                // バグの温床になっていたため、Phase 25.1m では明示トグルが立っている場合だけ生成する。
                if crate::config::env::builder_build_static_main_entry() {
                    let trace = crate::mir::builder::control_flow::joinir::trace::trace();
                    // NamingBox SSOT: Use encode_static_method for main/arity entry
                    let func_name =
                        crate::mir::naming::encode_static_method(&box_name, "main", params.len());
                    trace.stderr_if(
                        "[DEBUG] build_static_main_box: Before lower_static_method_as_function",
                        true,
                    );
                    trace.stderr_if(&format!("[DEBUG]   params.len() = {}", params.len()), true);
                    trace.stderr_if(&format!("[DEBUG]   body.len() = {}", body.len()), true);
                    trace.stderr_if(
                        &format!(
                            "[DEBUG]   variable_map = {:?}",
                            self.variable_ctx.variable_map
                        ),
                        true,
                    );
                    // Note: Metadata clearing is now handled by BoxCompilationContext (箱理論)
                    // See lifecycle.rs for context swap implementation.
                    let _ = self.lower_static_method_as_function(
                        func_name,
                        params.clone(),
                        body.clone(),
                        attrs.clone(),
                    );
                    trace.stderr_if(
                        "[DEBUG] build_static_main_box: After lower_static_method_as_function",
                        true,
                    );
                    trace.stderr_if(
                        &format!(
                            "[DEBUG]   variable_map = {:?}",
                            self.variable_ctx.variable_map
                        ),
                        true,
                    );
                }
                // Initialize local variables for Main.main() parameters
                // Note: These are local variables in the wrapper main() function, NOT parameters
                let saved_var_map = std::mem::take(&mut self.variable_ctx.variable_map);
                let script_args = collect_script_args_from_env();
                for p in params.iter() {
                    // Allocate a value ID using the current function's value generator
                    // This creates a local variable, not a parameter
                    let pid = self.next_value_id();
                    if p == "args" {
                        // new ArrayBox() with no args
                        self.emit_instruction(MirInstruction::NewBox {
                            dst: pid,
                            box_type: "ArrayBox".to_string(),
                            args: vec![],
                        })?;
                        self.type_ctx
                            .value_origin_newbox
                            .insert(pid, "ArrayBox".to_string());
                        self.type_ctx
                            .value_types
                            .insert(pid, super::MirType::Box("ArrayBox".to_string()));
                        self.emit_constructor_birth_marker(pid, "ArrayBox")?;
                        if let Some(args) = script_args.as_ref() {
                            for arg in args {
                                let val = crate::mir::builder::emission::constant::emit_string(
                                    self,
                                    arg.clone(),
                                )?;
                                self.emit_instruction(
                                    crate::mir::ssot::method_call::runtime_method_call(
                                        None,
                                        pid,
                                        "ArrayBox",
                                        "push",
                                        vec![val],
                                        super::EffectMask::MUT,
                                        crate::mir::definitions::call_unified::TypeCertainty::Known,
                                    ),
                                )?;
                            }
                        }
                    } else {
                        let v = crate::mir::builder::emission::constant::emit_void(self)?;
                        // ensure pid holds the emitted const id
                        self.emit_instruction(MirInstruction::Copy { dst: pid, src: v })?;
                        crate::mir::builder::metadata::propagate::propagate(self, v, pid);
                    }
                    self.variable_ctx.variable_map.insert(p.clone(), pid);
                    // 関数スコープ SlotRegistry にも登録しておくよ（観測専用）
                    if let Some(reg) = self.comp_ctx.current_slot_registry.as_mut() {
                        let ty = self.type_ctx.value_types.get(&pid).cloned();
                        reg.ensure_slot(p, ty);
                    }
                }
                // Phase 200-C: Store fn_body_ast for inline main() lowering
                if !self.comp_ctx.quiet_internal_logs {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[decls/fn_body_ast] Storing fn_body_ast with {} nodes for inline main()",
                        body.len()
                    ));
                }
                self.comp_ctx.fn_body_ast = Some(body.clone());
                self.set_current_function_runes(attrs);

                // Lower statements in order to preserve def→use
                let lowered = self.cf_block(body.clone());

                // Phase 200-C: Clear fn_body_ast after main() lowering
                self.comp_ctx.fn_body_ast = None;

                self.variable_ctx.variable_map = saved_var_map;
                lowered
            } else {
                Err("main method in static box is not a FunctionDeclaration".to_string())
            }
        } else {
            Err("static box must contain a main() method".to_string())
        };
        // Restore static box context
        self.comp_ctx.current_static_box = saved_static;
        out
    }

    /// Build box declaration: box Name { fields... methods... }
    pub(super) fn build_box_declaration(
        &mut self,
        name: String,
        methods: std::collections::HashMap<String, ASTNode>,
        fields: Vec<String>,
        weak_fields: Vec<String>,
    ) -> Result<(), String> {
        // Create a type registration constant (marker)
        crate::mir::builder::emission::constant::emit_string(self, format!("__box_type_{}", name))?;

        // Emit field metadata markers
        for field in fields {
            let _field_id = crate::mir::builder::emission::constant::emit_string(
                self,
                format!("__field_{}_{}", name, field),
            )?;
        }

        // Record weak fields for this box
        if !weak_fields.is_empty() {
            let set: HashSet<String> = weak_fields.into_iter().collect();
            self.comp_ctx.weak_fields_by_box.insert(name.clone(), set);
        }

        // Reserve method slots for user-defined instance methods (deterministic, starts at 4)
        let mut instance_methods: Vec<String> = Vec::new();
        for (mname, mast) in sorted_method_entries(&methods) {
            if let ASTNode::FunctionDeclaration { is_static, .. } = mast {
                if !*is_static {
                    instance_methods.push(mname.to_string());
                }
            }
        }
        if !instance_methods.is_empty() {
            let tyid = get_or_assign_type_id(&name);
            for (i, m) in instance_methods.iter().enumerate() {
                let slot = 4u16.saturating_add(i as u16);
                reserve_method_slot(tyid, m, slot);
            }
        }

        // Emit markers for declared methods (kept as metadata hints)
        for (method_name, method_ast) in sorted_method_entries(&methods) {
            if let ASTNode::FunctionDeclaration { .. } = method_ast {
                let _method_id = crate::mir::builder::emission::constant::emit_string(
                    self,
                    format!("__method_{}_{}", name, method_name),
                )?;
                // Track unified member getters: __get_<prop> | __get_once_<prop> | __get_birth_<prop>
                let kind_and_prop: Option<(super::PropertyKind, String)> =
                    if let Some(rest) = method_name.strip_prefix("__get_once_") {
                        Some((super::PropertyKind::Once, rest.to_string()))
                    } else if let Some(rest) = method_name.strip_prefix("__get_birth_") {
                        Some((super::PropertyKind::BirthOnce, rest.to_string()))
                    } else if let Some(rest) = method_name.strip_prefix("__get_") {
                        Some((super::PropertyKind::Computed, rest.to_string()))
                    } else {
                        None
                    };
                if let Some((k, prop)) = kind_and_prop {
                    use std::collections::HashMap;
                    let entry: &mut HashMap<String, super::PropertyKind> = self
                        .comp_ctx
                        .property_getters_by_box
                        .entry(name.clone())
                        .or_insert_with(HashMap::new);
                    entry.insert(prop, k);
                }
            }
        }

        Ok(())
    }
}

fn collect_script_args_from_env() -> Option<Vec<String>> {
    let raw = crate::config::env::builder_script_args_json()?;
    match serde_json::from_str::<Vec<String>>(&raw) {
        Ok(list) if !list.is_empty() => Some(list),
        _ => None,
    }
}
