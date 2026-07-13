//! Closed expression traversal for shadow name resolution.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::source_site::SourcePathSegmentV1;

use super::path::ShadowSourcePathV0;
use super::product::{ShadowAssignmentTargetV0, ShadowLexicalRefV0, ShadowResolveErrorV0};
use super::resolver::ShadowResolverV0;

impl<'ast> ShadowResolverV0<'ast> {
    pub(super) fn resolve_expr(
        &mut self,
        expr: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        match expr {
            ASTNode::Literal { .. } => Ok(()),
            ASTNode::Variable { name, .. } => self.resolve_named_use(name, path),
            ASTNode::Me { .. } => self.resolve_receiver_use(path, "Me"),
            lambda @ ASTNode::Lambda { .. } => self.record_lambda(lambda, path),
            ASTNode::UnaryOp { operand, .. } => {
                self.resolve_expr(operand, &path.child(SourcePathSegmentV1::Operand))
            }
            ASTNode::BinaryOp { left, right, .. } => {
                self.resolve_expr(left, &path.child(SourcePathSegmentV1::Lhs))?;
                self.resolve_expr(right, &path.child(SourcePathSegmentV1::Rhs))
            }
            ASTNode::AwaitExpression { expression, .. } => {
                self.resolve_expr(expression, &path.child(SourcePathSegmentV1::Operand))
            }
            ASTNode::ArrayLiteral { elements, .. } => {
                for (index, element) in elements.iter().enumerate() {
                    self.resolve_expr(
                        element,
                        &path.child(SourcePathSegmentV1::Element(index as u32)),
                    )?;
                }
                Ok(())
            }
            ASTNode::MapLiteral { entries, .. } => {
                for (index, (_, value)) in entries.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &path.child(SourcePathSegmentV1::EntryValue(index as u32)),
                    )?;
                }
                Ok(())
            }
            ASTNode::RecordLiteral { fields, .. } => {
                for (index, (_, value)) in fields.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &path.child(SourcePathSegmentV1::FieldValue(index as u32)),
                    )?;
                }
                Ok(())
            }
            ASTNode::RecordUpdate { base, updates, .. } => {
                self.resolve_expr(base, &path.child(SourcePathSegmentV1::Base))?;
                for (index, (_, value)) in updates.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &path.child(SourcePathSegmentV1::UpdateValue(index as u32)),
                    )?;
                }
                Ok(())
            }
            ASTNode::CheckExpr { items, .. } => {
                for (index, item) in items.iter().enumerate() {
                    self.resolve_expr(
                        &item.expression,
                        &path.child(SourcePathSegmentV1::CheckItem(index as u32)),
                    )?;
                }
                Ok(())
            }
            ASTNode::GroupedAssignmentExpr { lhs, rhs, .. } => {
                self.resolve_expr(rhs, &path.child(SourcePathSegmentV1::Value))?;
                self.resolve_named_assignment(lhs, &path.child(SourcePathSegmentV1::Target))
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                self.resolve_expr(object, &path.child(SourcePathSegmentV1::Receiver))?;
                self.resolve_arguments(arguments, path)
            }
            ASTNode::FieldAccess { object, .. } => {
                self.resolve_expr(object, &path.child(SourcePathSegmentV1::Receiver))
            }
            ASTNode::Index { target, index, .. } => {
                self.resolve_expr(target, &path.child(SourcePathSegmentV1::Target))?;
                self.resolve_expr(index, &path.child(SourcePathSegmentV1::Argument(0)))
            }
            ASTNode::FunctionCall { arguments, .. } => self.resolve_arguments(arguments, path),
            ASTNode::FromCall { arguments, .. } => self.resolve_arguments(arguments, path),
            ASTNode::Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(callee, &path.child(SourcePathSegmentV1::Callee))?;
                self.resolve_arguments(arguments, path)
            }
            ASTNode::New {
                arguments,
                field_initializers,
                ..
            } => {
                self.resolve_arguments(arguments, path)?;
                for (index, (_, value)) in field_initializers.iter().enumerate() {
                    self.resolve_expr(
                        value,
                        &path.child(SourcePathSegmentV1::Initializer(index as u32)),
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
        let resolved = match target {
            ASTNode::Variable { name, .. } => {
                let Some(binding) = self.lookup(name) else {
                    if self.ancestor_is_visible(name) {
                        return Err(ShadowResolveErrorV0::UnsupportedAncestorRebind {
                            name: name.clone().into(),
                            site: target_site,
                        });
                    }
                    return Err(ShadowResolveErrorV0::UnresolvedName {
                        name: name.clone().into(),
                        site: target_site,
                    });
                };
                ShadowAssignmentTargetV0::BindingRebind(binding)
            }
            ASTNode::FieldAccess { object, .. } => {
                let receiver_path = path.child(SourcePathSegmentV1::Receiver);
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
                let receiver_path = path.child(SourcePathSegmentV1::Target);
                self.resolve_expr(receiver, &receiver_path)?;
                self.resolve_expr(index, &path.child(SourcePathSegmentV1::Argument(0)))?;
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

    fn resolve_named_assignment(
        &mut self,
        name: &str,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let site = path.expr();
        let Some(binding) = self.lookup(name) else {
            if self.ancestor_is_visible(name) {
                return Err(ShadowResolveErrorV0::UnsupportedAncestorRebind {
                    name: name.into(),
                    site,
                });
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
            ShadowLexicalRefV0::Ancestor(name.into())
        } else {
            return Err(ShadowResolveErrorV0::UnresolvedName {
                name: name.into(),
                site,
            });
        };
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
            ShadowLexicalRefV0::Ancestor("me".into())
        } else {
            return Err(ShadowResolveErrorV0::UnsupportedExpression { kind, site });
        };
        self.record_use(site, lexical_ref);
        Ok(())
    }

    fn resolve_arguments(
        &mut self,
        arguments: &'ast [ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        for (index, argument) in arguments.iter().enumerate() {
            self.resolve_expr(
                argument,
                &path.child(SourcePathSegmentV1::Argument(index as u32)),
            )?;
        }
        Ok(())
    }
}
