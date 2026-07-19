//! 🎯 箱理論: Call構築 Orchestrator (Refactored 755→311 lines)
//!
//! # 責務
//! ASTからCall構築の統合制御（orchestration only, no implementation）
//! - build_function_call: 関数呼び出し構築
//! - build_method_call: メソッド呼び出し構築
//! - build_from_expression: from式構築
//!
//! # Delegation Strategy (実装は専用モジュールへ委譲)
//! - `debug_method_routing`: Debug tracing（179 lines）
//! - `function_preflight`: source-level special call gate
//! - `special_method_handlers`: Special method detection（122 lines）
//! - `static_resolution`: Static receiver resolution（182 lines）
//! - `receiver_binding`: Receiver normalization（54 lines）
//!
//! # Refactoring History
//! - Before: 755 lines monolithic implementation
//! - After: 311 lines orchestrator + 4 extracted modules (537 lines total)
//! - Net reduction: -444 lines of complexity in build.rs

use super::super::{Effect, EffectMask, MirBuilder, MirInstruction, ValueId};
#[allow(unused_imports)]
use super::debug_method_routing::*;
use super::static_resolution::BareStaticRecoveryEmissionV1;
use super::CallTarget;
use crate::ast::ASTNode;
use crate::mir::builder::callable_declaration_catalog::BareStaticRecoveryNoRecoveryReasonV1;

