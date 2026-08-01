//! Closed statement traversal with lexical scopes and exact control targets.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, SourceBindingSiteV1};

use super::path::ShadowSourcePathV0;
use super::product::{
    ShadowBindingKindV0, ShadowControlExitV0, ShadowExitOriginV0, ShadowRegionKindV0,
    ShadowResolveErrorV0, ShadowScopeKindV0,
};
use super::resolver::ShadowResolverV0;
use super::script_root_window::ScriptRootResolvedDemandV1;
use super::vocabulary::{classify_shadow_ast_disposition_v0, ShadowAstDispositionV0};

impl<'ast, 'schema> ShadowResolverV0<'ast, 'schema> {
    pub(super) fn resolve_root_statement(
        &mut self,
        statement: &'ast ASTNode,
        path: &ShadowSourcePathV0,
        demand: ScriptRootResolvedDemandV1,
    ) -> Result<(), ShadowResolveErrorV0> {
        match demand {
            ScriptRootResolvedDemandV1::LexicalCore => self.resolve_stmt(statement, path),
            ScriptRootResolvedDemandV1::QMarkPropagation(_) => {
                self.resolve_qmark_propagation(statement, path)
            }
            ScriptRootResolvedDemandV1::MatchControl(_) => {
                self.resolve_match_control(statement, path)
            }
            ScriptRootResolvedDemandV1::IfControl(_) => {
                let ASTNode::If {
                    condition,
                    then_body,
                    else_body,
                    ..
                } = statement
                else {
                    return Err(ShadowResolveErrorV0::UnsupportedStatement {
                        kind: "Script root If admission source drift",
                        site: path.stmt(),
                    });
                };
                self.resolve_if(statement, condition, then_body, else_body.as_deref(), path)
            }
            ScriptRootResolvedDemandV1::ReturnExit(_) => self.resolve_return(statement, path),
            ScriptRootResolvedDemandV1::BindingRebind(_) => {
                self.resolve_binding_rebind(statement, path)
            }
        }
    }

