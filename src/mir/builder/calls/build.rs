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

        if let Some(result) = self.try_handle_function_preflight(&name, &args)? {
            return Ok(result);
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

        let route_plan = self.plan_member_call_route(&object, &method)?;
        self.emit_member_call_from_plan(route_plan, object, method, arguments)
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
        self.enforce_moved_same_call_args_contract(args)?;
        let mut arg_values = Vec::new();

        for (arg_idx, arg_ast) in args.iter().enumerate() {
            if let ASTNode::Variable { name, .. } = arg_ast {
                if let Some(value) = self.variable_ctx.variable_map.get(name).copied() {
                    self.fail_if_record_value_call_arg_by_name(name, value)?;
                }
            }
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
                if let Some(result) = self.try_unique_static_method_recovery(&name, &arg_values)? {
                    return Ok(result);
                }
                // Dev-only additional resolver: suffix match
                if let Some(result) = self.try_tail_based_resolver(&name, &arg_values)? {
                    return Ok(result);
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
        self.type_ctx.value_types.insert(dst, return_type);
        Ok(dst)
    }
}
