//! Raw AST expression dispatcher.
//!
//! RAWPORT0 keeps exactly one AST match tree here. The legacy facade still
//! owns production behavior; later M0 commits parameterize this dispatcher
//! with the invocation child port rather than adding a second matcher.
use super::calls::{
    MethodCallDescentPortV1, MethodCallValueTerminalPortV1, RawLegacyMethodCallInputV1,
};
use super::declaration_order::{sorted_constructor_entries, sorted_method_entries};
use super::ops::{
    drive_ordinary_binary_expression_v1, drive_short_circuit_expression_v1,
    BinaryExpressionDescentPortV1, RawLegacyBinaryInputV1, RawLegacyShortCircuitInputV1,
    ShortCircuitExpressionDescentPortV1,
};
use super::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_expression_v1, drive_legacy_statement_v1,
    RawBoxMethodChildPortV1, RawLegacyChildLoweringPortV1, RawLoopChildEntryPortV1,
    RecursiveChildLoweringPortV1,
};
use super::stmts::{
    drive_local_statement_v1, drive_value_return_statement_v1, drive_variable_assignment_v1,
    LocalStatementDescentPortV1, RawLegacyLocalInputV1, RawLegacyValueReturnInputV1,
    RawLegacyVariableAssignmentInputV1, ReturnStatementDescentPortV1,
    VariableAssignmentDescentPortV1,
};
use super::ValueId;
use crate::ast::{
    ASTNode, AssignStmt, BinaryExpr, CallExpr, FieldAccessExpr, MethodCallExpr, ReturnStmt,
};
use hakorune_mir_builder::BoxCompilationContext;

enum StatementSurfaceDispatch {
    Lowered(ValueId),
    RegularExpression(ASTNode),
}

/// Capability set consumed by the one raw AST expression match tree.
///
/// M0 progressively moves every recursive raw surface into this port. The
/// legacy implementation remains the only production consumer until that
/// closure is complete; `RawInvocationChildPortV1` is intentionally not wired
/// here before all direct helper recursion has a port-aware sibling.
pub(super) trait RawExpressionDispatchPortV1:
    RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    > + BinaryExpressionDescentPortV1<BinaryInput = RawLegacyBinaryInputV1>
    + ShortCircuitExpressionDescentPortV1<ShortCircuitInput = RawLegacyShortCircuitInputV1>
    + MethodCallDescentPortV1<MethodCallInput = RawLegacyMethodCallInputV1>
    + MethodCallValueTerminalPortV1
    + LocalStatementDescentPortV1<LocalInput = RawLegacyLocalInputV1>
    + VariableAssignmentDescentPortV1<VariableAssignmentInput = RawLegacyVariableAssignmentInputV1>
    + ReturnStatementDescentPortV1<ReturnInput = RawLegacyValueReturnInputV1>
    + RawBoxMethodChildPortV1
    + RawLoopChildEntryPortV1
{
}

impl<Port> RawExpressionDispatchPortV1 for Port where
    Port: RecursiveChildLoweringPortV1<
            BodyInput = Vec<ASTNode>,
            StatementInput = ASTNode,
            ExpressionInput = ASTNode,
        > + BinaryExpressionDescentPortV1<BinaryInput = RawLegacyBinaryInputV1>
        + ShortCircuitExpressionDescentPortV1<ShortCircuitInput = RawLegacyShortCircuitInputV1>
        + MethodCallDescentPortV1<MethodCallInput = RawLegacyMethodCallInputV1>
        + MethodCallValueTerminalPortV1
        + LocalStatementDescentPortV1<LocalInput = RawLegacyLocalInputV1>
        + VariableAssignmentDescentPortV1<
            VariableAssignmentInput = RawLegacyVariableAssignmentInputV1,
        > + ReturnStatementDescentPortV1<ReturnInput = RawLegacyValueReturnInputV1>
        + RawBoxMethodChildPortV1
        + RawLoopChildEntryPortV1
{
}

