//! Body-free source callable header view.
//!
//! This view is deliberately separate from `FunctionSyntaxViewV1`: callable
//! indexing must not gain body traversal authority, while the function/lambda
//! resolver must not grow source-name or physical-symbol policy.

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};

use super::function_view::FunctionSyntaxViewV1;

/// Header/body pair borrowed from one exact function AST.
///
/// The pair prevents callers from combining a header from one declaration
/// with the body of another without widening either bounded view.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CallableFunctionSyntaxViewV1<'a> {
    header: CallableHeaderSyntaxViewV1<'a>,
    function: FunctionSyntaxViewV1<'a>,
}

impl<'a> CallableFunctionSyntaxViewV1<'a> {
    pub(crate) fn from_function_ast(function: &'a ASTNode) -> Option<Self> {
        Some(Self {
            header: CallableHeaderSyntaxViewV1::from_function_ast(function)?,
            function: FunctionSyntaxViewV1::from_ast(function)?,
        })
    }

    pub(crate) const fn header(self) -> CallableHeaderSyntaxViewV1<'a> {
        self.header
    }

    pub(crate) const fn function(self) -> FunctionSyntaxViewV1<'a> {
        self.function
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CallableHeaderSyntaxViewV1<'a> {
    name: &'a str,
    params: &'a [String],
    param_decls: &'a [ParamDecl],
    uses: &'a [String],
    attrs: &'a DeclarationAttrs,
    return_type_name: Option<&'a str>,
    is_static: bool,
    is_override: bool,
    metadata_is_empty: bool,
}

impl<'a> CallableHeaderSyntaxViewV1<'a> {
    pub(crate) fn from_function_ast(function: &'a ASTNode) -> Option<Self> {
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } = function
        else {
            return None;
        };
        Some(Self {
            name,
            params,
            param_decls,
            uses,
            attrs,
            return_type_name: return_type_name.as_deref(),
            is_static: *is_static,
            is_override: *is_override,
            metadata_is_empty: uses.is_empty() && contracts.is_empty() && attrs.is_empty(),
        })
    }

    pub(crate) const fn name(self) -> &'a str {
        self.name
    }

    pub(crate) const fn params(self) -> &'a [String] {
        self.params
    }

    pub(crate) const fn param_decls(self) -> &'a [ParamDecl] {
        self.param_decls
    }

    pub(crate) const fn uses(self) -> &'a [String] {
        self.uses
    }

    pub(crate) const fn attrs(self) -> &'a DeclarationAttrs {
        self.attrs
    }

    pub(crate) const fn return_type_name(self) -> Option<&'a str> {
        self.return_type_name
    }

    pub(crate) const fn is_static(self) -> bool {
        self.is_static
    }

    pub(crate) const fn is_override(self) -> bool {
        self.is_override
    }

    pub(crate) const fn metadata_is_empty(self) -> bool {
        self.metadata_is_empty
    }
}
