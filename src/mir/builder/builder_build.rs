use super::builder_calls::CallTarget;
use super::vars;
use super::{ConstValue, Effect, EffectMask, MirBuilder, MirInstruction, MirModule, ValueId};
use crate::ast::{ASTNode, LiteralValue};
use crate::mir::slot_registry::resolve_slot_by_type_name;

impl MirBuilder {
    /// Build a complete MIR module from AST
    pub fn build_module(&mut self, ast: ASTNode) -> Result<MirModule, String> {
        self.prepare_module()?;
        let result_value = self.lower_root(ast)?;
        self.finalize_module(result_value)
    }

    /// Build an expression and return its value ID
    pub(in crate::mir) fn build_expression(&mut self, ast: ASTNode) -> Result<ValueId, String> {
        // Delegated to exprs.rs to keep this file lean
        // Debug: Track recursion depth to detect infinite loops
        const MAX_RECURSION_DEPTH: usize = 200;
        self.recursion_depth += 1;
        if self.recursion_depth > MAX_RECURSION_DEPTH {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .error("\n[FATAL] ============================================");
            ring0.log.error(&format!(
                "[FATAL] Recursion depth exceeded {} in build_expression",
                MAX_RECURSION_DEPTH
            ));
            ring0
                .log
                .error(&format!("[FATAL] Current depth: {}", self.recursion_depth));
            ring0.log.error(&format!(
                "[FATAL] AST node type: {:?}",
                std::mem::discriminant(&ast)
            ));
            ring0
                .log
                .error("[FATAL] ============================================\n");
            return Err(format!(
                "Recursion depth exceeded: {} (possible infinite loop)",
                self.recursion_depth
            ));
        }

        let result = self.build_expression_impl(ast);
        self.recursion_depth -= 1;
        result
    }

    /// Build a literal value
    pub(super) fn build_literal(&mut self, literal: LiteralValue) -> Result<ValueId, String> {
        // Determine type without moving literal
        let ty_for_dst = match &literal {
            LiteralValue::Integer(_) => Some(super::MirType::Integer),
            LiteralValue::Float(_) => Some(super::MirType::Float),
            LiteralValue::Bool(_) => Some(super::MirType::Bool),
            LiteralValue::String(_) => Some(super::MirType::String),
            _ => None,
        };

        // Emit via ConstantEmissionBox（仕様不変の統一ルート）
        let dst = match literal {
            LiteralValue::Integer(n) => {
                crate::mir::builder::emission::constant::emit_integer(self, n)?
            }
            LiteralValue::Float(f) => crate::mir::builder::emission::constant::emit_float(self, f)?,
            LiteralValue::String(s) => {
                crate::mir::builder::emission::constant::emit_string(self, s)?
            }
            LiteralValue::Bool(b) => crate::mir::builder::emission::constant::emit_bool(self, b)?,
            LiteralValue::Null => crate::mir::builder::emission::constant::emit_null(self)?,
            LiteralValue::Void => crate::mir::builder::emission::constant::emit_void(self)?,
        };
        // Annotate type
        if let Some(ty) = ty_for_dst {
            self.type_ctx.value_types.insert(dst, ty);
        }

        Ok(dst)
    }

