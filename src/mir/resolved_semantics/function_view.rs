//! Borrowed canonical function syntax seam used before Lower decomposes it.

use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionBodyOriginV1 {
    Function,
    Lambda,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ReceiverPolicyV1 {
    DeclaredInstance,
    /// Observation-only current owner for one catalog-verified static-box
    /// method. This never declares a lexical `me` binding.
    StaticCurrentOwner,
    Absent,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FunctionSyntaxViewV1<'a> {
    params: &'a [String],
    body: &'a [ASTNode],
    receiver_policy: ReceiverPolicyV1,
    body_origin: FunctionBodyOriginV1,
}

impl<'a> FunctionSyntaxViewV1<'a> {
    pub(crate) fn from_ast(function: &'a ASTNode) -> Option<Self> {
        let ASTNode::FunctionDeclaration {
            params,
            body,
            is_static,
            ..
        } = function
        else {
            return None;
        };
        Some(Self {
            params,
            body,
            receiver_policy: if *is_static {
                ReceiverPolicyV1::Absent
            } else {
                ReceiverPolicyV1::DeclaredInstance
            },
            body_origin: FunctionBodyOriginV1::Function,
        })
    }

    pub(crate) fn from_lambda_ast(lambda: &'a ASTNode) -> Option<Self> {
        let ASTNode::Lambda { params, body, .. } = lambda else {
            return None;
        };
        Some(Self {
            params,
            body,
            receiver_policy: ReceiverPolicyV1::Absent,
            body_origin: FunctionBodyOriginV1::Lambda,
        })
    }

    /// Borrows the canonical function view from a catalog-owned declaration.
    ///
    /// The caller supplies an already-classified receiver policy. This view
    /// performs no namespace, name, or callable-identity inference.
    pub(in crate::mir) const fn from_borrowed_function_parts(
        params: &'a [String],
        body: &'a [ASTNode],
        receiver_policy: ReceiverPolicyV1,
    ) -> Self {
        Self {
            params,
            body,
            receiver_policy,
            body_origin: FunctionBodyOriginV1::Function,
        }
    }

    pub(crate) const fn params(self) -> &'a [String] {
        self.params
    }

    pub(crate) const fn body(self) -> &'a [ASTNode] {
        self.body
    }

    pub(crate) const fn receiver_policy(self) -> ReceiverPolicyV1 {
        self.receiver_policy
    }

    pub(crate) const fn body_origin(self) -> FunctionBodyOriginV1 {
        self.body_origin
    }
}
