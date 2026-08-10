//! Exact source-site and syntax-view lookup for the legacy callable semantic owner.
//!
//! This is behavior-preserving cutover support.  It does not issue resolver
//! identity, repair source coordinates, or select a callable family.  The
//! final callable semantic package will retire this lookup together with the
//! Builder-owned second resolver authority.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{FunctionSyntaxViewV1, ReceiverPolicyV1};

use super::callable_declaration_catalog::{
    SameModuleCallableNamespaceV1, SelectedNormalCallableKeyV1, SelectedNormalCallableSourceSiteV1,
};

pub(super) fn function_at_site<'source>(
    statements: &'source [ASTNode],
    key: &SelectedNormalCallableKeyV1,
    site: &SelectedNormalCallableSourceSiteV1,
) -> Result<&'source ASTNode, String> {
    let function = match site {
        SelectedNormalCallableSourceSiteV1::ProgramFunction { statement_index } => {
            statements.get(*statement_index)
        }
        SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
            statement_index,
            method_key,
        } => match statements.get(*statement_index) {
            Some(ASTNode::BoxDeclaration { methods, .. }) => {
                methods.get_declaration(method_key.as_ref())
            }
            _ => None,
        },
    }
    .ok_or_else(|| "[freeze:contract][mir/callable-semantic/source-site]".to_owned())?;
    if !matches!(function, ASTNode::FunctionDeclaration { .. }) {
        return Err("[freeze:contract][mir/callable-semantic/source-kind]".to_owned());
    }
    match (key, site) {
        (
            SelectedNormalCallableKeyV1::TopLevel(_),
            SelectedNormalCallableSourceSiteV1::ProgramFunction { .. },
        )
        | (
            SelectedNormalCallableKeyV1::Cataloged(_),
            SelectedNormalCallableSourceSiteV1::ProgramBoxMethod { .. },
        ) => Ok(function),
        _ => Err("[freeze:contract][mir/callable-semantic/key-site]".to_owned()),
    }
}

pub(super) fn view_for_key<'source>(
    function: &'source ASTNode,
    key: &SelectedNormalCallableKeyV1,
) -> Result<FunctionSyntaxViewV1<'source>, String> {
    let ASTNode::FunctionDeclaration { params, body, .. } = function else {
        unreachable!("function_at_site checked the kind")
    };
    let receiver = match key {
        SelectedNormalCallableKeyV1::TopLevel(_) => ReceiverPolicyV1::Absent,
        SelectedNormalCallableKeyV1::Cataloged(key) => match key.namespace() {
            SameModuleCallableNamespaceV1::StaticBoxMethod => ReceiverPolicyV1::StaticCurrentOwner,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => ReceiverPolicyV1::DeclaredInstance,
        },
    };
    Ok(FunctionSyntaxViewV1::from_borrowed_function_parts(
        params, body, receiver,
    ))
}