impl super::MirBuilder {
    fn try_build_statement_surface_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        ast: ASTNode,
    ) -> Result<StatementSurfaceDispatch, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
        match ast {
            ASTNode::Program { statements, .. } => {
                Ok(StatementSurfaceDispatch::Lowered(drive_legacy_body_v1(
                    self, port, statements,
                )?))
            }
            ASTNode::ScopeBox { body, .. } => {
                if let Some(value) = self.try_build_guard_let_scopebox_with_port_v1(port, body.clone())? {
                    Ok(StatementSurfaceDispatch::Lowered(value))
                } else {
                    Ok(StatementSurfaceDispatch::Lowered(drive_legacy_body_v1(
                        self, port, body,
                    )?))
                }
            }
            ASTNode::TaskScope {
                body,
                source_keyword,
                ..
            } => Ok(StatementSurfaceDispatch::Lowered(
                super::stmts::task_scope_stmt::build_task_scope_statement(
                    self,
                    body.clone(),
                    source_keyword.clone(),
                )?,
            )),
            ASTNode::ContextScope {
                source_keyword,
                name,
                ..
            } => Err(format!(
                "[freeze:contract][mir_builder/context_scope_lowering_missing] spelling={} name={} context propagation is owned by CONC-CONTEXT-002",
                source_keyword, name
            )),
            ASTNode::Print { expression, .. } => Ok(StatementSurfaceDispatch::Lowered(
                super::stmts::print_stmt::build_print_statement_with_port_v1(
                    self,
                    port,
                    *expression,
                )?,
            )),
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
                Ok(StatementSurfaceDispatch::Lowered(self.cf_if_with_port_v1(
                    port,
                    *condition,
                    then_node,
                    else_node,
                )?))
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                if crate::config::env::builder_loopform_debug() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug("[exprs.rs] statement surface Loop route matched");
                }
                Ok(StatementSurfaceDispatch::Lowered(
                    port.lower_loop(self, *condition, body)?,
                ))
            }
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                ..
            } => Ok(StatementSurfaceDispatch::Lowered(
                super::control_flow::exception::cf_try_catch_with_port_v1(
                    self,
                    port,
                try_body,
                catch_clauses,
                finally_body,
                )?,
            )),
            ASTNode::Throw { expression, .. } => Ok(StatementSurfaceDispatch::Lowered(
                super::control_flow::exception::cf_throw_with_port_v1(self, port, *expression)?,
            )),
            node @ ASTNode::Assignment { .. } => {
                let stmt = AssignStmt::try_from(node).expect("ASTNode::Assignment must convert");
                Ok(StatementSurfaceDispatch::Lowered(
                    self.build_assignment_statement_expression_with_port_v1(port, stmt)?,
                ))
            }
            ASTNode::CompoundAssignment {
                target,
                operator,
                value,
                ..
            } => Ok(StatementSurfaceDispatch::Lowered(
                self.build_compound_assignment_statement_with_port_v1(
                    port,
                    *target,
                    operator,
                    *value,
                )?,
            )),
            node @ ASTNode::Return { .. } => {
                let stmt = ReturnStmt::try_from(node).expect("ASTNode::Return must convert");
                Ok(StatementSurfaceDispatch::Lowered(
                    self.build_return_statement_expression_with_port_v1(port, stmt)?,
                ))
            }
            ASTNode::Local {
                variables,
                initial_values,
                declared_type_names,
                ..
            } => Ok(StatementSurfaceDispatch::Lowered(
                drive_local_statement_v1(
                    self,
                    port,
                    &RawLegacyLocalInputV1::new(
                        variables,
                        initial_values,
                        declared_type_names,
                    ),
                )?,
            )),
            ASTNode::Outbox { variables, .. } => Ok(StatementSurfaceDispatch::Lowered(
                super::stmts::variable_stmt::build_outbox_statement(self, variables.clone())?,
            )),
            ASTNode::Nowait {
                variable,
                expression,
                ..
            } => Ok(StatementSurfaceDispatch::Lowered(
                self.build_nowait_statement_expression_with_port_v1(
                    port,
                    variable,
                    *expression,
                )?,
            )),
            ASTNode::UsingStatement { .. } => Ok(StatementSurfaceDispatch::Lowered(
                crate::mir::builder::emission::constant::emit_void(self)?,
            )),
            ast => Ok(StatementSurfaceDispatch::RegularExpression(ast)),
        }
    }

    fn build_nowait_statement_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        variable: String,
        expression: ASTNode,
    ) -> Result<ValueId, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
        super::stmts::async_stmt::build_nowait_statement_with_port_v1(
            self, port, variable, expression,
        )
    }

    fn build_assignment_statement_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        stmt: AssignStmt,
    ) -> Result<ValueId, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
        if let ASTNode::FieldAccess { object, field, .. } = stmt.target.as_ref() {
            self.build_field_assignment_with_port_v1(
                port,
                *object.clone(),
                field.clone(),
                *stmt.value.clone(),
            )
        } else if let ASTNode::Index { target, index, .. } = stmt.target.as_ref() {
            self.build_index_assignment_with_port_v1(
                port,
                *target.clone(),
                *index.clone(),
                *stmt.value.clone(),
            )
        } else if let ASTNode::Variable { name, .. } = stmt.target.as_ref() {
            let input = RawLegacyVariableAssignmentInputV1::new(name.clone(), *stmt.value);
            drive_variable_assignment_v1(self, port, &input)
        } else {
            Err("Complex assignment targets not yet supported".to_string())
        }
    }

    fn build_return_statement_expression_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        stmt: ReturnStmt,
    ) -> Result<ValueId, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
        match stmt.value {
            Some(value) => {
                let input = RawLegacyValueReturnInputV1::new(*value);
                drive_value_return_statement_v1(self, port, &input)
            }
            None => super::stmts::return_stmt::build_return_statement(self, None),
        }
    }

    /// Legacy facade for the one generic raw AST dispatcher.
    //
    // It deliberately creates its raw child port only at the legacy root. A
    // recursive descent receives that same port from the generic core instead
    // of rebuilding it at every Binary/MethodCall/Weak/BlockExpr boundary.
    pub(super) fn build_expression_impl(&mut self, ast: ASTNode) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_expression_impl_with_port_v1(&mut port, ast)
    }

    /// The sole raw AST match tree, parameterized by its child descent.
    pub(super) fn build_expression_impl_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        ast: ASTNode,
    ) -> Result<ValueId, String>
    where
        Port: RawExpressionDispatchPortV1,
    {
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
        let ast = match self.try_build_statement_surface_expression_with_port_v1(port, ast)? {
            StatementSurfaceDispatch::Lowered(value) => return Ok(value),
            StatementSurfaceDispatch::RegularExpression(ast) => ast,
        };
        match ast {
            // Regular expressions
            ASTNode::Literal { value, .. } => self.build_literal(value),

            node @ ASTNode::BinaryOp { .. } => {
                // Use BinaryExpr for clear destructuring (no behavior change)
                let e = BinaryExpr::try_from(node).expect("ASTNode::BinaryOp must convert");
                let left = *e.left;
                let right = *e.right;
                match e.operator {
                    operator @ (crate::ast::BinaryOperator::And
                    | crate::ast::BinaryOperator::Or) => {
                        let input = RawLegacyShortCircuitInputV1::new(left, operator, right);
                        drive_short_circuit_expression_v1(self, port, &input)
                    }
                    operator => {
                        let input = RawLegacyBinaryInputV1::new(left, operator, right);
                        drive_ordinary_binary_expression_v1(self, port, &input)
                    }
                }
            }

            ASTNode::CheckExpr { items, .. } => {
                self.build_check_expression_with_port_v1(port, items)
            }

            ASTNode::UnaryOp {
                operator, operand, ..
            } => {
                match operator {
                    // Phase 285W-Syntax-0: weak <expr> → WeakRef(New)
                    crate::ast::UnaryOperator::Weak => {
                        let box_val = drive_legacy_expression_v1(self, port, *operand)?;
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
                        super::ops::unary::build_unary_op_with_port_v1(
                            self, port, op_string, *operand,
                        )
                    }
                }
            }

            ASTNode::Variable { name, .. } => self.build_variable_access(name.clone()),

            ASTNode::Me { .. } => super::stmts::variable_stmt::build_me_expression(self),

            node @ ASTNode::MethodCall { .. } => {
                let m = MethodCallExpr::try_from(node).expect("ASTNode::MethodCall must convert");
                let input = RawLegacyMethodCallInputV1::new(*m.object, m.method, m.arguments);
                self.build_method_call_from_input_v1(port, &input)
            }

            ASTNode::FromCall {
                parent,
                method,
                arguments,
                ..
            } => self.build_from_expression_with_port_v1(
                port,
                parent.clone(),
                method.clone(),
                arguments.clone(),
            ),

            // Phase 152-A: Grouped assignment expression (x = expr)
            // Stage-3 only. Value/type same as rhs, side effect assigns to lhs.
            // Remains outside ASN0 and returns the SSA ValueId.
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                let input = RawLegacyVariableAssignmentInputV1::new(lhs, *rhs);
                drive_variable_assignment_v1(self, port, &input)
            }

            ASTNode::Index { target, index, .. } => {
                self.build_index_expression_with_port_v1(port, *target.clone(), *index.clone())
            }

            node @ ASTNode::FunctionCall { .. } => {
                let c = CallExpr::try_from(node).expect("ASTNode::FunctionCall must convert");
                self.build_function_call_with_port_v1(port, c.name, c.arguments)
            }

            ASTNode::Call {
                callee, arguments, ..
            } => self.build_indirect_call_expression_with_port_v1(
                port,
                *callee.clone(),
                arguments.clone(),
            ),

            ASTNode::QMarkPropagate { expression, .. } => {
                self.build_qmark_propagate_expression_with_port_v1(port, *expression.clone())
            }

            ASTNode::MatchExpr {
                scrutinee,
                arms,
                else_expr,
                ..
            } => self.build_peek_expression_with_port_v1(
                port,
                *scrutinee.clone(),
                arms.clone(),
                *else_expr.clone(),
            ),

            ASTNode::EnumMatchExpr {
                enum_name,
                scrutinee,
                arms,
                else_expr,
                ..
            } => self.build_enum_match_expression_with_port_v1(
                port,
                enum_name.clone(),
                *scrutinee.clone(),
                arms.clone(),
                else_expr.clone(),
            ),

            ASTNode::Lambda { params, body, .. } => {
                self.build_lambda_expression(params.clone(), body.clone())
            }

            ASTNode::BoxDeclaration {
                name,
                methods,
                is_static,
                fields,
                field_decls,
                constructors,
                init_fields,
                weak_fields,
                is_sync,
                ..
            } => {
                if is_sync {
                    return Err(format!(
                        "[freeze:contract][mir_builder/sync_box_lowering_missing] box={} sync box serialized runtime behavior is owned by CONC-SYNCBOX-003",
                        name
                    ));
                }
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
                        // See lifecycle.rs for context creation and context swap.
                        // Phase 285LLVM-1.1: Register static box (no fields)
                        self.comp_ctx.register_user_box(name.clone());
                        // Use BoxCompilationContext even in script/test mode to isolate metadata per static box.
                        let saved_var_map =
                            std::mem::take(&mut self.function_state.variable_ctx.variable_map);
                        let saved_type_ctx = self.function_state.type_ctx.take_snapshot();
                        let saved_slot_registry = self.comp_ctx.current_slot_registry.take();
                        let saved_comp_ctx = self.comp_ctx.compilation_context.take();
                        self.comp_ctx.compilation_context = Some(BoxCompilationContext::new());
                        for (method_name, method_ast) in sorted_method_entries(&methods) {
                            if let ASTNode::FunctionDeclaration {
                                params,
                                param_decls,
                                return_type_name,
                                body,
                                uses,
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
                            }
                        }
                        self.comp_ctx.compilation_context = saved_comp_ctx;
                        self.function_state.variable_ctx.variable_map = saved_var_map;
                        self.function_state
                            .type_ctx
                            .restore_snapshot(saved_type_ctx);
                        self.comp_ctx.current_slot_registry = saved_slot_registry;
                        // Return void for declaration context
                        Ok(crate::mir::builder::emission::constant::emit_void(self)?)
                    }
                } else {
                    // Instance box: register type and lower instance methods/ctors as functions
                    // Phase 285LLVM-1.1: Register with field information for LLVM harness
                    self.comp_ctx.register_user_box_declared_fields(
                        name.clone(),
                        &fields,
                        &field_decls,
                        &init_fields,
                        &weak_fields,
                    );
                    self.build_box_declaration(
                        name.clone(),
                        methods.clone(),
                        fields.clone(),
                        weak_fields.clone(),
                    )?;
                    for (ctor_key, ctor_ast) in sorted_constructor_entries(&constructors) {
                        if let ASTNode::FunctionDeclaration {
                            params,
                            param_decls,
                            return_type_name,
                            body,
                            uses,
                            attrs,
                            ..
                        } = ctor_ast
                        {
                            let func_name = format!("{}.{}", name, ctor_key);
                            self.lower_method_as_function(
                                func_name,
                                name.clone(),
                                params.clone(),
                                param_decls.clone(),
                                return_type_name.clone(),
                                body.clone(),
                                uses.clone(),
                                attrs.clone(),
                            )?;
                        }
                    }
                    for (method_name, method_ast) in sorted_method_entries(&methods) {
                        if let ASTNode::FunctionDeclaration {
                            params,
                            param_decls,
                            return_type_name,
                            body,
                            is_static,
                            uses,
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
                                port.lower_instance_box_method(
                                    self,
                                    func_name,
                                    name.clone(),
                                    params.clone(),
                                    param_decls.clone(),
                                    return_type_name.clone(),
                                    body.clone(),
                                    uses.clone(),
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
                self.build_field_access_with_port_v1(port, *f.object.clone(), f.field.clone())
            }

            ASTNode::New {
                class,
                arguments,
                field_initializers,
                ..
            } => self.build_new_expression_with_field_initializers_with_port_v1(
                port,
                class.clone(),
                arguments.clone(),
                field_initializers.clone(),
            ),

            ASTNode::ArrayLiteral { elements, .. } => {
                self.build_array_literal_with_port_v1(port, elements)
            }
            ASTNode::MapLiteral { entries, .. } => {
                self.build_map_literal_with_port_v1(port, entries)
            }

            ASTNode::AwaitExpression { expression, .. } => {
                super::stmts::async_stmt::build_await_expression_with_port_v1(
                    self,
                    port,
                    *expression.clone(),
                )
            }

            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => self.build_record_literal_value_with_port_v1(
                port,
                record_type_name.clone(),
                fields.clone(),
            ),
            ASTNode::RecordUpdate { base, updates, .. } => {
                self.build_record_update_value_with_port_v1(port, *base.clone(), updates.clone())
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
                    let _ = drive_legacy_statement_v1(self, port, stmt)?;
                }

                drive_legacy_expression_v1(self, port, *tail_expr)
            }

            _ => Err(format!("Unsupported AST node type: {:?}", ast)),
        }
    }
}
