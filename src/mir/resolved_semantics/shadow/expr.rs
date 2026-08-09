//! Closed expression traversal for shadow name resolution.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{ExprChildRoleV1, ReceiverPolicyV1};

use super::path::ShadowSourcePathV0;
use super::product::{
    ShadowAncestorCaptureAccessV0, ShadowAssignmentTargetV0, ShadowLexicalRefV0,
    ShadowMethodCallReceiverV0, ShadowQualifiedReceiverDispositionV0, ShadowResolveErrorV0,
};
use super::resolver::ShadowResolverV0;

impl<'ast, 'schema> ShadowResolverV0<'ast, 'schema> {
    fn expr_child_path(
        parent: &ASTNode,
        path: &ShadowSourcePathV0,
        role: ExprChildRoleV1,
    ) -> ShadowSourcePathV0 {
        path.child(
            role.segment_for(parent)
                .expect("[freeze:contract][source_path/expr_child_role]"),
        )
    }

    pub(super) fn resolve_expr(
        &mut self,
        expr: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        if !self.allows_expression(expr) {
            return Err(ShadowResolveErrorV0::UnsupportedExpression {
                kind: expr.node_type(),
                site: path.expr(),
            });
        }
        self.record_expression_site(path.expr());
        self.record_expression_shape(expr, path.expr());
        match expr {
            ASTNode::Literal { .. } => Ok(()),
            ASTNode::Variable { name, .. } => self.resolve_named_use(name, path),
            ASTNode::Me { .. } => self.resolve_receiver_use(path, "Me"),
            lambda @ ASTNode::Lambda { .. } => self.record_lambda(lambda, path),
            ASTNode::BlockExpr {
                prelude_stmts,
                tail_expr,
                ..
            } => self.resolve_block_expr(expr, prelude_stmts, tail_expr, path),
            ASTNode::UnaryOp { operand, .. } => self.resolve_expr(
                operand,
                &Self::expr_child_path(expr, path, ExprChildRoleV1::UnaryOperand),
            ),
            ASTNode::BinaryOp { left, right, .. } => {
                self.resolve_expr(
                    left,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::BinaryLeft),
                )?;
                self.resolve_expr(
                    right,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::BinaryRight),
                )
            }
            ASTNode::AwaitExpression { expression, .. } => {
                self.record_effect(
                    path.expr(),
                    crate::mir::resolved_semantics::body_shape::BodyEffectKindV1::Await,
                );
                self.resolve_expr(
                    expression,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::AwaitOperand),
                )
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                for (index, element) in elements.iter().enumerate() {
                    self.resolve_expr(
                        element,
                        &Self::expr_child_path(
                            expr,
                            path,
                            ExprChildRoleV1::ArrayElement(index as u32),
                        ),
                    )?;
                }
                Ok(())
            }
            ASTNode::MapLiteral { entries, .. } => {
                for (index, (_, value)) in entries.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &Self::expr_child_path(
                            expr,
                            path,
                            ExprChildRoleV1::MapEntryValue(index as u32),
                        ),
                    )?;
                }
                Ok(())
            }
            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => {
                if self.is_script_lexical_core() {
                    self.admit_record_literal(path.expr(), record_type_name, fields)?;
                }
                for (index, (_, value)) in fields.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &Self::expr_child_path(
                            expr,
                            path,
                            ExprChildRoleV1::RecordFieldValue(index as u32),
                        ),
                    )?;
                }
                Ok(())
            }
            ASTNode::RecordUpdate { base, updates, .. } => {
                self.resolve_expr(
                    base,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::RecordUpdateBase),
                )?;
                for (index, (_, value)) in updates.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &Self::expr_child_path(
                            expr,
                            path,
                            ExprChildRoleV1::RecordUpdateValue(index as u32),
                        ),
                    )?;
                }
                Ok(())
            }
            ASTNode::CheckExpr { items, .. } => {
                for (index, item) in items.iter().enumerate() {
                    self.resolve_expr(
                        &item.expression,
                        &Self::expr_child_path(
                            expr,
                            path,
                            ExprChildRoleV1::CheckItem(index as u32),
                        ),
                    )?;
                }
                Ok(())
            }
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                self.resolve_expr(
                    rhs,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::GroupedAssignmentValue),
                )?;
                self.resolve_named_assignment(
                    lhs,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::GroupedAssignmentTarget),
                )
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                let receiver_path = Self::expr_child_path(expr, path, ExprChildRoleV1::Receiver);
                self.record_effect(
                    path.expr(),
                    crate::mir::resolved_semantics::body_shape::BodyEffectKindV1::Call,
                );
                self.record_relation(
                    path.node(),
                    crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver,
                    receiver_path.expr(),
                );
                self.resolve_method_call_receiver(path.expr(), object, &receiver_path)?;
                self.resolve_arguments(expr, arguments, path)
            }
            ASTNode::FieldAccess { object, .. } => {
                let receiver_path = Self::expr_child_path(expr, path, ExprChildRoleV1::Receiver);
                self.record_relation(
                    path.node(),
                    crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver,
                    receiver_path.expr(),
                );
                self.resolve_expr(object, &receiver_path)
            }
            ASTNode::Index { target, index, .. } => {
                self.resolve_expr(
                    target,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::IndexTarget),
                )?;
                self.resolve_expr(
                    index,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::IndexSubscript),
                )
            }
            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                self.record_effect(
                    path.expr(),
                    crate::mir::resolved_semantics::body_shape::BodyEffectKindV1::Call,
                );
                self.record_direct_call(path.expr(), name, arguments.len())?;
                self.resolve_arguments(expr, arguments, path)
            }
            ASTNode::FromCall {
                parent,
                method,
                arguments,
                ..
            } => {
                if self.is_script_lexical_core() {
                    self.admit_enum_variant(path.expr(), parent, method, arguments)?;
                }
                self.resolve_arguments(expr, arguments, path)
            }
            ASTNode::EnumMatchExpr {
                enum_name,
                scrutinee,
                arms,
                else_expr,
                ..
            } if self.is_script_lexical_core() => self.resolve_direct_enum_match(
                expr,
                enum_name,
                scrutinee,
                arms,
                else_expr.as_deref(),
                path,
            ),
            ASTNode::Call {
                callee, arguments, ..
            } => {
                self.record_effect(
                    path.expr(),
                    crate::mir::resolved_semantics::body_shape::BodyEffectKindV1::Call,
                );
                self.resolve_expr(
                    callee,
                    &Self::expr_child_path(expr, path, ExprChildRoleV1::CallCallee),
                )?;
                self.resolve_arguments(expr, arguments, path)
            }
            ASTNode::New {
                arguments,
                field_initializers,
                ..
            } => {
                self.record_effect(
                    path.expr(),
                    crate::mir::resolved_semantics::body_shape::BodyEffectKindV1::Allocation,
                );
                self.resolve_arguments(expr, arguments, path)?;
                for (index, (_, value)) in field_initializers.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &Self::expr_child_path(
                            expr,
                            path,
                            ExprChildRoleV1::NewFieldInitializer(index as u32),
                        ),
                    )?;
                }
                Ok(())
            }
            other => Err(ShadowResolveErrorV0::UnsupportedExpression {
                kind: other.node_type(),
                site: path.expr(),
            }),
        }
    }

    pub(super) fn resolve_assignment_target(
        &mut self,
        target: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let target_site = path.expr();
        self.record_expression_site(target_site.clone());
        self.record_expression_shape(target, target_site.clone());
        let resolved = match target {
            ASTNode::Variable { name, .. } => {
                let Some(binding) = self.lookup(name) else {
                    if self.ancestor_is_visible(name) {
                        self.record_ancestor_capture(
                            target_site.clone(),
                            name,
                            ShadowAncestorCaptureAccessV0::Rebind,
                        );
                        self.record_assignment(
                            target_site,
                            ShadowAssignmentTargetV0::AncestorRebind(name.clone().into()),
                        );
                        return Ok(());
                    }
                    return Err(ShadowResolveErrorV0::UnresolvedName {
                        name: name.clone().into(),
                        site: target_site,
                    });
                };
                self.invalidate_array_initialized_local(binding);
                ShadowAssignmentTargetV0::BindingRebind(binding)
            }
            ASTNode::FieldAccess { object, .. } => {
                let receiver_path = Self::expr_child_path(target, path, ExprChildRoleV1::Receiver);
                self.resolve_expr(object, &receiver_path)?;
                ShadowAssignmentTargetV0::FieldWrite {
                    receiver: receiver_path.expr(),
                }
            }
            ASTNode::Index {
                target: receiver,
                index,
                ..
            } => {
                let receiver_path =
                    Self::expr_child_path(target, path, ExprChildRoleV1::IndexTarget);
                self.resolve_expr(receiver, &receiver_path)?;
                self.resolve_expr(
                    index,
                    &Self::expr_child_path(target, path, ExprChildRoleV1::IndexSubscript),
                )?;
                ShadowAssignmentTargetV0::IndexWrite {
                    receiver: receiver_path.expr(),
                }
            }
            _ => {
                return Err(ShadowResolveErrorV0::UnsupportedAssignmentTarget {
                    site: target_site,
                });
            }
        };
        self.record_assignment(target_site, resolved);
        Ok(())
    }

    pub(super) fn resolve_compound_assignment_target(
        &mut self,
        target: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        if matches!(target, ASTNode::Variable { .. }) {
            self.resolve_expr(target, path)?;
        }
        self.resolve_assignment_target(target, path)
    }

    fn resolve_method_call_receiver(
        &mut self,
        call_site: crate::mir::resolved_semantics::SourceExprSiteV1,
        object: &'ast ASTNode,
        receiver_path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        if matches!(object, ASTNode::Me { .. })
            && self.receiver_policy() == ReceiverPolicyV1::StaticCurrentOwner
        {
            self.record_expression_site(receiver_path.expr());
            self.record_expression_shape(object, receiver_path.expr());
            if self.observes_all_method_calls() {
                self.record_method_call_observation(
                    call_site,
                    receiver_path.expr(),
                    ShadowMethodCallReceiverV0::CurrentOwner,
                )?;
            }
            return Ok(());
        }

        if !self.observes_all_method_calls() {
            return self.resolve_expr(object, receiver_path);
        }

        let receiver_site = receiver_path.expr();
        let receiver = match object {
            ASTNode::Variable { .. } => {
                self.request_qualified_receiver(receiver_site.clone());
                self.resolve_expr(object, receiver_path)?;
                ShadowMethodCallReceiverV0::Qualified(
                    self.qualified_receiver_disposition(&receiver_site)
                        .expect("[freeze:contract][shadow/method_call_qualified_observation]"),
                )
            }
            ASTNode::Me { .. } => {
                self.resolve_expr(object, receiver_path)?;
                ShadowMethodCallReceiverV0::CurrentOwner
            }
            _ => {
                self.resolve_expr(object, receiver_path)?;
                ShadowMethodCallReceiverV0::Dynamic
            }
        };
        self.record_method_call_observation(call_site, receiver_site, receiver)
    }

    fn resolve_named_assignment(
        &mut self,
        name: &str,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let site = path.expr();
        let Some(binding) = self.lookup(name) else {
            if self.ancestor_is_visible(name) {
                self.record_ancestor_capture(
                    site.clone(),
                    name,
                    ShadowAncestorCaptureAccessV0::Rebind,
                );
                self.record_assignment(site, ShadowAssignmentTargetV0::AncestorRebind(name.into()));
                return Ok(());
            }
            return Err(ShadowResolveErrorV0::UnresolvedName {
                name: name.into(),
                site,
            });
        };
        self.record_assignment(site, ShadowAssignmentTargetV0::BindingRebind(binding));
        Ok(())
    }

    fn resolve_named_use(
        &mut self,
        name: &str,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let site = path.expr();
        let lexical_ref = if let Some(binding) = self.lookup(name) {
            ShadowLexicalRefV0::Local(binding)
        } else if self.ancestor_is_visible(name) {
            self.record_ancestor_capture(site.clone(), name, ShadowAncestorCaptureAccessV0::Read);
            ShadowLexicalRefV0::Ancestor(name.into())
        } else if self.qualified_receiver_is_requested(&site) {
            self.record_qualified_receiver_disposition(
                site,
                ShadowQualifiedReceiverDispositionV0::ProvenUnbound,
            )?;
            return Ok(());
        } else {
            return Err(ShadowResolveErrorV0::UnresolvedName {
                name: name.into(),
                site,
            });
        };
        if self.qualified_receiver_is_requested(&site) {
            self.record_qualified_receiver_disposition(
                site.clone(),
                ShadowQualifiedReceiverDispositionV0::Bound,
            )?;
        }
        self.record_use(site, lexical_ref);
        Ok(())
    }

    fn resolve_receiver_use(
        &mut self,
        path: &ShadowSourcePathV0,
        kind: &'static str,
    ) -> Result<(), ShadowResolveErrorV0> {
        let site = path.expr();
        let lexical_ref = if let Some(binding) = self.receiver() {
            ShadowLexicalRefV0::Local(binding)
        } else if self.ancestor_is_visible("me") {
            self.record_ancestor_capture(site.clone(), "me", ShadowAncestorCaptureAccessV0::Read);
            ShadowLexicalRefV0::Ancestor("me".into())
        } else {
            return Err(ShadowResolveErrorV0::UnsupportedExpression { kind, site });
        };
        self.record_use(site, lexical_ref);
        Ok(())
    }

    fn resolve_arguments(
        &mut self,
        parent: &'ast ASTNode,
        arguments: &'ast [ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        for (index, argument) in arguments.iter().enumerate() {
            self.resolve_expr(
                argument,
                &Self::expr_child_path(parent, path, ExprChildRoleV1::CallArgument(index as u32)),
            )?;
        }
        Ok(())
    }
}
