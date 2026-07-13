//! Closed statement traversal with lexical scopes and exact control targets.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::source_site::{SourceBindingSiteV1, SourcePathSegmentV1};

use super::path::ShadowSourcePathV0;
use super::product::{
    ShadowBindingKindV0, ShadowControlExitV0, ShadowRegionKindV0, ShadowResolveErrorV0,
    ShadowScopeKindV0,
};
use super::resolver::ShadowResolverV0;
use super::vocabulary::{classify_shadow_ast_disposition_v0, ShadowAstDispositionV0};

impl ShadowResolverV0 {
    pub(super) fn resolve_body<F>(
        &mut self,
        body: &[ASTNode],
        path_for: F,
    ) -> Result<(), ShadowResolveErrorV0>
    where
        F: Fn(usize) -> ShadowSourcePathV0,
    {
        for (index, statement) in body.iter().enumerate() {
            self.resolve_stmt(statement, &path_for(index))?;
        }
        Ok(())
    }

    fn resolve_stmt(
        &mut self,
        statement: &ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        match statement {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => self.resolve_declaration(variables, initial_values, path, false, true),
            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } => self.resolve_declaration(variables, initial_values, path, true, false),
            ASTNode::Assignment { target, value, .. } => {
                self.resolve_assignment_target(target, &path.child(SourcePathSegmentV1::Target))?;
                self.resolve_expr(value, &path.child(SourcePathSegmentV1::Value))
            }
            ASTNode::CompoundAssignment { target, value, .. } => {
                self.resolve_compound_assignment_target(
                    target,
                    &path.child(SourcePathSegmentV1::Target),
                )?;
                self.resolve_expr(value, &path.child(SourcePathSegmentV1::Value))
            }
            ASTNode::Print { expression, .. } => {
                self.resolve_expr(expression, &path.child(SourcePathSegmentV1::Value))
            }
            ASTNode::Nowait {
                variable,
                expression,
                ..
            } => {
                self.resolve_expr(expression, &path.child(SourcePathSegmentV1::Value))?;
                self.declare_binding(
                    variable,
                    ShadowBindingKindV0::Nowait,
                    SourceBindingSiteV1::Nowait {
                        statement: path.stmt(),
                    },
                )?;
                Ok(())
            }
            ASTNode::ScopeBox { body, .. } => self.resolve_scope_box(body, path),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => self.resolve_if(condition, then_body, else_body.as_deref(), path),
            ASTNode::Loop {
                condition, body, ..
            } => self.resolve_loop(condition, body, path),
            ASTNode::Break { .. } => self.resolve_loop_exit(path, false),
            ASTNode::Continue { .. } => self.resolve_loop_exit(path, true),
            ASTNode::Return { value, .. } => {
                if let Some(value) = value {
                    self.resolve_expr(value, &path.child(SourcePathSegmentV1::Value))?;
                }
                self.record_exit(
                    path.stmt(),
                    ShadowControlExitV0::Return {
                        target_function: self.function_region(),
                    },
                );
                Ok(())
            }
            expression if is_closed_expression(expression) => self.resolve_expr(expression, path),
            other => Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: other.node_type(),
                site: path.stmt(),
            }),
        }
    }

    fn resolve_declaration(
        &mut self,
        variables: &[String],
        initial_values: &[Option<Box<ASTNode>>],
        path: &ShadowSourcePathV0,
        outbox: bool,
        resolve_initializers: bool,
    ) -> Result<(), ShadowResolveErrorV0> {
        // Match current Lower semantics: every initializer observes the
        // environment before any binding in this declaration is inserted.
        // Outbox initializers are currently non-semantic compatibility data
        // and are deliberately not observed by Lower or this resolver.
        if resolve_initializers {
            for index in 0..variables.len() {
                let Some(Some(initial)) = initial_values.get(index) else {
                    continue;
                };
                self.resolve_expr(
                    initial,
                    &path.child(SourcePathSegmentV1::Initializer(index as u32)),
                )?;
            }
        }
        for (index, name) in variables.iter().enumerate() {
            let ordinal = index as u32;
            let origin = if outbox {
                SourceBindingSiteV1::Outbox {
                    statement: path.stmt(),
                    ordinal,
                }
            } else {
                SourceBindingSiteV1::Local {
                    statement: path.stmt(),
                    ordinal,
                }
            };
            let kind = if outbox {
                ShadowBindingKindV0::Outbox { ordinal }
            } else {
                ShadowBindingKindV0::Local { ordinal }
            };
            self.declare_binding(name, kind, origin)?;
        }
        Ok(())
    }

    fn resolve_scope_box(
        &mut self,
        body: &[ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let body_path = path.child(SourcePathSegmentV1::ScopeBodyRoot);
        let (region, _) = self.enter_region_scope(
            ShadowRegionKindV0::LexicalScope,
            ShadowScopeKindV0::LexicalBlock,
            &body_path,
        );
        let result = self.resolve_body(body, |index| {
            path.child(SourcePathSegmentV1::ScopeBody(index as u32))
        });
        self.leave_region_scope(region);
        result
    }

    fn resolve_if(
        &mut self,
        condition: &ASTNode,
        then_body: &[ASTNode],
        else_body: Option<&[ASTNode]>,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        self.resolve_expr(condition, &path.child(SourcePathSegmentV1::IfCondition))?;
        let if_region = self.enter_control_region(ShadowRegionKindV0::If, path);

        let then_path = path.child(SourcePathSegmentV1::IfThenBody);
        let (then_region, _) = self.enter_region_scope(
            ShadowRegionKindV0::IfThen,
            ShadowScopeKindV0::IfThen,
            &then_path,
        );
        let then_result = self.resolve_body(then_body, |index| {
            path.child(SourcePathSegmentV1::IfThen(index as u32))
        });
        self.leave_region_scope(then_region);
        then_result?;

        if let Some(else_body) = else_body {
            let else_path = path.child(SourcePathSegmentV1::IfElseBody);
            let (else_region, _) = self.enter_region_scope(
                ShadowRegionKindV0::IfElse,
                ShadowScopeKindV0::IfElse,
                &else_path,
            );
            let else_result = self.resolve_body(else_body, |index| {
                path.child(SourcePathSegmentV1::IfElse(index as u32))
            });
            self.leave_region_scope(else_region);
            else_result?;
        }
        self.leave_control_region(if_region);
        Ok(())
    }

    fn resolve_loop(
        &mut self,
        condition: &ASTNode,
        body: &[ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        self.resolve_expr(condition, &path.child(SourcePathSegmentV1::LoopCondition))?;
        let body_path = path.child(SourcePathSegmentV1::LoopBodyRoot);
        let (loop_region, _) = self.enter_region_scope_with_origins(
            ShadowRegionKindV0::Loop,
            ShadowScopeKindV0::LoopBody,
            path,
            &body_path,
        );
        self.push_loop(loop_region);
        let result = self.resolve_body(body, |index| {
            path.child(SourcePathSegmentV1::LoopBody(index as u32))
        });
        self.pop_loop(loop_region);
        self.leave_region_scope(loop_region);
        result
    }

    fn resolve_loop_exit(
        &mut self,
        path: &ShadowSourcePathV0,
        is_continue: bool,
    ) -> Result<(), ShadowResolveErrorV0> {
        let target = self
            .nearest_loop()
            .ok_or_else(|| ShadowResolveErrorV0::ExitOutsideLoop {
                kind: if is_continue { "Continue" } else { "Break" },
                site: path.stmt(),
            })?;
        let exit = if is_continue {
            ShadowControlExitV0::Continue {
                target_loop: target,
            }
        } else {
            ShadowControlExitV0::Break {
                target_loop: target,
            }
        };
        self.record_exit(path.stmt(), exit);
        Ok(())
    }
}

fn is_closed_expression(node: &ASTNode) -> bool {
    classify_shadow_ast_disposition_v0(node) == ShadowAstDispositionV0::CurrentResolvedExpression
}
