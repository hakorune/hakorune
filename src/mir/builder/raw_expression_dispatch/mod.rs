//! Raw AST expression dispatcher.
//!
//! RAWPORT0 keeps exactly one AST match tree here. Every remaining caller
//! supplies its child port directly; do not add a second matcher or facade.
mod block_expr;
mod input_view;
mod nonmain_static_box_lifecycle;
mod statement_surface;
mod static_box_state;
#[cfg(test)]
mod tests;

pub(in crate::mir::builder) use input_view::{
    RawBodyInputViewV1, RawLegacyBodyInputV1, RawLegacyStatementInputV1, RawStatementInputViewV1,
};

pub(in crate::mir::builder) fn unsupported_raw_ast_node_error_v1(ast: &ASTNode) -> String {
    format!("Unsupported AST node type: {:?}", ast)
}

use self::block_expr::{lower_prepared_raw_block_expr_with_port_v1, PreparedRawBlockExprV1};
pub(in crate::mir::builder) use self::nonmain_static_box_lifecycle::PreparedRawNonMainStaticBoxLifecycleV1;
use super::builder_build::PreparedRawNewExpressionV1;
use super::calls::{
    lower_prepared_raw_function_preflight_with_port_v1, MethodCallDescentPortV1,
    PreparedRawFromCallV1, PreparedRawFunctionPreflightV1, RawLegacyMethodCallInputV1,
};
use super::exprs_enum_match::PreparedRawEnumMatchV1;
use super::fields::PreparedRawFieldReadV1;
use super::indexing::PreparedRawIndexReadV1;
use super::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use super::me_call_header_observation::MethodCallLoweringPortV1;
use super::ops::{
    drive_ordinary_binary_expression_v1, drive_short_circuit_expression_v1,
    lower_prepared_raw_unary_with_port_v1, BinaryExpressionDescentPortV1, PreparedRawUnaryV1,
    RawLegacyBinaryInputV1, RawLegacyShortCircuitInputV1, ShortCircuitExpressionDescentPortV1,
};
use super::raw_structured_child_scope::RawStructuredChildScopePortV1;
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
use crate::mir::resolved_semantics::ExprChildRoleV1;

