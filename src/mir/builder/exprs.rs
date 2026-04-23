// Expression lowering split from builder.rs to keep files lean
use super::declaration_order::{sorted_constructor_entries, sorted_method_entries};
use super::{MirInstruction, ValueId};
use crate::ast::{
    ASTNode, AssignStmt, BinaryExpr, CallExpr, FieldAccessExpr, MethodCallExpr, ReturnStmt,
};
use crate::mir::builder::observe::types as type_trace;
use hakorune_mir_builder::BoxCompilationContext;

impl super::MirBuilder {
    // Main expression dispatcher
    pub(super) fn build_expression_impl(&mut self, ast: ASTNode) -> Result<ValueId, String> {
        // Track current source span for downstream instruction emission
        self.metadata_ctx.set_current_span(ast.span());
        if crate::config::env::builder_loopform_debug() {
            if matches!(ast, ASTNode::Loop { .. }) {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[build_expression_impl] === ENTRY === processing Loop node"
                ));
            }
        }
        match ast {
            // Control flow constructs (formerly in exprs_legacy)
            ASTNode::Program { statements, .. } => {
                // Sequentially lower statements and return last value (or Void)
                self.cf_block(statements)
            }
            ASTNode::ScopeBox { body, .. } => self.cf_block(body),
            ASTNode::Print { expression, .. } => {
                super::stmts::print_stmt::build_print_statement(self, *expression)
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                use crate::ast::Span;
                let then_node = ASTNode::Program {
                    statements: then_body,
                    span: Span::unknown(),
                };
                let else_node = else_body.map(|b| ASTNode::Program {
                    statements: b,
                    span: Span::unknown(),
                });
                self.cf_if(*condition, then_node, else_node)
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                if crate::config::env::builder_loopform_debug() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[exprs.rs:35] FIRST Loop route matched");
                }
                self.cf_loop(*condition, body)
            }
            ASTNode::While {
                condition, body, ..
            } => {
                // Desugar Stage-3 while into legacy loop(condition) { body }
                self.cf_loop(*condition, body)
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => self.cf_try_catch(try_body, catch_clauses, finally_body),
            ASTNode::Throw { expression, .. } => self.cf_throw(*expression),

            // Regular expressions
            ASTNode::Literal { value, .. } => self.build_literal(value),

            node @ ASTNode::BinaryOp { .. } => {
                // Use BinaryExpr for clear destructuring (no behavior change)
                let e = BinaryExpr::try_from(node).expect("ASTNode::BinaryOp must convert");
                self.build_binary_op(*e.left, e.operator, *e.right)
            }

            ASTNode::UnaryOp {
                operator, operand, ..
            } => {
                match operator {
                    // Phase 285W-Syntax-0: weak <expr> → WeakRef(New)
                    crate::ast::UnaryOperator::Weak => {
                        let box_val = self.build_expression_impl(*operand)?;
                        self.emit_weak_new(box_val)
                    }
                    // Traditional unary operators
                    _ => {
                        let op_string = match operator {
                            crate::ast::UnaryOperator::Minus => "-".to_string(),
                            crate::ast::UnaryOperator::Not => "not".to_string(),
                            crate::ast::UnaryOperator::BitNot => "~".to_string(),
                            crate::ast::UnaryOperator::Weak => unreachable!("handled above"),
                        };
                        self.build_unary_op(op_string, *operand)
                    }
                }
            }

            ASTNode::Variable { name, .. } => self.build_variable_access(name.clone()),

            ASTNode::Me { .. } => super::stmts::variable_stmt::build_me_expression(self),

            node @ ASTNode::MethodCall { .. } => {
                let m = MethodCallExpr::try_from(node).expect("ASTNode::MethodCall must convert");
                if (m.method == "is" || m.method == "as") && m.arguments.len() == 1 {
                    if let Some(type_name) = Self::extract_string_literal(&m.arguments[0]) {
                        let obj_val = self.build_expression_impl(*m.object.clone())?;
                        let ty = Self::parse_type_name_to_mir(&type_name);
                        let dst = self.next_value_id();
                        let op = if m.method == "is" {
                            crate::mir::TypeOpKind::Check
                        } else {
                            crate::mir::TypeOpKind::Cast
                        };
                        self.emit_instruction(MirInstruction::TypeOp {
                            dst,
                            op,
                            value: obj_val,
                            ty,
                        })?;
                        return Ok(dst);
                    }
                }
                self.build_method_call(*m.object.clone(), m.method.clone(), m.arguments.clone())
            }

            ASTNode::FromCall {
                parent,
                method,
                arguments,
                ..
            } => self.build_from_expression(parent.clone(), method.clone(), arguments.clone()),

            node @ ASTNode::Assignment { .. } => {
                // Use AssignStmt wrapper for clearer destructuring (no behavior change)
                let stmt = AssignStmt::try_from(node).expect("ASTNode::Assignment must convert");
                if let ASTNode::FieldAccess { object, field, .. } = stmt.target.as_ref() {
                    self.build_field_assignment(*object.clone(), field.clone(), *stmt.value.clone())
                } else if let ASTNode::Index { target, index, .. } = stmt.target.as_ref() {
                    self.build_index_assignment(
                        *target.clone(),
                        *index.clone(),
                        *stmt.value.clone(),
                    )
                } else if let ASTNode::Variable { name, .. } = stmt.target.as_ref() {
                    self.build_assignment(name.clone(), *stmt.value.clone())
                } else {
                    Err("Complex assignment targets not yet supported".to_string())
                }
            }

            // Phase 152-A: Grouped assignment expression (x = expr)
            // Stage-3 only. Value/type same as rhs, side effect assigns to lhs.
            // Reuses existing build_assignment logic, returns the SSA ValueId.
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                self.build_assignment(lhs.clone(), *rhs.clone())
            }

            ASTNode::Index { target, index, .. } => {
                self.build_index_expression(*target.clone(), *index.clone())
            }

            node @ ASTNode::FunctionCall { .. } => {
                let c = CallExpr::try_from(node).expect("ASTNode::FunctionCall must convert");
                self.build_function_call(c.name, c.arguments)
            }

            ASTNode::Call {
                callee, arguments, ..
            } => self.build_indirect_call_expression(*callee.clone(), arguments.clone()),

            ASTNode::QMarkPropagate { expression, .. } => {
                self.build_qmark_propagate_expression(*expression.clone())
            }

            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => self.build_peek_expression(*scrutinee.clone(), arms.clone(), *else_expr.clone()),

            ASTNode::Lambda { params, body, .. } => {
                self.build_lambda_expression(params.clone(), body.clone())
            }

            node @ ASTNode::Return { .. } => {
                // Use ReturnStmt wrapper for consistent access (no behavior change)
                let stmt = ReturnStmt::try_from(node).expect("ASTNode::Return must convert");
                super::stmts::return_stmt::build_return_statement(self, stmt.value.clone())
            }

            // Control flow: break/continue are handled inside LoopBuilder context
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => super::stmts::variable_stmt::build_local_statement(
                self,
                variables.clone(),
                initial_values.clone(),
            ),

            ASTNode::Outbox { variables, .. } => {
                let func_name = self
                    .scope_ctx
                    .current_function
                    .as_ref()
                    .map(|f| f.signature.name.as_str())
                    .unwrap_or("<unknown>");
                Err(format!(
                    "[freeze:contract][outbox/lowering_not_implemented] fn={} vars={}",
                    func_name,
                    variables.join(",")
                ))
            }

            ASTNode::BoxDeclaration {
                name,
                methods,
                is_static,
                fields,
                field_decls,
                constructors,
                weak_fields,
                ..
            } => {
                if is_static && name == "Main" {
                    // Special entry box: materialize main() as Program and lower others as static functions
                    self.build_static_main_box(name.clone(), methods.clone())
                } else if is_static {
                    // In App mode (Main/main present), static boxes are lowered in lower_root().
                    // Here we only handle Script/Test mode or non-root contexts.
                    let is_app_mode = self.root_is_app_mode.unwrap_or(false);
                    if is_app_mode {
                        // Already lowered by lifecycle pass; return Void as a pure declaration.
                        Ok(crate::mir::builder::emission::constant::emit_void(self)?)
                    } else {
                        // Generic static box: lower all static methods into standalone MIR functions (BoxName.method/N)
                        // Note: Metadata clearing is now handled by BoxCompilationContext (箱理論)
                        // See lifecycle.rs for context creation and builder_calls.rs for context swap
                        // Phase 285LLVM-1.1: Register static box (no fields)
                        self.comp_ctx.register_user_box(name.clone());
                        // Use BoxCompilationContext even in script/test mode to isolate metadata per static box.
                        let saved_var_map = std::mem::take(&mut self.variable_ctx.variable_map);
                        let saved_type_ctx = self.type_ctx.take_snapshot();
                        let saved_slot_registry = self.comp_ctx.current_slot_registry.take();
                        let saved_comp_ctx = self.comp_ctx.compilation_context.take();
                        self.comp_ctx.compilation_context = Some(BoxCompilationContext::new());
                        for (method_name, method_ast) in sorted_method_entries(&methods) {
                            if let ASTNode::FunctionDeclaration {
                                params,
                                body,
                                attrs,
                                ..
                            } = method_ast
                            {
                                let func_name = format!(
                                    "{}.{}{}",
                                    name,
                                    method_name,
                                    format!("/{}", params.len())
                                );
                                self.lower_static_method_as_function(
                                    func_name,
                                    params.clone(),
                                    body.clone(),
                                    attrs.clone(),
                                )?;
                                // Index static method for fallback resolution of bare calls
                                self.comp_ctx
                                    .static_method_index
                                    .entry(method_name.to_string())
                                    .or_insert_with(Vec::new)
                                    .push((name.clone(), params.len()));
                            }
                        }
                        self.comp_ctx.compilation_context = saved_comp_ctx;
                        self.variable_ctx.variable_map = saved_var_map;
                        self.type_ctx.restore_snapshot(saved_type_ctx);
                        self.comp_ctx.current_slot_registry = saved_slot_registry;
                        // Return void for declaration context
                        Ok(crate::mir::builder::emission::constant::emit_void(self)?)
                    }
                } else {
                    // Instance box: register type and lower instance methods/ctors as functions
                    // Phase 285LLVM-1.1: Register with field information for LLVM harness
                    if field_decls.is_empty() {
                        self.comp_ctx
                            .register_user_box_with_fields(name.clone(), fields.clone());
                    } else {
                        self.comp_ctx
                            .register_user_box_with_field_decls(name.clone(), field_decls.clone());
                    }
                    self.build_box_declaration(
                        name.clone(),
                        methods.clone(),
                        fields.clone(),
                        weak_fields.clone(),
                    )?;
                    for (ctor_key, ctor_ast) in sorted_constructor_entries(&constructors) {
                        if let ASTNode::FunctionDeclaration {
                            params,
                            body,
                            attrs,
                            ..
                        } = ctor_ast
                        {
                            let func_name = format!("{}.{}", name, ctor_key);
                            self.lower_method_as_function(
                                func_name,
                                name.clone(),
                                params.clone(),
                                body.clone(),
                                attrs.clone(),
                            )?;
                        }
                    }
                    for (method_name, method_ast) in sorted_method_entries(&methods) {
                        if let ASTNode::FunctionDeclaration {
                            params,
                            body,
                            is_static,
                            attrs,
                            ..
                        } = method_ast
                        {
                            if !is_static {
                                let func_name = format!(
                                    "{}.{}{}",
                                    name,
                                    method_name,
                                    format!("/{}", params.len())
                                );
                                self.lower_method_as_function(
                                    func_name,
                                    name.clone(),
                                    params.clone(),
                                    body.clone(),
                                    attrs.clone(),
                                )?;
                            }
                        }
                    }
                    Ok(crate::mir::builder::emission::constant::emit_void(self)?)
                }
            }

            node @ ASTNode::FieldAccess { .. } => {
                let f = FieldAccessExpr::try_from(node).expect("ASTNode::FieldAccess must convert");
                self.build_field_access(*f.object.clone(), f.field.clone())
            }

            ASTNode::New {
                class, arguments, ..
            } => self.build_new_expression(class.clone(), arguments.clone()),

            ASTNode::ArrayLiteral { elements, .. } => {
                let arr_id = self.next_value_id();
                self.emit_instruction(MirInstruction::NewBox {
                    dst: arr_id,
                    box_type: "ArrayBox".to_string(),
                    args: vec![],
                })?;
                // Explicit birth() to satisfy runtime invariant (NewBox→birth)
                self.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
                    None,
                    arr_id,
                    "ArrayBox",
                    "birth",
                    vec![],
                    super::EffectMask::MUT,
                    crate::mir::definitions::call_unified::TypeCertainty::Known,
                ))?;
                self.type_ctx
                    .value_origin_newbox
                    .insert(arr_id, "ArrayBox".to_string());
                self.type_ctx
                    .value_types
                    .insert(arr_id, super::MirType::Box("ArrayBox".to_string()));
                // TypeRegistry + trace for deterministic debug
                self.comp_ctx
                    .type_registry
                    .record_newbox(arr_id, "ArrayBox".to_string());
                self.comp_ctx
                    .type_registry
                    .record_type(arr_id, super::MirType::Box("ArrayBox".to_string()));
                type_trace::origin("newbox:ArrayLiteral", arr_id, "ArrayBox");
                type_trace::ty(
                    "newbox:ArrayLiteral",
                    arr_id,
                    &super::MirType::Box("ArrayBox".to_string()),
                );
                let mut element_types = Vec::new();
                for e in elements {
                    let v = self.build_expression_impl(e)?;
                    let element_type = self.type_ctx.value_types.get(&v).cloned().or_else(|| {
                        self.type_ctx
                            .value_origin_newbox
                            .get(&v)
                            .map(|box_name| super::MirType::Box(box_name.clone()))
                    });
                    self.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
                        None,
                        arr_id,
                        "ArrayBox",
                        "push",
                        vec![v],
                        super::EffectMask::MUT,
                        crate::mir::definitions::call_unified::TypeCertainty::Known,
                    ))?;
                    element_types.push(element_type);
                }
                crate::mir::builder::types::array_element::record_array_literal_elements(
                    self,
                    arr_id,
                    &element_types,
                );
                Ok(arr_id)
            }
            ASTNode::MapLiteral { entries, .. } => {
                let map_id = self.next_value_id();
                self.emit_instruction(MirInstruction::NewBox {
                    dst: map_id,
                    box_type: "MapBox".to_string(),
                    args: vec![],
                })?;
                // Explicit birth() to satisfy runtime invariant (NewBox→birth)
                self.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
                    None,
                    map_id,
                    "MapBox",
                    "birth",
                    vec![],
                    super::EffectMask::MUT,
                    crate::mir::definitions::call_unified::TypeCertainty::Known,
                ))?;
                self.type_ctx
                    .value_origin_newbox
                    .insert(map_id, "MapBox".to_string());
                self.type_ctx
                    .value_types
                    .insert(map_id, super::MirType::Box("MapBox".to_string()));
                self.comp_ctx
                    .type_registry
                    .record_newbox(map_id, "MapBox".to_string());
                self.comp_ctx
                    .type_registry
                    .record_type(map_id, super::MirType::Box("MapBox".to_string()));
                type_trace::origin("newbox:MapLiteral", map_id, "MapBox");
                type_trace::ty(
                    "newbox:MapLiteral",
                    map_id,
                    &super::MirType::Box("MapBox".to_string()),
                );
                for (k, expr) in entries {
                    // const string key
                    let k_id = crate::mir::builder::emission::constant::emit_string(self, k)?;
                    let v_id = self.build_expression_impl(expr)?;
                    self.emit_instruction(crate::mir::ssot::method_call::runtime_method_call(
                        None,
                        map_id,
                        "MapBox",
                        "set",
                        vec![k_id, v_id],
                        super::EffectMask::MUT,
                        crate::mir::definitions::call_unified::TypeCertainty::Known,
                    ))?;
                }
                Ok(map_id)
            }

            ASTNode::Nowait {
                variable,
                expression,
                ..
            } => super::stmts::async_stmt::build_nowait_statement(
                self,
                variable.clone(),
                *expression.clone(),
            ),

            ASTNode::AwaitExpression { expression, .. } => {
                super::stmts::async_stmt::build_await_expression(self, *expression.clone())
            }

            // UsingStatement: namespace resolution is done at parser/runner level.
            // No MIR emission needed - just return void.
            ASTNode::UsingStatement { .. } => {
                Ok(crate::mir::builder::emission::constant::emit_void(self)?)
            }

            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => {
                // Phase B2-6: BlockExpr in expression position.
                //
                // v1 safety contract: disallow non-local exits that can escape prelude scope.
                // `break/continue` inside nested loops are allowed.
                for stmt in &prelude_stmts {
                    if stmt.contains_non_local_exit_outside_loops() {
                        return Err(
                            "[freeze:contract][blockexpr] exit stmt is forbidden in BlockExpr prelude"
                                .to_string(),
                        );
                    }
                }
                for stmt in prelude_stmts {
                    let _ = self.build_statement(stmt)?;
                }

                self.build_expression_impl(*tail_expr)
            }

            _ => Err(format!("Unsupported AST node type: {:?}", ast)),
        }
    }

    fn infer_index_target_class(&self, target_val: ValueId) -> Option<String> {
        if let Some(cls) = self.type_ctx.value_origin_newbox.get(&target_val) {
            return Some(cls.clone());
        }
        self.type_ctx
            .value_types
            .get(&target_val)
            .and_then(|ty| match ty {
                super::MirType::Box(name) => Some(name.clone()),
                super::MirType::String => Some("String".to_string()),
                super::MirType::Integer => Some("Integer".to_string()),
                super::MirType::Float => Some("Float".to_string()),
                _ => None,
            })
    }

    fn format_index_target_kind(class_hint: Option<&String>) -> String {
        class_hint
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("unknown")
            .to_string()
    }

    pub(super) fn build_index_expression(
        &mut self,
        target: ASTNode,
        index: ASTNode,
    ) -> Result<ValueId, String> {
        let target_val = self.build_expression(target)?;
        let class_hint = self.infer_index_target_class(target_val);

        match class_hint.as_deref() {
            Some("ArrayBox") => {
                let index_val = self.build_expression(index)?;
                let dst = self.next_value_id();
                self.emit_box_or_plugin_call(
                    Some(dst),
                    target_val,
                    "get".to_string(),
                    None,
                    vec![index_val],
                    super::EffectMask::READ,
                )?;
                Ok(dst)
            }
            Some("MapBox") => {
                let index_val = self.build_expression(index)?;
                let dst = self.next_value_id();
                self.emit_box_or_plugin_call(
                    Some(dst),
                    target_val,
                    "get".to_string(),
                    None,
                    vec![index_val],
                    super::EffectMask::READ,
                )?;
                Ok(dst)
            }
            _ => Err(format!(
                "index operator is only supported for Array/Map (found {})",
                Self::format_index_target_kind(class_hint.as_ref())
            )),
        }
    }

    pub(super) fn build_index_assignment(
        &mut self,
        target: ASTNode,
        index: ASTNode,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let target_val = self.build_expression(target)?;
        let class_hint = self.infer_index_target_class(target_val);

        match class_hint.as_deref() {
            Some("ArrayBox") => {
                let index_val = self.build_expression(index)?;
                let value_val = self.build_expression(value)?;
                self.emit_box_or_plugin_call(
                    None,
                    target_val,
                    "set".to_string(),
                    None,
                    vec![index_val, value_val],
                    super::EffectMask::MUT,
                )?;
                Ok(value_val)
            }
            Some("MapBox") => {
                let index_val = self.build_expression(index)?;
                let value_val = self.build_expression(value)?;
                self.emit_box_or_plugin_call(
                    None,
                    target_val,
                    "set".to_string(),
                    None,
                    vec![index_val, value_val],
                    super::EffectMask::MUT,
                )?;
                Ok(value_val)
            }
            _ => Err(format!(
                "index assignment is only supported for Array/Map (found {})",
                Self::format_index_target_kind(class_hint.as_ref())
            )),
        }
    }
}