    /// Build variable access
    pub(super) fn build_variable_access(&mut self, name: String) -> Result<ValueId, String> {
        // Step 5-5-G: __pin$ variables should NEVER be accessed from variable_map
        // They are transient temporaries created during expression building and
        // should not persist across blocks. If we see one here, it's a compiler bug.
        if name.starts_with("__pin$") {
            return Err(format!(
                "COMPILER BUG: Attempt to access __pin$ temporary '{}' from variable_map. \
                 __pin$ variables should only exist as direct SSA values, not as named variables.",
                name
            ));
        }

        if let Some(&value_id) = self.variable_ctx.variable_map.get(&name) {
            // Removed: [build_variable_access:GHOST_v36] observation (PHI issue resolved)
            // Removed: [build_variable_access:index_of_trace] observation (PHI issue resolved)
            // Removed: [build_variable_access:VAR_j] observation (PHI issue resolved)
            // Debug-only observation: check if variable_map value is defined
            if crate::config::env::joinir_dev::debug_enabled() {
                if let Some(func) = self.scope_ctx.current_function.as_ref() {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                    if !def_blocks.contains_key(&value_id) {
                        // Found undefined ValueId returned from variable_map
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[call/arg_build:undefined_value] fn={} bb={:?} var_name={} v=%{} ast=Variable span=n/a next={}",
                            func.signature.name,
                            self.current_block,
                            name,
                            value_id.0,
                            func.next_value_id
                        ));
                    }
                }
            }
            Ok(value_id)
        } else {
            Err(self.undefined_variable_message(&name))
        }
    }

    pub(in crate::mir::builder) fn undefined_variable_message(&self, name: &str) -> String {
        // Enhance diagnostics using Using simple registry (Phase 1)
        let mut msg = format!("Undefined variable: {}", name);

        // Stage-3 keyword diagnostic (local/flow/try/catch/throw)
        if name == "local" && !crate::config::env::parser_stage3_enabled() {
            msg.push_str("\nHint: 'local' is a Stage-3 keyword. Prefer NYASH_FEATURES=stage3 (legacy: NYASH_PARSER_STAGE3=1 / HAKO_PARSER_STAGE3=1 for Stage-B).");
            msg.push_str("\nFor AotPrep verification, use tools/hakorune_emit_mir.sh which sets these automatically.");
        } else if (name == "flow" || name == "try" || name == "catch" || name == "throw")
            && !crate::config::env::parser_stage3_enabled()
        {
            msg.push_str(&format!("\nHint: '{}' is a Stage-3 keyword. Prefer NYASH_FEATURES=stage3 (legacy: NYASH_PARSER_STAGE3=1 / HAKO_PARSER_STAGE3=1 for Stage-B).", name));
        }

        let suggest = crate::using::simple_registry::suggest_using_for_symbol(name);
        if !suggest.is_empty() {
            msg.push_str("\nHint: symbol appears in using module(s): ");
            msg.push_str(&suggest.join(", "));
            msg.push_str(
                "\nConsider adding 'using <module> [as Alias]' or check nyash.toml [using].",
            );
        }

        msg
    }

    /// Build assignment
    pub(super) fn build_assignment(
        &mut self,
        var_name: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        // SSOT (LANGUAGE_REFERENCE_2025 / syntax-cheatsheet):
        // - Assignment to an undeclared name is an error.
        // - Use `local name = ...` (or `local name; name = ...`) to declare.
        vars::assignment_resolver::AssignmentResolverBox::ensure_declared(self, &var_name)?;

        let value_id = self.build_expression(value)?;

        // Removed: [build_expression:GHOST_v36_result] observation (PHI issue resolved)

        // Step 5-5-E: FIX variable map corruption bug
        // REMOVED pin_to_slot() call - it was causing __pin$ temporaries to overwrite
        // real variable names in the variable map.
        //
        // Root cause: pin_to_slot(raw_value_id, "@assign") would sometimes return
        // a ValueId from a previous __pin$ temporary (e.g., __pin$767$@binop_lhs),
        // causing variable_map["m"] to point to the wrong ValueId.
        //
        // SSA + PHI merges work correctly without explicit pinning here.
        // The expression building already creates necessary temporaries.

        // Step 5-5-F: NEVER insert __pin$ temporaries into variable_map
        // __pin$ variables are transient compiler-generated temporaries that should
        // never be tracked as real variables. They are used only within expression
        // building and should not persist across blocks or loops.
        //
        // BUG FIX: Previously, __pin$ variables would be inserted into variable_map,
        // causing stale references after LoopForm transformation renumbers blocks.
        // Result: VM would try to read undefined ValueIds (e.g., ValueId(270) at bb303).
        if !var_name.starts_with("__pin$") {
            // Phase 287: Release strong references for previous value BEFORE updating variable_map
            // This ensures "alive until overwrite, then dropped" semantics
            // ⚠️ Termination guard: don't emit after return/throw
            if !self.is_current_block_terminated() {
                if let Some(prev) = self.variable_ctx.variable_map.get(&var_name).copied() {
                    let _ =
                        self.emit_instruction(MirInstruction::ReleaseStrong { values: vec![prev] });
                }
            }

            // In SSA form, each assignment creates a new value
            self.variable_ctx
                .variable_map
                .insert(var_name.clone(), value_id);

            // Removed: [build_assignment:GHOST_v36_assigned] observation (PHI issue resolved)
            // Removed: [build_assignment:index_of_trace] observation (PHI issue resolved)
        }

        Ok(value_id)
    }

    /// Build new expression: new ClassName(arguments)
    pub(super) fn build_new_expression(
        &mut self,
        class: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        // Phase 9.78a: Unified Box creation using NewBox instruction
        // Core-13 pure mode: emit ExternCall(env.box.new) with type name const only
        if crate::config::env::mir_core13_pure() {
            // Emit Const String for type name（ConstantEmissionBox）
            let ty_id = crate::mir::builder::emission::constant::emit_string(self, class.clone())?;
            // Evaluate arguments (pass through to env.box.new shim)
            let mut arg_vals: Vec<ValueId> = Vec::with_capacity(arguments.len());
            for a in arguments {
                arg_vals.push(self.build_expression(a)?);
            }
            // Build arg list: [type, a1, a2, ...]
            let mut args: Vec<ValueId> = Vec::with_capacity(1 + arg_vals.len());
            args.push(ty_id);
            args.extend(arg_vals);
            // Call env.box.new
            // 📦 Hotfix 3: Use next_value_id() to respect function parameter reservation
            let dst = self.next_value_id();
            self.emit_extern_call_with_effects(
                "env.box",
                "new",
                args,
                Some(dst),
                EffectMask::PURE,
            )?;
            // 型注釈（最小）
            self.type_ctx
                .value_types
                .insert(dst, super::MirType::Box(class.clone()));
            return Ok(dst);
        }

        // Optimization: Primitive wrappers → emit Const directly when possible
        if class == "IntegerBox" && arguments.len() == 1 {
            if let ASTNode::Literal {
                value: LiteralValue::Integer(n),
                ..
            } = arguments[0].clone()
            {
                // 📦 Hotfix 3: Use next_value_id() to respect function parameter reservation
                let dst = self.next_value_id();
                self.emit_instruction(MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(n),
                })?;
                self.type_ctx
                    .value_types
                    .insert(dst, super::MirType::Integer);
                return Ok(dst);
            }
        }

        // First, evaluate all arguments to get their ValueIds
        let mut arg_values = Vec::new();
        for arg in arguments {
            let arg_value = self.build_expression(arg)?;
            arg_values.push(arg_value);
        }

        // Generate the destination ValueId
        // 📦 Hotfix 3: Use next_value_id() to respect function parameter reservation
        let dst = self.next_value_id();

        // Emit NewBox instruction for all Box types
        // VM will handle optimization for basic types internally
        self.emit_instruction(MirInstruction::NewBox {
            dst,
            box_type: class.clone(),
            args: arg_values.clone(),
        })?;
        // Phase 15.5: Unified box type handling
        // All boxes (including former core boxes) are treated uniformly as Box types
        self.type_ctx
            .value_types
            .insert(dst, super::MirType::Box(class.clone()));

        // Record origin for optimization: dst was created by NewBox of class
        self.type_ctx.value_origin_newbox.insert(dst, class.clone());

        // birth 呼び出し（Builder 正規化）
        // 優先: 低下済みグローバル関数 `<Class>.birth/Arity`（Arity は me を含まない）
        // 代替: 既存互換として BoxCall("birth")（プラグイン/ビルトインの初期化に対応）
        if class != "StringBox" {
            let arity = arg_values.len();
            let lowered =
                crate::mir::builder::calls::function_lowering::generate_method_function_name(
                    &class, "birth", arity,
                );
            let use_lowered = if let Some(ref module) = self.current_module {
                module.functions.contains_key(&lowered)
            } else {
                false
            };
            if use_lowered {
                // Call Global("Class.birth/Arity") with argv = [me, args...]
                let mut argv: Vec<ValueId> = Vec::with_capacity(1 + arity);
                argv.push(dst);
                argv.extend(arg_values.iter().copied());
                self.emit_legacy_call(None, CallTarget::Global(lowered), argv)?;
            } else {
                // Fallback policy:
                // - For user-defined boxes (no explicit constructor), do NOT emit BoxCall("birth").
                //   VM will treat plain NewBox as constructed; dev verify warns if needed.
                // - For builtins/plugins, keep BoxCall("birth") fallback to preserve legacy init.
                let is_user_box = self.comp_ctx.user_defined_boxes.contains_key(&class); // Phase 285LLVM-1.1: HashMap
                                                                                         // Dev safety: allow disabling birth() injection for builtins to avoid
                                                                                         // unified-call method dispatch issues while migrating. Off by default unless explicitly enabled.
                let allow_builtin_birth = crate::config::env::builder_birth_inject_builtins();
                if !is_user_box && allow_builtin_birth {
                    let birt_mid = resolve_slot_by_type_name(&class, "birth");
                    self.emit_box_or_plugin_call(
                        None,
                        dst,
                        "birth".to_string(),
                        birt_mid,
                        arg_values,
                        EffectMask::READ.add(Effect::ReadHeap),
                    )?;
                }
            }
        }

        Ok(dst)
    }

    /// Check if the current basic block is terminated
    pub(super) fn is_current_block_terminated(&self) -> bool {
        if let (Some(block_id), Some(ref function)) =
            (self.current_block, &self.scope_ctx.current_function)
        {
            if let Some(block) = function.get_block(block_id) {
                return block.is_terminated();
            }
        }
        false
    }
}
