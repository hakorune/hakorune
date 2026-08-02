//! Script-root resolved-demand dispatch for the shared shadow traversal.
//!
//! This module selects the already-issued root responsibility, then delegates
//! every recursive descent to the shared statement and expression traversal.
//! It must not become a Script-specific resolver.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::ExprChildRoleV1;

use super::path::ShadowSourcePathV0;
use super::product::ShadowResolveErrorV0;
use super::resolver::ShadowResolverV0;
use super::script_root_window::ScriptRootResolvedDemandV1;

pub(super) fn dispatch_resolved_script_root_statement<'ast, 'schema>(
    resolver: &mut ShadowResolverV0<'ast, 'schema>,
    statement: &'ast ASTNode,
    path: &ShadowSourcePathV0,
    demand: ScriptRootResolvedDemandV1,
) -> Result<(), ShadowResolveErrorV0> {
    match demand {
        ScriptRootResolvedDemandV1::LexicalCore => resolver.resolve_stmt(statement, path),
        ScriptRootResolvedDemandV1::QMarkPropagation(_) => {
            resolve_qmark_propagation(resolver, statement, path)
        }
        ScriptRootResolvedDemandV1::MatchControl(_) => resolve_match_control(resolver, statement, path),
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
            resolver.resolve_if(statement, condition, then_body, else_body.as_deref(), path)
        }
        ScriptRootResolvedDemandV1::ReturnExit(_) => resolver.resolve_return(statement, path),
        ScriptRootResolvedDemandV1::BindingRebind(_) => {
            resolver.resolve_binding_rebind(statement, path)
        }
        ScriptRootResolvedDemandV1::IndexWrite(_) => resolver.resolve_index_write(statement, path),
    }
}

fn resolve_qmark_propagation<'ast, 'schema>(
    resolver: &mut ShadowResolverV0<'ast, 'schema>,
    statement: &'ast ASTNode,
    path: &ShadowSourcePathV0,
) -> Result<(), ShadowResolveErrorV0> {
    let ASTNode::QMarkPropagate { expression, .. } = statement else {
        return Err(ShadowResolveErrorV0::UnsupportedStatement {
            kind: "Script root QMark admission source drift",
            site: path.stmt(),
        });
    };
    resolver.admit_qmark_propagation(path.expr())?;
    resolver.resolve_expr(
        expression,
        &ShadowResolverV0::stmt_expr_path(statement, path, ExprChildRoleV1::QMarkOperand),
    )
}

fn resolve_match_control<'ast, 'schema>(
    resolver: &mut ShadowResolverV0<'ast, 'schema>,
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
    resolver.admit_match_control(path.expr())?;
    resolver.resolve_expr(
        scrutinee,
        &ShadowResolverV0::stmt_expr_path(statement, path, ExprChildRoleV1::MatchScrutinee),
    )?;
    for (index, (_, expression)) in arms.iter().enumerate() {
        resolver.resolve_expr(
            expression,
            &ShadowResolverV0::stmt_expr_path(
                statement,
                path,
                ExprChildRoleV1::MatchArm(index as u32),
            ),
        )?;
    }
    resolver.resolve_expr(
        else_expr,
        &ShadowResolverV0::stmt_expr_path(statement, path, ExprChildRoleV1::MatchElse),
    )
}