    fn resolve_qmark_propagation(
        &mut self,
        statement: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let ASTNode::QMarkPropagate { expression, .. } = statement else {
            return Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: "Script root QMark admission source drift",
                site: path.stmt(),
            });
        };
        self.admit_qmark_propagation(path.expr())?;
        self.resolve_expr(
            expression,
            &Self::stmt_expr_path(statement, path, ExprChildRoleV1::QMarkOperand),
        )
    }

    fn resolve_match_control(
        &mut self,
        statement: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } = statement
        else {
            return Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: "Script root Match admission source drift",
                site: path.stmt(),
            });
        };
        self.admit_match_control(path.expr())?;
        self.resolve_expr(
            scrutinee,
            &Self::stmt_expr_path(statement, path, ExprChildRoleV1::MatchScrutinee),
        )?;
        for (index, (_, expression)) in arms.iter().enumerate() {
            self.resolve_expr(
                expression,
                &Self::stmt_expr_path(statement, path, ExprChildRoleV1::MatchArm(index as u32)),
            )?;
        }
        self.resolve_expr(
            else_expr,
            &Self::stmt_expr_path(statement, path, ExprChildRoleV1::MatchElse),
        )
    }

    fn stmt_expr_path(
        statement: &ASTNode,
        path: &ShadowSourcePathV0,
        role: ExprChildRoleV1,
    ) -> ShadowSourcePathV0 {
        path.child(
            role.segment_for(statement)
                .expect("[freeze:contract][source_path/stmt_expr_role]"),
        )
    }

    fn stmt_body_root_path(
        statement: &ASTNode,
        path: &ShadowSourcePathV0,
        role: BodyChildRoleV1,
    ) -> ShadowSourcePathV0 {
        let kind = role
            .kind_for(statement)
            .expect("[freeze:contract][source_path/stmt_body_role]");
        path.child(
            kind.root_segment()
                .expect("[freeze:contract][source_path/stmt_body_root]"),
        )
    }

    fn stmt_body_item_path(
        statement: &ASTNode,
        path: &ShadowSourcePathV0,
        role: BodyChildRoleV1,
        index: usize,
    ) -> ShadowSourcePathV0 {
        let kind = role
            .kind_for(statement)
            .expect("[freeze:contract][source_path/stmt_body_role]");
        path.child(kind.item_segment(index as u32))
    }

    pub(super) fn resolve_body<F>(
        &mut self,
        body: &'ast [ASTNode],
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
        statement: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        if !self.allows_statement(statement) {
            return Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: statement.node_type(),
                site: path.stmt(),
            });
        }
        match statement {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => self.resolve_declaration(statement, variables, initial_values, path, false, true),
            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } => self.resolve_declaration(statement, variables, initial_values, path, true, false),
            ASTNode::Assignment { target, value, .. } => {
                self.resolve_assignment_target(
                    target,
                    &Self::stmt_expr_path(statement, path, ExprChildRoleV1::AssignmentTarget),
                )?;
                self.resolve_expr(
                    value,
                    &Self::stmt_expr_path(statement, path, ExprChildRoleV1::AssignmentValue),
                )
            }
            ASTNode::CompoundAssignment { target, value, .. } => {
                self.resolve_compound_assignment_target(
                    target,
                    &Self::stmt_expr_path(
                        statement,
                        path,
                        ExprChildRoleV1::CompoundAssignmentTarget,
                    ),
                )?;
                self.resolve_expr(
                    value,
                    &Self::stmt_expr_path(
                        statement,
                        path,
                        ExprChildRoleV1::CompoundAssignmentValue,
                    ),
                )
            }
            ASTNode::Print { expression, .. } => self.resolve_expr(
                expression,
                &Self::stmt_expr_path(statement, path, ExprChildRoleV1::PrintValue),
            ),
            ASTNode::Nowait {
                variable,
                expression,
                ..
            } => {
                self.resolve_expr(
                    expression,
                    &Self::stmt_expr_path(statement, path, ExprChildRoleV1::NowaitValue),
                )?;
                self.declare_binding(
                    variable,
                    ShadowBindingKindV0::Nowait,
                    SourceBindingSiteV1::Nowait {
                        statement: path.stmt(),
                    },
                )?;
                Ok(())
            }
            ASTNode::ScopeBox { body, .. } => self.resolve_scope_box(statement, body, path),
            ASTNode::TaskScope { body, .. } => self.resolve_task_scope(statement, body, path),
            ASTNode::FastMemRegion { body, .. } => {
                self.resolve_fastmem_scope(statement, body, path)
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => self.resolve_if(statement, condition, then_body, else_body.as_deref(), path),
            ASTNode::Loop {
                condition, body, ..
            } => self.resolve_loop(statement, condition, body, path),
            ASTNode::Break { .. } => self.resolve_loop_exit(path, false),
            ASTNode::Continue { .. } => self.resolve_loop_exit(path, true),
            ASTNode::Return { .. } => self.resolve_return(statement, path),
            expression if is_closed_expression(expression) => self.resolve_expr(expression, path),
            other => Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: other.node_type(),
                site: path.stmt(),
            }),
        }
    }

    fn resolve_return(
        &mut self,
        statement: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let ASTNode::Return { value, .. } = statement else {
            return Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: "Script root Return admission source drift",
                site: path.stmt(),
            });
        };
        if let Some(value) = value {
            self.resolve_expr(
                value,
                &Self::stmt_expr_path(statement, path, ExprChildRoleV1::ReturnValue),
            )?;
        }
        self.record_exit(
            path.stmt(),
            ShadowExitOriginV0::ExplicitReturn,
            ShadowControlExitV0::Return {
                target_function: self.function_region(),
            },
        )
    }

    fn resolve_binding_rebind(
        &mut self,
        statement: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let has_variable_target = matches!(
            statement,
            ASTNode::Assignment { target, .. } | ASTNode::CompoundAssignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. })
        );
        if !has_variable_target {
            return Err(ShadowResolveErrorV0::UnsupportedStatement {
                kind: "Script root BindingRebind admission source drift",
                site: path.stmt(),
            });
        }
        self.resolve_stmt(statement, path)
    }

    fn resolve_declaration(
        &mut self,
        statement: &'ast ASTNode,
        variables: &[String],
        initial_values: &'ast [Option<Box<ASTNode>>],
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
                    &Self::stmt_expr_path(
                        statement,
                        path,
                        ExprChildRoleV1::LocalInitializer(index as u32),
                    ),
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
        statement: &'ast ASTNode,
        body: &'ast [ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let body_path = Self::stmt_body_root_path(statement, path, BodyChildRoleV1::ScopeBody);
        let (region, _) = self.enter_region_scope(
            ShadowRegionKindV0::LexicalScope,
            ShadowScopeKindV0::LexicalBlock,
            &body_path,
        );
        let result = self.resolve_body(body, |index| {
            Self::stmt_body_item_path(statement, path, BodyChildRoleV1::ScopeBody, index)
        });
        self.leave_region_scope(region);
        result
    }

    fn resolve_task_scope(
        &mut self,
        statement: &'ast ASTNode,
        body: &'ast [ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let body_path = Self::stmt_body_root_path(statement, path, BodyChildRoleV1::TaskScopeBody);
        let (region, _) = self.enter_region_scope(
            ShadowRegionKindV0::LexicalScope,
            ShadowScopeKindV0::LexicalBlock,
            &body_path,
        );
        let result = self.resolve_body(body, |index| {
            Self::stmt_body_item_path(statement, path, BodyChildRoleV1::TaskScopeBody, index)
        });
        self.leave_region_scope(region);
        result
    }

    fn resolve_fastmem_scope(
        &mut self,
        statement: &'ast ASTNode,
        body: &'ast [ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        let body_path = Self::stmt_body_root_path(statement, path, BodyChildRoleV1::FastMemBody);
        let (region, _) = self.enter_region_scope(
            ShadowRegionKindV0::LexicalScope,
            ShadowScopeKindV0::LexicalBlock,
            &body_path,
        );
        let result = self.resolve_body(body, |index| {
            Self::stmt_body_item_path(statement, path, BodyChildRoleV1::FastMemBody, index)
        });
        self.leave_region_scope(region);
        result
    }

    fn resolve_if(
        &mut self,
        statement: &'ast ASTNode,
        condition: &'ast ASTNode,
        then_body: &'ast [ASTNode],
        else_body: Option<&'ast [ASTNode]>,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        self.resolve_expr(
            condition,
            &Self::stmt_expr_path(statement, path, ExprChildRoleV1::IfCondition),
        )?;
        let if_region = self.enter_control_region(ShadowRegionKindV0::If, path);
        let result = (|| {
            let then_path = Self::stmt_body_root_path(statement, path, BodyChildRoleV1::IfThen);
            let (then_region, _) = self.enter_region_scope(
                ShadowRegionKindV0::IfThen,
                ShadowScopeKindV0::IfThen,
                &then_path,
            );
            let then_result = self.resolve_body(then_body, |index| {
                Self::stmt_body_item_path(statement, path, BodyChildRoleV1::IfThen, index)
            });
            self.leave_region_scope(then_region);
            then_result?;

            if let Some(else_body) = else_body {
                let else_path = Self::stmt_body_root_path(statement, path, BodyChildRoleV1::IfElse);
                let (else_region, _) = self.enter_region_scope(
                    ShadowRegionKindV0::IfElse,
                    ShadowScopeKindV0::IfElse,
                    &else_path,
                );
                let else_result = self.resolve_body(else_body, |index| {
                    Self::stmt_body_item_path(statement, path, BodyChildRoleV1::IfElse, index)
                });
                self.leave_region_scope(else_region);
                else_result?;
            }
            Ok(())
        })();
        self.leave_control_region(if_region);
        result
    }

    fn resolve_loop(
        &mut self,
        statement: &'ast ASTNode,
        condition: &'ast ASTNode,
        body: &'ast [ASTNode],
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        self.resolve_expr(
            condition,
            &Self::stmt_expr_path(statement, path, ExprChildRoleV1::LoopCondition),
        )?;
        let body_path = Self::stmt_body_root_path(statement, path, BodyChildRoleV1::LoopBody);
        let (loop_region, _) = self.enter_region_scope_with_origins(
            ShadowRegionKindV0::Loop,
            ShadowScopeKindV0::LoopBody,
            path,
            &body_path,
        );
        self.push_loop(loop_region);
        let result = self.resolve_body(body, |index| {
            Self::stmt_body_item_path(statement, path, BodyChildRoleV1::LoopBody, index)
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
        let (origin, transfer) = if is_continue {
            (
                ShadowExitOriginV0::ExplicitContinue,
                ShadowControlExitV0::Continue {
                    target_loop: target,
                },
            )
        } else {
            (
                ShadowExitOriginV0::ExplicitBreak,
                ShadowControlExitV0::Break {
                    target_loop: target,
                },
            )
        };
        self.record_exit(path.stmt(), origin, transfer)
    }
}

fn is_closed_expression(node: &ASTNode) -> bool {
    classify_shadow_ast_disposition_v0(node) == ShadowAstDispositionV0::CurrentResolvedExpression
}
