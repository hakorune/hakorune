// Declarations lowering: static boxes and box declarations
use super::calls::CanonicalFunctionSessionErrorV1;
use super::main_expansion::{OwnedVerifiedMainRootLoweringV1, VerifiedMainExpansionV1};
use super::module_lifecycle::RootCallableCapturePortV1;
use super::module_lowering_invocation::ModuleLoweringPortChildErrorV1;
use super::raw_static_main_compat_batch::PreparedRawStaticMainBoxCompatibilityV1;
use super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use super::{declaration_order::sorted_method_entries, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::slot_registry::{get_or_assign_type_id, reserve_method_slot};
use serde_json;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableMainCompatibilityLoweringErrorV1 {
    Session(CanonicalFunctionSessionErrorV1),
    Child(ModuleLoweringPortChildErrorV1),
    Lowering(String),
}

impl std::fmt::Display for CallableMainCompatibilityLoweringErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Session(error) => write!(formatter, "[callable-main/session] {error}"),
            Self::Child(error) => write!(formatter, "[callable-main/child] {error}"),
            Self::Lowering(error) => write!(formatter, "[callable-main/lowering] {error}"),
        }
    }
}

impl std::error::Error for CallableMainCompatibilityLoweringErrorV1 {}

impl From<String> for CallableMainCompatibilityLoweringErrorV1 {
    fn from(error: String) -> Self {
        Self::Lowering(error)
    }
}

impl From<CanonicalFunctionSessionErrorV1> for CallableMainCompatibilityLoweringErrorV1 {
    fn from(error: CanonicalFunctionSessionErrorV1) -> Self {
        Self::Session(error)
    }
}

impl From<ModuleLoweringPortChildErrorV1> for CallableMainCompatibilityLoweringErrorV1 {
    fn from(error: ModuleLoweringPortChildErrorV1) -> Self {
        Self::Child(error)
    }
}

impl super::MirBuilder {
    /// Build static box (e.g., Main) - extracts main() method body and converts to Program
    /// Also lowers other static methods into standalone MIR functions: BoxName.method/N
    pub(super) fn build_static_main_box(
        &mut self,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, String> {
        self.build_static_main_box_typed(box_name, methods)
            .map_err(|error| error.to_string())
    }

    pub(in crate::mir::builder) fn build_static_main_box_typed(
        &mut self,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_static_main_box_with_port_v1(&mut port, box_name, methods)
    }

    pub(in crate::mir::builder) fn build_static_main_box_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1>
    where
        Port: RootCallableCapturePortV1,
    {
        PreparedRawStaticMainBoxCompatibilityV1::prepare(box_name, methods)
            .lower_with_port_v1(self, port)
    }

    pub(in crate::mir::builder) fn build_verified_static_main_box_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        main: &VerifiedMainExpansionV1<'_>,
    ) -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1>
    where
        Port: RootCallableCapturePortV1,
    {
        for child in main.static_children() {
            let (symbol, params, param_decls, return_type_name, body, uses, attrs) =
                child.to_owned_lowering().into_parts();
            port.lower_static_box_method(
                self,
                symbol,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )?;
        }
        self.lower_verified_static_main_root_with_port_v1(port, main.to_owned_root_lowering())
    }

    fn lower_verified_static_main_root_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        root: OwnedVerifiedMainRootLoweringV1,
    ) -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1>
    where
        Port: RootCallableCapturePortV1,
    {
        let (box_name, callable_symbol, params, param_decls, return_type_name, body, uses, attrs) =
            root.into_parts();
        self.lower_static_main_function_parts_with_port_v1(
            port,
            &box_name,
            callable_symbol.as_deref(),
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }

    pub(super) fn lower_static_main_function_parts_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        box_name: &str,
        verified_callable_symbol: Option<&str>,
        params: Vec<String>,
        param_decls: Vec<crate::ast::ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: crate::ast::DeclarationAttrs,
    ) -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1>
    where
        Port: RootCallableCapturePortV1,
    {
        // Within this lowering, treat `me` receiver as this static box.
        let saved_static = self.comp_ctx.current_static_box.clone();
        self.comp_ctx.current_static_box = Some(box_name.to_owned());
        let out = (|| -> Result<ValueId, CallableMainCompatibilityLoweringErrorV1> {
            // Optional: materialize a callable function entry "BoxName.main/N" for harness/PyVM.
            // This static entryは通常の VM 実行では使用されず、過去の Hotfix 4 絡みの loop/control-flow
            // バグの温床になっていたため、Phase 25.1m では明示トグルが立っている場合だけ生成する。
            if self
                .comp_ctx
                .callable_main_compatibility_policy
                .is_required()
            {
                let trace = crate::mir::builder::control_flow::joinir::trace::trace();
                // NamingBox SSOT: Use encode_static_method for main/arity entry
                let func_name = verified_callable_symbol
                    .map(str::to_owned)
                    .unwrap_or_else(|| {
                        crate::mir::naming::encode_static_method(box_name, "main", params.len())
                    });
                trace.stderr_if(
                    "[DEBUG] build_static_main_box: Before lower_static_method_as_function",
                    true,
                );
                trace.stderr_if(&format!("[DEBUG]   params.len() = {}", params.len()), true);
                trace.stderr_if(&format!("[DEBUG]   body.len() = {}", body.len()), true);
                trace.stderr_if(
                    &format!(
                        "[DEBUG]   variable_map = {:?}",
                        self.function_state.variable_ctx.variable_map
                    ),
                    true,
                );
                // Note: Metadata clearing is now handled by BoxCompilationContext (箱理論)
                // See lifecycle.rs for context swap implementation.
                port.lower_static_box_method(
                    self,
                    func_name,
                    params.clone(),
                    param_decls.clone(),
                    return_type_name.clone(),
                    body.clone(),
                    uses.clone(),
                    attrs.clone(),
                )?;
                trace.stderr_if(
                    "[DEBUG] build_static_main_box: After lower_static_method_as_function",
                    true,
                );
                trace.stderr_if(
                    &format!(
                        "[DEBUG]   variable_map = {:?}",
                        self.function_state.variable_ctx.variable_map
                    ),
                    true,
                );
            }
            // Initialize local variables for Main.main() parameters
            // Note: These are local variables in the wrapper main() function, NOT parameters
            let saved_var_map = std::mem::take(&mut self.function_state.variable_ctx.variable_map);
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
                    self.function_state
                        .type_ctx
                        .value_origin_newbox
                        .insert(pid, "ArrayBox".to_string());
                    self.function_state
                        .type_ctx
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
                self.function_state
                    .variable_ctx
                    .variable_map
                    .insert(p.clone(), pid);
                // 関数スコープ SlotRegistry にも登録しておくよ（観測専用）
                if let Some(reg) = self.comp_ctx.current_slot_registry.as_mut() {
                    let ty = self.function_state.type_ctx.value_types.get(&pid).cloned();
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
            self.function_state.compilation.fn_body_ast = Some(body.clone());
            self.set_current_function_runes(&attrs);
            self.set_current_function_declared_capability_uses(&uses);

            // Lower statements in order to preserve def→use
            let lowered = port.lower_body(self, body.clone());

            // Phase 200-C: Clear fn_body_ast after main() lowering
            self.function_state.compilation.fn_body_ast = None;

            self.function_state.variable_ctx.variable_map = saved_var_map;
            lowered.map_err(CallableMainCompatibilityLoweringErrorV1::from)
        })();
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
                self.comp_ctx
                    .register_property_getter_method(name.clone(), method_name);
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