impl MirBuilder {
    // Build function call: name(args)
    pub fn build_function_call(
        &mut self,
        name: String,
        args: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        // Dev trace
        if crate::config::env::cli_verbose() {
            let cur_fun = self
                .function_state
                .current_function
                .as_ref()
                .map(|f| f.signature.name.clone())
                .unwrap_or_else(|| "<none>".to_string());
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[builder] function-call name={} static_ctx={} in_fn={}",
                name,
                self.comp_ctx.current_static_box.as_deref().unwrap_or(""),
                cur_fun
            ));
        }

        if let Some(result) = self.try_handle_function_preflight(&name, &args)? {
            return Ok(result);
        }

        if let Some(region) = self.current_fastmem_region() {
            if name.starts_with("mem.") {
                return crate::mir::builder::fastmem::calls::lower_fastmem_function_call(
                    self, region, name, args,
                );
            }
        }

        // 1. Build argument values
        let arg_values = self.build_call_args(&args)?;

        // 2. Special-case: global str(x) → x.str() normalization
        if name == "str" && arg_values.len() == 1 {
            return self.build_str_normalization(arg_values[0]);
        }

        // 3. Determine call route (unified builtin/extern vs resolved global)
        let use_unified = super::call_unified::is_unified_call_enabled()
            && (super::super::call_resolution::is_builtin_function(&name)
                || super::super::call_resolution::is_extern_function(&name));

        if !use_unified {
            self.build_resolved_function_call(name, arg_values)
        } else {
            self.build_unified_function_call(name, arg_values)
        }
    }

    // Build method call: object.method(arguments)
    pub fn build_method_call(
        &mut self,
        object: ASTNode,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let input =
            super::method_call_descent::RawLegacyMethodCallInputV1::new(object, method, arguments);
        let mut port = super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;
        self.build_method_call_from_input_v1(&mut port, &input)
    }

    pub(in crate::mir::builder) fn build_method_call_from_input_v1<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
    ) -> Result<ValueId, String>
    where
        Port: super::method_call_terminal::MethodCallValueTerminalPortV1,
    {
        let typeop = {
            let syntax = port.method_call_syntax(input)?;
            super::special_handlers::is_typeop_method(syntax.method(), syntax.arguments())
                .map(|type_name| (syntax.method().to_string(), type_name))
        };
        if let Some((method, type_name)) = typeop {
            let object_value =
                super::method_call_descent::lower_method_call_receiver_v1(self, port, input)?;
            let mut completion =
                super::method_call_descent::AssociatedMethodCallArgumentsV1::new(port, input);
            return self.handle_typeop_method_with_terminal(
                object_value,
                &method,
                &type_name,
                &mut completion,
            );
        }

        // Capture syntax before incrementing so syntax errors cannot alter entry depth.
        let method = port.method_call_syntax(input)?.method().to_string();

        // Debug: Check recursion depth
        const MAX_METHOD_DEPTH: usize = 100;
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_METHOD_DEPTH {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.error(&format!(
                "[FATAL] build_method_call recursion depth exceeded {}",
                MAX_METHOD_DEPTH
            ));
            ring0
                .log
                .error(&format!("[FATAL] Current depth: {}", self.recursion_depth));
            ring0.log.error(&format!("[FATAL] Method: {}", method));
            return Err(format!(
                "build_method_call recursion depth exceeded: {}",
                self.recursion_depth
            ));
        }

        let result = self.build_method_call_impl(port, input);
        self.recursion_depth -= 1;
        result
    }

    fn build_method_call_impl<Port>(
        &mut self,
        port: &mut Port,
        input: &Port::MethodCallInput,
    ) -> Result<ValueId, String>
    where
        Port: super::method_call_terminal::MethodCallValueTerminalPortV1,
    {
        // ========================================
        // Section 1: Debug Tracing (debug_method_routing module)
        // ========================================
        {
            let syntax = port.method_call_syntax(input)?;
            self.trace_method_call_if_enabled(syntax.receiver(), syntax.method());
        }

        // ========================================
        // Section 2: Special Method Handlers (special_method_handlers module)
        // ========================================

        match super::reserved_method_route::build_reserved_method_call_v1(self, port, input)? {
            super::reserved_method_route::ReservedMethodCallOutcomeV1::Ordinary => {}
            super::reserved_method_route::ReservedMethodCallOutcomeV1::Emitted(value) => {
                return Ok(value)
            }
        }

        self.build_member_method_call_v1(port, input)
    }

    // Build from expression: from Parent.method(arguments)
    pub fn build_from_expression(
        &mut self,
        parent: String,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        if let Some(result) =
            self.try_build_enum_variant_constructor(&parent, &method, arguments.clone())?
        {
            return Ok(result);
        }

        let arg_values = self.build_call_args(&arguments)?;
        let parent_value = crate::mir::builder::emission::constant::emit_string(self, parent)?;
        let result_id = self.next_value_id();
        self.emit_box_or_plugin_call(
            Some(result_id),
            parent_value,
            method,
            None,
            arg_values,
            EffectMask::READ.add(Effect::ReadHeap),
        )?;
        Ok(result_id)
    }

    pub(in crate::mir::builder) fn build_brand_constructor_call(
        &mut self,
        name: String,
        args: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        if args.len() != 1 {
            return Err(format!(
                "[brand/constructor-arity] {} expects exactly one value, got {}",
                name,
                args.len()
            ));
        }
        let mut args = args.into_iter();
        let value = args.next().expect("len checked");
        self.build_expression(value)
    }

    // ========================================
    // Private helper methods (small functions)
    // ========================================

    /// Build call arguments from AST
    pub(in crate::mir::builder) fn build_call_args(
        &mut self,
        args: &[ASTNode],
    ) -> Result<Vec<ValueId>, String> {
        super::call_argument_descent::drive_raw_call_arguments_v1(self, args)
    }

    /// Build a resolved global function call.
    fn build_resolved_function_call(
        &mut self,
        name: String,
        arg_values: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();

        // === ChatGPT5 Pro Design: Type-safe function call resolution ===
        let callee = match self.resolve_call_target(&name) {
            Ok(c) => c,
            Err(_e) => {
                // Additional resolver: unique static method
                let tail_recovery_allowed =
                    match self.try_unique_static_method_recovery(&name, &arg_values)? {
                        BareStaticRecoveryEmissionV1::Emitted(result) => {
                            return Ok(result);
                        }
                        BareStaticRecoveryEmissionV1::NoRecovery(
                            BareStaticRecoveryNoRecoveryReasonV1::NoCandidate,
                        ) => true,
                        BareStaticRecoveryEmissionV1::NoRecovery(
                            BareStaticRecoveryNoRecoveryReasonV1::Ambiguous { .. },
                        ) => false,
                    };
                // Dev-only additional resolver: suffix match
                if tail_recovery_allowed {
                    if let Some(result) = self.try_tail_based_resolver(&name, &arg_values)? {
                        return Ok(result);
                    }
                }
                return Err(format!(
                    "Unresolved function: '{}'. {}",
                    name,
                    super::super::call_resolution::suggest_resolution(&name)
                ));
            }
        };

        // Compatibility: keep func populated for older MIR consumers.
        let fun_val = crate::mir::builder::name_const::make_name_const_result(self, &name)?;

        // Emit new-style Call with type-safe callee
        self.emit_instruction(MirInstruction::Call {
            dst: Some(dst),
            func: fun_val,
            callee: Some(callee),
            args: arg_values,
            effects: EffectMask::READ.add(Effect::ReadHeap),
        })?;
        Ok(dst)
    }

    /// Build unified function call
    fn build_unified_function_call(
        &mut self,
        name: String,
        arg_values: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();
        self.emit_unified_call(Some(dst), CallTarget::Global(name), arg_values)?;
        Ok(dst)
    }

    pub(in crate::mir::builder) fn build_explicit_extern_call(
        &mut self,
        args: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        if args.is_empty() {
            return Err(
                "externcall requires a target string literal: externcall \"name\"(...)".to_string(),
            );
        }

        let extern_name = Self::extract_string_literal(&args[0]).ok_or_else(|| {
            "externcall target must be a string literal: externcall \"name\"(...)".to_string()
        })?;
        let arg_values = self.build_call_args(&args[1..])?;
        let return_type = super::extern_calls::explicit_extern_return_type(&extern_name);
        let (iface_name, method_name) =
            super::extern_calls::split_explicit_extern_name(&extern_name);

        let dst = self.next_value_id();
        self.emit_extern_call_with_effects(
            &iface_name,
            &method_name,
            arg_values,
            Some(dst),
            EffectMask::IO,
        )?;
        self.function_state
            .type_ctx
            .value_types
            .insert(dst, return_type);
        Ok(dst)
    }
}
