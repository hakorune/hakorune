//! Raw AST expression dispatcher.
//!
//! RAWPORT0 keeps exactly one AST match tree here. The legacy facade still
//! owns production behavior; later M0 commits parameterize this dispatcher
//! with the invocation child port rather than adding a second matcher.
mod block_expr;
mod input_view;
mod legacy_facade;
mod statement_surface;
#[cfg(test)]
mod tests;

pub(in crate::mir::builder) use input_view::{
    RawBodyInputViewV1, RawLegacyBodyInputV1, RawLegacyStatementInputV1, RawStatementInputViewV1,
};

use self::block_expr::{lower_prepared_raw_block_expr_with_port_v1, PreparedRawBlockExprV1};
use super::builder_build::PreparedRawNewExpressionV1;
use super::calls::{
    lower_prepared_raw_function_preflight_with_port_v1, MethodCallDescentPortV1,
    PreparedRawFromCallV1, PreparedRawFunctionPreflightV1, RawLegacyMethodCallInputV1,
};
use super::declaration_order::{sorted_constructor_entries, sorted_method_entries};
use super::exprs_enum_match::PreparedRawEnumMatchV1;
use super::fields::PreparedRawFieldReadV1;
use super::indexing::PreparedRawIndexReadV1;
use super::me_call_header_observation::MethodCallLoweringPortV1;
use super::ops::{
    drive_ordinary_binary_expression_v1, drive_short_circuit_expression_v1,
    lower_prepared_raw_unary_with_port_v1, BinaryExpressionDescentPortV1, PreparedRawUnaryV1,
    RawLegacyBinaryInputV1, RawLegacyShortCircuitInputV1, ShortCircuitExpressionDescentPortV1,
};
use super::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawFunctionHeaderLookupPortV1, RawLoopChildEntryPortV1,
    RecursiveChildLoweringPortV1,
};
use super::stmts::{
    drive_variable_assignment_v1, LocalStatementDescentPortV1, RawLegacyLocalInputV1,
    RawLegacyValueReturnInputV1, RawLegacyVariableAssignmentInputV1, ReturnStatementDescentPortV1,
    VariableAssignmentDescentPortV1,
};
use super::ValueId;
use crate::ast::{ASTNode, BinaryExpr, MethodCallExpr};
use hakorune_mir_builder::BoxCompilationContext;

/// Capability set consumed by the one raw AST expression match tree.
///
/// M0 progressively moves every recursive raw surface into this port. The
/// legacy implementation remains the only production consumer until that
/// closure is complete; `RawInvocationChildPortV1` is intentionally not wired
/// here before all direct helper recursion has a port-aware sibling.
pub(in crate::mir::builder) trait RawExpressionDispatchPortV1:
    RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    > + BinaryExpressionDescentPortV1<BinaryInput = RawLegacyBinaryInputV1>
    + ShortCircuitExpressionDescentPortV1<ShortCircuitInput = RawLegacyShortCircuitInputV1>
    + MethodCallDescentPortV1<MethodCallInput = RawLegacyMethodCallInputV1>
    + RawFunctionHeaderLookupPortV1
    + MethodCallLoweringPortV1
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
        + RawFunctionHeaderLookupPortV1
        + MethodCallLoweringPortV1
        + LocalStatementDescentPortV1<LocalInput = RawLegacyLocalInputV1>
        + VariableAssignmentDescentPortV1<
            VariableAssignmentInput = RawLegacyVariableAssignmentInputV1,
        > + ReturnStatementDescentPortV1<ReturnInput = RawLegacyValueReturnInputV1>
        + RawBoxMethodChildPortV1
        + RawLoopChildEntryPortV1
{
}

impl super::MirBuilder {
    /// The sole raw AST match tree, parameterized by its child descent.
    pub(in crate::mir::builder) fn build_expression_impl_with_port_v1<Port>(
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
        let ast = match statement_surface::try_build_with_port_v1(self, port, ast)? {
            statement_surface::StatementSurfaceDispatch::Lowered(value) => return Ok(value),
            statement_surface::StatementSurfaceDispatch::RegularExpression(ast) => ast,
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
                let prepared = PreparedRawUnaryV1::prepare(operator, *operand);
                lower_prepared_raw_unary_with_port_v1(self, port, prepared)
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
            } => {
                let prepared = PreparedRawFromCallV1::prepare(self, parent, method, arguments)?;
                self.lower_prepared_raw_from_call_with_port_v1(port, prepared)
            }

            // Phase 152-A: Grouped assignment expression (x = expr)
            // Stage-3 only. Value/type same as rhs, side effect assigns to lhs.
            // Shares the variable-name Assignment descent owner and returns the SSA ValueId.
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                let input = RawLegacyVariableAssignmentInputV1::new(lhs, *rhs);
                drive_variable_assignment_v1(self, port, &input)
            }

            ASTNode::Index { target, index, .. } => {
                let prepared = PreparedRawIndexReadV1::prepare(self, *target, *index)?;
                self.lower_prepared_raw_index_read_with_port_v1(port, prepared)
            }

            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                let prepared = PreparedRawFunctionPreflightV1::prepare(self, name, arguments);
                lower_prepared_raw_function_preflight_with_port_v1(self, port, prepared)
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
            } => {
                let prepared =
                    PreparedRawEnumMatchV1::prepare(self, enum_name, *scrutinee, arms, else_expr)?;
                self.lower_prepared_raw_enum_match_with_port_v1(port, prepared)
            }

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
                    // Main is a root-only entry.  The invocation port rejects
                    // nested Main before any root-main mutation; the legacy
                    // adapter preserves the existing inline-main behavior.
                    port.lower_static_main_box(self, name.clone(), methods.clone())
                } else if is_static {
                    // In App mode, the typed Program root owner lowers static boxes.
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

            ASTNode::FieldAccess { object, field, .. } => {
                let prepared = PreparedRawFieldReadV1::prepare(self, *object, field);
                self.lower_prepared_raw_field_read_with_port_v1(port, prepared)
            }

            ASTNode::New {
                class,
                arguments,
                field_initializers,
                ..
            } => {
                let prepared = PreparedRawNewExpressionV1::prepare(
                    self,
                    class,
                    arguments,
                    field_initializers,
                )?;
                self.lower_prepared_raw_new_expression_with_port_v1(port, prepared)
            }

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
                let prepared = PreparedRawBlockExprV1::prepare(prelude_stmts, *tail_expr)?;
                lower_prepared_raw_block_expr_with_port_v1(self, port, prepared)
            }

            _ => Err(format!("Unsupported AST node type: {:?}", ast)),
        }
    }
}