pub(in crate::mir::builder) fn reject_sync_box_lowering_v1(name: &str) -> String {
    format!(
        "[freeze:contract][mir_builder/sync_box_lowering_missing] box={name} \
         sync box serialized runtime behavior is owned by CONC-SYNCBOX-003"
    )
}

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
                let left_source =
                    port.prepare_expression_child_source_v1(&node, ExprChildRoleV1::BinaryLeft)?;
                let right_source =
                    port.prepare_expression_child_source_v1(&node, ExprChildRoleV1::BinaryRight)?;
                // Use BinaryExpr for clear destructuring (no behavior change)
                let e = BinaryExpr::try_from(node).expect("ASTNode::BinaryOp must convert");
                let left = *e.left;
                let right = *e.right;
                let mut scoped =
                    RawStructuredChildScopePortV1::new(port, vec![left_source, right_source], Vec::new());
                let result = match e.operator {
                    operator @ (crate::ast::BinaryOperator::And
                    | crate::ast::BinaryOperator::Or) => {
                        let input = RawLegacyShortCircuitInputV1::new(left, operator, right);
                        drive_short_circuit_expression_v1(self, &mut scoped, &input)
                    }
                    operator => {
                        let input = RawLegacyBinaryInputV1::new(left, operator, right);
                        drive_ordinary_binary_expression_v1(self, &mut scoped, &input)
                    }
                };
                scoped.complete_exact_demands_v1()?;
                result
            }

            node @ ASTNode::CheckExpr { .. } => {
                let sources = match &node {
                    ASTNode::CheckExpr { items, .. } => items
                        .iter()
                        .enumerate()
                        .map(|(index, _)| {
                            port.prepare_expression_child_source_v1(
                                &node,
                                ExprChildRoleV1::CheckItem(index as u32),
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                    _ => unreachable!("check match arm retains its AST shape"),
                };
                let items = match node {
                    ASTNode::CheckExpr { items, .. } => items,
                    _ => unreachable!("check match arm retains its AST shape"),
                };
                self.build_check_expression_with_port_v1(port, items, sources)
            }

            node @ ASTNode::UnaryOp { .. } => {
                let source =
                    port.prepare_expression_child_source_v1(&node, ExprChildRoleV1::UnaryOperand)?;
                let ASTNode::UnaryOp { operator, operand, .. } = node else {
                    unreachable!("unary match arm retains its AST shape");
                };
                let prepared = PreparedRawUnaryV1::prepare(operator, *operand);
                port.with_prepared_child_source_v1(source, |port| {
                    lower_prepared_raw_unary_with_port_v1(self, port, prepared)
                })
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
            node @ ASTNode::GroupedAssignmentExpr { .. } => {
                let value_source = port.prepare_expression_child_source_v1(
                    &node,
                    ExprChildRoleV1::GroupedAssignmentValue,
                )?;
                let ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } = node else {
                    unreachable!("matched GroupedAssignmentExpr")
                };
                let input = RawLegacyVariableAssignmentInputV1::new(lhs, *rhs);
                let mut scoped =
                    RawStructuredChildScopePortV1::new(port, vec![value_source], Vec::new());
                let value = drive_variable_assignment_v1(self, &mut scoped, &input)?;
                scoped.complete_exact_demands_v1()?;
                Ok(value)
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
            } => self.build_indirect_call_expression_with_port_v1(port, *callee, arguments),

            ASTNode::QMarkPropagate { expression, .. } => {
                self.build_qmark_propagate_expression_with_port_v1(port, *expression)
            }

            node @ ASTNode::MatchExpr { .. } => {
                use crate::mir::resolved_semantics::ExprChildRoleV1;

                let arm_count = match &node {
                    ASTNode::MatchExpr { arms, .. } => arms.len(),
                    _ => unreachable!(),
                };
                let mut sources = Vec::with_capacity(arm_count + 2);
                sources.push(
                    port.prepare_expression_child_source_v1(
                        &node,
                        ExprChildRoleV1::MatchScrutinee,
                    )?,
                );
                for index in 0..arm_count {
                    sources.push(port.prepare_expression_child_source_v1(
                        &node,
                        ExprChildRoleV1::MatchArm(index as u32),
                    )?);
                }
                sources.push(
                    port.prepare_expression_child_source_v1(
                        &node,
                        ExprChildRoleV1::MatchElse,
                    )?,
                );
                let ASTNode::MatchExpr {
                    scrutinee,
                    arms,
                    else_expr,
                    ..
                } = node
                else {
                    unreachable!()
                };
                let mut scoped =
                    super::raw_structured_child_scope::RawStructuredChildScopePortV1::new(
                        port,
                        sources,
                        Vec::new(),
                    );
                self.build_peek_expression_with_port_v1(
                    &mut scoped,
                    *scrutinee,
                    arms,
                    *else_expr,
                )
            }

            node @ ASTNode::EnumMatchExpr { .. } => {
                use crate::mir::resolved_semantics::ExprChildRoleV1;

                let source = port.prepare_expression_child_source_v1(
                    &node,
                    ExprChildRoleV1::EnumMatchScrutinee,
                )?;
                let ASTNode::EnumMatchExpr {
                    enum_name,
                    scrutinee,
                    arms,
                    else_expr,
                    ..
                } = node
                else {
                    unreachable!()
                };
                let prepared =
                    PreparedRawEnumMatchV1::prepare(self, enum_name, *scrutinee, arms, else_expr)?;
                let mut scoped =
                    super::raw_structured_child_scope::RawStructuredChildScopePortV1::new(
                        port,
                        vec![source],
                        Vec::new(),
                    );
                self.lower_prepared_raw_enum_match_with_port_v1(&mut scoped, prepared)
            }

            ASTNode::Lambda { params, body, .. } => {
                super::raw_lambda_capture_lifecycle::PreparedRawLambdaLexicalCaptureLifecycleV1::prepare(
                    params, body,
                )?
                .lower_with_builder_v1(self)
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
                    return Err(reject_sync_box_lowering_v1(&name));
                }
                if is_static && name == "Main" {
                    // Main is a root-only entry.  The invocation port rejects
                    // nested Main before any root-main mutation; the legacy
                    // adapter preserves the existing inline-main behavior.
                    port.lower_static_main_box(self, name.clone(), methods.clone())
                } else if is_static {
                    PreparedRawNonMainStaticBoxLifecycleV1::prepare(name, methods)
                        .lower_with_port_v1(self, port)
                } else {
                    // Instance box: register type and lower instance methods/ctors as functions
                    // Phase 285LLVM-1.1: Register with field information for LLVM harness
                    PreparedInstanceBoxDeclarationLifecycleV1::prepare(
                        &name,
                        &methods,
                        &fields,
                        &field_decls,
                        &constructors,
                        &init_fields,
                        &weak_fields,
                    )
                    .lower_raw_with_port_v1(self, port)?;
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

            node @ ASTNode::ArrayLiteral { .. } => {
                let element_count = match &node {
                    ASTNode::ArrayLiteral { elements, .. } => elements.len(),
                    _ => unreachable!("array match arm retains its AST shape"),
                };
                let sources = (0..element_count)
                    .map(|index| {
                        port.prepare_expression_child_source_v1(
                            &node,
                            ExprChildRoleV1::ArrayElement(index as u32),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let ASTNode::ArrayLiteral { elements, .. } = node else {
                    unreachable!("array match arm retains its AST shape")
                };
                let mut scoped = super::raw_structured_child_scope::RawStructuredChildScopePortV1::new(
                    port,
                    sources,
                    Vec::new(),
                );
                let value = self.build_array_literal_with_port_v1(&mut scoped, elements)?;
                scoped.complete_exact_demands_v1()?;
                Ok(value)
            }
            ASTNode::MapLiteral { entries, .. } => {
                self.build_map_literal_with_port_v1(port, entries)
            }

            node @ ASTNode::AwaitExpression { .. } => {
                let source =
                    port.prepare_expression_child_source_v1(&node, ExprChildRoleV1::AwaitOperand)?;
                let ASTNode::AwaitExpression { expression, .. } = node else {
                    unreachable!("await match arm retains its AST shape");
                };
                port.with_prepared_child_source_v1(source, |port| {
                    super::stmts::async_stmt::build_await_expression_with_port_v1(
                        self,
                        port,
                        *expression,
                    )
                })
            }

            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => self.build_record_literal_value_with_port_v1(port, record_type_name, fields),
            ASTNode::RecordUpdate { base, updates, .. } => {
                self.build_record_update_value_with_port_v1(port, *base, updates)
            }

            node @ ASTNode::BlockExpr { .. } => {
                use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
                let prelude = port.prepare_body_child_source_v1(
                    &node,
                    BodyChildRoleV1::BlockExprPrelude,
                )?;
                let tail =
                    port.prepare_expression_child_source_v1(&node, ExprChildRoleV1::BlockExprTail)?;
                let ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    ..
                } = node
                else {
                    unreachable!()
                };
                let prepared = PreparedRawBlockExprV1::prepare(prelude_stmts, *tail_expr)?;
                let mut scoped = super::raw_structured_child_scope::
                    RawStructuredChildScopePortV1::for_block_expression(port, prelude, tail);
                lower_prepared_raw_block_expr_with_port_v1(self, &mut scoped, prepared)
            }

            _ => Err(unsupported_raw_ast_node_error_v1(&ast)),
        }
    }
}
