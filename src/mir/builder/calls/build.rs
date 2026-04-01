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
//! - `special_method_handlers`: Special method detection（122 lines）
//! - `static_resolution`: Static receiver resolution（182 lines）
//! - `receiver_binding`: Receiver normalization（54 lines）
//!
//! # Refactoring History
//! - Before: 755 lines monolithic implementation
//! - After: 311 lines orchestrator + 4 extracted modules (537 lines total)
//! - Net reduction: -444 lines of complexity in build.rs

use super::super::{Effect, EffectMask, MirBuilder, MirInstruction, MirType, ValueId};
#[allow(unused_imports)]
use super::debug_method_routing::*;
use super::special_handlers;
use super::CallTarget;
use crate::ast::ASTNode;
use std::collections::BTreeMap;

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
                .scope_ctx
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

        // 0. Phase 285W-Syntax-0.1: Reject weak(...) function call syntax
        // SSOT: docs/reference/language/lifecycle.md - weak <expr> is the ONLY valid syntax
        if name == "weak" {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .error("[Phase285W-0.1] Rejecting weak(...) function call");
            return Err(format!(
                "Invalid syntax: weak(...). Use unary operator: weak <expr>\n\
                 Help: Change 'weak(obj)' to 'weak obj' (unary operator, no parentheses)\n\
                 SSOT: docs/reference/language/lifecycle.md"
            ));
        }

        if name == "externcall" {
            return self.build_explicit_extern_call(args);
        }

        // 1. TypeOp wiring: isType(value, "Type"), asType(value, "Type")
        if let Some(result) = self.try_build_typeop_function(&name, &args)? {
            return Ok(result);
        }

        // 2. Math function handling
        let raw_args = args.clone();
        if let Some(res) = self.try_handle_math_function(&name, raw_args) {
            return res;
        }

        // 3. Build argument values
        let arg_values = self.build_call_args(&args)?;

        // 4. Special-case: global str(x) → x.str() normalization
        if name == "str" && arg_values.len() == 1 {
            return self.build_str_normalization(arg_values[0]);
        }

        // 5. Determine call route (unified vs legacy)
        let use_unified = super::call_unified::is_unified_call_enabled()
            && (super::super::call_resolution::is_builtin_function(&name)
                || super::super::call_resolution::is_extern_function(&name));

        if !use_unified {
            self.build_legacy_function_call(name, arg_values)
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

        let result = self.build_method_call_impl(object, method, arguments);
        self.recursion_depth -= 1;
        result
    }

    fn build_method_call_impl(
        &mut self,
        object: ASTNode,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        // ========================================
        // Section 1: Debug Tracing (debug_method_routing module)
        // ========================================
        self.trace_method_call_if_enabled(&object, &method);

        // ========================================
        // Section 2: Special Method Handlers (special_method_handlers module)
        // ========================================

        // 0. Dev-only: __mir__.log / __mir__.mark → MirInstruction::Debug 列へ lowering
        if let Some(result) = self.try_build_mir_debug_method_call(&object, &method, &arguments)? {
            return Ok(result);
        }

        // Phase 288.1: REPL session variable bridge: __repl.get/set → ExternCall
        if let Some(result) = self.try_build_repl_method_call(&object, &method, &arguments)? {
            return Ok(result);
        }

        // ========================================
        // Section 3: Static Resolution (static_resolution module)
        // ========================================

        // 1. Static box method call: BoxName.method(args)
        if let Some(result) =
            self.try_build_static_receiver_method_call(&object, &method, &arguments)?
        {
            return Ok(result);
        }

        // 2. Handle env.* methods
        if let Some(res) = self.try_handle_env_method(&object, &method, &arguments) {
            return res;
        }

        // ========================================
        // Section 4: Receiver Normalization (receiver_binding module)
        // ========================================

        // 3. Phase 269 P1.2: ReceiverNormalizeBox - MethodCall 共通入口 SSOT
        if let Some(result) =
            self.try_normalize_this_me_method_call(&object, &method, &arguments)?
        {
            return Ok(result);
        }

        // 4. Build object value
        let object_value = self.build_expression(object.clone())?;

        // Phase 287 P4: Debug object value after build_expression
        if crate::config::env::builder_static_call_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[P287-DEBUG] After build_expression: object_value={:?}",
                object_value
            ));
        }

        // Debug trace for receiver (debug_method_routing module)
        self.trace_receiver_if_enabled(&object, object_value);

        // ========================================
        // Section 5: TypeOp Detection (special_handlers module)
        // ========================================

        // 5. Handle TypeOp methods: value.is("Type") / value.as("Type")
        if let Some(type_name) = special_handlers::is_typeop_method(&method, &arguments) {
            return self.handle_typeop_method(object_value, &method, &type_name);
        }

        // ========================================
        // Section 6: Standard Method Call (fallback)
        // ========================================

        // 6. Fallback: standard Box/Plugin method call
        self.handle_standard_method_call(object_value, method, &arguments)
    }

    // Build from expression: from Parent.method(arguments)
    pub fn build_from_expression(
        &mut self,
        parent: String,
        method: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
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

    // ========================================
    // Private helper methods (small functions)
    // ========================================

    /// Try handle env.* extern methods
    fn try_handle_env_method(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &Vec<ASTNode>,
    ) -> Option<Result<ValueId, String>> {
        let ASTNode::FieldAccess {
            object: env_obj,
            field: env_field,
            ..
        } = object
        else {
            return None;
        };
        if let ASTNode::Variable { name: env_name, .. } = env_obj.as_ref() {
            if env_name != "env" {
                return None;
            }
            // Build arguments once
            let arg_values = match self.build_call_args(arguments) {
                Ok(values) => values,
                Err(e) => return Some(Err(e)),
            };
            let iface = env_field.as_str();
            let m = method;
            let mut extern_call = |iface_name: &str,
                                   method_name: &str,
                                   effects: EffectMask,
                                   returns: bool|
             -> Result<ValueId, String> {
                let result_id = self.next_value_id();
                let dst = if returns { Some(result_id) } else { None };
                self.emit_extern_call_with_effects(
                    iface_name,
                    method_name,
                    arg_values.clone(),
                    dst,
                    effects,
                )?;
                if returns {
                    Ok(result_id)
                } else {
                    Ok(crate::mir::builder::emission::constant::emit_void(self)?)
                }
            };
            // Use the new module for env method spec
            if let Some((iface_name, method_name, effects, returns)) =
                super::extern_calls::get_env_method_spec(iface, m)
            {
                return Some(extern_call(&iface_name, &method_name, effects, returns));
            }
            return None;
        }
        None
    }

    /// Build call arguments from AST
    pub(in crate::mir::builder) fn build_call_args(
        &mut self,
        args: &[ASTNode],
    ) -> Result<Vec<ValueId>, String> {
        self.enforce_moved_same_call_args_contract(args)?;
        let mut arg_values = Vec::new();

        for (arg_idx, arg_ast) in args.iter().enumerate() {
            let v = self.build_expression(arg_ast.clone())?;

            // Debug-only observation: check for undefined ValueId immediately after build
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some(func) = self.scope_ctx.current_function.as_ref() {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                    if !def_blocks.contains_key(&v) {
                        // Found undefined ValueId - log AST type and span
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[call/arg_build:undefined_value] fn={} bb={:?} arg_idx={} v=%{} ast={} span={:?} next={}",
                            func.signature.name,
                            self.current_block,
                            arg_idx,
                            v.0,
                            arg_ast.node_type(),
                            arg_ast.span(),
                            func.next_value_id
                        ));
                    }
                }
            }

            arg_values.push(v);
        }

        Ok(arg_values)
    }

    /// S8 minimal moved-state contract:
    /// in strict+planner_required mode, reusing the same variable in one call arg list
    /// (`f(x, x)`) is treated as use-after-move and fails fast.
    fn enforce_moved_same_call_args_contract(&self, args: &[ASTNode]) -> Result<(), String> {
        if !crate::config::env::joinir_dev::strict_planner_required_enabled() {
            return Ok(());
        }
        let mut first_seen: BTreeMap<&str, usize> = BTreeMap::new();
        for (idx, arg) in args.iter().enumerate() {
            let ASTNode::Variable { name, .. } = arg else {
                continue;
            };
            if let Some(prev) = first_seen.insert(name.as_str(), idx) {
                return Err(format!(
                    "[freeze:contract][moved/use_after_move_same_call] var={} first_arg={} reused_arg={}",
                    name, prev, idx
                ));
            }
        }
        Ok(())
    }

    /// Build legacy function call
    fn build_legacy_function_call(
        &mut self,
        name: String,
        arg_values: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();

        // === ChatGPT5 Pro Design: Type-safe function call resolution ===
        let callee = match self.resolve_call_target(&name) {
            Ok(c) => c,
            Err(_e) => {
                // Fallback: unique static method
                if let Some(result) = self.try_static_method_fallback(&name, &arg_values)? {
                    return Ok(result);
                }
                // Tail-based fallback (disabled by default)
                if let Some(result) = self.try_tail_based_fallback(&name, &arg_values)? {
                    return Ok(result);
                }
                return Err(format!(
                    "Unresolved function: '{}'. {}",
                    name,
                    super::super::call_resolution::suggest_resolution(&name)
                ));
            }
        };

        // Legacy compatibility: Create dummy func value for old systems
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

    fn build_explicit_extern_call(&mut self, args: Vec<ASTNode>) -> Result<ValueId, String> {
        if args.is_empty() {
            return Err(
                "externcall requires a target string literal: externcall \"name\"(...)".to_string(),
            );
        }

        let extern_name = Self::extract_string_literal(&args[0]).ok_or_else(|| {
            "externcall target must be a string literal: externcall \"name\"(...)".to_string()
        })?;
        let arg_values = self.build_call_args(&args[1..])?;
        let return_type = match extern_name.as_str() {
            "hako_mem_alloc"
            | "hako_mem_realloc"
            | "hako_mem_free"
            | "hako_osvm_reserve_bytes_i64" => MirType::Integer,
            "nyash.box.from_i8_string" => MirType::Box("StringBox".to_string()),
            _ => MirType::Unknown,
        };
        let (iface_name, method_name) = match extern_name.rsplit_once('.') {
            Some((iface, method)) if !iface.is_empty() && !method.is_empty() => {
                (iface.to_string(), method.to_string())
            }
            _ => ("".to_string(), extern_name),
        };

        let dst = self.next_value_id();
        self.emit_extern_call_with_effects(
            &iface_name,
            &method_name,
            arg_values,
            Some(dst),
            EffectMask::IO,
        )?;
        self.type_ctx.value_types.insert(dst, return_type);
        Ok(dst)
    }
}
